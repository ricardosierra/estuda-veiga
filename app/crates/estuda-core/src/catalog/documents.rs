#[cfg(test)]
mod tests {
    use std::{
        fs,
        io::{Cursor, Read, Write},
        path::PathBuf,
    };

    use zip::{ZipArchive, ZipWriter, write::FileOptions};

    use super::{DocumentFormat, extract_document};

    fn original(relative_path: &str) -> PathBuf {
        let mut root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        while !root.join("originais").is_dir() {
            assert!(
                root.pop(),
                "não foi possível localizar a raiz do acervo acadêmico"
            );
        }
        root.join(relative_path)
    }

    fn insert_table_after_first_paragraph(source: &[u8]) -> Vec<u8> {
        let mut archive = ZipArchive::new(Cursor::new(source)).unwrap();
        let mut output = ZipWriter::new(Cursor::new(Vec::new()));
        for index in 0..archive.len() {
            let mut entry = archive.by_index(index).unwrap();
            let name = entry.name().to_owned();
            if entry.is_dir() {
                output.add_directory(name, FileOptions::default()).unwrap();
                continue;
            }
            let mut contents = Vec::new();
            entry.read_to_end(&mut contents).unwrap();
            if name == "word/document.xml" {
                let mut xml = String::from_utf8(contents).unwrap();
                let first_paragraph_end = xml.find("</w:p>").unwrap() + "</w:p>".len();
                xml.insert_str(
                    first_paragraph_end,
                    "<w:tbl><w:tr><w:tc><w:p><w:r><w:t>TABELA ENTRE PARAGRAFOS</w:t></w:r></w:p></w:tc></w:tr></w:tbl>",
                );
                contents = xml.into_bytes();
            }
            output.start_file(name, FileOptions::default()).unwrap();
            output.write_all(&contents).unwrap();
        }
        output.finish().unwrap().into_inner()
    }

    #[test]
    fn extracts_markdown_as_source_without_executing_or_rewriting_html() {
        let source = "# Aula\n\n<script>alert('nao executar')</script>\n\n| conceito | exemplo |\n|---|---|\n| dado | texto |\n";
        let extracted =
            extract_document(PathBuf::from("aula.md").as_path(), source.as_bytes()).unwrap();

        assert_eq!(extracted.format, DocumentFormat::Markdown);
        assert_eq!(extracted.text, source);
        assert!(extracted.warnings.is_empty());
    }

    #[test]
    fn extracts_json_without_reordering_or_dropping_unknown_fields() {
        let source = "{\n  \"z_desconhecido\": {\"ordem\": 2},\n  \"a\": 1\n}\n";
        let extracted =
            extract_document(PathBuf::from("registro.json").as_path(), source.as_bytes()).unwrap();

        assert_eq!(extracted.format, DocumentFormat::Json);
        assert_eq!(extracted.text, source);
        assert!(extracted.warnings.is_empty());
    }

    #[test]
    fn extracts_html_as_plain_text_instead_of_returning_markup() {
        let source =
            "<html><body><h1>Aula local</h1><p>Texto <strong>legível</strong>.</p></body></html>";
        let extracted =
            extract_document(PathBuf::from("aula.html").as_path(), source.as_bytes()).unwrap();

        assert!(extracted.text.contains("Aula local"));
        assert!(extracted.text.contains("Texto legível"));
        assert!(!extracted.text.contains("<strong>"));
        assert!(!extracted.warnings.is_empty());
    }

    #[test]
    fn extracts_existing_docx_in_paragraph_order_without_changing_source() {
        let path =
            original("originais/paradigmas-de-programacao/trabalho2-paradigmas-ricardo.docx");
        let bytes_before = fs::read(&path).unwrap();

        let extracted = extract_document(&path, &bytes_before).unwrap();

        assert_eq!(extracted.format, DocumentFormat::Docx);
        assert!(!extracted.text.trim().is_empty());
        let introduction = extracted.text.find("1 INTRODUÇÃO").unwrap();
        let conclusion = extracted.text.find("6 CONSIDERAÇÕES FINAIS").unwrap();
        assert!(introduction < conclusion);
        assert_eq!(fs::read(path).unwrap(), bytes_before);
    }

    #[test]
    fn preserves_table_position_between_docx_paragraphs() {
        let path =
            original("originais/paradigmas-de-programacao/trabalho2-paradigmas-ricardo.docx");
        let source = fs::read(path).unwrap();
        let fixture = insert_table_after_first_paragraph(&source);

        let extracted =
            extract_document(PathBuf::from("fixture.docx").as_path(), &fixture).unwrap();

        let first = extracted
            .text
            .find("UNIVERSIDADE VEIGA DE ALMEIDA")
            .unwrap();
        let table = extracted.text.find("TABELA ENTRE PARAGRAFOS").unwrap();
        let following = extracted.text.find("CURSO DE GRADUAÇÃO").unwrap();
        assert!(first < table && table < following);
    }

    #[test]
    fn extracts_existing_pages_text_storages_without_changing_source() {
        let path =
            original("originais/paradigmas-de-programacao/trabalho-paradigmas-ricardo.pages");
        let bytes_before = fs::read(&path).unwrap();

        let extracted = extract_document(&path, &bytes_before).unwrap();

        assert_eq!(extracted.format, DocumentFormat::Pages);
        assert!(!extracted.text.trim().is_empty());
        assert_eq!(fs::read(path).unwrap(), bytes_before);
    }
}
use serde::{Deserialize, Serialize};
use std::{
    io::{Cursor, Read},
    path::Path,
    str::Utf8Error,
};
use thiserror::Error;

const MAX_DOCUMENT_BYTES: usize = 64 * 1024 * 1024;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DocumentFormat {
    Markdown,
    PlainText,
    Alg,
    Csv,
    Json,
    Html,
    Pdf,
    Docx,
    Pages,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PageBoundary {
    pub page: usize,
    pub start_byte: usize,
    pub end_byte: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TextSection {
    pub label: String,
    pub start_byte: usize,
    pub end_byte: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExtractedDocument {
    pub text: String,
    pub format: DocumentFormat,
    pub page_boundaries: Vec<PageBoundary>,
    pub sections: Vec<TextSection>,
    pub warnings: Vec<String>,
}

#[derive(Debug, Error)]
pub enum DocumentError {
    #[error("o arquivo excede o limite de 64 MB para leitura local")]
    TooLarge,
    #[error("o arquivo de texto não está codificado em UTF-8: {0}")]
    InvalidUtf8(#[from] Utf8Error),
    #[error("não foi possível extrair o conteúdo do documento: {0}")]
    Extract(String),
    #[error("formato de documento não suportado: {0}")]
    Unsupported(String),
}

pub fn extract_document(path: &Path, bytes: &[u8]) -> Result<ExtractedDocument, DocumentError> {
    if bytes.len() > MAX_DOCUMENT_BYTES {
        return Err(DocumentError::TooLarge);
    }

    let extension = path
        .extension()
        .and_then(|value| value.to_str())
        .unwrap_or_default()
        .to_ascii_lowercase();

    let mut extracted = match extension.as_str() {
        "md" | "markdown" => text_document(bytes, DocumentFormat::Markdown)?,
        "txt" => text_document(bytes, DocumentFormat::PlainText)?,
        "alg" => text_document(bytes, DocumentFormat::Alg)?,
        "csv" => text_document(bytes, DocumentFormat::Csv)?,
        "html" | "htm" => html_document(bytes)?,
        "json" => {
            let mut document = text_document(bytes, DocumentFormat::Json)?;
            if serde_json::from_str::<serde_json::Value>(&document.text).is_err() {
                document.warnings.push(
                    "O conteúdo não é um JSON válido; foi preservado como texto original.".into(),
                );
            }
            document
        }
        "pdf" => pdf_document(bytes)?,
        "docx" => docx_document(bytes)?,
        "pages" => pages_document(bytes)?,
        other => return Err(DocumentError::Unsupported(other.to_owned())),
    };
    extracted.text = extracted.text.replace("\r\n", "\n").replace('\r', "\n");
    Ok(extracted)
}

fn text_document(bytes: &[u8], format: DocumentFormat) -> Result<ExtractedDocument, DocumentError> {
    let text = std::str::from_utf8(bytes)?.to_owned();
    Ok(ExtractedDocument {
        text,
        format,
        page_boundaries: Vec::new(),
        sections: Vec::new(),
        warnings: Vec::new(),
    })
}

fn html_document(bytes: &[u8]) -> Result<ExtractedDocument, DocumentError> {
    let source = std::str::from_utf8(bytes)?;
    let parsed = scraper::Html::parse_document(source);
    let text = parsed
        .root_element()
        .text()
        .map(str::trim)
        .filter(|part| !part.is_empty())
        .collect::<Vec<_>>()
        .join(" ")
        .replace(" .", ".")
        .replace(" ,", ",")
        .replace(" ;", ";")
        .replace(" :", ":")
        .replace(" !", "!")
        .replace(" ?", "?");
    Ok(ExtractedDocument {
        text,
        format: DocumentFormat::Html,
        page_boundaries: Vec::new(),
        sections: Vec::new(),
        warnings: vec![
            "O HTML foi convertido em texto simples; a marcação original não é exibida.".into(),
        ],
    })
}

fn pdf_document(bytes: &[u8]) -> Result<ExtractedDocument, DocumentError> {
    let pages = crate::pdf::split_pdf_pages(bytes)
        .map_err(|error| DocumentError::Extract(error.to_string()))?;
    if pages.iter().all(|page| page.trim().is_empty()) {
        return Ok(ExtractedDocument {
            text: String::new(),
            format: DocumentFormat::Pdf,
            page_boundaries: Vec::new(),
            sections: Vec::new(),
            warnings: vec![
                "Este PDF não contém texto selecionável; a extração local não faz OCR.".into(),
            ],
        });
    }

    let mut text = String::new();
    let mut page_boundaries = Vec::with_capacity(pages.len());
    for (index, page) in pages.into_iter().enumerate() {
        if index > 0 {
            text.push_str("\n\n");
        }
        text.push_str(&format!("## Página {}\n\n", index + 1));
        let start_byte = text.len();
        text.push_str(page.trim());
        let end_byte = text.len();
        page_boundaries.push(PageBoundary {
            page: index + 1,
            start_byte,
            end_byte,
        });
    }
    Ok(ExtractedDocument {
        text,
        format: DocumentFormat::Pdf,
        page_boundaries,
        sections: Vec::new(),
        warnings: Vec::new(),
    })
}

fn docx_document(bytes: &[u8]) -> Result<ExtractedDocument, DocumentError> {
    let (text, ordered_blocks) = extract_docx_body(bytes)?;
    let (text, structure_warning) = if ordered_blocks {
        (text, None)
    } else {
        let fallback = docx_lite::extract_text_from_bytes(bytes)
            .map_err(|error| DocumentError::Extract(error.to_string()))?;
        (
            fallback,
            Some("O documento usa estruturas OOXML que impedem preservar a ordem original de todos os blocos.".to_owned()),
        )
    };
    let mut warnings =
        vec!["A paginação e a formatação visual do DOCX não são preservadas.".to_owned()];
    if let Some(warning) = structure_warning {
        warnings.push(warning);
    }
    Ok(ExtractedDocument {
        text,
        format: DocumentFormat::Docx,
        page_boundaries: Vec::new(),
        sections: Vec::new(),
        warnings,
    })
}

#[derive(Clone, Copy)]
enum DocxBlockKind {
    Paragraph,
    Table,
}

fn extract_docx_body(bytes: &[u8]) -> Result<(String, bool), DocumentError> {
    let mut archive = zip::ZipArchive::new(Cursor::new(bytes))
        .map_err(|error| DocumentError::Extract(format!("DOCX inválido: {error}")))?;
    let mut xml = String::new();
    archive
        .by_name("word/document.xml")
        .map_err(|error| DocumentError::Extract(format!("DOCX sem word/document.xml: {error}")))?
        .read_to_string(&mut xml)
        .map_err(|error| {
            DocumentError::Extract(format!("não foi possível ler XML do DOCX: {error}"))
        })?;

    let mut reader = quick_xml::Reader::from_str(&xml);
    reader.config_mut().trim_text(false);
    let mut depth = 0usize;
    let mut body_depth = None;
    let mut block_kind = None;
    let mut block_depth = 0usize;
    let mut in_text_node = false;
    let mut current_block = String::new();
    let mut blocks = Vec::new();

    loop {
        match reader.read_event() {
            Ok(quick_xml::events::Event::Start(element)) => {
                let name = element.local_name();
                let name = name.as_ref();
                if name == b"body" && body_depth.is_none() {
                    body_depth = Some(depth + 1);
                } else if body_depth.is_some() && block_kind.is_none() {
                    if name == b"p" {
                        block_kind = Some(DocxBlockKind::Paragraph);
                        block_depth = depth + 1;
                    } else if name == b"tbl" {
                        block_kind = Some(DocxBlockKind::Table);
                        block_depth = depth + 1;
                    }
                }
                if block_kind.is_some() {
                    if name == b"t" || name == b"delText" {
                        in_text_node = true;
                    } else if name == b"tab" {
                        current_block.push('\t');
                    } else if name == b"br" || name == b"cr" {
                        current_block.push('\n');
                    } else if matches!(block_kind, Some(DocxBlockKind::Table)) {
                        if name == b"tr" && !current_block.is_empty() {
                            current_block.push('\n');
                        } else if name == b"tc"
                            && !current_block.is_empty()
                            && !current_block.ends_with(['\n', '\t'])
                        {
                            current_block.push('\t');
                        }
                    }
                }
                depth += 1;
            }
            Ok(quick_xml::events::Event::Empty(element)) => {
                let name = element.local_name();
                match name.as_ref() {
                    b"tab" if block_kind.is_some() => current_block.push('\t'),
                    b"br" | b"cr" if block_kind.is_some() => current_block.push('\n'),
                    _ => {}
                }
            }
            Ok(quick_xml::events::Event::Text(text)) if in_text_node => {
                let decoded = text.unescape().map_err(|error| {
                    DocumentError::Extract(format!("XML de texto inválido: {error}"))
                })?;
                current_block.push_str(&decoded);
            }
            Ok(quick_xml::events::Event::End(element)) => {
                let name = element.local_name();
                let name = name.as_ref();
                if (name == b"t" || name == b"delText") && in_text_node {
                    in_text_node = false;
                }
                if let Some(kind) = block_kind {
                    let is_end = match kind {
                        DocxBlockKind::Paragraph => name == b"p",
                        DocxBlockKind::Table => name == b"tbl",
                    };
                    if is_end && depth == block_depth {
                        let value = current_block.trim().to_owned();
                        if !value.is_empty() {
                            blocks.push(value);
                        }
                        current_block.clear();
                        block_kind = None;
                    }
                }
                if name == b"body" && body_depth == Some(depth) {
                    body_depth = None;
                }
                depth = depth.saturating_sub(1);
            }
            Ok(quick_xml::events::Event::Eof) => break,
            Ok(_) => {}
            Err(error) => {
                return Err(DocumentError::Extract(format!(
                    "XML do DOCX inválido: {error}"
                )));
            }
        }
    }

    let ordered = !blocks.is_empty();
    Ok((blocks.join("\n\n"), ordered))
}

fn pages_document(bytes: &[u8]) -> Result<ExtractedDocument, DocumentError> {
    let package = iwork::package::Package::from_bytes(bytes)
        .map_err(|error| DocumentError::Extract(error.to_string()))?;
    let document = iwork::Document::from_package(package)
        .map_err(|error| DocumentError::Extract(error.to_string()))?;
    let mut text = String::new();
    let mut sections = Vec::new();
    for storage in document.text_storages() {
        let value = storage.text.trim();
        if value.is_empty() {
            continue;
        }
        if !text.is_empty() {
            text.push_str("\n\n");
        }
        let start_byte = text.len();
        text.push_str(value);
        sections.push(TextSection {
            label: format!("Armazenamento de texto {}", storage.identifier),
            start_byte,
            end_byte: text.len(),
        });
    }
    Ok(ExtractedDocument {
        text,
        format: DocumentFormat::Pages,
        page_boundaries: Vec::new(),
        sections,
        warnings: vec!["A paginação e a formatação visual do Pages não são preservadas.".into()],
    })
}
