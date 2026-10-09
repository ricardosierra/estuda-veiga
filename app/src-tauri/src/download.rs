use reqwest::{Client, StatusCode, header};
use thiserror::Error;
use tokio::sync::watch;
use url::Url;

const MAX_PDF_BYTES: usize = 60 * 1024 * 1024;
const MAX_REDIRECTS: usize = 4;

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

pub async fn download_pdf(
    raw_url: &str,
    referer: Option<&str>,
    cookie: Option<(&str, &str)>,
    cancellation: &mut watch::Receiver<bool>,
) -> Result<(Vec<u8>, Url), DownloadError> {
    if *cancellation.borrow() {
        return Err(DownloadError::Cancelled);
    }
    let client = Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .timeout(std::time::Duration::from_secs(45))
        .build()
        .map_err(|_| DownloadError::Network("não foi possível criar a conexão protegida".into()))?;
    let mut current = validate_material_url(raw_url)?;
    for hop in 0..=MAX_REDIRECTS {
        let mut request = client.get(current.clone());
        if let Some(mut referer) = referer.and_then(|value| Url::parse(value).ok()) {
            if referer.host_str() == current.host_str() {
                referer.set_query(None);
                referer.set_fragment(None);
                request = request.header(header::REFERER, referer.as_str());
            }
        }
        if let Some((cookie_host, value)) = cookie {
            if current.host_str() == Some(cookie_host) && !value.is_empty() {
                request = request.header(header::COOKIE, value);
            }
        }
        let response = tokio::select! {
            _ = wait_for_cancellation(cancellation) => return Err(DownloadError::Cancelled),
            result = request.send() => result
                .map_err(|_| DownloadError::Network("a conexão foi interrompida".into()))?,
        };
        if response.status().is_redirection() {
            if hop == MAX_REDIRECTS {
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
            current = validate_material_url(next.as_str())?;
            continue;
        }
        if !response.status().is_success() {
            return Err(DownloadError::Http(response.status()));
        }
        if response
            .content_length()
            .is_some_and(|length| length > MAX_PDF_BYTES as u64)
        {
            return Err(DownloadError::TooLarge);
        }
        let content_type = response
            .headers()
            .get(header::CONTENT_TYPE)
            .and_then(|value| value.to_str().ok())
            .unwrap_or("")
            .to_ascii_lowercase();
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
            if bytes.len().saturating_add(chunk.len()) > MAX_PDF_BYTES {
                return Err(DownloadError::TooLarge);
            }
            bytes.extend_from_slice(&chunk);
        }
        if !bytes.starts_with(b"%PDF-") {
            return Err(DownloadError::NotPdf);
        }
        if !content_type.contains("pdf")
            && !current.path().to_ascii_lowercase().ends_with(".pdf")
            && bytes.len() < 8
        {
            return Err(DownloadError::NotPdf);
        }
        return Ok((bytes, current));
    }
    Err(DownloadError::TooManyRedirects)
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

    #[tokio::test]
    async fn cancellation_stops_before_a_network_request_starts() {
        let (sender, mut cancellation) = watch::channel(true);
        let result = download_pdf(
            "https://uva.instructure.com/courses/1/files/2/download",
            None,
            None,
            &mut cancellation,
        )
        .await;
        assert!(matches!(result, Err(DownloadError::Cancelled)));
        drop(sender);
    }
}
