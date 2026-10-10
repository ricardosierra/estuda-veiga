use scraper::{Html, Selector};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
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
            let Some(href) = anchor.value().attr("href") else {
                continue;
            };
            let path = Url::parse(&format!(
                "https://uva.instructure.com{}",
                if href.starts_with('/') {
                    href.to_owned()
                } else {
                    format!("/{href}")
                }
            ))
            .ok()
            .map(|url| url.path().to_owned())
            .unwrap_or_default();
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
            if !(lower.contains("/modules/items/")
                || lower.contains("/files/")
                || lower.contains("/pages/")
                || lower.ends_with(".pdf"))
            {
                continue;
            }
            if !seen.insert(href.to_owned()) {
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
