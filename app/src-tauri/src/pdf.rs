use pdf_extract::Document;
use thiserror::Error;
use unicode_normalization::{UnicodeNormalization, char::is_combining_mark};
use url::Url;

#[derive(Debug, Error)]
pub enum PdfError {
    #[error("o arquivo não tem assinatura PDF válida")]
    InvalidSignature,
    #[error("PDF inválido ou protegido: {0}")]
    Extract(String),
    #[error("a extração parou antes de todas as páginas ({extracted}/{expected})")]
    Partial { extracted: usize, expected: usize },
}

pub fn split_pdf_pages(bytes: &[u8]) -> Result<Vec<String>, PdfError> {
    if !bytes.starts_with(b"%PDF-") {
        return Err(PdfError::InvalidSignature);
    }
    let document =
        Document::load_mem(bytes).map_err(|error| PdfError::Extract(error.to_string()))?;
    let expected = document.get_pages().len();
    if expected == 0 {
        return Err(PdfError::Extract("PDF sem páginas".into()));
    }
    let pages = pdf_extract::extract_text_from_mem_by_pages(bytes)
        .map_err(|error| PdfError::Extract(error.to_string()))?;
    if pages.len() != expected {
        return Err(PdfError::Partial {
            extracted: pages.len(),
            expected,
        });
    }
    Ok(pages
        .into_iter()
        .map(|page| page.trim().to_owned())
        .collect())
}

pub fn pages_to_markdown(title: &str, origin: &str, pages: &[String]) -> String {
    let safe_title = escape_markdown_text(title.trim());
    let safe_origin = Url::parse(origin)
        .ok()
        .map(|mut url| {
            url.set_query(None);
            url.set_fragment(None);
            url.to_string()
        })
        .unwrap_or_else(|| "Origem local".to_owned());

    let mut output = format!(
        "# {safe_title}\n\n> Conversão local de PDF para texto. Confira a paginação e a ordem no material oficial.\n\n- **Origem:** <{safe_origin}>\n- **Páginas:** {}\n\n",
        pages.len()
    );
    for (index, page) in pages.iter().enumerate() {
        output.push_str(&format!("## Página {}\n\n", index + 1));
        if page.trim().is_empty() {
            output.push_str("_Nenhum texto extraível nesta página._\n\n");
        } else {
            output.push_str(page.trim());
            output.push_str("\n\n");
        }
    }
    output
}

pub fn safe_slug(value: &str) -> String {
    let mut slug = String::new();
    let mut separator = false;
    for character in value.nfd() {
        if is_combining_mark(character) {
            continue;
        }
        if character.is_ascii_alphanumeric() {
            if separator && !slug.is_empty() {
                slug.push('-');
            }
            slug.push(character.to_ascii_lowercase());
            separator = false;
        } else {
            separator = true;
        }
    }
    if slug.is_empty() {
        "material".to_owned()
    } else {
        slug
    }
}

fn escape_markdown_text(value: &str) -> String {
    value
        .chars()
        .map(|character| match character {
            '&' => "&amp;".to_owned(),
            '<' => "&lt;".to_owned(),
            '>' => "&gt;".to_owned(),
            '\\' | '`' | '*' | '_' | '{' | '}' | '[' | ']' | '(' | ')' | '#' | '+' | '-' | '.'
            | '!' | '|' => {
                format!("\\{character}")
            }
            other => other.to_string(),
        })
        .collect()
}
