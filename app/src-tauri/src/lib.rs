mod archive;
mod browser;
mod calendar;
mod canvas;
mod download;
mod lesson;
mod pdf;
mod storage;

use browser::{CanvasSession, CanvasState, CanvasStatus};
use calendar::{CalendarEvent, CalendarSyncReport, assignment_events, valid_semester};
use canvas::{Course, CourseKind, ModuleGroup};
use lesson::{PracticePrompt, PracticeResult, grade_answers};
use std::{collections::HashMap, path::PathBuf};
use storage::{AcademicStore, ImportedMaterial, Overview};
use tauri::{AppHandle, Manager, State};
use tokio::sync::{Mutex, watch};

struct AppState {
    store: AcademicStore,
    session: Mutex<Option<CanvasSession>>,
    active_imports: Mutex<HashMap<String, watch::Sender<bool>>>,
}

#[tauri::command]
async fn canvas_status(state: State<'_, AppState>) -> Result<CanvasStatus, String> {
    let session = state.session.lock().await;
    match session.as_ref() {
        Some(session) => Ok(session.status().await),
        None => Ok(CanvasStatus {
            state: CanvasState::Disconnected,
            message: "Canvas ainda não conectado.".into(),
            current_url: None,
        }),
    }
}

#[tauri::command]
async fn set_chromium_path(state: State<'_, AppState>, path: String) -> Result<(), String> {
    let selected = PathBuf::from(path);
    let path = if selected.is_dir() {
        ["Contents/MacOS/Chromium", "Contents/MacOS/Google Chrome"]
            .into_iter()
            .map(|suffix| selected.join(suffix))
            .find(|candidate| candidate.is_file())
            .ok_or_else(|| {
                "O pacote selecionado não contém um executável Chromium/Chrome conhecido."
                    .to_owned()
            })?
    } else {
        selected
    };
    if !path.is_file() {
        return Err("Selecione o executável do Chromium ou Chrome.".into());
    }
    state
        .store
        .set_setting("chromium_executable", &path.to_string_lossy())
        .map_err(error_text)
}

#[tauri::command]
async fn set_library_path(state: State<'_, AppState>, path: String) -> Result<(), String> {
    let path = PathBuf::from(path);
    if !path.is_dir() {
        return Err("Selecione uma pasta existente para o acervo.".into());
    }
    state.store.set_library_path(path).map_err(error_text)
}

#[tauri::command]
async fn connect_canvas(
    app: AppHandle,
    state: State<'_, AppState>,
) -> Result<CanvasStatus, String> {
    let mut slot = state.session.lock().await;
    if let Some(session) = slot.as_ref() {
        return Ok(session.status().await);
    }
    let executable = state
        .store
        .get_setting("chromium_executable")
        .map_err(error_text)?
        .map(PathBuf::from)
        .filter(|path| path.is_file())
        .or_else(detect_chromium)
        .ok_or_else(|| {
            "Chromium não encontrado. Escolha o executável para continuar.".to_owned()
        })?;
    let profile = app
        .path()
        .app_data_dir()
        .map_err(|error| error.to_string())?
        .join("chromium-profile");
    let session = CanvasSession::launch(&executable.to_string_lossy(), &profile)
        .await
        .map_err(error_text)?;
    let status = session.status().await;
    *slot = Some(session);
    Ok(status)
}

#[tauri::command]
async fn disconnect_canvas(state: State<'_, AppState>) -> Result<(), String> {
    if let Some(session) = state.session.lock().await.take() {
        session.close().await;
    }
    Ok(())
}

#[tauri::command]
async fn list_courses(state: State<'_, AppState>) -> Result<Vec<Course>, String> {
    let session = state.session.lock().await;
    let session = session
        .as_ref()
        .ok_or_else(|| "Conecte-se ao Canvas primeiro.".to_owned())?;
    if !matches!(session.status().await.state, CanvasState::Authenticated) {
        return Err("Entre manualmente no Canvas na janela Chromium e tente novamente.".into());
    }
    let courses = session.list_courses().await.map_err(error_text)?;
    if courses.is_empty() {
        return Err(
            "A página não apresentou disciplinas. Confira o acesso ou atualize a página do Canvas."
                .into(),
        );
    }
    state.store.save_courses(&courses).map_err(error_text)?;
    Ok(courses)
}

#[tauri::command]
async fn load_course(
    course_id: String,
    state: State<'_, AppState>,
) -> Result<Vec<ModuleGroup>, String> {
    let session = state.session.lock().await;
    session
        .as_ref()
        .ok_or_else(|| "Conecte-se ao Canvas primeiro.".to_owned())?
        .load_course(&course_id)
        .await
        .map_err(error_text)
}

#[tauri::command]
async fn open_module_item(
    course_id: String,
    href: String,
    state: State<'_, AppState>,
) -> Result<Vec<browser::MaterialChoice>, String> {
    let mut session = state.session.lock().await;
    session
        .as_mut()
        .ok_or_else(|| "Conecte-se ao Canvas primeiro.".to_owned())?
        .open_module_item(&course_id, &href)
        .await
        .map_err(error_text)
}

#[tauri::command]
fn list_calendar_events(
    semester: String,
    state: State<'_, AppState>,
) -> Result<Vec<CalendarEvent>, String> {
    if !valid_semester(&semester) {
        return Err("Informe um semestre no formato AAAA.1 ou AAAA.2.".into());
    }
    state
        .store
        .list_calendar_events(&semester)
        .map_err(error_text)
}

#[tauri::command]
async fn sync_calendar(
    semester: String,
    state: State<'_, AppState>,
) -> Result<CalendarSyncReport, String> {
    if !valid_semester(&semester) {
        return Err("Informe um semestre no formato AAAA.1 ou AAAA.2.".into());
    }
    let session = state.session.lock().await;
    let session = session
        .as_ref()
        .ok_or_else(|| "Conecte-se ao Canvas primeiro.".to_owned())?;
    if !matches!(session.status().await.state, CanvasState::Authenticated) {
        return Err("Entre manualmente no Canvas na janela Chromium e tente novamente.".into());
    }

    let courses = session
        .list_courses()
        .await
        .map_err(error_text)?
        .into_iter()
        .filter(|course| course.kind == CourseKind::Current)
        .collect::<Vec<_>>();
    if courses.is_empty() {
        return Err("O Canvas não apresentou matérias atuais para buscar prazos.".into());
    }
    state.store.save_courses(&courses).map_err(error_text)?;

    let mut report = CalendarSyncReport {
        courses_checked: 0,
        events_saved: 0,
        errors: Vec::new(),
    };
    for course in courses {
        match session.list_assignment_events(&course).await {
            Ok(assignments) => {
                let events = assignment_events(&course, &semester, assignments);
                let count = events.len();
                if let Err(error) = state
                    .store
                    .replace_course_calendar_events(&semester, &course.id, &events)
                {
                    report.errors.push(format!("{}: {}", course.title, error));
                } else {
                    report.courses_checked += 1;
                    report.events_saved += count;
                }
            }
            Err(error) => report.errors.push(format!("{}: {}", course.title, error)),
        }
    }
    Ok(report)
}

#[tauri::command]
async fn import_material(
    state: State<'_, AppState>,
    job_id: String,
    course: Course,
    module_title: String,
    title: String,
    material_id: String,
) -> Result<archive::ImportedBundle, String> {
    if uuid::Uuid::parse_str(&job_id).is_err() {
        return Err("Identificador da importação inválido.".into());
    }
    let mut cancellation = {
        let mut jobs = state.active_imports.lock().await;
        if jobs.contains_key(&job_id) {
            return Err("Esta importação já está em andamento.".into());
        }
        let (sender, receiver) = watch::channel(false);
        jobs.insert(job_id.clone(), sender);
        receiver
    };
    let result = import_material_inner(
        &state,
        &course,
        &module_title,
        &title,
        &material_id,
        &mut cancellation,
    )
    .await;
    state.active_imports.lock().await.remove(&job_id);
    result
}

async fn import_material_inner(
    state: &AppState,
    course: &Course,
    module_title: &str,
    title: &str,
    material_id: &str,
    cancellation: &mut watch::Receiver<bool>,
) -> Result<archive::ImportedBundle, String> {
    let (url, referer, cookie, cookie_host) = {
        let session = state.session.lock().await;
        let session = session
            .as_ref()
            .ok_or_else(|| "Conecte-se ao Canvas primeiro.".to_owned())?;
        let url = session.material_url(&material_id).map_err(error_text)?;
        let parsed = download::validate_material_url(&url).map_err(error_text)?;
        let host = parsed.host_str().unwrap_or_default().to_owned();
        let cookie = if host.ends_with(".amazonaws.com") {
            String::new()
        } else {
            session.cookies_for_host(&host).await.map_err(error_text)?
        };
        let referer = session.current_url().await.map_err(error_text)?;
        (url, referer, cookie, host)
    };
    let cookie = if cookie.is_empty() {
        None
    } else {
        Some((cookie_host, cookie))
    };
    let (pdf_bytes, final_url) = download::download_pdf(
        &url,
        referer.as_deref(),
        cookie
            .as_ref()
            .map(|(host, value)| (host.as_str(), value.as_str())),
        cancellation,
    )
    .await
    .map_err(error_text)?;
    if *cancellation.borrow() {
        return Err("Importação cancelada.".into());
    }
    let pages = pdf::split_pdf_pages(&pdf_bytes).map_err(error_text)?;
    if *cancellation.borrow() {
        return Err("Importação cancelada.".into());
    }
    archive::import_pdf(
        &state.store,
        &course,
        &module_title,
        &title,
        final_url.as_str(),
        &pdf_bytes,
        &pages,
    )
    .map_err(error_text)
}

#[tauri::command]
async fn cancel_import(state: State<'_, AppState>, job_id: String) -> Result<(), String> {
    let jobs = state.active_imports.lock().await;
    let sender = jobs
        .get(&job_id)
        .ok_or_else(|| "A importação não está mais em andamento.".to_owned())?;
    sender
        .send(true)
        .map_err(|_| "A importação não está mais em andamento.".to_owned())
}

#[tauri::command]
fn list_materials(state: State<'_, AppState>) -> Result<Vec<ImportedMaterial>, String> {
    state.store.list_materials().map_err(error_text)
}

#[tauri::command]
fn overview(state: State<'_, AppState>) -> Result<Overview, String> {
    state.store.overview().map_err(error_text)
}

#[tauri::command]
fn get_lesson(material_id: String, state: State<'_, AppState>) -> Result<String, String> {
    let material = state
        .store
        .find_material(&material_id)
        .map_err(error_text)?
        .ok_or_else(|| "Material não encontrado.".to_owned())?;
    std::fs::read_to_string(material.lesson_path)
        .map_err(|_| "Não foi possível ler a aula salva.".to_owned())
}

#[tauri::command]
fn start_practice(
    material_id: String,
    state: State<'_, AppState>,
) -> Result<Vec<PracticePrompt>, String> {
    let questions = state
        .store
        .load_questions(&material_id)
        .map_err(error_text)?;
    Ok(questions
        .iter()
        .enumerate()
        .map(|(position, question)| PracticePrompt {
            position,
            prompt: question.prompt.clone(),
            source_page: question.source_page,
        })
        .collect())
}

#[tauri::command]
fn submit_practice(
    material_id: String,
    answers: Vec<String>,
    state: State<'_, AppState>,
) -> Result<PracticeResult, String> {
    let material = state
        .store
        .find_material(&material_id)
        .map_err(error_text)?
        .ok_or_else(|| "Material não encontrado.".to_owned())?;
    let questions = state
        .store
        .load_questions(&material_id)
        .map_err(error_text)?;
    let result = grade_answers(&answers, &questions);
    state
        .store
        .record_session(
            &uuid::Uuid::new_v4().to_string(),
            &material.course_id,
            &material_id,
            result.correct,
            result.total,
            result.percent,
        )
        .map_err(error_text)?;
    Ok(result)
}

fn detect_chromium() -> Option<PathBuf> {
    [
        "/opt/homebrew/bin/chromium",
        "/Applications/Chromium.app/Contents/MacOS/Chromium",
        "/Applications/Google Chrome.app/Contents/MacOS/Google Chrome",
        "/Applications/Google Chrome Canary.app/Contents/MacOS/Google Chrome Canary",
    ]
    .into_iter()
    .map(PathBuf::from)
    .find(|path| path.is_file())
}

fn error_text(error: impl std::fmt::Display) -> String {
    error.to_string()
}

pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .setup(|app| {
            let data_directory = app.path().app_data_dir()?;
            let manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
            let library = manifest
                .join("../..")
                .canonicalize()
                .unwrap_or_else(|_| manifest.join("../.."));
            let store = AcademicStore::open(&data_directory.join("estuda-uva.sqlite3"), library)
                .map_err(std::io::Error::other)?;
            app.manage(AppState {
                store,
                session: Mutex::new(None),
                active_imports: Mutex::new(HashMap::new()),
            });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            canvas_status,
            set_chromium_path,
            set_library_path,
            connect_canvas,
            disconnect_canvas,
            list_courses,
            load_course,
            open_module_item,
            list_calendar_events,
            sync_calendar,
            import_material,
            cancel_import,
            list_materials,
            overview,
            get_lesson,
            start_practice,
            submit_practice
        ])
        .run(tauri::generate_context!())
        .expect("erro ao iniciar Estuda UVA");
}

#[cfg(test)]
mod tests {
    use lopdf::{
        Document, Object, Stream,
        content::{Content, Operation},
        dictionary,
    };

    use crate::{
        archive::import_pdf,
        browser::run_browser_smoke,
        canvas::{
            CourseKind, discover_material_links, group_module_items, parse_courses,
            parse_module_items,
        },
        download::validate_material_url,
        lesson::{build_lesson, grade_answers, score_answer},
        pdf::{pages_to_markdown, safe_slug, split_pdf_pages},
    };

    fn two_page_pdf() -> Vec<u8> {
        let mut document = Document::with_version("1.5");
        let pages_id = document.new_object_id();
        let font_id = document.add_object(dictionary! {
            "Type" => "Font", "Subtype" => "Type1", "BaseFont" => "Courier"
        });
        let resources_id = document.add_object(dictionary! {
            "Font" => dictionary! { "F1" => font_id }
        });
        let mut page_ids = Vec::new();
        for phrase in ["Primeira pagina UVA", "Segunda pagina Rust"] {
            let content = Content {
                operations: vec![
                    Operation::new("BT", vec![]),
                    Operation::new("Tf", vec!["F1".into(), 18.into()]),
                    Operation::new("Td", vec![40.into(), 760.into()]),
                    Operation::new("Tj", vec![Object::string_literal(phrase)]),
                    Operation::new("ET", vec![]),
                ],
            };
            let content_id =
                document.add_object(Stream::new(dictionary! {}, content.encode().unwrap()));
            page_ids.push(document.add_object(dictionary! {
                "Type" => "Page", "Parent" => pages_id, "Contents" => content_id
            }));
        }
        document.objects.insert(pages_id, Object::Dictionary(dictionary! {
            "Type" => "Pages", "Kids" => page_ids.iter().copied().map(Object::Reference).collect::<Vec<_>>(),
            "Count" => 2, "Resources" => resources_id, "MediaBox" => vec![0.into(), 0.into(), 595.into(), 842.into()]
        }));
        let catalog_id =
            document.add_object(dictionary! { "Type" => "Catalog", "Pages" => pages_id });
        document.trailer.set("Root", catalog_id);
        document.compress();
        let mut bytes = Vec::new();
        document.save_to(&mut bytes).unwrap();
        bytes
    }

    fn contains_pdf_file(directory: &std::path::Path) -> bool {
        std::fs::read_dir(directory)
            .unwrap()
            .flatten()
            .any(|entry| {
                let path = entry.path();
                if path.is_dir() {
                    contains_pdf_file(&path)
                } else {
                    path.extension().is_some_and(|extension| extension == "pdf")
                }
            })
    }

    #[test]
    fn extracts_pdf_into_separate_page_text() {
        let pages = split_pdf_pages(&two_page_pdf()).unwrap();
        assert_eq!(pages.len(), 2);
        assert!(pages[0].contains("Primeira pagina UVA"));
        assert!(pages[1].contains("Segunda pagina Rust"));
    }

    #[test]
    fn markdown_preserves_page_origins_and_removes_query_secrets() {
        let md = pages_to_markdown(
            "Introdução <script>",
            "https://ead.uva.br/aula.pdf?token=nao-gravar#page=8",
            &["Texto da página 1".into(), "Texto da página 2".into()],
        );
        assert!(md.contains("## Página 1"));
        assert!(md.contains("## Página 2"));
        assert!(md.contains("https://ead.uva.br/aula.pdf"));
        assert!(!md.contains("token=nao-gravar"));
        assert!(!md.contains("<script>"));
    }

    #[test]
    fn slug_is_portuguese_safe_and_nonempty() {
        assert_eq!(safe_slug("Introdução à Web / U1"), "introducao-a-web-u1");
        assert_eq!(safe_slug(" !!! "), "material");
    }

    #[test]
    fn course_and_module_parsers_keep_dynamic_ids_and_skip_activities() {
        let courses = parse_courses(
            r#"
          <h2>Disciplinas atuais</h2><a class="course" href="/courses/8765">Desenvolvimento Web</a>
          <h2>Inscrições anteriores</h2><a class="course" href="/courses/4321">Bases de Programação</a>
        "#,
        );
        assert_eq!(courses.len(), 2);
        assert_eq!(courses[0].id, "8765");
        assert_eq!(courses[0].kind, CourseKind::Current);
        assert_eq!(courses[1].kind, CourseKind::Past);

        let items = parse_module_items(
            r#"
          <div class="context_module" id="module_10">
            <h3 class="ig-header-title">Unidade 1</h3>
            <div class="ig-row"><a class="ig-title" href="/courses/8765/modules/items/21">E-book U1</a></div>
            <div class="ig-row"><a class="ig-title" href="/courses/8765/assignments/31">Entregar prova</a></div>
            <div class="ig-row"><a class="ig-title" href="/courses/8765/discussion_topics/41">Fórum avaliativo</a></div>
          </div>
        "#,
        );
        assert_eq!(items.len(), 1);
        assert_eq!(items[0].module_title, "Unidade 1");
        assert_eq!(items[0].title, "E-book U1");
        let groups = group_module_items(items);
        assert_eq!(groups.len(), 1);
        assert_eq!(groups[0].title, "Unidade 1");
        assert_eq!(groups[0].items.len(), 1);
    }

    #[test]
    fn material_discovery_includes_embedded_ead_documents_without_pdf_suffix() {
        let materials = discover_material_links(
            r#"
              <iframe title="E-book da unidade" src="https://ead.uva.br/reader/book/314159"></iframe>
              <a href="https://uva.instructure.com/courses/8765/files/42/download">Arquivo PDF</a>
              <iframe title="vídeo externo" src="https://video.example.net/embed/42"></iframe>
            "#,
            "https://uva.instructure.com/courses/8765/modules/items/21",
        );
        assert_eq!(materials.len(), 2);
        assert_eq!(materials[0].title, "E-book da unidade");
        assert!(materials[0].embedded);
        assert_eq!(materials[1].title, "Arquivo PDF");
        assert!(!materials[1].embedded);
    }

    #[test]
    fn material_urls_require_https_and_uva_host_boundaries() {
        assert!(validate_material_url("https://ead.uva.br/aula.pdf").is_ok());
        assert!(validate_material_url("https://uva.instructure.com/files/12").is_ok());
        assert!(validate_material_url("https://evil-eaduva.br/aula.pdf").is_err());
        assert!(validate_material_url("https://ead.uva.br.attacker.example/aula.pdf").is_err());
        assert!(validate_material_url("https://usuario@ead.uva.br/aula.pdf").is_err());
        assert!(validate_material_url("https://ead.uva.br:8080/aula.pdf").is_err());
        assert!(validate_material_url("http://ead.uva.br/aula.pdf").is_err());
        assert!(validate_material_url("file:///etc/passwd").is_err());
    }

    #[test]
    fn lesson_and_score_are_deterministic_and_trace_pages() {
        let inputs = [
            (
                1,
                "# HTML\nHTML significa HyperText Markup Language. HTML estrutura o conteúdo da página.",
            ),
            (2, "## CSS\nCSS controla a apresentação visual da página."),
        ];
        let lesson = build_lesson("Desenvolvimento Web", &inputs);
        assert!(lesson.contains("Página 1"));
        assert!(lesson.contains("Página 2"));
        assert_eq!(lesson, build_lesson("Desenvolvimento Web", &inputs));
        assert_eq!(score_answer(" html ", "HTML"), 1);
        assert_eq!(score_answer("minha resposta", "HTML"), 0);
        let (_, questions) = crate::lesson::generate_lesson("Desenvolvimento Web", &inputs);
        let grade = grade_answers(&["HyperText Markup Language".into()], &questions);
        assert_eq!(grade.correct, 1);
        assert_eq!(grade.total, 1);
        assert_eq!(grade.percent, 100);
        assert_eq!(grade.answers[0].source_page, 1);
    }

    #[test]
    fn imported_material_is_markdown_idempotent_and_never_keeps_raw_pdf() {
        let temporary = tempfile::tempdir().unwrap();
        let library = temporary.path().join("Faculdade");
        let store = crate::storage::AcademicStore::open(
            &temporary.path().join("app/db.sqlite"),
            library.clone(),
        )
        .unwrap();
        let course = crate::canvas::Course {
            id: "8765".into(),
            title: "Desenvolvimento Web".into(),
            url: "https://uva.instructure.com/courses/8765".into(),
            kind: CourseKind::Current,
        };
        let course_dir = library.join("materias/desenvolvimento-web-8765");
        let materials_dir = course_dir.join("materiais");
        std::fs::create_dir_all(&materials_dir).unwrap();
        std::fs::write(
            library.join("materias/README.md"),
            "Índice manual da faculdade.\n",
        )
        .unwrap();
        std::fs::write(
            course_dir.join("README.md"),
            "Observação manual da matéria.\n",
        )
        .unwrap();
        let manual_material = materials_dir.join("e-book-u1.md");
        std::fs::write(&manual_material, "Anotação que não pode ser sobrescrita.\n").unwrap();
        let bytes = two_page_pdf();
        let pages = split_pdf_pages(&bytes).unwrap();
        let first = import_pdf(
            &store,
            &course,
            "Unidade 1",
            "E-book U1",
            "https://ead.uva.br/aula.pdf?download_token=secret",
            &bytes,
            &pages,
        )
        .unwrap();
        let again = import_pdf(
            &store,
            &course,
            "Unidade 1",
            "E-book U1",
            "https://ead.uva.br/aula.pdf?download_token=secret",
            &bytes,
            &pages,
        )
        .unwrap();
        assert_eq!(first.material.id, again.material.id);
        assert!(first.material.path.ends_with("e-book-u1-2.md"));
        assert_eq!(
            std::fs::read_to_string(&manual_material).unwrap(),
            "Anotação que não pode ser sobrescrita.\n"
        );
        assert!(
            std::fs::read_to_string(library.join("materias/README.md"))
                .unwrap()
                .contains("Índice manual da faculdade.")
        );
        assert!(
            std::fs::read_to_string(course_dir.join("README.md"))
                .unwrap()
                .contains("Observação manual da matéria.")
        );
        assert!(std::path::Path::new(&first.material.path).exists());
        assert!(std::path::Path::new(&first.material.lesson_path).exists());
        assert!(
            !std::path::Path::new(&first.material.path)
                .to_string_lossy()
                .contains("secret")
        );
        assert_eq!(store.list_materials().unwrap().len(), 1);
        assert!(!contains_pdf_file(temporary.path()));
        assert!(first.lesson_markdown.contains("Prática de recuperação"));
    }

    #[test]
    fn image_only_or_invalid_pdf_does_not_create_archive_output() {
        let temporary = tempfile::tempdir().unwrap();
        let library = temporary.path().join("Faculdade");
        let store = crate::storage::AcademicStore::open(
            &temporary.path().join("app/db.sqlite"),
            library.clone(),
        )
        .unwrap();
        assert!(split_pdf_pages(b"this is not a PDF").is_err());
        let course = crate::canvas::Course {
            id: "8765".into(),
            title: "Desenvolvimento Web".into(),
            url: "https://uva.instructure.com/courses/8765".into(),
            kind: CourseKind::Current,
        };
        let result = import_pdf(
            &store,
            &course,
            "Unidade 1",
            "E-book U1",
            "https://ead.uva.br/aula.pdf",
            &two_page_pdf(),
            &[" ".into(), "\n\t".into()],
        );
        assert!(matches!(result, Err(crate::archive::ArchiveError::NoText)));
        assert_eq!(store.list_materials().unwrap().len(), 0);
        assert!(!library.join("materias").exists());
    }

    #[tokio::test]
    async fn launches_real_isolated_chromium_and_reads_fixture_dom() {
        if !std::path::Path::new("/opt/homebrew/bin/chromium").exists() {
            panic!("Chromium esperado em /opt/homebrew/bin/chromium está ausente");
        }
        let profile = tempfile::tempdir().unwrap();
        run_browser_smoke("/opt/homebrew/bin/chromium", profile.path(), true)
            .await
            .unwrap();
        assert!(profile.path().exists());
    }
}
