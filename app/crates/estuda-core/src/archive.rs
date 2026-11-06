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
    #[error("origem acadêmica fora da lista HTTPS permitida")]
    UnsafeOrigin,
    #[error("o PDF não tem texto suficiente para montar uma aula; o original não foi guardado")]
    NoText,
    #[error("o conteúdo recebido não corresponde à assinatura de um PDF")]
    InvalidPdf,
    #[error(transparent)]
    Storage(#[from] StorageError),
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ImportedBundle {
    pub material: ImportedMaterial,
    pub lesson_markdown: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArchiveImportStatus {
    Captured,
    Reused,
    NeedsReview,
}

#[derive(Debug, Clone)]
pub struct ArchiveImportOutcome {
    pub bundle: ImportedBundle,
    pub status: ArchiveImportStatus,
    pub message: Option<String>,
}

pub fn reuse_existing_material(
    store: &AcademicStore,
    course: &Course,
    semester: Option<&str>,
    origin: &str,
) -> Result<Option<ImportedBundle>, ArchiveError> {
    crate::download::validate_material_url(origin).map_err(|_| ArchiveError::UnsafeOrigin)?;
    let clean_origin = normalize_origin(origin);
    let Some(mut material) = store.find_material_by_origin(&course.id, &clean_origin)? else {
        return Ok(None);
    };
    let library_root = store.library_path()?;
    let Some(material_markdown) = read_existing_file_beneath(&library_root, &material.path) else {
        return Ok(None);
    };
    if material.text_sha256.is_some() && material.sha256 != hex_digest(material_markdown.as_bytes())
    {
        return Ok(None);
    }
    let Some(lesson_markdown) = read_existing_file_beneath(&library_root, &material.lesson_path)
    else {
        return Ok(None);
    };
    if let Some(semester) = semester
        && material.semester.as_deref() != Some(semester)
    {
        store.set_material_semester(&material.id, semester)?;
        material.semester = Some(semester.to_owned());
        refresh_indexes(store, &library_root.join("materias"))?;
    }
    Ok(Some(ImportedBundle {
        material,
        lesson_markdown,
    }))
}

fn read_existing_file_beneath(root: &Path, path: &str) -> Option<String> {
    let root = fs::canonicalize(root).ok()?;
    let path = fs::canonicalize(path).ok()?;
    if !path.starts_with(root) || !path.is_file() {
        return None;
    }
    fs::read_to_string(path).ok()
}

fn normalize_origin(origin: &str) -> String {
    Url::parse(origin)
        .ok()
        .map(|mut url| {
            let _ = url.set_username("");
            let _ = url.set_password(None);
            url.set_query(None);
            url.set_fragment(None);
            url.to_string()
        })
        .unwrap_or_default()
}

pub fn import_pdf(
    store: &AcademicStore,
    course: &Course,
    module_title: &str,
    title: &str,
    semester: Option<&str>,
    origin: &str,
    pdf_bytes: &[u8],
    pages: &[String],
) -> Result<ImportedBundle, ArchiveError> {
    import_pdf_with_metadata(
        store,
        course,
        module_title,
        title,
        semester,
        origin,
        None,
        None,
        None,
        pdf_bytes,
        pages,
    )
    .map(|outcome| outcome.bundle)
}

pub fn import_pdf_with_metadata(
    store: &AcademicStore,
    course: &Course,
    module_title: &str,
    title: &str,
    semester: Option<&str>,
    origin: &str,
    source_validator_host: Option<&str>,
    source_etag: Option<&str>,
    source_last_modified: Option<&str>,
    pdf_bytes: &[u8],
    pages: &[String],
) -> Result<ArchiveImportOutcome, ArchiveError> {
    if !pdf_bytes.starts_with(b"%PDF-") {
        return Err(ArchiveError::InvalidPdf);
    }
    if pages.iter().all(|page| page.trim().len() < 6) {
        return Err(ArchiveError::NoText);
    }
    let source_url =
        crate::canvas::sanitize_source_url(origin).ok_or(ArchiveError::UnsafeOrigin)?;
    let source_key = pdf_source_key(origin);
    let extracted_text = pages.join("\n\n");
    let markdown = with_capture_metadata(
        store,
        course,
        module_title,
        title,
        &source_url,
        "PDF convertido localmente",
        &pages_to_markdown(title, &source_url, pages),
    )?;
    let page_refs = pages
        .iter()
        .enumerate()
        .map(|(index, page)| ((index + 1) as u32, page.as_str()))
        .collect::<Vec<_>>();
    let (lesson_body, questions) = generate_lesson(&course.title, &page_refs);
    let lesson = with_capture_metadata(
        store,
        course,
        module_title,
        title,
        &source_url,
        "Aula derivada do PDF",
        &lesson_body,
    )?;
    persist_material_version(
        store,
        course,
        module_title,
        title,
        semester,
        &source_url,
        &source_key,
        "pdf",
        pages.len(),
        &extracted_text,
        &markdown,
        Some(&lesson),
        Some(&questions),
        source_etag,
        source_last_modified,
        source_validator_host,
    )
}

pub fn import_canvas_page(
    store: &AcademicStore,
    course: &Course,
    module_title: &str,
    title: &str,
    semester: Option<&str>,
    origin: &str,
    markdown: &str,
) -> Result<ArchiveImportOutcome, ArchiveError> {
    let source_url =
        crate::canvas::sanitize_source_url(origin).ok_or(ArchiveError::UnsafeOrigin)?;
    let markdown = sanitize_lesson_markdown(markdown);
    let extracted_text = markdown.trim();
    if extracted_text
        .chars()
        .filter(|character| !character.is_whitespace())
        .count()
        < 6
    {
        return Err(ArchiveError::NoText);
    }
    let source_key = crate::canvas::content_key(None, &source_url, "canvas_page")
        .ok_or(ArchiveError::UnsafeOrigin)?;
    let persisted_markdown = with_capture_metadata(
        store,
        course,
        module_title,
        title,
        &source_url,
        "Página textual do Canvas",
        &markdown,
    )?;
    persist_material_version(
        store,
        course,
        module_title,
        title,
        semester,
        &source_url,
        &source_key,
        "canvas_page",
        1,
        extracted_text,
        &persisted_markdown,
        None,
        None,
        None,
        None,
        None,
    )
}

pub fn inspect_existing_material(
    store: &AcademicStore,
    source_key: &str,
) -> Result<Option<ImportedMaterial>, ArchiveError> {
    let Some(material) = store.find_latest_material_by_source_key(source_key)? else {
        return Ok(None);
    };
    let library_root = store.library_path()?;
    let Some(markdown) = read_existing_file_beneath(&library_root, &material.path) else {
        return Ok(None);
    };
    if material.text_sha256.is_some() && material.sha256 != hex_digest(markdown.as_bytes()) {
        return Ok(None);
    }
    Ok(Some(material))
}

pub fn reuse_material_for_capture(
    store: &AcademicStore,
    existing: &ImportedMaterial,
    course: &Course,
    module_title: &str,
    title: &str,
    semester: Option<&str>,
    validator_host: Option<&str>,
    etag: Option<&str>,
    last_modified: Option<&str>,
) -> Result<Option<ArchiveImportOutcome>, ArchiveError> {
    let library_root = store.library_path()?;
    let Some(material_markdown) = read_existing_file_beneath(&library_root, &existing.path) else {
        return Ok(None);
    };
    if existing.text_sha256.is_some() && existing.sha256 != hex_digest(material_markdown.as_bytes())
    {
        return Ok(None);
    }
    store.save_material_association(&material_association(
        &existing.id,
        course,
        module_title,
        title,
        semester,
    ))?;
    let mut updated = existing.clone();
    if existing.course_id == course.id
        && semester.is_some()
        && existing.semester.as_deref() != semester
    {
        store.set_material_semester(&existing.id, semester.unwrap_or_default())?;
        updated.semester = semester.map(str::to_owned);
    }
    if validator_host.is_some() || etag.is_some() || last_modified.is_some() {
        updated.source_validator_host = validator_host.map(str::to_owned);
        updated.source_etag = etag.map(str::to_owned);
        updated.source_last_modified = last_modified.map(str::to_owned);
        store.save_material(&updated, &updated.sha256)?;
    }
    refresh_indexes(store, &library_root.join("materias"))?;
    let lesson_markdown = read_existing_file_beneath(&library_root, &existing.lesson_path)
        .unwrap_or(material_markdown);
    Ok(Some(ArchiveImportOutcome {
        bundle: ImportedBundle {
            material: store.find_material(&existing.id)?.unwrap_or(updated),
            lesson_markdown,
        },
        status: ArchiveImportStatus::Reused,
        message: None,
    }))
}

fn persist_material_version(
    store: &AcademicStore,
    course: &Course,
    module_title: &str,
    title: &str,
    semester: Option<&str>,
    source_url: &str,
    source_key: &str,
    source_kind: &str,
    pages: usize,
    extracted_text: &str,
    markdown: &str,
    lesson: Option<&str>,
    questions: Option<&[crate::lesson::LessonQuestion]>,
    source_etag: Option<&str>,
    source_last_modified: Option<&str>,
    source_validator_host: Option<&str>,
) -> Result<ArchiveImportOutcome, ArchiveError> {
    let text_sha256 = hex_digest(extracted_text.as_bytes());
    let markdown_sha256 = hex_digest(markdown.as_bytes());
    let latest = store.find_latest_material_by_source_key(source_key)?;
    if let Some(mut existing) =
        store.find_material_by_source_key_and_text_sha256(source_key, &text_sha256)?
    {
        let library_root = store.library_path()?;
        if let Some(existing_markdown) = read_existing_file_beneath(&library_root, &existing.path) {
            let intact = existing.sha256 == hex_digest(existing_markdown.as_bytes());
            if intact {
                store.save_material_association(&material_association(
                    &existing.id,
                    course,
                    module_title,
                    title,
                    semester,
                ))?;
                if existing.course_id == course.id
                    && semester.is_some()
                    && existing.semester.as_deref() != semester
                {
                    store.set_material_semester(&existing.id, semester.unwrap_or_default())?;
                    existing.semester = semester.map(str::to_owned);
                }
                if source_validator_host.is_some()
                    || source_etag.is_some()
                    || source_last_modified.is_some()
                {
                    existing.source_validator_host = source_validator_host.map(str::to_owned);
                    existing.source_etag = source_etag.map(str::to_owned);
                    existing.source_last_modified = source_last_modified.map(str::to_owned);
                    store.save_material(&existing, &existing.sha256)?;
                }
                refresh_indexes(store, &store.library_path()?.join("materias"))?;
                let lesson_markdown =
                    read_existing_file_beneath(&library_root, &existing.lesson_path)
                        .unwrap_or_else(|| existing_markdown.clone());
                return Ok(ArchiveImportOutcome {
                    bundle: ImportedBundle {
                        material: store.find_material(&existing.id)?.unwrap_or(existing),
                        lesson_markdown,
                    },
                    status: ArchiveImportStatus::Reused,
                    message: None,
                });
            }
        }
    }

    let library_root = store.library_path()?;
    let local_edit_detected = latest.as_ref().is_some_and(|material| {
        material.text_sha256.is_some()
            && read_existing_file_beneath(&library_root, &material.path)
                .is_some_and(|contents| material.sha256 != hex_digest(contents.as_bytes()))
    });
    let changed_source = latest
        .as_ref()
        .and_then(|material| material.text_sha256.as_deref())
        .is_some_and(|previous_hash| previous_hash != text_sha256);
    let needs_review = local_edit_detected || changed_source;
    let root = library_root.join("materias");
    let course_directory_name = format!("{}-{}", safe_slug(&course.title), safe_slug(&course.id));
    let course_dir = root.join(course_directory_name);
    let material_dir = course_dir.join("materiais");
    fs::create_dir_all(&material_dir).map_err(io_error)?;

    let version_suffix = if needs_review {
        format!("-{}", &text_sha256[..12])
    } else {
        String::new()
    };
    let markdown_path = write_new_or_equal(
        &material_dir.join(format!("{}{}.md", safe_slug(title), version_suffix)),
        markdown,
    )?;
    let lesson_content = lesson.unwrap_or(markdown);
    let lesson_target = if lesson.is_none() {
        markdown_path.clone()
    } else {
        let primary = course_dir.join("AULA.md");
        let lesson_name = if needs_review {
            format!("AULA-{}-{}.md", safe_slug(title), &text_sha256[..12])
        } else if primary.exists() {
            format!("AULA-{}.md", safe_slug(title))
        } else {
            "AULA.md".into()
        };
        course_dir.join(lesson_name)
    };
    let lesson_path = if lesson.is_none() {
        markdown_path.clone()
    } else {
        write_new_or_equal(&lesson_target, lesson_content)?
    };
    let material_id_base = hex_digest(format!("{source_key}\0{text_sha256}").as_bytes());
    let id = if local_edit_detected
        && latest
            .as_ref()
            .is_some_and(|material| material.text_sha256.as_deref() == Some(text_sha256.as_str()))
    {
        format!("{}-{}", material_id_base, Uuid::new_v4().simple())
    } else {
        material_id_base
    };
    let material = ImportedMaterial {
        id: id.clone(),
        course_id: course.id.clone(),
        course_title: course.title.clone(),
        module_title: module_title.to_owned(),
        title: title.to_owned(),
        path: markdown_path.display().to_string(),
        lesson_path: lesson_path.display().to_string(),
        origin: source_url.to_owned(),
        pages,
        sha256: markdown_sha256.clone(),
        semester: semester.map(str::to_owned),
        imported_at: String::new(),
        source_key: source_key.to_owned(),
        source_kind: source_kind.to_owned(),
        source_validator_host: source_validator_host.map(str::to_owned),
        source_etag: source_etag.map(str::to_owned),
        source_last_modified: source_last_modified.map(str::to_owned),
        text_sha256: Some(text_sha256),
        associations: Vec::new(),
    };
    store.save_material(&material, &markdown_sha256)?;
    if let Some(questions) = questions {
        store.save_questions(&id, questions)?;
    }
    refresh_indexes(store, &root)?;
    let saved_material = store.find_material(&id)?.unwrap_or(material);
    Ok(ArchiveImportOutcome {
        bundle: ImportedBundle {
            material: saved_material,
            lesson_markdown: lesson_content.to_owned(),
        },
        status: if needs_review {
            ArchiveImportStatus::NeedsReview
        } else {
            ArchiveImportStatus::Captured
        },
        message: needs_review.then(|| {
            "A origem mudou ou há uma edição local; a nova versão foi preservada para revisão."
                .into()
        }),
    })
}

fn material_association(
    material_id: &str,
    course: &Course,
    module_title: &str,
    title: &str,
    semester: Option<&str>,
) -> crate::canvas_capture::MaterialAssociation {
    crate::canvas_capture::MaterialAssociation {
        material_id: material_id.to_owned(),
        course_id: course.id.clone(),
        course_title: course.title.clone(),
        module_title: module_title.to_owned(),
        item_title: title.to_owned(),
        semester: semester.map(str::to_owned),
    }
}

fn pdf_source_key(source_url: &str) -> String {
    let file_id = Url::parse(source_url).ok().and_then(|url| {
        url.path_segments()?
            .collect::<Vec<_>>()
            .windows(2)
            .find_map(|parts| {
                (parts[0] == "files" && parts[1].bytes().all(|byte| byte.is_ascii_digit()))
                    .then(|| parts[1].to_owned())
            })
    });
    crate::canvas::content_key(file_id.as_deref(), source_url, "pdf").unwrap_or_default()
}

fn with_capture_metadata(
    store: &AcademicStore,
    course: &Course,
    module_title: &str,
    title: &str,
    source_url: &str,
    source_type: &str,
    body: &str,
) -> Result<String, ArchiveError> {
    let captured_at = store.current_timestamp()?;
    let metadata = format!(
        "<!-- estuda-uva-capture -->\n> **Disciplina:** {}  \n> **Módulo:** {}  \n> **Material:** {}  \n> **Fonte:** {}  \n> **Captura:** {} · {}\n\n",
        crate::pdf::escape_markdown_text(
            &course
                .title
                .split_whitespace()
                .collect::<Vec<_>>()
                .join(" ")
        ),
        crate::pdf::escape_markdown_text(
            &module_title
                .split_whitespace()
                .collect::<Vec<_>>()
                .join(" ")
        ),
        crate::pdf::escape_markdown_text(&title.split_whitespace().collect::<Vec<_>>().join(" ")),
        source_url,
        captured_at,
        source_type,
    );
    Ok(format!("{metadata}{}", sanitize_lesson_markdown(body)))
}

fn sanitize_lesson_markdown(markdown: &str) -> String {
    let sanitized = markdown
        .lines()
        .map(|line| {
            let trimmed = line.trim_end();
            if trimmed.to_ascii_lowercase().contains("javascript:")
                || trimmed.to_ascii_lowercase().contains("data:")
            {
                trimmed.replace("javascript:", "").replace("data:", "")
            } else {
                trimmed.to_owned()
            }
        })
        .collect::<Vec<_>>()
        .join("\n");
    format!("{}\n", sanitized.trim())
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
                "- [{}]({}) — {} páginas · {} · {}\n  - Aula: [{}]({})\n",
                md_link(&material.title),
                md_path(&material_rel),
                material.pages,
                md_link(&material.module_title),
                material
                    .semester
                    .as_deref()
                    .map(|term| format!("período {term}"))
                    .unwrap_or_else(|| "período não identificado".to_owned()),
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::canvas::CourseKind;

    fn setup() -> (tempfile::TempDir, AcademicStore, Course) {
        let temporary = tempfile::tempdir().unwrap();
        let library = temporary.path().join("Faculdade");
        let store =
            AcademicStore::open(&temporary.path().join("app/estuda.sqlite3"), library).unwrap();
        let course = Course {
            id: "41".into(),
            title: "Bases de Programação".into(),
            url: "https://uva.instructure.com/courses/41".into(),
            kind: CourseKind::Current,
        };
        (temporary, store, course)
    }

    #[test]
    fn pdf_source_key_uses_the_canvas_file_id_when_present() {
        let source_url = "https://uva.instructure.com/courses/41/files/82?download=1#page=2";
        let expected = crate::canvas::content_key(Some("82"), source_url, "pdf").unwrap();

        assert_eq!(pdf_source_key(source_url), expected);
    }

    #[test]
    fn canvas_page_import_writes_searchable_markdown_and_source_metadata() {
        let (_temporary, store, course) = setup();
        let outcome = import_canvas_page(
            &store,
            &course,
            "Unidade 1",
            "Introdução",
            Some("2026.2"),
            "https://uva.instructure.com/courses/41/pages/introducao?token=do-not-store",
            "# Introdução\n\nConteúdo pesquisável da aula.\n",
        )
        .unwrap();
        let material = store
            .find_material(&outcome.bundle.material.id)
            .unwrap()
            .unwrap();
        let markdown = fs::read_to_string(&material.path).unwrap();

        assert_eq!(outcome.status, ArchiveImportStatus::Captured);
        assert!(markdown.contains("Conteúdo pesquisável da aula"));
        assert!(markdown.contains("https://uva.instructure.com/courses/41/pages/introducao"));
        assert!(!markdown.contains("do-not-store"));
        assert_eq!(material.source_kind, "canvas_page");
        assert!(material.text_sha256.is_some());
        assert_eq!(material.associations.len(), 1);
        assert_eq!(material.associations[0].course_id, "41");
    }

    #[test]
    fn unchanged_material_reuses_indexed_markdown() {
        let (_temporary, store, course) = setup();
        let first = import_canvas_page(
            &store,
            &course,
            "Unidade 1",
            "Introdução",
            None,
            "https://uva.instructure.com/courses/41/pages/introducao",
            "# Introdução\n\nTexto estável.\n",
        )
        .unwrap();
        let second = import_canvas_page(
            &store,
            &course,
            "Unidade 2",
            "Introdução",
            None,
            "https://uva.instructure.com/courses/41/pages/introducao?ver=2",
            "# Introdução\n\nTexto estável.\n",
        )
        .unwrap();

        assert_eq!(second.status, ArchiveImportStatus::Reused);
        assert_eq!(first.bundle.material.id, second.bundle.material.id);
        assert_eq!(store.list_materials().unwrap().len(), 1);
    }

    #[test]
    fn changed_source_creates_reviewable_version_without_overwrite() {
        let (_temporary, store, course) = setup();
        let first = import_canvas_page(
            &store,
            &course,
            "Unidade 1",
            "Introdução",
            None,
            "https://uva.instructure.com/courses/41/pages/introducao",
            "# Introdução\n\nVersão inicial.\n",
        )
        .unwrap();
        let first_path = PathBuf::from(&first.bundle.material.path);
        let changed = import_canvas_page(
            &store,
            &course,
            "Unidade 1",
            "Introdução",
            None,
            "https://uva.instructure.com/courses/41/pages/introducao",
            "# Introdução\n\nVersão atualizada.\n",
        )
        .unwrap();

        assert_eq!(changed.status, ArchiveImportStatus::NeedsReview);
        assert_ne!(first.bundle.material.id, changed.bundle.material.id);
        assert!(
            fs::read_to_string(first_path)
                .unwrap()
                .contains("Versão inicial")
        );
        assert!(
            fs::read_to_string(&changed.bundle.material.path)
                .unwrap()
                .contains("Versão atualizada")
        );
    }

    #[test]
    fn capture_does_not_replace_edited_local_material() {
        let (_temporary, store, course) = setup();
        let first = import_canvas_page(
            &store,
            &course,
            "Unidade 1",
            "Introdução",
            None,
            "https://uva.instructure.com/courses/41/pages/introducao",
            "# Introdução\n\nTexto original.\n",
        )
        .unwrap();
        fs::write(
            &first.bundle.material.path,
            "Anotação manual que deve permanecer.\n",
        )
        .unwrap();
        let changed = import_canvas_page(
            &store,
            &course,
            "Unidade 1",
            "Introdução",
            None,
            "https://uva.instructure.com/courses/41/pages/introducao",
            "# Introdução\n\nTexto novo.\n",
        )
        .unwrap();

        assert_eq!(changed.status, ArchiveImportStatus::NeedsReview);
        assert!(
            fs::read_to_string(&first.bundle.material.path)
                .unwrap()
                .contains("Anotação manual")
        );
        assert_ne!(first.bundle.material.path, changed.bundle.material.path);
    }

    #[test]
    fn pdf_without_selectable_text_is_ignored_with_safe_reason() {
        let (_temporary, store, course) = setup();
        let result = import_pdf(
            &store,
            &course,
            "Unidade 1",
            "PDF sem texto",
            None,
            "https://ead.uva.br/aula.pdf",
            b"%PDF-1.7 synthetic",
            &["  ".into(), "\n".into()],
        );

        assert!(matches!(result, Err(ArchiveError::NoText)));
        assert_eq!(store.list_materials().unwrap().len(), 0);
        assert!(!_temporary.path().join("Faculdade/materias").exists());
    }
}
