use crate::{
    canvas::Course,
    lesson::generate_lesson,
    pdf::{pages_to_markdown, safe_slug},
    storage::{AcademicStore, ImportedMaterial, StorageError},
};
use sha2::{Digest, Sha256};
use std::{
    fs::{self, OpenOptions},
    io::Write,
    path::{Path, PathBuf},
};
use thiserror::Error;
use url::Url;
use uuid::Uuid;

#[derive(Debug, Error)]
pub enum ArchiveError {
    #[error("falha ao gravar o acervo: {0}")]
    Io(String),
    #[error("o PDF não tem texto suficiente para montar uma aula; o original não foi guardado")]
    NoText,
    #[error(transparent)]
    Storage(#[from] StorageError),
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ImportedBundle {
    pub material: ImportedMaterial,
    pub lesson_markdown: String,
}

pub fn import_pdf(
    store: &AcademicStore,
    course: &Course,
    module_title: &str,
    title: &str,
    origin: &str,
    pdf_bytes: &[u8],
    pages: &[String],
) -> Result<ImportedBundle, ArchiveError> {
    if pages.iter().all(|page| page.trim().len() < 6) {
        return Err(ArchiveError::NoText);
    }
    let root = store.library_path()?.join("materias");
    let course_directory_name = format!("{}-{}", safe_slug(&course.title), safe_slug(&course.id));
    let course_dir = root.join(course_directory_name);
    let material_dir = course_dir.join("materiais");
    fs::create_dir_all(&material_dir).map_err(io_error)?;

    let clean_origin = Url::parse(origin)
        .ok()
        .map(|mut url| {
            url.set_query(None);
            url.set_fragment(None);
            url.to_string()
        })
        .unwrap_or_default();
    let content_hash = hex_digest(pdf_bytes);
    let id = hex_digest(format!("{}\n{}\n{}", course.id, clean_origin, content_hash).as_bytes());
    if let Some(existing) = store.find_material(&id)? {
        let lesson_markdown = fs::read_to_string(&existing.lesson_path).unwrap_or_default();
        return Ok(ImportedBundle {
            material: existing,
            lesson_markdown,
        });
    }

    let material_filename = format!("{}.md", safe_slug(title));
    let markdown_path = write_new_or_equal(
        &material_dir.join(material_filename),
        &pages_to_markdown(title, origin, pages),
    )?;
    let page_refs = pages
        .iter()
        .enumerate()
        .map(|(index, page)| ((index + 1) as u32, page.as_str()))
        .collect::<Vec<_>>();
    let (lesson, questions) = generate_lesson(&course.title, &page_refs);
    let main_lesson = course_dir.join("AULA.md");
    let lesson_target = if main_lesson.exists() {
        course_dir.join(format!("AULA-{}.md", safe_slug(title)))
    } else {
        main_lesson
    };
    let lesson_path = write_new_or_equal(&lesson_target, &lesson)?;

    let material = ImportedMaterial {
        id: id.clone(),
        course_id: course.id.clone(),
        course_title: course.title.clone(),
        module_title: module_title.to_owned(),
        title: title.to_owned(),
        path: markdown_path.display().to_string(),
        lesson_path: lesson_path.display().to_string(),
        origin: clean_origin,
        pages: pages.len(),
        imported_at: String::new(),
    };
    store.save_material(&material, &content_hash)?;
    store.save_questions(&id, &questions)?;
    refresh_indexes(store, &root)?;
    Ok(ImportedBundle {
        material: store.find_material(&id)?.unwrap_or(material),
        lesson_markdown: lesson,
    })
}

fn refresh_indexes(store: &AcademicStore, archive_root: &Path) -> Result<(), ArchiveError> {
    let materials = store.list_materials()?;
    let mut grouped = std::collections::BTreeMap::<String, Vec<ImportedMaterial>>::new();
    for material in materials {
        grouped
            .entry(material.course_id.clone())
            .or_default()
            .push(material);
    }
    for course_materials in grouped.values() {
        let first = &course_materials[0];
        let directory = archive_root.join(format!(
            "{}-{}",
            safe_slug(&first.course_title),
            safe_slug(&first.course_id)
        ));
        let mut entries = String::new();
        for material in course_materials {
            let material_path = PathBuf::from(&material.path);
            let lesson_path = PathBuf::from(&material.lesson_path);
            let material_rel = material_path
                .strip_prefix(&directory)
                .unwrap_or(&material_path)
                .to_string_lossy()
                .replace('\\', "/");
            let lesson_rel = lesson_path
                .strip_prefix(&directory)
                .unwrap_or(&lesson_path)
                .to_string_lossy()
                .replace('\\', "/");
            entries.push_str(&format!(
                "- [{}]({}) — {} páginas · {}\n  - Aula: [{}]({})\n",
                md_link(&material.title),
                md_path(&material_rel),
                material.pages,
                md_link(&material.module_title),
                md_link(&material.title),
                md_path(&lesson_rel)
            ));
        }
        upsert_section(
            &directory.join("README.md"),
            "# Materiais e aulas",
            "<!-- estuda-uva:start -->",
            "<!-- estuda-uva:end -->",
            &entries,
        )?;
    }

    let mut root_entries = String::new();
    for course_materials in grouped.values() {
        let first = &course_materials[0];
        let course_dir_name = format!(
            "{}-{}",
            safe_slug(&first.course_title),
            safe_slug(&first.course_id)
        );
        root_entries.push_str(&format!(
            "- [{}]({}/README.md) — {} material(is)\n",
            md_link(&first.course_title),
            md_path(&course_dir_name),
            course_materials.len()
        ));
    }
    upsert_section(
        &archive_root.join("README.md"),
        "# Materiais acadêmicos importados",
        "<!-- estuda-uva:start -->",
        "<!-- estuda-uva:end -->",
        &root_entries,
    )?;
    Ok(())
}

fn upsert_section(
    path: &Path,
    heading: &str,
    start: &str,
    end: &str,
    contents: &str,
) -> Result<(), ArchiveError> {
    let mut text = if path.exists() {
        fs::read_to_string(path).map_err(io_error)?
    } else {
        format!("{heading}\n\n")
    };
    let block = format!("{start}\n## Conteúdo gerado pelo Estuda UVA\n\n{contents}{end}");
    if let Some(start_index) = text.find(start) {
        if let Some(end_relative) = text[start_index..].find(end) {
            let end_index = start_index + end_relative + end.len();
            text.replace_range(start_index..end_index, &block);
        } else {
            text.truncate(start_index);
            text.push_str(&block);
        }
    } else {
        if !text.ends_with('\n') {
            text.push('\n');
        }
        text.push('\n');
        text.push_str(&block);
        text.push('\n');
    }
    replace_atomically(path, text.as_bytes())
}

fn write_new_or_equal(base: &Path, contents: &str) -> Result<PathBuf, ArchiveError> {
    for suffix in 1u32..=10_000 {
        let candidate = if suffix == 1 {
            base.to_path_buf()
        } else {
            let stem = base
                .file_stem()
                .and_then(|value| value.to_str())
                .unwrap_or("material");
            let ext = base
                .extension()
                .and_then(|value| value.to_str())
                .unwrap_or("md");
            base.with_file_name(format!("{stem}-{suffix}.{ext}"))
        };
        if candidate.exists() {
            if fs::read_to_string(&candidate).is_ok_and(|existing| existing == contents) {
                return Ok(candidate);
            }
            continue;
        }
        match create_without_overwrite(&candidate, contents.as_bytes()) {
            Ok(()) => return Ok(candidate),
            Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => continue,
            Err(error) => return Err(io_error(error)),
        }
    }
    Err(ArchiveError::Io(
        "foram encontradas muitas colisões de nome".into(),
    ))
}

fn create_without_overwrite(path: &Path, bytes: &[u8]) -> std::io::Result<()> {
    let parent = path
        .parent()
        .ok_or_else(|| std::io::Error::other("caminho sem pasta"))?;
    let temporary = parent.join(format!(".estuda-uva-{}.tmp", Uuid::new_v4()));
    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&temporary)?;
    file.write_all(bytes)?;
    file.sync_all()?;
    let result = fs::hard_link(&temporary, path);
    let _ = fs::remove_file(temporary);
    result
}

fn replace_atomically(path: &Path, bytes: &[u8]) -> Result<(), ArchiveError> {
    let parent = path
        .parent()
        .ok_or_else(|| ArchiveError::Io("caminho sem pasta".into()))?;
    let temporary = parent.join(format!(".estuda-uva-{}.tmp", Uuid::new_v4()));
    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&temporary)
        .map_err(io_error)?;
    file.write_all(bytes).map_err(io_error)?;
    file.sync_all().map_err(io_error)?;
    fs::rename(&temporary, path).map_err(io_error)
}

fn md_link(value: &str) -> String {
    value
        .replace('\\', "\\\\")
        .replace('[', "\\[")
        .replace(']', "\\]")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
}
fn md_path(value: &str) -> String {
    value
        .replace(' ', "%20")
        .replace('(', "%28")
        .replace(')', "%29")
}
fn hex_digest(value: &[u8]) -> String {
    Sha256::digest(value)
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect()
}
fn io_error(error: std::io::Error) -> ArchiveError {
    ArchiveError::Io(error.to_string())
}
