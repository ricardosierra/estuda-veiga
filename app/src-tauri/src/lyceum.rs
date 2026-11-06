use scraper::{Html, Selector};
use serde::{Deserialize, Serialize};
use std::{
    fs::{self, OpenOptions},
    io::Write,
    path::{Component, Path},
    time::{SystemTime, UNIX_EPOCH},
};
use unicode_normalization::UnicodeNormalization;
use url::Url;

pub const PORTAL_START_URL: &str = "https://portalaluno.uva.br/Aluno/PortalAluno";
pub const MAX_PORTAL_PAGES: usize = 50;
const MAX_TABLE_ROWS: usize = 10_000;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum PortalState {
    Disconnected,
    WaitingForLogin,
    Authenticated,
    Partial,
    Error,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PortalStatus {
    pub state: PortalState,
    pub message: String,
    pub current_url: Option<String>,
    pub pages_read: u32,
    pub inaccessible_sections: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CapturedTable {
    pub caption: Option<String>,
    pub headers: Vec<String>,
    pub rows: Vec<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CapturedField {
    pub label: String,
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CapturedLink {
    pub label: String,
    pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CapturedPage {
    pub source_url: String,
    pub title: String,
    pub headings: Vec<String>,
    pub fields: Vec<CapturedField>,
    pub tables: Vec<CapturedTable>,
    pub links: Vec<CapturedLink>,
    pub captured_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PortalCapture {
    pub captured_at: String,
    pub pages_read: u32,
    pub pages: Vec<CapturedPage>,
    pub inaccessible_sections: Vec<String>,
    pub warnings: Vec<String>,
    pub complete: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub json_path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub markdown_path: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VisibleLink {
    pub label: String,
    pub href: String,
    pub context: String,
    pub rel: String,
}

#[derive(Debug, Clone)]
pub struct ParsedPage {
    pub page: CapturedPage,
    pub targets: Vec<String>,
    pub warnings: Vec<String>,
    pub truncated: bool,
}

pub fn sanitize_source_url(value: &str) -> Option<String> {
    let mut parsed = Url::parse(value).ok()?;
    if parsed.scheme() != "https" || !is_official_host(parsed.host_str()?) {
        return None;
    }
    parsed.set_username("").ok()?;
    parsed.set_password(None).ok()?;
    parsed.set_query(None);
    parsed.set_fragment(None);
    Some(parsed.to_string())
}

fn is_official_host(host: &str) -> bool {
    host.eq_ignore_ascii_case("portalaluno.uva.br")
        || host.eq_ignore_ascii_case("sis.lyceum.com.br")
        || host.to_ascii_lowercase().ends_with(".uva.br")
}

fn is_authentication_route(value: &str) -> bool {
    let route = normalized(value.split('?').next().unwrap_or(value));
    route
        .split(|character: char| !character.is_ascii_alphanumeric())
        .any(|segment| matches!(segment, "login" | "sso" | "autologin"))
}

pub fn navigation_target(base: &str, href: &str) -> Option<String> {
    let base = Url::parse(base).ok()?;
    let mut target = base.join(href).ok()?;
    if target.scheme() != "https"
        || !is_official_host(target.host_str()?)
        || is_authentication_route(target.as_str())
    {
        return None;
    }
    target.set_username("").ok()?;
    target.set_password(None).ok()?;
    let safe_query = target
        .query_pairs()
        .filter(|(key, value)| is_safe_pagination_pair(key, value))
        .map(|(key, value)| (key.into_owned(), value.into_owned()))
        .collect::<Vec<_>>();
    target.set_query(None);
    if !safe_query.is_empty() {
        target.query_pairs_mut().extend_pairs(safe_query);
    }
    if let Some(fragment) = target.fragment().map(str::to_owned) {
        let (route, fragment_query) = fragment.split_once('?').unwrap_or((&fragment, ""));
        let safe_fragment_query = url::form_urlencoded::parse(fragment_query.as_bytes())
            .filter(|(key, value)| is_safe_pagination_pair(key, value))
            .collect::<Vec<_>>();
        let safe_fragment = if safe_fragment_query.is_empty() {
            route.to_owned()
        } else {
            let mut serializer = url::form_urlencoded::Serializer::new(String::new());
            serializer.extend_pairs(
                safe_fragment_query
                    .iter()
                    .map(|(key, value)| (key.as_ref(), value.as_ref())),
            );
            format!("{route}?{}", serializer.finish())
        };
        target.set_fragment(Some(&safe_fragment));
    }
    if sanitize_source_url(target.as_str()).is_none() {
        return None;
    }
    Some(target.to_string())
}

fn is_safe_pagination_pair(key: &str, value: &str) -> bool {
    matches!(
        key.to_ascii_lowercase().as_str(),
        "page" | "pagina" | "pag" | "p" | "offset" | "per_page"
    ) && value.len() <= 8
        && value.bytes().all(|byte| byte.is_ascii_digit())
}

pub fn is_read_only_navigation(link: &VisibleLink) -> bool {
    if is_authentication_route(&link.href) {
        return false;
    }
    let label = normalized(&link.label);
    let context = normalized(&link.context);
    let href = normalized(&link.href);
    let pagination = link.rel.eq_ignore_ascii_case("next")
        || context.contains("pagin")
            && (label.contains("proxima")
                || label.contains("anterior")
                || label.chars().all(|ch| ch.is_ascii_digit()));
    if pagination {
        return true;
    }
    let action_words = [
        "solicitar",
        "trancar",
        "cancelar",
        "matricular",
        "inscrever",
        "alterar",
        "editar",
        "excluir",
        "financeiro",
        "boleto",
        "pagamento",
        "perfil",
        "sair",
        "logout",
        "protocolo",
    ];
    if action_words
        .iter()
        .any(|word| label.contains(word) || href.contains(word))
    {
        return false;
    }
    let academic_words = [
        "academ",
        "historico",
        "nota",
        "frequencia",
        "falta",
        "matriz",
        "curriculo",
        "curso",
        "disciplina",
        "boletim",
        "desempenho",
        "avaliacao",
        "rendimento",
        "servico",
    ];
    academic_words
        .iter()
        .any(|word| label.contains(word) || context.contains(word) || href.contains(word))
}

pub fn parse_page(
    source_url: &str,
    html: &str,
    visible_links: &[VisibleLink],
    captured_at: &str,
) -> Result<ParsedPage, String> {
    let source_url = sanitize_source_url(source_url)
        .ok_or_else(|| "a página não pertence a um domínio oficial UVA permitido".to_owned())?;
    let document = Html::parse_document(html);
    let title_selector = selector("title")?;
    let heading_selector = selector("h1, h2, h3, h4")?;
    let title = document
        .select(&title_selector)
        .next()
        .map(|node| clean_text(&node.text().collect::<String>()))
        .filter(|value| !value.is_empty())
        .unwrap_or_else(|| "Página acadêmica".to_owned());
    let headings = document
        .select(&heading_selector)
        .map(|node| clean_text(&node.text().collect::<String>()))
        .filter(|value| !value.is_empty())
        .take(200)
        .collect::<Vec<_>>();
    let fields = parse_fields(&document)?;
    let (tables, tables_truncated) = parse_tables(&document)?;
    let mut warnings = Vec::new();
    if tables_truncated {
        warnings.push(format!(
            "Uma ou mais tabelas de {} ultrapassaram o limite de {MAX_TABLE_ROWS} linhas.",
            title
        ));
    }
    let mut links = Vec::new();
    let mut targets = Vec::new();
    for link in visible_links.iter().take(2_000) {
        if !is_read_only_navigation(link) {
            continue;
        }
        let Some(target) = navigation_target(&source_url, &link.href) else {
            continue;
        };
        let label = if is_sensitive_label(&link.label) {
            "Identificador omitido".to_owned()
        } else {
            clean_text(&link.label)
        };
        links.push(CapturedLink {
            label,
            url: sanitize_source_url(&target).unwrap_or_default(),
        });
        targets.push(target);
    }
    targets.sort();
    targets.dedup();
    Ok(ParsedPage {
        page: CapturedPage {
            source_url,
            title,
            headings,
            fields,
            tables,
            links,
            captured_at: captured_at.to_owned(),
        },
        targets,
        warnings,
        truncated: tables_truncated,
    })
}

pub fn persist_capture(
    root: &Path,
    sync: &crate::sync::SyncEngine,
    mut capture: PortalCapture,
) -> Result<PortalCapture, String> {
    let capture_id = uuid::Uuid::new_v4();
    let base = format!("materias/catalogo/capturas/lyceum-{capture_id}");
    let json_path = format!("{base}.json");
    let markdown_path = format!("{base}.md");
    capture.json_path = Some(json_path.clone());
    capture.markdown_path = Some(markdown_path.clone());

    let json = serde_json::to_vec_pretty(&capture).map_err(|error| error.to_string())?;
    let markdown = render_markdown(&capture);
    if json.len() > 64 * 1024 * 1024 || markdown.len() as u64 > 64 * 1024 * 1024 {
        return Err("A captura excedeu o limite local de 64 MB e não foi salva.".into());
    }

    write_new_file_beneath(root, &json_path, &json)?;
    write_new_file_beneath(root, &markdown_path, markdown.as_bytes()).map_err(|error| {
        format!(
            "O JSON da captura foi preservado em {json_path}; o Markdown ficou pendente: {error}"
        )
    })?;

    let report = sync.reconcile_all()?;
    let capture_errors = report
        .errors
        .iter()
        .filter(|error| error.starts_with(&json_path) || error.starts_with(&markdown_path))
        .cloned()
        .collect::<Vec<_>>();
    if !capture_errors.is_empty() {
        return Err(format!(
            "JSON e Markdown foram preservados; a sincronização local ficou pendente: {}",
            capture_errors.join("; ")
        ));
    }
    if sync.read_content(&json_path)?.is_none() || sync.read_content(&markdown_path)?.is_none() {
        return Err(
            "JSON e Markdown foram preservados, mas a captura ainda não aparece no SQLite local."
                .into(),
        );
    }
    Ok(capture)
}

fn capture_to_markdown(capture: &PortalCapture) -> String {
    let mut output = format!(
        "# Captura acadêmica do Portal UVA\n\n- Capturada em: {}\n- Páginas lidas: {}\n- Cobertura: {}\n",
        md_text(&capture.captured_at),
        capture.pages_read,
        if capture.complete {
            "completa"
        } else {
            "parcial"
        },
    );
    if !capture.inaccessible_sections.is_empty() {
        output.push_str("\n## Seções indisponíveis\n");
        for section in &capture.inaccessible_sections {
            output.push_str(&format!("- {}\n", md_text(section)));
        }
    }
    if !capture.warnings.is_empty() {
        output.push_str("\n## Avisos\n");
        for warning in &capture.warnings {
            output.push_str(&format!("- {}\n", md_text(warning)));
        }
    }
    for (page_index, page) in capture.pages.iter().enumerate() {
        output.push_str(&format!(
            "\n## {}\n\n- Fonte: {}\n- Capturada em: {}\n",
            md_text(&page.title),
            page.source_url,
            md_text(&page.captured_at),
        ));
        if !page.headings.is_empty() {
            output.push_str("\n### Títulos da página\n");
            for heading in &page.headings {
                output.push_str(&format!("- {}\n", md_text(heading)));
            }
        }
        if !page.fields.is_empty() {
            output.push_str("\n### Campos\n\n| Campo | Valor |\n| --- | --- |\n");
            for field in &page.fields {
                output.push_str(&format!(
                    "| {} | {} |\n",
                    md_cell(&field.label),
                    md_cell(&field.value)
                ));
            }
        }
        for (table_index, table) in page.tables.iter().enumerate() {
            output.push_str(&format!(
                "\n### {}\n\n",
                table
                    .caption
                    .as_deref()
                    .map(md_text)
                    .filter(|value| !value.is_empty())
                    .unwrap_or_else(|| format!("Tabela {}", table_index + 1))
            ));
            output.push('|');
            for header in &table.headers {
                output.push_str(&format!(" {} |", md_cell(header)));
            }
            output.push_str("\n|");
            for _ in &table.headers {
                output.push_str(" --- |");
            }
            output.push('\n');
            for row in &table.rows {
                output.push('|');
                for cell in row {
                    output.push_str(&format!(" {} |", md_cell(cell)));
                }
                output.push('\n');
            }
        }
        if !page.links.is_empty() {
            output.push_str("\n### Links acadêmicos visíveis\n");
            for link in &page.links {
                output.push_str(&format!("- [{}]({})\n", md_text(&link.label), link.url));
            }
        }
        if page_index + 1 < capture.pages.len() {
            output.push('\n');
        }
    }
    output
}

pub(crate) fn write_new_file_beneath(
    root: &Path,
    relative_path: &str,
    contents: &[u8],
) -> Result<(), String> {
    let relative = Path::new(relative_path);
    if relative.is_absolute()
        || relative
            .components()
            .any(|component| !matches!(component, Component::Normal(_)))
    {
        return Err("O caminho da captura local é inválido.".into());
    }
    let root = root
        .canonicalize()
        .map_err(|error| format!("Não foi possível acessar o acervo: {error}"))?;
    let parent_relative = relative
        .parent()
        .ok_or_else(|| "O caminho da captura é inválido.".to_owned())?;
    let mut parent = root.clone();
    for component in parent_relative.components() {
        let Component::Normal(name) = component else {
            return Err("O caminho da captura local é inválido.".into());
        };
        let next = parent.join(name);
        match fs::symlink_metadata(&next) {
            Ok(metadata) if metadata.file_type().is_symlink() || !metadata.is_dir() => {
                return Err("A pasta de captura contém um link ou item incompatível.".into());
            }
            Ok(_) => {}
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
                fs::create_dir(&next).map_err(|error| {
                    format!("Não foi possível criar a pasta de captura: {error}")
                })?;
            }
            Err(error) => {
                return Err(format!(
                    "Não foi possível preparar a pasta de captura: {error}"
                ));
            }
        }
        parent = next
            .canonicalize()
            .map_err(|error| format!("Não foi possível validar a pasta de captura: {error}"))?;
        if !parent.starts_with(&root) {
            return Err("A pasta de captura está fora do acervo configurado.".into());
        }
    }
    let name = relative
        .file_name()
        .ok_or_else(|| "O nome da captura é inválido.".to_owned())?;
    let path = parent.join(name);
    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&path)
        .map_err(|error| {
            if error.kind() == std::io::ErrorKind::AlreadyExists {
                "A captura já existe; o arquivo anterior foi preservado.".to_owned()
            } else {
                format!("Não foi possível criar o arquivo local de captura: {error}")
            }
        })?;
    if let Err(error) = file.write_all(contents).and_then(|()| file.sync_all()) {
        drop(file);
        let _ = fs::remove_file(&path);
        return Err(format!(
            "Não foi possível concluir o arquivo local de captura: {error}"
        ));
    }
    Ok(())
}

pub fn login_required(current_url: Option<&str>, html: &str) -> bool {
    let page_text = normalized(html);
    current_url.is_some_and(|value| {
        let value = normalized(value);
        value.contains("/login") || value.contains("/sso")
    }) || current_url.is_some_and(is_authentication_route)
        || page_text.contains("type=\"password\"")
        || page_text.contains("name=\"password\"")
        || page_text.contains("id=\"button-login\"")
        || page_text.contains("sessao expirou")
}

pub fn render_markdown(capture: &PortalCapture) -> String {
    capture_to_markdown(capture)
}

pub fn captured_at_utc() -> String {
    let seconds = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    let days = (seconds / 86_400) as i64;
    let time_of_day = seconds % 86_400;
    let (year, month, day) = civil_from_days(days);
    format!(
        "{year:04}-{month:02}-{day:02}T{:02}:{:02}:{:02}Z",
        time_of_day / 3_600,
        (time_of_day / 60) % 60,
        time_of_day % 60
    )
}

fn civil_from_days(days: i64) -> (i64, i64, i64) {
    let z = days + 719_468;
    let era = if z >= 0 { z } else { z - 146_096 } / 146_097;
    let day_of_era = z - era * 146_097;
    let year_of_era =
        (day_of_era - day_of_era / 1_460 + day_of_era / 36_524 - day_of_era / 146_096) / 365;
    let mut year = year_of_era + era * 400;
    let day_of_year = day_of_era - (365 * year_of_era + year_of_era / 4 - year_of_era / 100);
    let month_parameter = (5 * day_of_year + 2) / 153;
    let day = day_of_year - (153 * month_parameter + 2) / 5 + 1;
    let month = month_parameter + if month_parameter < 10 { 3 } else { -9 };
    year += i64::from(month <= 2);
    (year, month, day)
}

fn parse_fields(document: &Html) -> Result<Vec<CapturedField>, String> {
    let dl_selector = selector("dl")?;
    let dt_selector = selector("dt")?;
    let dd_selector = selector("dd")?;
    let mut fields = Vec::new();
    for list in document.select(&dl_selector) {
        let labels = list.select(&dt_selector).collect::<Vec<_>>();
        let values = list.select(&dd_selector).collect::<Vec<_>>();
        for (label, value) in labels.into_iter().zip(values) {
            let label = clean_text(&label.text().collect::<String>());
            let value = clean_text(&value.text().collect::<String>());
            if !label.is_empty() && !value.is_empty() {
                let sensitive = is_sensitive_label(&label);
                fields.push(CapturedField {
                    label: if sensitive {
                        "Identificador omitido".into()
                    } else {
                        label
                    },
                    value: if sensitive {
                        "[identificador omitido]".into()
                    } else {
                        value
                    },
                });
            }
        }
    }
    Ok(fields)
}

fn parse_tables(document: &Html) -> Result<(Vec<CapturedTable>, bool), String> {
    let table_selector = selector("table")?;
    let row_selector = selector("tr")?;
    let cell_selector = selector("th, td")?;
    let caption_selector = selector("caption")?;
    let mut tables = Vec::new();
    let mut truncated = false;
    for table in document.select(&table_selector).take(100) {
        let rows = table
            .select(&row_selector)
            .take(MAX_TABLE_ROWS + 2)
            .map(|row| {
                row.select(&cell_selector)
                    .map(|cell| clean_text(&cell.text().collect::<String>()))
                    .collect::<Vec<_>>()
            })
            .filter(|row| !row.is_empty())
            .collect::<Vec<_>>();
        if rows.is_empty() {
            continue;
        }
        let caption = table
            .select(&caption_selector)
            .next()
            .map(|caption| clean_text(&caption.text().collect::<String>()))
            .filter(|value| !value.is_empty());
        let has_header = table.select(&selector("thead")?).next().is_some()
            || table
                .select(&selector("tr:first-child th")?)
                .next()
                .is_some();
        let mut data_rows = rows.clone();
        let raw_headers = if has_header {
            data_rows.remove(0)
        } else {
            (1..=rows.iter().map(Vec::len).max().unwrap_or(0))
                .map(|index| format!("Coluna {index}"))
                .collect()
        };
        truncated |= data_rows.len() > MAX_TABLE_ROWS;
        let width = raw_headers.len().max(1);
        let headers = (0..width)
            .map(|index| {
                let value = raw_headers
                    .get(index)
                    .cloned()
                    .unwrap_or_else(|| format!("Coluna {}", index + 1));
                if is_sensitive_label(&value) {
                    "Identificador omitido".to_owned()
                } else {
                    value
                }
            })
            .collect::<Vec<_>>();
        let rows = data_rows
            .into_iter()
            .take(MAX_TABLE_ROWS)
            .map(|row| {
                (0..width)
                    .map(|index| {
                        if headers
                            .get(index)
                            .is_some_and(|header| header == "Identificador omitido")
                        {
                            "[identificador omitido]".to_owned()
                        } else {
                            row.get(index).cloned().unwrap_or_default()
                        }
                    })
                    .collect::<Vec<_>>()
            })
            .collect();
        tables.push(CapturedTable {
            caption,
            headers,
            rows,
        });
    }
    Ok((tables, truncated))
}

fn selector(value: &str) -> Result<Selector, String> {
    Selector::parse(value).map_err(|_| "seletor HTML inválido no capturador acadêmico".to_owned())
}

fn clean_text(value: &str) -> String {
    value.split_whitespace().collect::<Vec<_>>().join(" ")
}

fn normalized(value: &str) -> String {
    value
        .nfd()
        .filter(|character| !matches!(*character as u32, 0x0300..=0x036f))
        .flat_map(char::to_lowercase)
        .collect::<String>()
}

fn is_sensitive_label(value: &str) -> bool {
    let label = normalized(value);
    label.trim() == "nome"
        || label.contains("nome completo")
        || label
            .split(|character: char| !character.is_ascii_alphanumeric())
            .any(|word| {
                matches!(
                    word,
                    "cpf"
                        | "ra"
                        | "matricula"
                        | "identificacao"
                        | "documento"
                        | "aluno"
                        | "estudante"
                        | "discente"
                )
            })
}

fn md_text(value: &str) -> String {
    clean_text(value).replace('\n', " ").replace('\r', " ")
}

fn md_cell(value: &str) -> String {
    md_text(value).replace('|', "\\|")
}

#[cfg(test)]
mod tests {
    use super::{
        MAX_TABLE_ROWS, PortalCapture, VisibleLink, is_read_only_navigation, navigation_target,
        parse_page, persist_capture, write_new_file_beneath,
    };

    #[test]
    fn portal_capture_is_preserved_in_markdown_json_and_sqlite() {
        let root = tempfile::tempdir().unwrap();
        let database = root.path().join(".estuda-uva.sqlite3");
        let sync = crate::sync::SyncEngine::open(&database, root.path()).unwrap();
        let capture = PortalCapture {
            captured_at: "2026-10-10T12:00:00Z".into(),
            pages_read: 1,
            pages: vec![parse_page(
                "https://portalaluno.uva.br/boletim?token=nao-salvar",
                r#"<html><head><title>Boletim</title></head><body><h1>Notas</h1>
                   <table><thead><tr><th>Disciplina</th><th>Nota</th><th>Frequência</th></tr></thead>
                   <tbody><tr><td>Algoritmos</td><td>8,5</td><td>92%</td></tr></tbody></table></body></html>"#,
                &[],
                "2026-10-10T12:00:00Z",
            )
            .unwrap()
            .page],
            inaccessible_sections: Vec::new(),
            warnings: Vec::new(),
            complete: true,
            json_path: None,
            markdown_path: None,
        };

        let saved = persist_capture(root.path(), &sync, capture).unwrap();
        let json_path = root.path().join(saved.json_path.as_ref().unwrap());
        let markdown_path = root.path().join(saved.markdown_path.as_ref().unwrap());
        let json: serde_json::Value =
            serde_json::from_slice(&std::fs::read(&json_path).unwrap()).unwrap();
        let markdown = std::fs::read_to_string(&markdown_path).unwrap();

        assert_eq!(json["pages"][0]["tables"][0]["rows"][0][1], "8,5");
        assert!(markdown.contains("| Algoritmos | 8,5 | 92% |"));
        assert!(!markdown.contains("token=nao-salvar"));
        assert_eq!(
            sync.read_content(saved.json_path.as_ref().unwrap())
                .unwrap()
                .as_deref(),
            Some(std::str::from_utf8(&std::fs::read(&json_path).unwrap()).unwrap())
        );
        assert_eq!(
            sync.read_content(saved.markdown_path.as_ref().unwrap())
                .unwrap()
                .as_deref(),
            Some(markdown.as_str())
        );
    }

    #[test]
    fn capture_writer_preserves_existing_files() {
        let root = tempfile::tempdir().unwrap();
        let path = root.path().join("materias/catalogo/capturas/capture.json");
        std::fs::create_dir_all(path.parent().unwrap()).unwrap();
        std::fs::write(&path, b"captura anterior").unwrap();

        let result = write_new_file_beneath(
            root.path(),
            "materias/catalogo/capturas/capture.json",
            b"nova captura",
        );

        assert!(result.is_err());
        assert_eq!(std::fs::read(path).unwrap(), b"captura anterior");
    }

    #[cfg(unix)]
    #[test]
    fn capture_writer_rejects_symlinked_parent_directories() {
        use std::os::unix::fs::symlink;

        let root = tempfile::tempdir().unwrap();
        let outside = tempfile::tempdir().unwrap();
        symlink(outside.path(), root.path().join("materias")).unwrap();

        let result = write_new_file_beneath(
            root.path(),
            "materias/catalogo/capturas/capture.json",
            b"conteudo",
        );

        assert!(result.is_err());
        assert!(
            !outside
                .path()
                .join("catalogo/capturas/capture.json")
                .exists()
        );
    }

    #[test]
    fn captured_tables_keep_grade_attendance_and_unknown_columns_without_identifiers() {
        let html = r#"
          <html><head><title>Notas por disciplina</title></head><body>
            <h1>Notas e frequência</h1>
            <table><thead><tr><th>Disciplina</th><th>Nota</th><th>Frequência</th><th>Detalhe futuro</th><th>RA</th><th>Nome</th></tr></thead>
              <tbody><tr><td>Algoritmos</td><td>8,5</td><td>92%</td><td>Em andamento</td><td>1234567</td><td>Estudante</td></tr></tbody>
            </table>
          </body></html>
        "#;
        let capture = parse_page(
            "https://sis.lyceum.com.br/AOnline3/index.html#/notas?session=hidden",
            html,
            &[],
            "2026-10-10T12:00:00Z",
        )
        .unwrap();

        assert_eq!(capture.page.title, "Notas por disciplina");
        assert_eq!(
            capture.page.source_url,
            "https://sis.lyceum.com.br/AOnline3/index.html"
        );
        assert_eq!(capture.page.tables[0].headers[1], "Nota");
        assert_eq!(capture.page.tables[0].headers[2], "Frequência");
        assert_eq!(capture.page.tables[0].headers[3], "Detalhe futuro");
        assert_eq!(capture.page.tables[0].headers[4], "Identificador omitido");
        assert_eq!(capture.page.tables[0].headers[5], "Identificador omitido");
        assert_eq!(capture.page.tables[0].rows[0][0], "Algoritmos");
        assert_eq!(capture.page.tables[0].rows[0][4], "[identificador omitido]");
        assert_eq!(capture.page.tables[0].rows[0][5], "[identificador omitido]");
    }

    #[test]
    fn oversized_portal_tables_are_marked_incomplete_and_capped() {
        let mut html = String::from(
            "<html><head><title>Histórico</title></head><body><table><thead><tr><th>Disciplina</th></tr></thead><tbody>",
        );
        for index in 0..=MAX_TABLE_ROWS {
            html.push_str(&format!("<tr><td>Disciplina {index}</td></tr>"));
        }
        html.push_str("</tbody></table></body></html>");

        let parsed = parse_page(
            "https://sis.lyceum.com.br/AOnline3/#/historico",
            &html,
            &[],
            "2026-10-10T12:00:00Z",
        )
        .unwrap();

        assert!(parsed.truncated);
        assert!(parsed.warnings[0].contains("limite de 10000 linhas"));
        assert_eq!(parsed.page.tables[0].rows.len(), MAX_TABLE_ROWS);
    }

    #[test]
    fn navigation_follows_read_only_academic_sections_and_pagination_only() {
        assert!(is_read_only_navigation(&VisibleLink {
            label: "Histórico escolar".into(),
            href: "https://sis.lyceum.com.br/AOnline3/#/historico".into(),
            context: "menu acadêmico".into(),
            rel: String::new(),
        },));
        assert!(!is_read_only_navigation(&VisibleLink {
            label: "Solicitar trancamento".into(),
            href: "https://sis.lyceum.com.br/AOnline3/#/trancamento".into(),
            context: "serviços".into(),
            rel: String::new(),
        },));
        assert!(is_read_only_navigation(&VisibleLink {
            label: "Próxima".into(),
            href: "https://sis.lyceum.com.br/AOnline3/#/historico?page=2".into(),
            context: "Paginação".into(),
            rel: "next".into(),
        },));
    }

    #[test]
    fn navigation_removes_credentials_from_spa_fragment_and_keeps_safe_pagination() {
        let target = navigation_target(
            "https://sis.lyceum.com.br/AOnline3/#/historico",
            "#/historico?page=3&token=nao-gravar",
        )
        .unwrap();

        assert_eq!(
            target,
            "https://sis.lyceum.com.br/AOnline3/#/historico?page=3"
        );
        assert!(!target.contains("token"));
    }

    #[test]
    fn authentication_routes_are_not_followed_even_with_an_academic_link_label() {
        let target = "https://sis.lyceum.com.br/AOnline3/#/autologin?a=1&b=2";
        let link = VisibleLink {
            label: "Histórico".into(),
            href: target.into(),
            context: "menu acadêmico".into(),
            rel: String::new(),
        };

        assert!(!is_read_only_navigation(&link));
        assert!(
            navigation_target("https://sis.lyceum.com.br/AOnline3/#/home/avisos", target).is_none()
        );
    }
}
