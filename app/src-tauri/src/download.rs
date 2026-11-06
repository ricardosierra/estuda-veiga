use reqwest::{Client, StatusCode, header};
use thiserror::Error;
use tokio::sync::watch;
use url::Url;

const MAX_PDF_BYTES: usize = 60 * 1024 * 1024;
const MAX_REDIRECTS: usize = 4;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DownloadValidators {
    pub host: String,
    pub etag: Option<String>,
    pub last_modified: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DownloadLimits {
    pub max_bytes: usize,
    pub max_redirects: usize,
    pub timeout_seconds: usize,
}

impl Default for DownloadLimits {
    fn default() -> Self {
        Self {
            max_bytes: MAX_PDF_BYTES,
            max_redirects: MAX_REDIRECTS,
            timeout_seconds: 45,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DownloadOutcome {
    NotModified {
        final_url: Url,
        etag: Option<String>,
        last_modified: Option<String>,
    },
    Downloaded {
        bytes: Vec<u8>,
        final_url: Url,
        etag: Option<String>,
        last_modified: Option<String>,
    },
}

#[derive(Debug, Error)]
pub enum DownloadError {
    #[error("URL bloqueada: use HTTPS e um endereço oficial UVA/Canvas")]
    UnsafeUrl,
    #[error("o servidor redirecionou muitas vezes")]
    TooManyRedirects,
    #[error("resposta HTTP inesperada ({0})")]
    Http(StatusCode),
    #[error("arquivo maior que o limite local de 60 MiB")]
    TooLarge,
    #[error("limites de captura inválidos")]
    InvalidLimits,
    #[error("o endereço não entregou um PDF válido")]
    NotPdf,
    #[error("importação cancelada")]
    Cancelled,
    #[error("falha de rede: {0}")]
    Network(String),
}

pub fn validate_material_url(raw: &str) -> Result<Url, DownloadError> {
    let url = Url::parse(raw).map_err(|_| DownloadError::UnsafeUrl)?;
    if url.scheme() != "https"
        || !url.username().is_empty()
        || url.password().is_some()
        || url.port().is_some_and(|port| port != 443)
    {
        return Err(DownloadError::UnsafeUrl);
    }
    let Some(host) = url.host_str().map(str::to_ascii_lowercase) else {
        return Err(DownloadError::UnsafeUrl);
    };
    if !is_official_uva_host(&host) && !is_signed_s3_url(&url) {
        return Err(DownloadError::UnsafeUrl);
    }
    Ok(url)
}

fn is_official_uva_host(host: &str) -> bool {
    host == "uva.instructure.com" || host == "uva.br" || host.ends_with(".uva.br")
}

fn is_signed_s3_url(url: &Url) -> bool {
    let Some(host) = url.host_str() else {
        return false;
    };
    let labels = host.split('.').collect::<Vec<_>>();
    if labels.len() < 3 || labels[labels.len() - 2..] != ["amazonaws", "com"] {
        return false;
    }
    let service_labels = &labels[..labels.len() - 2];
    let Some(s3_index) = service_labels.iter().position(|label| *label == "s3") else {
        return false;
    };
    let bucket_labels = &service_labels[..s3_index];
    if bucket_labels.iter().any(|label| {
        label.is_empty()
            || !label
                .bytes()
                .all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'-')
    }) {
        return false;
    }
    let endpoint_labels = &service_labels[s3_index + 1..];
    let endpoint_valid = match endpoint_labels {
        [] => true,
        [region] => valid_region(region),
        [dualstack, region] => *dualstack == "dualstack" && valid_region(region),
        _ => false,
    };
    if !endpoint_valid {
        return false;
    }
    let query = url
        .query_pairs()
        .map(|(key, _)| key.to_ascii_lowercase())
        .collect::<std::collections::HashSet<_>>();
    let signature_v2 = query.contains("signature")
        && query.contains("awsaccesskeyid")
        && query.contains("expires");
    let signature_v4 = query.contains("x-amz-signature")
        && query.contains("x-amz-credential")
        && query.contains("x-amz-date");
    signature_v2 || signature_v4
}

fn valid_region(value: &str) -> bool {
    !value.is_empty()
        && value
            .bytes()
            .all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'-')
}

async fn wait_for_cancellation(cancellation: &mut watch::Receiver<bool>) {
    while !*cancellation.borrow() {
        if cancellation.changed().await.is_err() {
            break;
        }
    }
}

fn safe_validator(value: Option<&str>) -> Option<&str> {
    value
        .map(str::trim)
        .filter(|value| !value.is_empty() && value.len() <= 500)
        .filter(|value| value.bytes().all(|byte| (0x20..=0x7e).contains(&byte)))
}

fn response_validator(headers: &header::HeaderMap, name: header::HeaderName) -> Option<String> {
    headers
        .get(name)
        .and_then(|value| value.to_str().ok())
        .and_then(|value| safe_validator(Some(value)))
        .map(str::to_owned)
}

fn validators_for_host<'a>(
    current_host: &str,
    validators: Option<&'a DownloadValidators>,
    crossed_host: bool,
) -> (Option<&'a str>, Option<&'a str>) {
    if crossed_host {
        return (None, None);
    }
    let Some(validators) =
        validators.filter(|validators| validators.host.eq_ignore_ascii_case(current_host))
    else {
        return (None, None);
    };
    (
        safe_validator(validators.etag.as_deref()),
        safe_validator(validators.last_modified.as_deref()),
    )
}

fn valid_pdf_response(content_type: &str, path: &str, bytes: &[u8]) -> bool {
    if !bytes.starts_with(b"%PDF-") {
        return false;
    }
    let media_type = content_type
        .split(';')
        .next()
        .unwrap_or_default()
        .trim()
        .to_ascii_lowercase();
    media_type == "application/pdf"
        || (media_type == "application/octet-stream" && path.to_ascii_lowercase().ends_with(".pdf"))
}

fn not_modified_outcome(
    status: StatusCode,
    final_url: Url,
    headers: &header::HeaderMap,
) -> Option<DownloadOutcome> {
    (status == StatusCode::NOT_MODIFIED).then(|| DownloadOutcome::NotModified {
        final_url,
        etag: response_validator(headers, header::ETAG),
        last_modified: response_validator(headers, header::LAST_MODIFIED),
    })
}

pub async fn download_pdf(
    raw_url: &str,
    referer: Option<&str>,
    cookie: Option<(&str, &str)>,
    validators: Option<&DownloadValidators>,
    limits: &DownloadLimits,
    cancellation: &mut watch::Receiver<bool>,
) -> Result<DownloadOutcome, DownloadError> {
    if limits.max_bytes == 0
        || limits.max_bytes > MAX_PDF_BYTES
        || limits.max_redirects > MAX_REDIRECTS
        || !(5..=120).contains(&limits.timeout_seconds)
    {
        return Err(DownloadError::InvalidLimits);
    }
    if *cancellation.borrow() {
        return Err(DownloadError::Cancelled);
    }
    let client = Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .timeout(std::time::Duration::from_secs(
            limits.timeout_seconds as u64,
        ))
        .build()
        .map_err(|_| DownloadError::Network("não foi possível criar a conexão protegida".into()))?;
    let mut current = validate_material_url(raw_url)?;
    let initial_host = current.host_str().unwrap_or_default().to_owned();
    let mut crossed_host = false;
    for hop in 0..=limits.max_redirects {
        if *cancellation.borrow() {
            return Err(DownloadError::Cancelled);
        }
        let mut request = client.get(current.clone());
        if let Some(mut referer) = referer.and_then(|value| Url::parse(value).ok()) {
            if referer.host_str() == current.host_str() {
                referer.set_query(None);
                referer.set_fragment(None);
                request = request.header(header::REFERER, referer.as_str());
            }
        }
        if let Some((cookie_host, value)) = cookie {
            if !crossed_host
                && cookie_host.eq_ignore_ascii_case(&initial_host)
                && current
                    .host_str()
                    .is_some_and(|host| host.eq_ignore_ascii_case(cookie_host))
                && !value.is_empty()
            {
                request = request.header(header::COOKIE, value);
            }
        }
        let (etag, last_modified) = validators_for_host(
            current.host_str().unwrap_or_default(),
            validators,
            crossed_host,
        );
        if let Some(etag) = etag {
            if let Ok(value) = header::HeaderValue::from_str(etag) {
                request = request.header(header::IF_NONE_MATCH, value);
            }
        }
        if let Some(last_modified) = last_modified {
            if let Ok(value) = header::HeaderValue::from_str(last_modified) {
                request = request.header(header::IF_MODIFIED_SINCE, value);
            }
        }
        let response = tokio::select! {
            _ = wait_for_cancellation(cancellation) => return Err(DownloadError::Cancelled),
            result = request.send() => result
                .map_err(|_| DownloadError::Network("a conexão foi interrompida".into()))?,
        };
        if let Some(outcome) = not_modified_outcome(
            response.status(),
            sanitize_final_url(current.clone())?,
            response.headers(),
        ) {
            return Ok(outcome);
        }
        if response.status().is_redirection() {
            if hop == limits.max_redirects {
                return Err(DownloadError::TooManyRedirects);
            }
            let location = response
                .headers()
                .get(header::LOCATION)
                .and_then(|value| value.to_str().ok())
                .ok_or(DownloadError::UnsafeUrl)?;
            let next = current
                .join(location)
                .map_err(|_| DownloadError::UnsafeUrl)?;
            let next = validate_material_url(next.as_str())?;
            if current.host_str() != next.host_str() {
                crossed_host = true;
            }
            current = next;
            continue;
        }
        if !response.status().is_success() {
            return Err(DownloadError::Http(response.status()));
        }
        if response
            .content_length()
            .is_some_and(|length| length > limits.max_bytes as u64)
        {
            return Err(DownloadError::TooLarge);
        }
        let content_type = response
            .headers()
            .get(header::CONTENT_TYPE)
            .and_then(|value| value.to_str().ok())
            .unwrap_or("")
            .to_ascii_lowercase();
        let etag = response_validator(response.headers(), header::ETAG);
        let last_modified = response_validator(response.headers(), header::LAST_MODIFIED);
        let mut response = response;
        let mut bytes = Vec::new();
        loop {
            let chunk = tokio::select! {
                _ = wait_for_cancellation(cancellation) => return Err(DownloadError::Cancelled),
                result = response.chunk() => result
                    .map_err(|_| DownloadError::Network("a resposta foi interrompida".into()))?,
            };
            let Some(chunk) = chunk else {
                break;
            };
            if bytes.len().saturating_add(chunk.len()) > limits.max_bytes {
                return Err(DownloadError::TooLarge);
            }
            bytes.extend_from_slice(&chunk);
        }
        if !valid_pdf_response(&content_type, current.path(), &bytes) {
            return Err(DownloadError::NotPdf);
        }
        return Ok(DownloadOutcome::Downloaded {
            bytes,
            final_url: sanitize_final_url(current)?,
            etag,
            last_modified,
        });
    }
    Err(DownloadError::TooManyRedirects)
}

fn sanitize_final_url(mut url: Url) -> Result<Url, DownloadError> {
    if validate_material_url(url.as_str()).is_err() {
        return Err(DownloadError::UnsafeUrl);
    }
    url.set_query(None);
    url.set_fragment(None);
    Ok(url)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_only_presigned_amazon_s3_storage_outside_uva() {
        assert!(validate_material_url(
            "https://uva-files.s3.us-east-1.amazonaws.com/course/lesson.pdf?X-Amz-Algorithm=AWS4-HMAC-SHA256&X-Amz-Credential=key%2Fscope&X-Amz-Date=20261009T120000Z&X-Amz-Signature=signature"
        )
        .is_ok());
        assert!(validate_material_url(
            "https://example-bucket.s3.amazonaws.com/file.pdf?AWSAccessKeyId=key&Expires=123&Signature=signature"
        )
        .is_ok());
        assert!(
            validate_material_url("https://uva-files.s3.amazonaws.com/course/lesson.pdf").is_err()
        );
        assert!(validate_material_url(
            "https://uva-files.s3.amazonaws.com.evil.example/course/lesson.pdf?AWSAccessKeyId=key&Expires=123&Signature=signature"
        )
        .is_err());
        assert!(validate_material_url(
            "https://uva-files.s3.us-east-1.amazonaws.com.evil.example/course/lesson.pdf?X-Amz-Signature=x&X-Amz-Credential=y&X-Amz-Date=z"
        )
        .is_err());
    }

    #[test]
    fn conditional_get_returns_not_modified_without_pdf_bytes() {
        let mut headers = header::HeaderMap::new();
        headers.insert(header::ETAG, header::HeaderValue::from_static("\"v1\""));
        headers.insert(
            header::LAST_MODIFIED,
            header::HeaderValue::from_static("Wed, 14 Oct 2026 10:00:00 GMT"),
        );
        let outcome = not_modified_outcome(
            StatusCode::NOT_MODIFIED,
            Url::parse("https://uva.instructure.com/courses/1/files/2").unwrap(),
            &headers,
        )
        .unwrap();
        assert_eq!(
            outcome,
            DownloadOutcome::NotModified {
                final_url: Url::parse("https://uva.instructure.com/courses/1/files/2").unwrap(),
                etag: Some("\"v1\"".into()),
                last_modified: Some("Wed, 14 Oct 2026 10:00:00 GMT".into()),
            }
        );
    }

    #[test]
    fn conditional_headers_are_not_forwarded_across_hosts() {
        let validators = DownloadValidators {
            host: "uva.instructure.com".into(),
            etag: Some("\"private-validator\"".into()),
            last_modified: Some("Wed, 14 Oct 2026 10:00:00 GMT".into()),
        };
        assert_eq!(
            validators_for_host("uva.instructure.com", Some(&validators), false),
            (
                Some("\"private-validator\""),
                Some("Wed, 14 Oct 2026 10:00:00 GMT")
            )
        );
        assert_eq!(
            validators_for_host("uva.instructure.com", Some(&validators), true),
            (None, None)
        );
        assert_eq!(
            validators_for_host("files.uva.br", Some(&validators), false),
            (None, None)
        );
    }

    #[test]
    fn pdf_download_rejects_invalid_type_and_signature() {
        assert!(!valid_pdf_response("text/html", "/aula.pdf", b"%PDF-1.7"));
        assert!(!valid_pdf_response(
            "application/pdf",
            "/aula.pdf",
            b"not a pdf"
        ));
        assert!(valid_pdf_response(
            "application/pdf",
            "/download",
            b"%PDF-1.7"
        ));
        assert!(valid_pdf_response(
            "application/octet-stream",
            "/aula.pdf",
            b"%PDF-1.7"
        ));
        assert!(!valid_pdf_response(
            "application/octet-stream",
            "/download",
            b"%PDF-1.7"
        ));
    }

    #[test]
    fn pdf_redirect_revalidates_each_host() {
        assert!(validate_material_url("https://evil.example/aula.pdf").is_err());
        assert!(validate_material_url(
            "https://uva-files.s3.us-east-1.amazonaws.com/aula.pdf?X-Amz-Algorithm=AWS4-HMAC-SHA256&X-Amz-Credential=key%2Fscope&X-Amz-Date=20261015T120000Z&X-Amz-Signature=sig"
        )
        .is_ok());
    }

    #[tokio::test]
    async fn cancellation_stops_before_a_network_request_starts() {
        let (sender, mut cancellation) = watch::channel(true);
        let result = download_pdf(
            "https://uva.instructure.com/courses/1/files/2/download",
            None,
            None,
            None,
            &DownloadLimits::default(),
            &mut cancellation,
        )
        .await;
        assert!(matches!(result, Err(DownloadError::Cancelled)));
        drop(sender);
    }
}
