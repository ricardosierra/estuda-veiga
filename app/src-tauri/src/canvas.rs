use scraper::{Html, Selector, node::Node};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::HashSet;
use thiserror::Error;
use url::Url;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CourseKind {
    Current,
    Past,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Course {
    pub id: String,
    pub title: String,
    pub url: String,
    pub kind: CourseKind,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ModuleItem {
    pub module_title: String,
    pub title: String,
    pub href: String,
    pub item_type: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ModuleGroup {
    pub title: String,
    pub items: Vec<ModuleItem>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MaterialLink {
    pub title: String,
    pub href: String,
    pub embedded: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CanvasCoursePage {
    pub courses: Vec<Course>,
    pub next_href: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct CanvasCourseListing {
    pub courses: Vec<Course>,
    pub pages_traversed: usize,
    pub complete: bool,
    pub warning: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct CanvasCourseInventory {
    pub modules: Vec<ModuleGroup>,
    pub complete: bool,
    pub warning: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CanvasModuleItemContent {
    pub canvas_item_id: Option<String>,
    pub title: String,
    pub source_url: String,
    pub markdown: Option<String>,
    pub pdf_links: Vec<MaterialLink>,
    pub unsupported_reason: Option<String>,
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum CanvasParseError {
    #[error("o endereço da página do Canvas não é seguro")]
    UnsafePageUrl,
    #[error("o conteúdo textual da aula não foi encontrado")]
    MissingContent,
}

pub struct CourseListingAccumulator {
    max_pages: usize,
    courses: Vec<Course>,
    seen_course_ids: HashSet<String>,
    seen_page_urls: HashSet<String>,
    pages_traversed: usize,
    complete: bool,
    warning: Option<String>,
}

impl CourseListingAccumulator {
    pub fn new(max_pages: usize) -> Self {
        Self {
            max_pages: max_pages.clamp(1, 100),
            courses: Vec::new(),
            seen_course_ids: HashSet::new(),
            seen_page_urls: HashSet::new(),
            pages_traversed: 0,
            complete: false,
            warning: None,
        }
    }

    pub fn push_page(&mut self, page_url: &str, page: CanvasCoursePage) -> Option<String> {
        let Some(page_url) = canonical_course_list_url(page_url) else {
            self.warning = Some("A paginação do Canvas apontou para um endereço inseguro.".into());
            return None;
        };
        let page_key = page_url.as_str().to_owned();
        if !self.seen_page_urls.insert(page_key.clone()) {
            self.warning = Some("A paginação repetiu uma página do Canvas.".into());
            return None;
        }
        self.pages_traversed += 1;
        for course in page.courses {
            if self.seen_course_ids.insert(course.id.clone()) {
                self.courses.push(course);
            }
        }
        let Some(raw_next) = page.next_href else {
            self.complete = true;
            return None;
        };
        let Some(next_url) =
            validate_next_course_href(page_url.as_str(), &raw_next, &self.seen_page_urls)
        else {
            self.warning = Some(
                "A paginação do Canvas foi interrompida por um link inválido ou repetido.".into(),
            );
            return None;
        };
        if self.pages_traversed >= self.max_pages {
            self.warning = Some(format!(
                "A listagem foi interrompida no limite configurado de {} páginas.",
                self.max_pages
            ));
            return None;
        }
        Some(next_url)
    }

    pub fn finish(self) -> CanvasCourseListing {
        CanvasCourseListing {
            courses: self.courses,
            pages_traversed: self.pages_traversed,
            complete: self.complete,
            warning: self.warning,
        }
    }
}

pub fn parse_courses(html: &str) -> Vec<Course> {
    let document = Html::parse_document(html);
    let Ok(anchor_selector) = Selector::parse("a[href]") else {
        return Vec::new();
    };
    let Ok(heading_selector) = Selector::parse("h1, h2, h3, h4, h5, h6") else {
        return Vec::new();
    };
    let headings = document
        .select(&heading_selector)
        .map(|element| {
            let text = element.text().collect::<Vec<_>>().join(" ").to_lowercase();
            (element.id(), text)
        })
        .collect::<Vec<_>>();
    let mut courses = Vec::new();
    let mut seen = HashSet::new();
    for anchor in document.select(&anchor_selector) {
        let Some(href) = anchor.value().attr("href") else {
            continue;
        };
        let Some(id) = exact_course_id(href) else {
            continue;
        };
        if !seen.insert(id.clone()) {
            continue;
        }
        let title = anchor
            .text()
            .collect::<Vec<_>>()
            .join(" ")
            .split_whitespace()
            .collect::<Vec<_>>()
            .join(" ");
        if title.is_empty() {
            continue;
        }
        let anchor_id = anchor.id();
        let section = headings
            .iter()
            .filter(|(heading_id, _)| *heading_id < anchor_id)
            .last()
            .map(|(_, text)| text.as_str())
            .unwrap_or("");
        let kind = if [
            "anteriores",
            "passadas",
            "concluídas",
            "concluidas",
            "previous",
            "past",
            "completed",
        ]
        .iter()
        .any(|word| section.contains(word))
        {
            CourseKind::Past
        } else {
            CourseKind::Current
        };
        let url = format!("https://uva.instructure.com/courses/{id}");
        courses.push(Course {
            id,
            title,
            url,
            kind,
        });
    }
    courses
}

pub fn parse_course_page(html: &str, page_url: &str) -> CanvasCoursePage {
    let courses = if canonical_course_list_url(page_url).is_some() {
        parse_courses(html)
    } else {
        Vec::new()
    };
    let next_href = Selector::parse("a[rel~='next'], a.next_page")
        .ok()
        .and_then(|selector| {
            Html::parse_document(html)
                .select(&selector)
                .find_map(|anchor| anchor.value().attr("href"))
                .map(str::to_owned)
        });
    CanvasCoursePage { courses, next_href }
}

pub fn validate_next_course_href(
    current_page: &str,
    next_href: &str,
    seen_pages: &HashSet<String>,
) -> Option<String> {
    let current = canonical_course_list_url(current_page)?;
    let mut next = current.join(next_href).ok()?;
    if !is_canvas_course_list_url(&next) || !next.username().is_empty() || next.password().is_some()
    {
        return None;
    }
    next.set_fragment(None);
    let next = next.to_string();
    let mut current_key = current;
    current_key.set_fragment(None);
    if next == current_key.as_str() || seen_pages.contains(&next) {
        return None;
    }
    Some(next)
}

fn canonical_course_list_url(raw: &str) -> Option<Url> {
    let mut url = Url::parse(raw).ok()?;
    if !is_canvas_course_list_url(&url) || !url.username().is_empty() || url.password().is_some() {
        return None;
    }
    url.set_fragment(None);
    Some(url)
}

fn is_canvas_course_list_url(url: &Url) -> bool {
    url.scheme() == "https"
        && url.host_str() == Some("uva.instructure.com")
        && url.port_or_known_default() == Some(443)
        && url.path() == "/courses"
}

fn exact_course_id(href: &str) -> Option<String> {
    let path = Url::parse(&format!(
        "https://uva.instructure.com{}",
        if href.starts_with('/') {
            href.to_owned()
        } else {
            format!("/{href}")
        }
    ))
    .ok()?
    .path()
    .to_owned();
    let parts = path
        .split('/')
        .filter(|part| !part.is_empty())
        .collect::<Vec<_>>();
    if parts.len() == 2
        && parts[0] == "courses"
        && parts[1].bytes().all(|byte| byte.is_ascii_digit())
    {
        Some(parts[1].to_owned())
    } else {
        None
    }
}

pub fn parse_module_items(html: &str) -> Vec<ModuleItem> {
    let document = Html::parse_document(html);
    let Ok(module_selector) = Selector::parse(".context_module, [data-module-id], .module") else {
        return Vec::new();
    };
    let Ok(title_selector) =
        Selector::parse(".ig-header-title, .module_name, [data-module-name], h2, h3")
    else {
        return Vec::new();
    };
    let Ok(anchor_selector) = Selector::parse("a[href]") else {
        return Vec::new();
    };
    let mut items = Vec::new();
    let mut seen = HashSet::new();
    for module in document.select(&module_selector) {
        if element_is_unavailable(module) {
            continue;
        }
        let module_title = module
            .select(&title_selector)
            .next()
            .map(|title| {
                title
                    .text()
                    .collect::<Vec<_>>()
                    .join(" ")
                    .split_whitespace()
                    .collect::<Vec<_>>()
                    .join(" ")
            })
            .filter(|title| !title.is_empty())
            .unwrap_or_else(|| "Módulo sem título".to_owned());
        for anchor in module.select(&anchor_selector) {
            if element_is_unavailable(anchor) {
                continue;
            }
            let Some(href) = anchor.value().attr("href") else {
                continue;
            };
            let Some(url) = resolve_canvas_url(href).or_else(|| {
                let url = crate::download::validate_material_url(href).ok()?;
                let path = url.path().to_ascii_lowercase();
                (path.ends_with(".pdf") || path.contains("/files/")).then_some(url)
            }) else {
                continue;
            };
            let path = url.path().to_owned();
            let lower = path.to_lowercase();
            if [
                "/assignments/",
                "/discussion_topics/",
                "/quizzes/",
                "/submissions/",
            ]
            .iter()
            .any(|blocked| lower.contains(blocked))
            {
                continue;
            }
            let is_official_external_pdf =
                url.host_str() != Some("uva.instructure.com") && lower.ends_with(".pdf");
            if !(lower.contains("/modules/items/")
                || lower.contains("/files/")
                || lower.contains("/pages/")
                || is_official_external_pdf)
            {
                continue;
            }
            let mut canonical_url = url;
            canonical_url.set_query(None);
            canonical_url.set_fragment(None);
            if !seen.insert(canonical_url.to_string()) {
                continue;
            }
            let title = anchor
                .text()
                .collect::<Vec<_>>()
                .join(" ")
                .split_whitespace()
                .collect::<Vec<_>>()
                .join(" ");
            if title.is_empty() {
                continue;
            }
            let item_type = if lower.contains("/modules/items/") {
                "module_item"
            } else if lower.contains("/files/") || lower.ends_with(".pdf") {
                "file"
            } else {
                "page"
            };
            items.push(ModuleItem {
                module_title: module_title.clone(),
                title,
                href: href.to_owned(),
                item_type: item_type.to_owned(),
            });
        }
    }
    items
}

fn resolve_canvas_url(raw: &str) -> Option<Url> {
    let base = Url::parse("https://uva.instructure.com/").ok()?;
    let url = base.join(raw).ok()?;
    (url.scheme() == "https"
        && url.host_str() == Some("uva.instructure.com")
        && url.port_or_known_default() == Some(443)
        && url.username().is_empty()
        && url.password().is_none())
    .then_some(url)
}

pub fn module_item_url(course_id: &str, href: &str) -> Option<Url> {
    if course_id.is_empty() || !course_id.bytes().all(|byte| byte.is_ascii_digit()) {
        return None;
    }
    let base = Url::parse(&format!("https://uva.instructure.com/courses/{course_id}/")).ok()?;
    let url = base.join(href).ok()?;
    if url.scheme() != "https"
        || url.host_str() != Some("uva.instructure.com")
        || url.port_or_known_default() != Some(443)
        || !url.username().is_empty()
        || url.password().is_some()
    {
        return None;
    }
    let path = url.path().to_ascii_lowercase();
    let blocked = [
        "/assignments/",
        "/discussion_topics/",
        "/quizzes/",
        "/submissions/",
        "/gradebook",
        "/grades",
    ];
    if blocked.iter().any(|part| path.contains(part)) {
        return None;
    }
    let course_prefix = format!("/courses/{course_id}/");
    let is_course_module_item = path.strip_prefix(&course_prefix).is_some_and(|tail| {
        tail.starts_with("modules/items/")
            || tail.starts_with("pages/")
            || tail.starts_with("files/")
    });
    let is_course_independent_file = path.starts_with("/files/");
    (is_course_module_item || is_course_independent_file).then_some(url)
}

fn element_is_unavailable(element: scraper::ElementRef<'_>) -> bool {
    let mut current = Some(element);
    while let Some(node) = current {
        let value = node.value();
        let class_tokens = value
            .attr("class")
            .unwrap_or_default()
            .split_ascii_whitespace()
            .map(str::to_ascii_lowercase);
        if value.attr("hidden").is_some()
            || value.attr("aria-hidden") == Some("true")
            || value.attr("aria-disabled") == Some("true")
            || value.attr("data-published") == Some("false")
            || class_tokens.into_iter().any(|token| {
                matches!(
                    token.as_str(),
                    "hidden" | "unpublished" | "locked" | "module_locked" | "module_unpublished"
                )
            })
        {
            return true;
        }
        current = node.parent().and_then(scraper::ElementRef::wrap);
    }
    false
}

pub fn group_module_items(items: Vec<ModuleItem>) -> Vec<ModuleGroup> {
    let mut groups = Vec::<ModuleGroup>::new();
    let mut positions = std::collections::HashMap::<String, usize>::new();
    for item in items {
        let position = *positions
            .entry(item.module_title.clone())
            .or_insert_with(|| {
                let position = groups.len();
                groups.push(ModuleGroup {
                    title: item.module_title.clone(),
                    items: Vec::new(),
                });
                position
            });
        groups[position].items.push(item);
    }
    groups
}

pub fn discover_material_links(html: &str, page_url: &str) -> Vec<MaterialLink> {
    let document = Html::parse_document(html);
    let mut found = Vec::new();
    let mut seen = HashSet::new();
    for (selector_text, attr, embedded) in [
        ("iframe[src], embed[src]", "src", true),
        ("object[data]", "data", true),
        ("a[href]", "href", false),
    ] {
        let Ok(selector) = Selector::parse(selector_text) else {
            continue;
        };
        for element in document.select(&selector) {
            let Some(raw) = element.value().attr(attr) else {
                continue;
            };
            let Ok(base) = Url::parse(page_url) else {
                continue;
            };
            let Ok(url) = base.join(raw) else { continue };
            let path = url.path().to_lowercase();
            let file_path = path.ends_with(".pdf") || path.contains("/files/");
            let trusted_embedded_material =
                embedded && crate::download::validate_material_url(url.as_str()).is_ok();
            if !(file_path || trusted_embedded_material) || !seen.insert(url.to_string()) {
                continue;
            }
            let title = element
                .value()
                .attr("title")
                .or_else(|| element.value().attr("aria-label"))
                .map(str::to_owned)
                .filter(|value| !value.trim().is_empty())
                .or_else(|| {
                    let text = element
                        .text()
                        .collect::<Vec<_>>()
                        .join(" ")
                        .trim()
                        .to_owned();
                    (!text.is_empty()).then_some(text)
                })
                .unwrap_or_else(|| "Material PDF".to_owned());
            found.push(MaterialLink {
                title,
                href: url.to_string(),
                embedded,
            });
        }
    }
    found
}

pub fn parse_canvas_page(
    html: &str,
    page_url: &str,
) -> Result<CanvasModuleItemContent, CanvasParseError> {
    let parsed_url = Url::parse(page_url).map_err(|_| CanvasParseError::UnsafePageUrl)?;
    if parsed_url.scheme() != "https"
        || parsed_url.host_str() != Some("uva.instructure.com")
        || parsed_url.username() != ""
        || parsed_url.password().is_some()
    {
        return Err(CanvasParseError::UnsafePageUrl);
    }

    let document = Html::parse_document(html);
    let title = Selector::parse("h1, [data-testid='page-title']")
        .ok()
        .and_then(|selector| {
            document.select(&selector).find_map(|element| {
                let text = element.text().collect::<Vec<_>>().join(" ");
                normalize_text(&text).filter(|text| !text.is_empty())
            })
        })
        .unwrap_or_else(|| "Aula do Canvas".to_owned());
    let content_selector = Selector::parse(".user_content, #wiki_page_show, #content, main, body")
        .map_err(|_| CanvasParseError::MissingContent)?;
    let content_root = document
        .select(&content_selector)
        .find(|element| {
            !normalize_text(&element.text().collect::<Vec<_>>().join(" "))
                .unwrap_or_default()
                .is_empty()
        })
        .ok_or(CanvasParseError::MissingContent)?;

    let mut base_url = parsed_url.clone();
    base_url.set_query(None);
    base_url.set_fragment(None);
    let pdf_links = discover_material_links(html, page_url);
    let mut body = String::new();
    render_markdown_node(content_root, &base_url, &mut body, 0);
    let body = body.trim().to_owned();
    let markdown = if body.is_empty() {
        None
    } else {
        Some(format!("# {}\n\n{}\n", escape_markdown_text(&title), body))
    };
    let source_url = sanitize_source_url(page_url).ok_or(CanvasParseError::UnsafePageUrl)?;
    let canvas_item_id = canvas_resource_id(&parsed_url);
    let unsupported_reason = (markdown.is_none() && pdf_links.is_empty())
        .then(|| "A página não contém texto selecionável nem PDF acessível.".to_owned());

    Ok(CanvasModuleItemContent {
        canvas_item_id,
        title,
        source_url,
        markdown,
        pdf_links,
        unsupported_reason,
    })
}

pub fn content_key(canvas_id: Option<&str>, source_url: &str, source_kind: &str) -> Option<String> {
    if !matches!(source_kind, "pdf" | "canvas_page") {
        return None;
    }
    let canonical_url = sanitize_source_url(source_url)?;
    let identity = canvas_id
        .filter(|id| {
            !id.is_empty()
                && id.len() <= 256
                && id
                    .bytes()
                    .all(|b| b.is_ascii_alphanumeric() || b"_-".contains(&b))
        })
        .map(|id| format!("canvas:{id}"))
        .unwrap_or(canonical_url);
    let digest = Sha256::digest(format!("{source_kind}\0{identity}").as_bytes());
    Some(format!("{source_kind}:{digest:x}"))
}

pub fn reference_key(course_id: &str, module_title: &str, item_href: &str) -> Option<String> {
    if course_id.is_empty() || !course_id.bytes().all(|byte| byte.is_ascii_digit()) {
        return None;
    }
    let item_url = Url::parse(&format!("https://uva.instructure.com/courses/{course_id}/"))
        .ok()?
        .join(item_href)
        .ok()?;
    if item_url.scheme() != "https"
        || item_url.host_str() != Some("uva.instructure.com")
        || item_url.username() != ""
        || item_url.password().is_some()
    {
        return None;
    }
    let mut item_url = item_url;
    item_url.set_query(None);
    item_url.set_fragment(None);
    let canonical = format!("{course_id}\0{}\0{}", module_title.trim(), item_url);
    let digest = Sha256::digest(canonical.as_bytes());
    Some(format!("ref:{digest:x}"))
}

pub fn sanitize_source_url(raw: &str) -> Option<String> {
    let mut url = Url::parse(raw).ok()?;
    if crate::download::validate_material_url(raw).is_err() {
        return None;
    }
    url.set_query(None);
    url.set_fragment(None);
    Some(url.to_string())
}

fn canvas_resource_id(url: &Url) -> Option<String> {
    let segments = url.path_segments()?.collect::<Vec<_>>();
    segments
        .windows(2)
        .find(|parts| parts[0] == "files")
        .map(|parts| parts[1].to_owned())
        .filter(|id| !id.is_empty() && id.bytes().all(|byte| byte.is_ascii_digit()))
}

fn normalize_text(text: &str) -> Option<String> {
    Some(text.split_whitespace().collect::<Vec<_>>().join(" "))
}

fn escape_markdown_text(text: &str) -> String {
    let mut escaped = String::with_capacity(text.len());
    for character in text.chars() {
        if matches!(
            character,
            '\\' | '`' | '*' | '_' | '[' | ']' | '#' | '<' | '>'
        ) {
            escaped.push('\\');
        }
        escaped.push(character);
    }
    escaped
}

fn render_markdown_node(
    node: scraper::ElementRef<'_>,
    base_url: &Url,
    output: &mut String,
    depth: usize,
) {
    for child in node.children() {
        match child.value() {
            Node::Text(text) => output.push_str(&escape_markdown_text(text)),
            Node::Element(element) => {
                let Some(child_element) = scraper::ElementRef::wrap(child) else {
                    continue;
                };
                let tag = element.name();
                if matches!(
                    tag,
                    "script"
                        | "style"
                        | "noscript"
                        | "svg"
                        | "iframe"
                        | "embed"
                        | "object"
                        | "form"
                        | "button"
                        | "input"
                ) {
                    continue;
                }
                match tag {
                    "h1" | "h2" | "h3" | "h4" | "h5" | "h6" => {
                        append_block_separator(output);
                        let level = tag[1..].parse::<usize>().unwrap_or(2).clamp(1, 6);
                        output.push_str(&"#".repeat(level));
                        output.push(' ');
                        render_markdown_node(child_element, base_url, output, depth);
                        output.push_str("\n\n");
                    }
                    "p" | "section" | "article" | "div" => {
                        append_block_separator(output);
                        render_markdown_node(child_element, base_url, output, depth);
                        output.push_str("\n\n");
                    }
                    "ul" | "ol" => {
                        append_block_separator(output);
                        render_markdown_node(child_element, base_url, output, depth + 1);
                        output.push('\n');
                    }
                    "li" => {
                        append_block_separator(output);
                        output.push_str(&"  ".repeat(depth.saturating_sub(1)));
                        output.push_str("- ");
                        render_markdown_node(child_element, base_url, output, depth);
                        output.push('\n');
                    }
                    "br" => output.push('\n'),
                    "hr" => output.push_str("\n---\n\n"),
                    "strong" | "b" => {
                        output.push_str("**");
                        render_markdown_node(child_element, base_url, output, depth);
                        output.push_str("**");
                    }
                    "em" | "i" => {
                        output.push('*');
                        render_markdown_node(child_element, base_url, output, depth);
                        output.push('*');
                    }
                    "a" => {
                        let mut label = String::new();
                        render_markdown_node(child_element, base_url, &mut label, depth);
                        let label = label.trim().to_owned();
                        let safe_href = element
                            .attr("href")
                            .and_then(|href| safe_markdown_href(href, base_url));
                        if let Some(href) = safe_href.filter(|_| !label.is_empty()) {
                            output.push('[');
                            output.push_str(&label);
                            output.push_str("](");
                            output.push_str(&href);
                            output.push(')');
                        } else {
                            output.push_str(&label);
                        }
                    }
                    "pre" => {
                        append_block_separator(output);
                        output.push_str("```text\n");
                        render_markdown_node(child_element, base_url, output, depth);
                        output.push_str("\n```\n\n");
                    }
                    _ => render_markdown_node(child_element, base_url, output, depth),
                }
            }
            _ => {}
        }
    }
}

fn append_block_separator(output: &mut String) {
    while output.ends_with(' ') || output.ends_with('\n') {
        output.pop();
    }
    if !output.is_empty() {
        output.push_str("\n\n");
    }
}

fn safe_markdown_href(raw: &str, base_url: &Url) -> Option<String> {
    let mut url = base_url.join(raw).ok()?;
    let host = url.host_str()?.to_ascii_lowercase();
    if url.scheme() != "https"
        || !url.username().is_empty()
        || url.password().is_some()
        || url.port_or_known_default() != Some(443)
        || !(host == "uva.instructure.com" || host == "uva.br" || host.ends_with(".uva.br"))
    {
        return None;
    }
    url.set_query(None);
    url.set_fragment(None);
    Some(url.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn course_pagination_rejects_external_and_repeated_links() {
        let page = "https://uva.instructure.com/courses?page=2";
        let visited = HashSet::from([page.to_owned()]);

        assert_eq!(
            validate_next_course_href(page, "https://attacker.example/courses?page=3", &visited),
            None
        );
        assert_eq!(validate_next_course_href(page, "?page=2", &visited), None);
        assert_eq!(
            validate_next_course_href(page, "?page=3", &visited),
            Some("https://uva.instructure.com/courses?page=3".to_owned())
        );
    }

    #[test]
    fn course_listing_reports_configured_page_limit_as_partial() {
        let first = CanvasCoursePage {
            courses: vec![Course {
                id: "41".into(),
                title: "Bases de Programação".into(),
                url: "https://uva.instructure.com/courses/41".into(),
                kind: CourseKind::Current,
            }],
            next_href: Some("https://uva.instructure.com/courses?page=2".into()),
        };
        let mut accumulator = CourseListingAccumulator::new(1);

        assert_eq!(
            accumulator.push_page("https://uva.instructure.com/courses", first),
            None
        );
        let listing = accumulator.finish();
        assert_eq!(listing.pages_traversed, 1);
        assert_eq!(listing.courses.len(), 1);
        assert!(!listing.complete);
        assert!(listing.warning.unwrap().contains("limite"));
    }

    #[test]
    fn parse_module_items_preserves_visible_order_and_excludes_assessments() {
        let html = r#"
            <div class="context_module" data-module-id="5">
              <h2 class="ig-header-title">Unidade 1</h2>
              <div class="ig-row"><a href="/courses/41/pages/aula-1">Aula 1</a></div>
              <div class="ig-row"><a href="/courses/41/files/82">Texto de apoio</a></div>
              <div class="ig-row"><a href="/courses/41/quizzes/9">Quiz</a></div>
              <div class="ig-row locked"><a href="/courses/41/pages/aula-bloqueada">Aula bloqueada</a></div>
            </div>
        "#;

        let items = parse_module_items(html);
        assert_eq!(
            items
                .iter()
                .map(|item| item.title.as_str())
                .collect::<Vec<_>>(),
            ["Aula 1", "Texto de apoio"]
        );
        assert_eq!(items[0].module_title, "Unidade 1");
    }

    #[test]
    fn inventory_excludes_hidden_unpublished_and_assessment_items() {
        let html = r#"
            <div class="context_module" data-module-id="5">
              <h2 class="ig-header-title">Unidade 1</h2>
              <div class="ig-row unpublished"><a href="/courses/41/pages/rascunho">Rascunho</a></div>
              <div class="ig-row"><a aria-disabled="true" href="/courses/41/pages/bloqueada">Bloqueada</a></div>
              <div class="ig-row"><a href="/courses/41/assignments/8">Entrega</a></div>
              <div class="ig-row"><a href="/courses/41/pages/publicada">Publicada</a></div>
            </div>
        "#;

        let items = parse_module_items(html);
        assert_eq!(items.len(), 1);
        assert_eq!(items[0].title, "Publicada");
    }

    #[test]
    fn parse_module_items_includes_official_external_pdf_links() {
        let html = r#"
            <div class="context_module" data-module-id="5">
              <h2 class="ig-header-title">Unidade 1</h2>
              <div class="ig-row"><a href="https://ead.uva.br/disciplinas/grad/material.pdf?signature=temporaria">Apostila</a></div>
              <div class="ig-row"><a href="https://attacker.example/material.pdf">Link externo</a></div>
            </div>
        "#;

        let items = parse_module_items(html);
        assert_eq!(items.len(), 1);
        assert_eq!(items[0].title, "Apostila");
        assert_eq!(items[0].item_type, "file");
    }

    #[test]
    fn parse_canvas_page_drops_scripts_and_preserves_headings_lists_and_safe_links() {
        let html = r#"
            <main id="content"><h1>Desenvolvimento Frontend I</h1>
              <div class="user_content">
                <h2>Unidade 2</h2><p>Texto introdutório.</p>
                <script>window.secret = "nao executar";</script>
                <ul><li>Primeiro tópico</li><li>Segundo tópico</li></ul>
                <p><a href="/courses/41/pages/proxima?access_token=segredo#parte">Próxima aula</a>
                   <a href="javascript:alert(1)">Link perigoso</a></p>
                <iframe title="Material de apoio" src="https://ead.uva.br/aula.pdf?signature=temporaria"></iframe>
              </div>
            </main>
        "#;

        let content = parse_canvas_page(
            html,
            "https://uva.instructure.com/courses/41/pages/frontend-u2",
        )
        .unwrap();
        let markdown = content.markdown.unwrap();
        assert!(markdown.contains("# Desenvolvimento Frontend I"));
        assert!(markdown.contains("## Unidade 2"));
        assert!(markdown.contains("- Primeiro tópico"));
        assert!(markdown.contains("Próxima aula"));
        assert!(!markdown.contains("window.secret"));
        assert!(!markdown.contains("javascript:"));
        assert!(!markdown.contains("access_token"));
        assert_eq!(content.pdf_links.len(), 1);
    }

    #[test]
    fn content_key_drops_query_and_fragment() {
        let first = content_key(
            None,
            "https://uva.instructure.com/courses/41/files/82?download_token=segredo#page=2",
            "pdf",
        )
        .unwrap();
        let second = content_key(
            None,
            "https://uva.instructure.com/courses/41/files/82?download_token=rotacionado#page=5",
            "pdf",
        )
        .unwrap();

        assert_eq!(first, second);
        assert!(!first.contains("segredo"));
        assert!(!first.contains("rotacionado"));
    }

    #[test]
    fn module_item_url_rejects_assessment_cross_course_and_external_routes() {
        assert!(module_item_url("41", "/courses/41/modules/items/7").is_some());
        assert!(module_item_url("41", "/courses/41/pages/aula-1").is_some());
        assert!(module_item_url("41", "/courses/41/quizzes/8").is_none());
        assert!(module_item_url("41", "/courses/42/pages/outra-materia").is_none());
        assert!(module_item_url("41", "/courses/42/pages/outra-materia.pdf").is_none());
        assert!(module_item_url("41", "https://attacker.example/courses/41/pages/aula").is_none());
    }
}
