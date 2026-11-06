use crate::{
    anthropic::{AnthropicClient, AnthropicConfig, AnthropicError, SummaryPair, SummarySource},
    catalog::{classification::classify_path, scanner::CatalogService},
    storage::{AcademicStore, ImportedMaterial, StudySummary},
};
use serde::Serialize;
use sha2::{Digest, Sha256};
use std::{
    collections::BTreeMap,
    future::Future,
    path::{Component, Path},
    sync::Arc,
};
use tauri::State;
use unicode_normalization::{UnicodeNormalization, char::is_combining_mark};

const MAX_SUMMARY_SOURCE_BYTES: usize = 2 * 1024 * 1024;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SummarySourcePreview {
    pub relative_path: String,
    pub title: String,
    pub format: String,
    pub size_bytes: u64,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SubjectSummarySnapshot {
    pub sources: Vec<SummarySourcePreview>,
    pub current_source_hash: String,
    pub latest_summary: Option<StudySummary>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct SummarySourceManifestEntry {
    relative_path: String,
    title: String,
    content_sha256: String,
}

struct CollectedSubjectSources {
    previews: Vec<SummarySourcePreview>,
    sources: Vec<SummarySource>,
    source_manifest_json: String,
    current_source_hash: String,
}

pub fn collect_subject_sources(
    catalog: &CatalogService,
    store: &AcademicStore,
    course_title: &str,
    semester: &str,
) -> Result<Vec<SummarySource>, String> {
    validate_subject(course_title, semester)?;
    let materials = store.list_materials().map_err(|error| error.to_string())?;
    let root = store.library_path().map_err(|error| error.to_string())?;
    Ok(collect_subject_source_set(catalog, &materials, &root, course_title, semester)?.sources)
}

fn normalize_subject(value: &str) -> String {
    value
        .nfd()
        .filter(|character| !is_combining_mark(*character))
        .flat_map(char::to_lowercase)
        .filter(|character| character.is_ascii_alphanumeric() || character.is_whitespace())
        .collect::<String>()
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
}

fn validate_subject(course_title: &str, semester: &str) -> Result<(), String> {
    if course_title.trim().is_empty() || course_title.chars().count() > 300 {
        return Err("O nome da matéria é inválido.".into());
    }
    if semester.trim().is_empty() || semester.len() > 32 {
        return Err("O semestre da matrícula ativa é inválido.".into());
    }
    Ok(())
}

fn collect_subject_source_set(
    catalog: &CatalogService,
    imported_materials: &[ImportedMaterial],
    library_path: &Path,
    course_title: &str,
    semester: &str,
) -> Result<CollectedSubjectSources, String> {
    validate_subject(course_title, semester)?;
    let wanted_course = normalize_subject(course_title);
    let canonical_root = library_path
        .canonicalize()
        .map_err(|_| "Não foi possível acessar a pasta do acervo.".to_owned())?;
    let snapshot = catalog.snapshot()?;
    if Path::new(&snapshot.root).canonicalize().ok().as_deref() != Some(canonical_root.as_path()) {
        return Err(
            "A pasta do catálogo local mudou durante a leitura; atualize e tente novamente.".into(),
        );
    }
    let mut candidates = BTreeMap::<String, (SummarySourcePreview, SummarySource)>::new();
    let mut total_source_bytes = 0_usize;

    for document in snapshot.documents {
        if !document.searchable
            || document.read_status != "ready"
            || document.category != "material acadêmico"
            || document.semester.as_deref() != Some(semester)
            || document
                .course
                .as_deref()
                .is_none_or(|course| normalize_subject(course) != wanted_course)
        {
            continue;
        }
        let Ok(content) = catalog.read_document(&document.relative_path) else {
            continue;
        };
        if content.markdown.trim().is_empty() {
            continue;
        }
        insert_candidate(
            &mut candidates,
            &mut total_source_bytes,
            SummarySourcePreview {
                relative_path: document.relative_path.clone(),
                title: document.title,
                format: document.format,
                size_bytes: document.size_bytes,
            },
            SummarySource {
                label: document.relative_path,
                markdown: content.markdown,
            },
        )?;
    }

    for material in imported_materials {
        if normalize_subject(&material.course_title) != wanted_course
            || material.semester.as_deref() != Some(semester)
        {
            continue;
        }
        let relative_path = imported_relative_path(&canonical_root, &material.path)?;
        if candidates.contains_key(&relative_path) {
            continue;
        }
        let classification = classify_path(Path::new(&relative_path));
        if classification.category != "material acadêmico"
            || classification.semester.as_deref() != Some(semester)
            || classification
                .course
                .as_deref()
                .is_none_or(|course| normalize_subject(course) != wanted_course)
        {
            continue;
        }
        let Ok(content) = catalog.read_document(&relative_path) else {
            continue;
        };
        if !content.document.searchable || content.markdown.trim().is_empty() {
            continue;
        }
        insert_candidate(
            &mut candidates,
            &mut total_source_bytes,
            SummarySourcePreview {
                relative_path: relative_path.clone(),
                title: content.document.title,
                format: content.document.format,
                size_bytes: content.document.size_bytes,
            },
            SummarySource {
                label: relative_path,
                markdown: content.markdown,
            },
        )?;
    }

    let mut selected = candidates.into_values().collect::<Vec<_>>();
    selected.dedup_by(|left, right| left.0.relative_path == right.0.relative_path);
    Ok(make_source_set(selected))
}

fn insert_candidate(
    candidates: &mut BTreeMap<String, (SummarySourcePreview, SummarySource)>,
    total_source_bytes: &mut usize,
    preview: SummarySourcePreview,
    source: SummarySource,
) -> Result<(), String> {
    if candidates.contains_key(&preview.relative_path) {
        return Ok(());
    }
    let next_size = total_source_bytes.saturating_add(source.markdown.len());
    if next_size > MAX_SUMMARY_SOURCE_BYTES {
        return Err(
            "Os materiais selecionados ultrapassam o limite de texto para uma geração. Reduza o conteúdo acadêmico associado à matéria e tente novamente.".into(),
        );
    }
    *total_source_bytes = next_size;
    candidates.insert(preview.relative_path.clone(), (preview, source));
    Ok(())
}

fn imported_relative_path(canonical_root: &Path, raw_path: &str) -> Result<String, String> {
    let requested = Path::new(raw_path);
    if requested.as_os_str().is_empty()
        || (!requested.is_absolute()
            && requested
                .components()
                .any(|component| !matches!(component, Component::Normal(_) | Component::CurDir)))
    {
        return Err("O caminho do material importado é inválido.".into());
    }
    let candidate = if requested.is_absolute() {
        requested.to_path_buf()
    } else {
        canonical_root.join(requested)
    };
    let canonical = candidate
        .canonicalize()
        .map_err(|_| "Não foi possível ler o material importado no acervo.".to_owned())?;
    if !canonical.starts_with(canonical_root) {
        return Err("Um material importado está fora da pasta do acervo.".into());
    }
    canonical
        .strip_prefix(canonical_root)
        .map(|path| path.to_string_lossy().replace('\\', "/"))
        .map_err(|_| "Um material importado está fora da pasta do acervo.".to_owned())
}

fn make_source_set(
    mut selected: Vec<(SummarySourcePreview, SummarySource)>,
) -> CollectedSubjectSources {
    selected.sort_by(|left, right| left.0.relative_path.cmp(&right.0.relative_path));
    selected.dedup_by(|left, right| left.0.relative_path == right.0.relative_path);
    let manifest = selected
        .iter()
        .map(|(preview, source)| SummarySourceManifestEntry {
            relative_path: preview.relative_path.clone(),
            title: preview.title.clone(),
            content_sha256: format!("{:x}", Sha256::digest(source.markdown.as_bytes())),
        })
        .collect::<Vec<_>>();
    let source_manifest_json =
        serde_json::to_string(&manifest).expect("source manifest contains serializable strings");
    let current_source_hash = format!("{:x}", Sha256::digest(source_manifest_json.as_bytes()));
    let previews = selected
        .iter()
        .map(|(preview, _)| preview.clone())
        .collect();
    let sources = selected.into_iter().map(|(_, source)| source).collect();
    CollectedSubjectSources {
        previews,
        sources,
        source_manifest_json,
        current_source_hash,
    }
}

async fn generate_and_store_with<F, Fut>(
    store: &AcademicStore,
    course_id: &str,
    course_title: &str,
    semester: &str,
    confirmed_source_hash: &str,
    collected: &CollectedSubjectSources,
    generate: F,
) -> Result<StudySummary, String>
where
    F: FnOnce(String, Vec<SummarySource>) -> Fut,
    Fut: Future<Output = Result<SummaryPair, AnthropicError>>,
{
    if collected.sources.is_empty() {
        return Err("Não há materiais legíveis para resumir.".into());
    }
    if collected.current_source_hash != confirmed_source_hash {
        return Err(
            "Os materiais desta matéria mudaram desde a confirmação. Atualize a lista e confirme novamente.".into(),
        );
    }
    let pair = generate(course_title.to_owned(), collected.sources.clone())
        .await
        .map_err(|error| error.to_string())?;
    if pair.short_text.trim().is_empty() || pair.long_text.trim().is_empty() {
        return Err("A resposta da API Anthropic não contém os dois resumos completos.".into());
    }

    let summary = StudySummary {
        id: uuid::Uuid::new_v4().to_string(),
        course_id: course_id.to_owned(),
        course_title: course_title.to_owned(),
        semester: semester.to_owned(),
        source_hash: collected.current_source_hash.clone(),
        source_manifest_json: collected.source_manifest_json.clone(),
        short_text: pair.short_text,
        long_text: pair.long_text,
        model: pair.model,
        input_tokens: pair.input_tokens,
        output_tokens: pair.output_tokens,
        created_at: String::new(),
    };
    store
        .save_study_summary(&summary)
        .map_err(|error| error.to_string())?;
    store
        .latest_study_summary(course_id, semester)
        .map_err(|error| error.to_string())?
        .ok_or_else(|| "O resumo foi salvo, mas não pôde ser recarregado.".to_owned())
}

async fn refreshed_subject_sources(
    store: &AcademicStore,
    catalog: &Arc<CatalogService>,
    course_title: &str,
    semester: &str,
) -> Result<CollectedSubjectSources, String> {
    validate_subject(course_title, semester)?;
    let imported_materials = store.list_materials().map_err(|error| error.to_string())?;
    let library_path = store.library_path().map_err(|error| error.to_string())?;
    let course_title = course_title.to_owned();
    let semester = semester.to_owned();
    let catalog = Arc::clone(catalog);
    tauri::async_runtime::spawn_blocking(move || {
        catalog.refresh()?;
        collect_subject_source_set(
            &catalog,
            &imported_materials,
            &library_path,
            &course_title,
            &semester,
        )
    })
    .await
    .map_err(|_| "Não foi possível atualizar o catálogo local.".to_owned())?
}

#[tauri::command]
pub(crate) async fn get_subject_summary_snapshot(
    course_id: String,
    course_title: String,
    semester: String,
    state: State<'_, crate::AppState>,
) -> Result<SubjectSummarySnapshot, String> {
    validate_subject(&course_title, &semester)?;
    if course_id.trim().is_empty() || course_id.len() > 256 {
        return Err("A matrícula ativa desta matéria é inválida.".into());
    }
    let collected =
        refreshed_subject_sources(&state.store, &state.catalog, &course_title, &semester).await?;
    let latest_summary = state
        .store
        .latest_study_summary(&course_id, &semester)
        .map_err(|error| error.to_string())?;
    Ok(SubjectSummarySnapshot {
        sources: collected.previews,
        current_source_hash: collected.current_source_hash,
        latest_summary,
    })
}

#[tauri::command]
pub(crate) async fn generate_subject_summaries(
    course_id: String,
    course_title: String,
    semester: String,
    confirmed_source_hash: String,
    state: State<'_, crate::AppState>,
) -> Result<StudySummary, String> {
    validate_subject(&course_title, &semester)?;
    if course_id.trim().is_empty() || course_id.len() > 256 {
        return Err("A matrícula ativa desta matéria é inválida.".into());
    }
    let collected =
        refreshed_subject_sources(&state.store, &state.catalog, &course_title, &semester).await?;
    if collected.sources.is_empty() {
        return Err("Não há materiais legíveis para resumir.".into());
    }
    if collected.current_source_hash != confirmed_source_hash {
        return Err(
            "Os materiais desta matéria mudaram desde a confirmação. Atualize a lista e confirme novamente.".into(),
        );
    }
    let client = AnthropicClient::from_config(
        AnthropicConfig::read_default().map_err(|error| error.to_string())?,
    );
    generate_and_store_with(
        &state.store,
        &course_id,
        &course_title,
        &semester,
        &confirmed_source_hash,
        &collected,
        move |subject, sources| async move { client.generate_pair(&subject, &sources).await },
    )
    .await
}

#[cfg(test)]
mod tests {
    use super::{
        CollectedSubjectSources, SummarySourcePreview, collect_subject_sources,
        generate_and_store_with, make_source_set,
    };
    use crate::{
        anthropic::{AnthropicError, SummaryPair, SummarySource},
        catalog::scanner::CatalogService,
        storage::{AcademicStore, ImportedMaterial, StudySummary},
    };
    use std::{
        path::Path,
        sync::{
            Arc,
            atomic::{AtomicUsize, Ordering},
        },
    };

    fn prepare_subject(root: &Path) -> (CatalogService, AcademicStore) {
        let library = root.join("library");
        let course = library.join("materias/fontes-uva/2026.2/bases-de-programacao");
        let previous_semester = library.join("materias/fontes-uva/2025.2/bases-de-programacao");
        std::fs::create_dir_all(&course).unwrap();
        std::fs::create_dir_all(&previous_semester).unwrap();
        std::fs::write(
            course.join("aula-local.md"),
            "Conceito local de programação.",
        )
        .unwrap();
        for official_record in [
            "registro_academico.md",
            "registro-de-notas.md",
            "notas-oficiais.md",
            "boletim-escolar.md",
            "histórico-escolar.md",
            "histórico-acadêmico.md",
            "registro-de-frequencia.md",
            "registro-de-presenca.md",
        ] {
            std::fs::write(
                course.join(official_record),
                "Informação oficial privada de notas ou frequência.",
            )
            .unwrap();
        }
        std::fs::write(
            previous_semester.join("aula-antiga.md"),
            "Conteúdo de outro semestre.",
        )
        .unwrap();
        std::fs::write(
            library.join("REGISTRO_ACADEMICO.md"),
            "Nota e frequência oficiais não são material de aula.",
        )
        .unwrap();
        let catalog = CatalogService::new(library.clone()).unwrap();
        let store = AcademicStore::open(&root.join("app.sqlite3"), library).unwrap();
        (catalog, store)
    }

    fn imported_material(path: String, lesson_path: &str) -> ImportedMaterial {
        ImportedMaterial {
            id: "canvas-material-1".into(),
            course_id: "canvas-course".into(),
            course_title: "Bases de Programação".into(),
            module_title: "Unidade 1".into(),
            title: "Material importado".into(),
            path,
            lesson_path: lesson_path.into(),
            origin: "canvas:file:1".into(),
            pages: 1,
            semester: Some("2026.2".into()),
            imported_at: "2026-10-15T12:00:00Z".into(),
            ..ImportedMaterial::default()
        }
    }

    fn test_summary(id: &str, source_hash: &str) -> StudySummary {
        StudySummary {
            id: id.into(),
            course_id: "enrollment-1".into(),
            course_title: "Bases de Programação".into(),
            semester: "2026.2".into(),
            source_hash: source_hash.into(),
            source_manifest_json: "[]".into(),
            short_text: "Resumo curto anterior".into(),
            long_text: "Resumo longo anterior".into(),
            model: "claude-haiku-5-5".into(),
            input_tokens: 100,
            output_tokens: 800,
            created_at: "2026-10-14T12:00:00Z".into(),
        }
    }

    fn source_set(items: &[(&str, &str, &str)]) -> CollectedSubjectSources {
        make_source_set(
            items
                .iter()
                .map(|(path, title, text)| {
                    (
                        SummarySourcePreview {
                            relative_path: (*path).into(),
                            title: (*title).into(),
                            format: "md".into(),
                            size_bytes: text.len() as u64,
                        },
                        SummarySource {
                            label: (*path).into(),
                            markdown: (*text).into(),
                        },
                    )
                })
                .collect(),
        )
    }

    #[test]
    fn collects_only_readable_material_for_matching_course_and_semester() {
        let temporary = tempfile::tempdir().unwrap();
        let (catalog, store) = prepare_subject(temporary.path());
        let root = store.library_path().unwrap();
        let course = root.join("materias/fontes-uva/2026.2/bases-de-programacao");
        let imported_path = course.join("canvas-importado.md");
        std::fs::write(&imported_path, "Conteúdo importado do Canvas.").unwrap();
        let unreadable_path = course.join("aula-inacessivel.md");
        std::fs::write(
            &unreadable_path,
            "Este arquivo ficará indisponível após a atualização.",
        )
        .unwrap();
        store
            .save_material(
                &imported_material(
                    "materias/fontes-uva/2026.2/bases-de-programacao/canvas-importado.md".into(),
                    "materias/fontes-uva/2026.2/bases-de-programacao/aula-derivada.md",
                ),
                "fixture-hash",
            )
            .unwrap();
        catalog.refresh().unwrap();
        std::fs::remove_file(unreadable_path).unwrap();

        let sources =
            collect_subject_sources(&catalog, &store, "BASES DE PROGRAMACAO", "2026.2").unwrap();

        let paths = sources
            .iter()
            .map(|source| source.label.as_str())
            .collect::<Vec<_>>();
        assert_eq!(
            paths,
            [
                "materias/fontes-uva/2026.2/bases-de-programacao/aula-local.md",
                "materias/fontes-uva/2026.2/bases-de-programacao/canvas-importado.md",
            ]
        );
        assert!(
            sources
                .iter()
                .any(|source| source.markdown.contains("Conteúdo importado do Canvas"))
        );
        assert!(!sources.iter().any(|source| {
            source.label.contains("2025.2")
                || source.label.contains("REGISTRO_ACADEMICO")
                || source.label.contains("registro-de-notas")
                || source.label.contains("notas-oficiais")
                || source.label.contains("boletim-escolar")
                || source.label.contains("histórico-escolar")
                || source.label.contains("histórico-acadêmico")
                || source.label.contains("registro-de-frequencia")
                || source.label.contains("registro-de-presenca")
                || source.label.contains("inacessivel")
                || source.label.contains("aula-derivada")
        }));
    }

    #[test]
    fn rejects_imported_material_outside_library_root() {
        let temporary = tempfile::tempdir().unwrap();
        let (catalog, store) = prepare_subject(temporary.path());
        let outside = temporary.path().join("fora-do-acervo.md");
        std::fs::write(&outside, "Não enviar este conteúdo.").unwrap();
        store
            .save_material(
                &imported_material(outside.display().to_string(), "aula.md"),
                "hash",
            )
            .unwrap();
        catalog.refresh().unwrap();

        let error = collect_subject_sources(&catalog, &store, "Bases de Programação", "2026.2")
            .unwrap_err();
        assert!(error.contains("fora da pasta"));
    }

    #[test]
    fn source_hash_is_order_independent_and_content_sensitive() {
        let first = source_set(&[
            ("materias/b.md", "B", "Conteúdo B"),
            ("materias/a.md", "A", "Conteúdo A"),
        ]);
        let reordered = source_set(&[
            ("materias/a.md", "A", "Conteúdo A"),
            ("materias/b.md", "B", "Conteúdo B"),
        ]);
        let edited = source_set(&[
            ("materias/b.md", "B", "Conteúdo B atualizado"),
            ("materias/a.md", "A", "Conteúdo A"),
        ]);

        assert_eq!(first.current_source_hash, reordered.current_source_hash);
        assert_ne!(first.current_source_hash, edited.current_source_hash);
        assert!(!first.source_manifest_json.contains("Conteúdo A"));
    }

    #[tokio::test]
    async fn changed_source_hash_requires_new_confirmation() {
        let temporary = tempfile::tempdir().unwrap();
        let library = temporary.path().join("library");
        let store = AcademicStore::open(&temporary.path().join("app.sqlite3"), library).unwrap();
        let current = source_set(&[("materias/a.md", "A", "Conteúdo atual")]);
        let calls = Arc::new(AtomicUsize::new(0));
        let generator_calls = Arc::clone(&calls);

        let error = generate_and_store_with(
            &store,
            "enrollment-1",
            "Bases de Programação",
            "2026.2",
            "hash-confirmado-anteriormente",
            &current,
            move |_, _| async move {
                generator_calls.fetch_add(1, Ordering::SeqCst);
                Err(AnthropicError::Request)
            },
        )
        .await
        .unwrap_err();

        assert!(error.contains("mudaram"));
        assert_eq!(calls.load(Ordering::SeqCst), 0);
    }

    #[tokio::test]
    async fn no_sources_prevents_api_generation() {
        let temporary = tempfile::tempdir().unwrap();
        let library = temporary.path().join("library");
        let store = AcademicStore::open(&temporary.path().join("app.sqlite3"), library).unwrap();
        let empty = source_set(&[]);
        let calls = Arc::new(AtomicUsize::new(0));
        let generator_calls = Arc::clone(&calls);

        let error = generate_and_store_with(
            &store,
            "enrollment-1",
            "Bases de Programação",
            "2026.2",
            &empty.current_source_hash,
            &empty,
            move |_, _| async move {
                generator_calls.fetch_add(1, Ordering::SeqCst);
                Err(AnthropicError::Request)
            },
        )
        .await
        .unwrap_err();

        assert!(error.contains("materiais legíveis"));
        assert_eq!(calls.load(Ordering::SeqCst), 0);
    }

    #[tokio::test]
    async fn failed_generation_keeps_previous_summary() {
        let temporary = tempfile::tempdir().unwrap();
        let library = temporary.path().join("library");
        let store = AcademicStore::open(&temporary.path().join("app.sqlite3"), library).unwrap();
        store
            .save_study_summary(&test_summary("summary-previous", "previous-hash"))
            .unwrap();
        let current = source_set(&[("materias/a.md", "A", "Conteúdo novo")]);

        let error = generate_and_store_with(
            &store,
            "enrollment-1",
            "Bases de Programação",
            "2026.2",
            &current.current_source_hash,
            &current,
            |_, _| async { Err(AnthropicError::Request) },
        )
        .await
        .unwrap_err();

        assert!(error.contains("API Anthropic"));
        let saved = store
            .latest_study_summary("enrollment-1", "2026.2")
            .unwrap()
            .unwrap();
        assert_eq!(saved.id, "summary-previous");
        assert_eq!(saved.source_hash, "previous-hash");
    }

    #[tokio::test]
    async fn successful_generation_persists_complete_pair_and_utc_timestamp() {
        let temporary = tempfile::tempdir().unwrap();
        let library = temporary.path().join("library");
        let store = AcademicStore::open(&temporary.path().join("app.sqlite3"), library).unwrap();
        let current = source_set(&[("materias/a.md", "A", "Conteúdo confirmado")]);

        let summary = generate_and_store_with(
            &store,
            "enrollment-1",
            "Bases de Programação",
            "2026.2",
            &current.current_source_hash,
            &current,
            |_, _| async {
                Ok(SummaryPair {
                    short_text: "Resumo curto completo".into(),
                    long_text: "Resumo longo completo".into(),
                    input_tokens: 120,
                    output_tokens: 900,
                    model: "claude-haiku-5-5".into(),
                })
            },
        )
        .await
        .unwrap();

        assert_eq!(summary.source_hash, current.current_source_hash);
        assert_eq!(summary.short_text, "Resumo curto completo");
        assert_eq!(summary.long_text, "Resumo longo completo");
        assert_eq!(summary.input_tokens, 120);
        assert_eq!(summary.output_tokens, 900);
        assert!(summary.created_at.contains('T'));
        assert!(summary.created_at.ends_with('Z'));
        assert!(summary.source_manifest_json.contains("materias/a.md"));
        assert!(!summary.source_manifest_json.contains("Conteúdo confirmado"));
    }
}
