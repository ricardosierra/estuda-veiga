mod anthropic;
mod archive;
mod browser;
mod calendar;
mod canvas;
mod canvas_capture;
mod catalog;
mod download;
mod lesson;
mod lyceum;
mod pdf;
mod repository;
mod storage;
mod study_summaries;
mod sync;
mod sync_watcher;
mod tutor;

use browser::{CanvasSession, CanvasState, CanvasStatus};
use calendar::{CalendarEvent, CalendarSyncReport, assignment_events, valid_semester};
use canvas::{Course, CourseKind, ModuleGroup};
use canvas_capture::{
    CanvasCaptureLimits, CaptureCounts, CaptureItem, CaptureItemStatus, CaptureRun,
    CaptureRunStatus, CourseCaptureInventory, build_capture_plan, capture_pdf_link,
    is_support_course, resumable_items, terminal_run_status,
};
use catalog::scanner::{
    CatalogService, LocalCatalogSnapshot, LocalDocumentContent, LocalSearchFilter,
    LocalSearchResponse,
};
use lesson::{PracticePrompt, PracticeResult, grade_answers};
use repository::RepositoryStatus;
use std::{
    collections::{HashMap, HashSet, VecDeque},
    path::PathBuf,
    sync::Arc,
};
use storage::{
    AcademicStore, ImportedMaterial, LessonInteraction, LessonSession, Overview, StudyHistoryEntry,
};
use sync::{SyncConflict, SyncEngine, SyncReport};
use tauri::{AppHandle, Emitter, Manager, State};
use tokio::sync::{Mutex, watch};
use tutor::{
    TutorActivity, TutorAssessment, TutorSectionInput, assess_tutor_answer, build_tutor_activity,
};

struct AppState {
    store: AcademicStore,
    sync: Arc<SyncEngine>,
    catalog: Arc<CatalogService>,
    watcher: sync_watcher::SyncWatcher,
    canvas_operation: Mutex<()>,
    session: Mutex<Option<CanvasSession>>,
    active_imports: Mutex<HashMap<String, watch::Sender<bool>>>,
    active_captures: Mutex<HashMap<String, watch::Sender<bool>>>,
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
async fn open_lyceum(
    app: AppHandle,
    state: State<'_, AppState>,
) -> Result<lyceum::PortalStatus, String> {
    let _canvas_operation = state.canvas_operation.lock().await;
    let mut slot = state.session.lock().await;
    if slot.is_none() {
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
        *slot = Some(
            CanvasSession::launch(&executable.to_string_lossy(), &profile)
                .await
                .map_err(error_text)?,
        );
    }
    slot.as_mut()
        .ok_or_else(|| "Não foi possível abrir a janela do portal UVA.".to_owned())?
        .open_lyceum()
        .await
        .map_err(error_text)
}

#[tauri::command]
async fn lyceum_status(state: State<'_, AppState>) -> Result<lyceum::PortalStatus, String> {
    let slot = state.session.lock().await;
    Ok(match slot.as_ref() {
        Some(session) => session.lyceum_status().await,
        None => lyceum::PortalStatus {
            state: lyceum::PortalState::Disconnected,
            message: "Lyceum ainda não foi aberto na janela do Estuda UVA.".into(),
            current_url: None,
            pages_read: 0,
            inaccessible_sections: Vec::new(),
        },
    })
}

#[tauri::command]
async fn refresh_lyceum_catalog(
    state: State<'_, AppState>,
) -> Result<lyceum::PortalCapture, String> {
    let _canvas_operation = state.canvas_operation.lock().await;
    let mut capture = {
        let slot = state.session.lock().await;
        slot.as_ref()
            .ok_or_else(|| "Abra o portal UVA e conclua o login manual primeiro.".to_owned())?
            .refresh_lyceum_catalog()
            .await
            .map_err(error_text)?
    };
    if capture.pages.is_empty() {
        return Err(
            "O portal não apresentou páginas acadêmicas legíveis; nenhum registro foi salvo."
                .into(),
        );
    }
    let root = state.store.library_path().map_err(error_text)?;
    let sync = Arc::clone(&state.sync);
    capture = tauri::async_runtime::spawn_blocking(move || {
        lyceum::persist_capture(&root, &sync, capture)
    })
    .await
    .map_err(error_text)??;
    let catalog = Arc::clone(&state.catalog);
    tauri::async_runtime::spawn_blocking(move || catalog.refresh())
        .await
        .map_err(error_text)??;
    Ok(capture)
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
    let path = PathBuf::from(path)
        .canonicalize()
        .map_err(|error| format!("Não foi possível acessar a pasta do acervo: {error}"))?;
    if !path.is_dir() {
        return Err("Selecione uma pasta existente para o acervo.".into());
    }
    let old_root = state.store.library_path().map_err(error_text)?;
    let old_database = state.store.database_path().map_err(error_text)?;
    state
        .store
        .set_library_path(path.clone())
        .map_err(error_text)?;
    let database = state.store.database_path().map_err(error_text)?;
    if let Err(error) = state.sync.set_workspace(&database, &path) {
        let _ = state.store.set_library_path(old_root.clone());
        return Err(error);
    }
    if let Err(error) = state.catalog.set_root(&path) {
        let _ = state.store.set_library_path(old_root.clone());
        let _ = state.sync.set_workspace(&old_database, &old_root);
        return Err(error);
    }
    if let Err(error) = state.watcher.set_root(&path) {
        let _ = state.store.set_library_path(old_root.clone());
        let _ = state.sync.set_workspace(&old_database, &old_root);
        let _ = state.catalog.set_root(&old_root);
        let _ = state.watcher.set_root(&old_root);
        return Err(error);
    }
    Ok(())
}

#[tauri::command]
async fn sync_local_catalog(state: State<'_, AppState>) -> Result<SyncReport, String> {
    let sync = Arc::clone(&state.sync);
    tauri::async_runtime::spawn_blocking(move || sync.reconcile_all())
        .await
        .map_err(error_text)?
}

#[tauri::command]
fn local_sync_conflicts(state: State<'_, AppState>) -> Result<Vec<SyncConflict>, String> {
    state.sync.open_conflicts()
}

#[tauri::command]
fn repository_status(state: State<'_, AppState>) -> Result<RepositoryStatus, String> {
    let root = state.store.library_path().map_err(error_text)?;
    repository::repository_status(&root)
}

#[tauri::command]
async fn edit_synced_document(
    relative_path: String,
    content: String,
    state: State<'_, AppState>,
) -> Result<SyncReport, String> {
    let sync = Arc::clone(&state.sync);
    tauri::async_runtime::spawn_blocking(move || {
        sync.set_content(&relative_path, &content)?;
        sync.reconcile_all()
    })
    .await
    .map_err(error_text)?
}

#[tauri::command]
async fn resolve_local_sync_conflict(
    relative_path: String,
    content: String,
    state: State<'_, AppState>,
) -> Result<SyncReport, String> {
    let sync = Arc::clone(&state.sync);
    tauri::async_runtime::spawn_blocking(move || {
        sync.resolve_conflict(&relative_path, &content)?;
        sync.reconcile_all()
    })
    .await
    .map_err(error_text)?
}

#[tauri::command]
async fn local_catalog_snapshot(
    state: State<'_, AppState>,
) -> Result<LocalCatalogSnapshot, String> {
    state.catalog.snapshot()
}

#[tauri::command]
async fn refresh_local_catalog(state: State<'_, AppState>) -> Result<LocalCatalogSnapshot, String> {
    let catalog = Arc::clone(&state.catalog);
    tauri::async_runtime::spawn_blocking(move || catalog.refresh())
        .await
        .map_err(error_text)?
}

#[tauri::command]
async fn read_local_document(
    relative_path: String,
    state: State<'_, AppState>,
) -> Result<LocalDocumentContent, String> {
    let catalog = Arc::clone(&state.catalog);
    tauri::async_runtime::spawn_blocking(move || catalog.read_document(&relative_path))
        .await
        .map_err(error_text)?
}

#[tauri::command]
async fn read_local_asset(
    relative_path: String,
    state: State<'_, AppState>,
) -> Result<Vec<u8>, String> {
    let catalog = Arc::clone(&state.catalog);
    tauri::async_runtime::spawn_blocking(move || catalog.read_asset(&relative_path))
        .await
        .map_err(error_text)?
}

#[tauri::command]
async fn search_local_library(
    query: String,
    state: State<'_, AppState>,
) -> Result<LocalSearchResponse, String> {
    let catalog = Arc::clone(&state.catalog);
    tauri::async_runtime::spawn_blocking(move || {
        catalog.search(&query, &LocalSearchFilter::default())
    })
    .await
    .map_err(error_text)?
}

#[tauri::command]
async fn academic_catalog(
    state: State<'_, AppState>,
) -> Result<catalog::academic::AcademicCatalog, String> {
    let sync = Arc::clone(&state.sync);
    tauri::async_runtime::spawn_blocking(move || {
        let report = sync.reconcile_all()?;
        let catalog_errors = report
            .errors
            .iter()
            .filter(|error| error.starts_with("materias/catalogo/"))
            .cloned()
            .collect::<Vec<_>>();
        if !catalog_errors.is_empty() {
            return Err(format!(
                "Não foi possível atualizar o catálogo acadêmico local: {}",
                catalog_errors.join("; ")
            ));
        }
        catalog::academic::load(&sync)
    })
    .await
    .map_err(error_text)?
}

#[tauri::command]
async fn connect_canvas(
    app: AppHandle,
    state: State<'_, AppState>,
) -> Result<CanvasStatus, String> {
    let _canvas_operation = state.canvas_operation.lock().await;
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
    let _canvas_operation = state.canvas_operation.lock().await;
    if let Some(session) = state.session.lock().await.take() {
        session.close().await;
    }
    Ok(())
}

#[tauri::command]
async fn list_courses(state: State<'_, AppState>) -> Result<Vec<Course>, String> {
    let _canvas_operation = state.canvas_operation.lock().await;
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
    let _canvas_operation = state.canvas_operation.lock().await;
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
    let _canvas_operation = state.canvas_operation.lock().await;
    let mut session = state.session.lock().await;
    session
        .as_mut()
        .ok_or_else(|| "Conecte-se ao Canvas primeiro.".to_owned())?
        .open_module_item(&course_id, &href)
        .await
        .map_err(error_text)
}

#[tauri::command]
async fn start_canvas_capture(
    app: AppHandle,
    state: State<'_, AppState>,
) -> Result<CaptureRun, String> {
    let mut active = state.active_captures.lock().await;
    if !active.is_empty() {
        return Err("Já existe uma captura em andamento.".into());
    }
    {
        let _canvas_operation = state.canvas_operation.lock().await;
        let session = state.session.lock().await;
        let session = session
            .as_ref()
            .ok_or_else(|| "Conecte-se ao Canvas e entre manualmente primeiro.".to_owned())?;
        if !matches!(session.status().await.state, CanvasState::Authenticated) {
            return Err("Entre manualmente no Canvas na janela Chromium antes de capturar.".into());
        }
    }
    let limits = state.store.get_capture_limits().map_err(error_text)?;
    let run = new_capture_run(uuid::Uuid::new_v4().simple().to_string(), limits);
    state.store.create_capture_run(&run).map_err(error_text)?;
    let (sender, receiver) = watch::channel(false);
    active.insert(run.id.clone(), sender);
    drop(active);
    spawn_canvas_capture(app, run.id.clone(), receiver);
    Ok(run)
}

#[tauri::command]
async fn resume_canvas_capture(
    app: AppHandle,
    run_id: String,
    state: State<'_, AppState>,
) -> Result<CaptureRun, String> {
    schedule_canvas_capture(app, &state, &run_id, true).await
}

#[tauri::command]
async fn cancel_canvas_capture(run_id: String, state: State<'_, AppState>) -> Result<(), String> {
    let active = state.active_captures.lock().await;
    let sender = active
        .get(&run_id)
        .ok_or_else(|| "Essa captura não está em andamento.".to_owned())?;
    sender
        .send(true)
        .map_err(|_| "Essa captura não está mais em andamento.".to_owned())
}

#[tauri::command]
async fn canvas_capture_status(
    run_id: Option<String>,
    state: State<'_, AppState>,
) -> Result<Option<CaptureRun>, String> {
    let mut run = match run_id {
        Some(run_id) => state.store.get_capture_run(&run_id).map_err(error_text),
        None => state.store.latest_capture().map_err(error_text),
    }?;
    if let Some(run) = run.as_mut() {
        let active = state.active_captures.lock().await.contains_key(&run.id);
        if !active && canvas_capture::recover_interrupted_capture_run(run) {
            state.store.update_capture_run(run).map_err(error_text)?;
        }
    }
    Ok(run)
}

#[tauri::command]
fn canvas_capture_items(
    run_id: String,
    offset: usize,
    limit: usize,
    state: State<'_, AppState>,
) -> Result<Vec<CaptureItem>, String> {
    state
        .store
        .list_capture_items(&run_id, offset, limit.min(200))
        .map_err(error_text)
}

#[tauri::command]
async fn retry_canvas_capture_item(
    app: AppHandle,
    run_id: String,
    reference_key: String,
    state: State<'_, AppState>,
) -> Result<CaptureRun, String> {
    let mut offset = 0;
    let mut selected = None;
    loop {
        let page = state
            .store
            .list_capture_items(&run_id, offset, 200)
            .map_err(error_text)?;
        if page.is_empty() {
            break;
        }
        if let Some(item) = page.iter().find(|item| item.reference_key == reference_key) {
            selected = Some(item.clone());
            break;
        }
        offset += page.len();
        if page.len() < 200 {
            break;
        }
    }
    let mut item = selected.ok_or_else(|| "O item não pertence a essa captura.".to_owned())?;
    if !item.retryable || item.status != CaptureItemStatus::Failed {
        return Err("Esse item não está disponível para nova tentativa.".into());
    }
    item.status = CaptureItemStatus::Pending;
    item.retryable = true;
    item.message = None;
    state
        .store
        .upsert_capture_item(&run_id, &item)
        .map_err(error_text)?;
    schedule_canvas_capture(app, &state, &run_id, true).await
}

#[tauri::command]
fn get_canvas_capture_limits(state: State<'_, AppState>) -> Result<CanvasCaptureLimits, String> {
    state.store.get_capture_limits().map_err(error_text)
}

#[tauri::command]
fn set_canvas_capture_limits(
    limits: CanvasCaptureLimits,
    state: State<'_, AppState>,
) -> Result<CanvasCaptureLimits, String> {
    limits.validate().map_err(str::to_owned)?;
    state
        .store
        .save_capture_limits(&limits)
        .map_err(error_text)?;
    Ok(limits)
}

fn new_capture_run(id: String, limits: CanvasCaptureLimits) -> CaptureRun {
    CaptureRun {
        id,
        scope: "all_visible".into(),
        limits_snapshot: limits,
        status: CaptureRunStatus::Running,
        counts: CaptureCounts::default(),
        current_course_id: None,
        current_course_title: None,
        current_module_title: None,
        current_item_title: None,
        warning: None,
        started_at: String::new(),
        updated_at: String::new(),
        finished_at: None,
    }
}

async fn schedule_canvas_capture(
    app: AppHandle,
    state: &AppState,
    run_id: &str,
    resume: bool,
) -> Result<CaptureRun, String> {
    let mut active = state.active_captures.lock().await;
    if !active.is_empty() {
        return Err("Já existe uma captura em andamento.".into());
    }
    let mut run = state
        .store
        .get_capture_run(run_id)
        .map_err(error_text)?
        .ok_or_else(|| "A captura não existe mais no banco local.".to_owned())?;
    if run.status == CaptureRunStatus::Completed {
        return Err("Essa captura já foi concluída.".into());
    }
    run.status = CaptureRunStatus::Running;
    run.finished_at = None;
    if resume {
        run.warning = None;
    }
    state.store.update_capture_run(&run).map_err(error_text)?;
    let (sender, receiver) = watch::channel(false);
    active.insert(run_id.to_owned(), sender);
    drop(active);
    spawn_canvas_capture(app, run_id.to_owned(), receiver);
    Ok(state
        .store
        .get_capture_run(run_id)
        .map_err(error_text)?
        .unwrap_or(run))
}

fn spawn_canvas_capture(app: AppHandle, run_id: String, cancellation: watch::Receiver<bool>) {
    tauri::async_runtime::spawn(async move {
        let state = app.state::<AppState>();
        run_canvas_capture(app.clone(), &state, &run_id, cancellation).await;
    });
}

#[derive(Debug, Clone, Copy)]
enum CaptureStop {
    WaitingForLogin,
    Cancelled,
    Fatal,
}

async fn discover_capture_plan(
    state: &AppState,
    limits: &CanvasCaptureLimits,
    cancellation: &watch::Receiver<bool>,
) -> Result<(canvas_capture::CapturePlan, Vec<Course>), CaptureStop> {
    if *cancellation.borrow() {
        return Err(CaptureStop::Cancelled);
    }
    let listing = {
        let _canvas_operation = state.canvas_operation.lock().await;
        let session = state.session.lock().await;
        let session = session.as_ref().ok_or(CaptureStop::WaitingForLogin)?;
        if !matches!(session.status().await.state, CanvasState::Authenticated) {
            return Err(CaptureStop::WaitingForLogin);
        }
        match session.list_all_courses(limits.max_course_pages).await {
            Ok(listing) => listing,
            Err(_) if !matches!(session.status().await.state, CanvasState::Authenticated) => {
                return Err(CaptureStop::WaitingForLogin);
            }
            Err(_) => return Err(CaptureStop::Fatal),
        }
    };
    let mut courses = listing
        .courses
        .into_iter()
        .filter(|course| !is_support_course(course))
        .collect::<Vec<_>>();
    let mut warnings = Vec::new();
    if let Some(warning) = listing.warning {
        warnings.push(warning);
    }
    if courses.len() > limits.max_courses {
        courses.truncate(limits.max_courses);
        warnings.push(format!(
            "A lista foi limitada a {} matérias.",
            limits.max_courses
        ));
    }
    if courses.is_empty() {
        return Err(CaptureStop::Fatal);
    }
    state
        .store
        .save_courses(&courses)
        .map_err(|_| CaptureStop::Fatal)?;
    let mut inventories = Vec::with_capacity(courses.len());
    for course in &courses {
        if *cancellation.borrow() {
            return Err(CaptureStop::Cancelled);
        }
        let inventory = {
            let _canvas_operation = state.canvas_operation.lock().await;
            let session = state.session.lock().await;
            let session = session.as_ref().ok_or(CaptureStop::WaitingForLogin)?;
            if !matches!(session.status().await.state, CanvasState::Authenticated) {
                return Err(CaptureStop::WaitingForLogin);
            }
            match session
                .load_course_inventory(&course.id, limits.max_items)
                .await
            {
                Ok(inventory) => inventory,
                Err(_) if !matches!(session.status().await.state, CanvasState::Authenticated) => {
                    return Err(CaptureStop::WaitingForLogin);
                }
                Err(_) => canvas::CanvasCourseInventory {
                    modules: Vec::new(),
                    complete: false,
                    warning: Some(format!(
                        "Não foi possível ler os módulos de {}.",
                        course.title
                    )),
                },
            }
        };
        inventories.push(CourseCaptureInventory {
            course: course.clone(),
            inventory,
        });
    }
    let mut plan = build_capture_plan(&inventories, limits.max_items);
    if let Some(warning) = plan.warning.take() {
        warnings.push(warning);
    }
    let mut unique_warnings = Vec::new();
    for warning in warnings {
        if !unique_warnings.contains(&warning) {
            unique_warnings.push(warning);
        }
    }
    plan.complete = unique_warnings.is_empty();
    plan.warning = (!unique_warnings.is_empty()).then(|| unique_warnings.join(" "));
    Ok((plan, courses))
}

async fn run_canvas_capture(
    app: AppHandle,
    state: &AppState,
    run_id: &str,
    mut cancellation: watch::Receiver<bool>,
) {
    let Some(mut run) = state.store.get_capture_run(run_id).ok().flatten() else {
        state.active_captures.lock().await.remove(run_id);
        return;
    };
    let (mut plan, courses) =
        match discover_capture_plan(state, &run.limits_snapshot, &cancellation).await {
            Ok(plan) => plan,
            Err(stop) => {
                run.status = match stop {
                    CaptureStop::WaitingForLogin => CaptureRunStatus::WaitingForLogin,
                    CaptureStop::Cancelled => CaptureRunStatus::Cancelled,
                    CaptureStop::Fatal => CaptureRunStatus::Failed,
                };
                run.warning = Some(match stop {
                    CaptureStop::WaitingForLogin => {
                        "A captura aguarda o login manual no Canvas.".into()
                    }
                    CaptureStop::Cancelled => {
                        "A captura foi cancelada; os itens confirmados foram mantidos.".into()
                    }
                    CaptureStop::Fatal => {
                        "Não foi possível enumerar as matérias disponíveis no Canvas.".into()
                    }
                });
                run.finished_at = None;
                let _ = state.store.update_capture_run(&run);
                emit_capture_update(&app, state, run_id);
                state.active_captures.lock().await.remove(run_id);
                return;
            }
        };
    run.counts.courses_total = plan.courses_total;
    run.status = CaptureRunStatus::Running;
    run.warning = plan.warning.clone();
    let existing_items = list_all_capture_items(&state.store, run_id).unwrap_or_default();
    let resuming = !existing_items.is_empty();
    let known_existing_refs = existing_items
        .iter()
        .map(|item| item.reference_key.clone())
        .collect::<HashSet<_>>();
    let mut existing_by_key = existing_items
        .into_iter()
        .map(|item| (item.reference_key.clone(), item))
        .collect::<HashMap<_, _>>();
    let mut queue = VecDeque::new();
    for mut item in plan.items.drain(..) {
        if let Some(existing) = existing_by_key.remove(&item.reference_key) {
            item.attempts = existing.attempts;
            item.material_id = existing.material_id;
            if resuming && item.source_kind == "canvas_page" {
                item.status = CaptureItemStatus::Pending;
                item.retryable = true;
                item.message = None;
            } else if matches!(
                existing.status,
                CaptureItemStatus::Captured
                    | CaptureItemStatus::Reused
                    | CaptureItemStatus::Ignored
                    | CaptureItemStatus::NeedsReview
            ) || (existing.status == CaptureItemStatus::Failed && !existing.retryable)
            {
                item.status = existing.status;
                item.retryable = existing.retryable;
                item.message = existing.message;
            } else {
                item.status = CaptureItemStatus::Pending;
                item.retryable = true;
            }
        }
        if state.store.upsert_capture_item(run_id, &item).is_ok() {
            queue.push_back(item);
        }
    }
    for mut item in existing_by_key.into_values() {
        let should_resume = !resumable_items(std::slice::from_ref(&item), resuming).is_empty();
        if should_resume {
            item.status = CaptureItemStatus::Pending;
            item.retryable = true;
            item.message = None;
            if state.store.upsert_capture_item(run_id, &item).is_ok() {
                queue.push_back(item);
            }
        }
    }
    run.current_course_id = None;
    run.current_course_title = None;
    run.current_module_title = None;
    run.current_item_title = None;
    let _ = state.store.update_capture_run(&run);
    emit_capture_update(&app, state, run_id);

    let academic_catalog = state.sync.read_academic_catalog().ok();
    let semester_by_course_id = courses
        .iter()
        .filter_map(|course| {
            academic_catalog
                .as_ref()
                .and_then(|catalog| canvas_capture::confirmed_current_semester(course, catalog))
                .map(|semester| (course.id.clone(), semester))
        })
        .collect::<HashMap<_, _>>();
    let course_by_id = courses
        .into_iter()
        .map(|course| (course.id.clone(), course))
        .collect::<HashMap<_, _>>();
    let mut raw_urls = plan.raw_urls;
    let mut known_refs = queue
        .iter()
        .map(|item| item.reference_key.clone())
        .chain(known_existing_refs)
        .collect::<HashSet<_>>();
    let mut next_position = queue.iter().map(|item| item.position).max().unwrap_or(0) + 1;
    let mut partial = !plan.complete;
    let mut stopped = None;
    while let Some(mut item) = queue.pop_front() {
        if *cancellation.borrow() {
            stopped = Some(CaptureStop::Cancelled);
            break;
        }
        if !resumable_items(std::slice::from_ref(&item), false).is_empty() {
            item.status = CaptureItemStatus::Running;
            item.retryable = false;
            item.attempts = item.attempts.saturating_add(1);
            item.message = None;
            if state.store.upsert_capture_item(run_id, &item).is_err() {
                partial = true;
                continue;
            }
            run.current_course_id = Some(item.course_id.clone());
            run.current_course_title = Some(item.course_title.clone());
            run.current_module_title = Some(item.module_title.clone());
            run.current_item_title = Some(item.title.clone());
            let _ = state.store.update_capture_run(&run);
            emit_capture_update(&app, state, run_id);
            let raw_url = raw_urls
                .get(&item.reference_key)
                .cloned()
                .unwrap_or_else(|| item.source_url.clone());
            let course = course_by_id.get(&item.course_id);
            let outcome = match course {
                Some(course) if item.source_kind == "pdf" => {
                    capture_pdf_item(
                        state,
                        course,
                        &item,
                        &raw_url,
                        semester_by_course_id.get(&course.id).map(String::as_str),
                        &run.limits_snapshot,
                        &mut cancellation,
                    )
                    .await
                }
                Some(course) if item.source_kind == "canvas_page" => {
                    capture_page_item(
                        state,
                        course,
                        &item,
                        &raw_url,
                        semester_by_course_id.get(&course.id).map(String::as_str),
                        run_id,
                        &mut raw_urls,
                        &mut known_refs,
                        &mut queue,
                        &mut next_position,
                        run.limits_snapshot.max_items,
                        &mut partial,
                        &cancellation,
                    )
                    .await
                }
                _ => CaptureItemOutcome::Ignored("Formato de aula não suportado.".into()),
            };
            match outcome {
                CaptureItemOutcome::Done(status, material_id, message) => {
                    item.status = status;
                    item.material_id = material_id;
                    item.retryable = false;
                    item.message = message;
                }
                CaptureItemOutcome::Ignored(message) => {
                    item.status = CaptureItemStatus::Ignored;
                    item.retryable = false;
                    item.message = Some(message);
                }
                CaptureItemOutcome::Failed(message) => {
                    item.status = CaptureItemStatus::Failed;
                    item.retryable = true;
                    item.message = Some(message);
                    partial = true;
                }
                CaptureItemOutcome::WaitingForLogin => {
                    item.status = CaptureItemStatus::Pending;
                    item.retryable = true;
                    let _ = state.store.upsert_capture_item(run_id, &item);
                    stopped = Some(CaptureStop::WaitingForLogin);
                    break;
                }
                CaptureItemOutcome::Cancelled => {
                    item.status = CaptureItemStatus::Pending;
                    item.retryable = true;
                    let _ = state.store.upsert_capture_item(run_id, &item);
                    stopped = Some(CaptureStop::Cancelled);
                    break;
                }
            }
            let _ = state.store.upsert_capture_item(run_id, &item);
            let _ = state.store.update_capture_run(&run);
            emit_capture_update(&app, state, run_id);
        }
    }
    let latest_items = list_all_capture_items(&state.store, run_id).unwrap_or_default();
    partial |= latest_items.iter().any(|item| {
        matches!(
            item.status,
            CaptureItemStatus::Failed | CaptureItemStatus::NeedsReview
        ) || matches!(
            item.status,
            CaptureItemStatus::Pending | CaptureItemStatus::Running
        )
    });
    run.status = match stopped {
        Some(CaptureStop::WaitingForLogin) => CaptureRunStatus::WaitingForLogin,
        Some(CaptureStop::Cancelled) => CaptureRunStatus::Cancelled,
        Some(CaptureStop::Fatal) => CaptureRunStatus::Failed,
        None => terminal_run_status(false, false, partial, false),
    };
    run.current_course_id = None;
    run.current_course_title = None;
    run.current_module_title = None;
    run.current_item_title = None;
    if partial && run.warning.is_none() {
        run.warning = Some("Alguns itens precisam de revisão ou podem ser retomados.".into());
    }
    run.finished_at = None;
    let _ = state.store.update_capture_run(&run);
    emit_capture_update(&app, state, run_id);
    state.active_captures.lock().await.remove(run_id);
}

fn list_all_capture_items(store: &AcademicStore, run_id: &str) -> Result<Vec<CaptureItem>, String> {
    let mut items = Vec::new();
    let mut offset = 0;
    loop {
        let page = store
            .list_capture_items(run_id, offset, 200)
            .map_err(error_text)?;
        if page.is_empty() {
            break;
        }
        offset += page.len();
        let finished = page.len() < 200;
        items.extend(page);
        if finished {
            break;
        }
    }
    Ok(items)
}

fn emit_capture_update(app: &AppHandle, state: &AppState, run_id: &str) {
    if let Ok(Some(run)) = state.store.get_capture_run(run_id) {
        let _ = app.emit("canvas-capture-updated", run);
    }
}

enum CaptureItemOutcome {
    Done(CaptureItemStatus, Option<String>, Option<String>),
    Ignored(String),
    Failed(String),
    WaitingForLogin,
    Cancelled,
}

async fn capture_pdf_item(
    state: &AppState,
    course: &Course,
    item: &CaptureItem,
    raw_url: &str,
    semester: Option<&str>,
    limits: &CanvasCaptureLimits,
    cancellation: &mut watch::Receiver<bool>,
) -> CaptureItemOutcome {
    if *cancellation.borrow() {
        return CaptureItemOutcome::Cancelled;
    }
    let source_key = item
        .content_key
        .clone()
        .or_else(|| canvas::content_key(item.canvas_item_id.as_deref(), raw_url, "pdf"));
    let existing = source_key.as_deref().and_then(|key| {
        archive::inspect_existing_material(&state.store, key)
            .ok()
            .flatten()
    });
    let validators = existing.as_ref().and_then(|material| {
        Some(download::DownloadValidators {
            host: material.source_validator_host.clone()?,
            etag: material.source_etag.clone(),
            last_modified: material.source_last_modified.clone(),
        })
    });
    let (referer, cookie) = {
        let _canvas_operation = state.canvas_operation.lock().await;
        let slot = state.session.lock().await;
        let Some(session) = slot.as_ref() else {
            return CaptureItemOutcome::WaitingForLogin;
        };
        if !matches!(session.status().await.state, CanvasState::Authenticated) {
            return CaptureItemOutcome::WaitingForLogin;
        }
        let parsed = match download::validate_material_url(raw_url) {
            Ok(url) => url,
            Err(_) => {
                return CaptureItemOutcome::Failed(
                    "A origem do PDF não passou pela validação de segurança.".into(),
                );
            }
        };
        let host = parsed.host_str().unwrap_or_default().to_owned();
        let referer = session.current_url().await.ok().flatten();
        let cookie = if host.ends_with(".amazonaws.com") {
            None
        } else {
            match session.cookies_for_host(&host).await {
                Ok(cookie) if !cookie.is_empty() => Some((host, cookie)),
                Ok(_) => None,
                Err(_) => {
                    return CaptureItemOutcome::Failed(
                        "Não foi possível validar a sessão do Canvas.".into(),
                    );
                }
            }
        };
        (referer, cookie)
    };
    if *cancellation.borrow() {
        return CaptureItemOutcome::Cancelled;
    }
    let cookie_header = cookie
        .as_ref()
        .map(|(cookie_host, value)| (cookie_host.as_str(), value.as_str()));
    let download = download::download_pdf(
        raw_url,
        referer.as_deref(),
        cookie_header,
        validators.as_ref(),
        &download::DownloadLimits {
            max_bytes: limits.max_pdf_bytes,
            max_redirects: limits.max_redirects,
            timeout_seconds: limits.item_timeout_seconds,
        },
        cancellation,
    )
    .await;
    let download = match download {
        Ok(download) => download,
        Err(download::DownloadError::Cancelled) => return CaptureItemOutcome::Cancelled,
        Err(_) => {
            if !canvas_session_authenticated(state).await {
                return CaptureItemOutcome::WaitingForLogin;
            }
            return CaptureItemOutcome::Failed("O PDF não pôde ser baixado com segurança.".into());
        }
    };
    if *cancellation.borrow() {
        return CaptureItemOutcome::Cancelled;
    }
    match download {
        download::DownloadOutcome::NotModified {
            final_url,
            etag,
            last_modified,
        } => {
            let Some(existing) = existing.as_ref() else {
                return CaptureItemOutcome::Failed(
                    "O servidor não enviou uma cópia local válida.".into(),
                );
            };
            match archive::reuse_material_for_capture(
                &state.store,
                existing,
                course,
                &item.module_title,
                &item.title,
                semester,
                final_url.host_str(),
                etag.as_deref(),
                last_modified.as_deref(),
            ) {
                Ok(Some(outcome)) => CaptureItemOutcome::Done(
                    CaptureItemStatus::Reused,
                    Some(outcome.bundle.material.id),
                    None,
                ),
                Ok(None) => CaptureItemOutcome::Failed(
                    "A cópia local precisa de revisão e não foi reutilizada.".into(),
                ),
                Err(_) => CaptureItemOutcome::Failed(
                    "Não foi possível atualizar o índice local do PDF.".into(),
                ),
            }
        }
        download::DownloadOutcome::Downloaded {
            bytes,
            final_url,
            etag,
            last_modified,
        } => {
            if *cancellation.borrow() {
                return CaptureItemOutcome::Cancelled;
            }
            let extraction = tauri::async_runtime::spawn_blocking(move || {
                let pages = pdf::split_pdf_pages(&bytes);
                (bytes, pages)
            })
            .await;
            let Ok((bytes, Ok(pages))) = extraction else {
                return CaptureItemOutcome::Ignored(
                    "O PDF não contém texto selecionável ou não pôde ser lido.".into(),
                );
            };
            if *cancellation.borrow() {
                return CaptureItemOutcome::Cancelled;
            }
            match archive::import_pdf_with_metadata(
                &state.store,
                course,
                &item.module_title,
                &item.title,
                semester,
                raw_url,
                final_url.host_str(),
                etag.as_deref(),
                last_modified.as_deref(),
                &bytes,
                &pages,
            ) {
                Ok(outcome) => CaptureItemOutcome::Done(
                    archive_status(outcome.status),
                    Some(outcome.bundle.material.id),
                    outcome.message,
                ),
                Err(archive::ArchiveError::NoText) => CaptureItemOutcome::Ignored(
                    "O PDF não contém texto selecionável; o arquivo original não foi guardado."
                        .into(),
                ),
                Err(_) => CaptureItemOutcome::Failed(
                    "Não foi possível salvar o texto do PDF no acervo.".into(),
                ),
            }
        }
    }
}

async fn capture_page_item(
    state: &AppState,
    course: &Course,
    item: &CaptureItem,
    raw_url: &str,
    semester: Option<&str>,
    run_id: &str,
    raw_urls: &mut HashMap<String, String>,
    known_refs: &mut std::collections::HashSet<String>,
    queue: &mut VecDeque<CaptureItem>,
    next_position: &mut usize,
    max_items: usize,
    truncated: &mut bool,
    cancellation: &watch::Receiver<bool>,
) -> CaptureItemOutcome {
    if *cancellation.borrow() {
        return CaptureItemOutcome::Cancelled;
    }
    let content = {
        let _canvas_operation = state.canvas_operation.lock().await;
        let mut slot = state.session.lock().await;
        let Some(session) = slot.as_mut() else {
            return CaptureItemOutcome::WaitingForLogin;
        };
        if !matches!(session.status().await.state, CanvasState::Authenticated) {
            return CaptureItemOutcome::WaitingForLogin;
        }
        let result = session.read_module_item(&course.id, raw_url).await;
        if !matches!(session.status().await.state, CanvasState::Authenticated) {
            return CaptureItemOutcome::WaitingForLogin;
        }
        match result {
            Ok(content) => content,
            Err(_) => {
                return CaptureItemOutcome::Failed("A página de aula não pôde ser lida.".into());
            }
        }
    };
    if *cancellation.borrow() {
        return CaptureItemOutcome::Cancelled;
    }
    for link in &content.pdf_links {
        if *cancellation.borrow() {
            return CaptureItemOutcome::Cancelled;
        }
        if let Some((mut linked_item, raw_link)) =
            capture_pdf_link(course, &item.module_title, &link.title, &link.href)
        {
            raw_urls.insert(linked_item.reference_key.clone(), raw_link);
            if known_refs.contains(&linked_item.reference_key) {
                continue;
            }
            if known_refs.len() >= max_items {
                *truncated = true;
                continue;
            }
            linked_item.position = *next_position;
            *next_position = next_position.saturating_add(1);
            if state
                .store
                .upsert_capture_item(run_id, &linked_item)
                .is_ok()
            {
                known_refs.insert(linked_item.reference_key.clone());
                queue.push_back(linked_item);
            }
        }
    }
    if let Some(markdown) = content.markdown.as_deref() {
        if *cancellation.borrow() {
            return CaptureItemOutcome::Cancelled;
        }
        return match archive::import_canvas_page(
            &state.store,
            course,
            &item.module_title,
            &content.title,
            semester,
            &content.source_url,
            markdown,
        ) {
            Ok(outcome) => CaptureItemOutcome::Done(
                archive_status(outcome.status),
                Some(outcome.bundle.material.id),
                outcome.message,
            ),
            Err(archive::ArchiveError::NoText) => {
                CaptureItemOutcome::Ignored("A página não contém texto selecionável.".into())
            }
            Err(_) => CaptureItemOutcome::Failed("Não foi possível salvar o texto da aula.".into()),
        };
    }
    CaptureItemOutcome::Ignored(content.unsupported_reason.unwrap_or_else(|| {
        "O item contém apenas materiais anexos, enfileirados separadamente.".into()
    }))
}

fn archive_status(status: archive::ArchiveImportStatus) -> CaptureItemStatus {
    match status {
        archive::ArchiveImportStatus::Captured => CaptureItemStatus::Captured,
        archive::ArchiveImportStatus::Reused => CaptureItemStatus::Reused,
        archive::ArchiveImportStatus::NeedsReview => CaptureItemStatus::NeedsReview,
    }
}

async fn canvas_session_authenticated(state: &AppState) -> bool {
    let _canvas_operation = state.canvas_operation.lock().await;
    let session = state.session.lock().await;
    match session.as_ref() {
        Some(session) => matches!(session.status().await.state, CanvasState::Authenticated),
        None => false,
    }
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
    let _canvas_operation = state.canvas_operation.lock().await;
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
    semester: Option<String>,
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
        semester.as_deref(),
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
    semester: Option<&str>,
    material_id: &str,
    cancellation: &mut watch::Receiver<bool>,
) -> Result<archive::ImportedBundle, String> {
    if *cancellation.borrow() {
        return Err("Importação cancelada.".into());
    }
    let (url, referer, cookie, cookie_host) = {
        let _canvas_operation = state.canvas_operation.lock().await;
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
    if let Some(existing) = archive::reuse_existing_material(&state.store, course, semester, &url)
        .map_err(error_text)?
    {
        if *cancellation.borrow() {
            return Err("Importação cancelada.".into());
        }
        return Ok(existing);
    }
    let cookie = if cookie.is_empty() {
        None
    } else {
        Some((cookie_host, cookie))
    };
    let download = download::download_pdf(
        &url,
        referer.as_deref(),
        cookie
            .as_ref()
            .map(|(host, value)| (host.as_str(), value.as_str())),
        None,
        &download::DownloadLimits::default(),
        cancellation,
    )
    .await
    .map_err(error_text)?;
    let download::DownloadOutcome::Downloaded {
        bytes: pdf_bytes,
        final_url,
        etag,
        last_modified,
    } = download
    else {
        return Err("O arquivo mudou durante a importação; tente novamente.".into());
    };
    if *cancellation.borrow() {
        return Err("Importação cancelada.".into());
    }
    let pages = pdf::split_pdf_pages(&pdf_bytes).map_err(error_text)?;
    if *cancellation.borrow() {
        return Err("Importação cancelada.".into());
    }
    archive::import_pdf_with_metadata(
        &state.store,
        &course,
        &module_title,
        &title,
        semester,
        &url,
        final_url.host_str(),
        etag.as_deref(),
        last_modified.as_deref(),
        &pdf_bytes,
        &pages,
    )
    .map(|outcome| outcome.bundle)
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
fn begin_lesson_session(
    resource_path: String,
    course_id: Option<String>,
    course_title: String,
    material_title: String,
    section_count: usize,
    state: State<'_, AppState>,
) -> Result<LessonSession, String> {
    state
        .store
        .begin_lesson_session(
            &resource_path,
            course_id.as_deref(),
            &course_title,
            &material_title,
            section_count,
        )
        .map_err(error_text)
}

#[tauri::command]
fn save_lesson_progress(
    session_id: String,
    section_index: usize,
    section_count: usize,
    completed: bool,
    state: State<'_, AppState>,
) -> Result<LessonSession, String> {
    state
        .store
        .save_lesson_progress(&session_id, section_index, section_count, completed)
        .map_err(error_text)
}

#[tauri::command]
fn build_lesson_activity(
    section: TutorSectionInput,
    session_id: String,
) -> Result<TutorActivity, String> {
    if session_id.is_empty()
        || session_id.len() > 128
        || section.index >= 100_000
        || section.title.len() > 512
        || section.markdown.len() > 1_048_576
        || section.source_page == Some(0)
    {
        return Err("A seção da aula excede os limites permitidos.".into());
    }
    let section = section.into_lesson_section();
    Ok(build_tutor_activity(&section, &session_id))
}

#[tauri::command]
fn assess_lesson_answer(activity: TutorActivity, response: String) -> TutorAssessment {
    assess_tutor_answer(&activity, &response)
}

#[tauri::command]
fn save_lesson_interaction(
    session_id: String,
    activity: TutorActivity,
    response: String,
    assessment: TutorAssessment,
    state: State<'_, AppState>,
) -> Result<LessonInteraction, String> {
    state
        .store
        .save_lesson_interaction(&session_id, &activity, &response, &assessment)
        .map_err(error_text)
}

#[tauri::command]
fn list_lesson_interactions(
    session_id: String,
    state: State<'_, AppState>,
) -> Result<Vec<LessonInteraction>, String> {
    state
        .store
        .list_lesson_interactions(&session_id)
        .map_err(error_text)
}

#[tauri::command]
fn get_lesson_interaction(
    session_id: String,
    activity_id: String,
    current_source_sha256: String,
    state: State<'_, AppState>,
) -> Result<Option<LessonInteraction>, String> {
    state
        .store
        .get_lesson_interaction(&session_id, &activity_id, &current_source_sha256)
        .map_err(error_text)
}

#[tauri::command]
fn get_lesson_session(
    session_id: String,
    state: State<'_, AppState>,
) -> Result<Option<LessonSession>, String> {
    state
        .store
        .get_lesson_session(&session_id)
        .map_err(error_text)
}

#[tauri::command]
fn list_study_history(
    course_title: Option<String>,
    limit: Option<usize>,
    state: State<'_, AppState>,
) -> Result<Vec<StudyHistoryEntry>, String> {
    state
        .store
        .list_study_history(course_title.as_deref(), limit.unwrap_or(200))
        .map_err(error_text)
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
            let store =
                AcademicStore::open_workspace(&data_directory.join("estuda-uva.sqlite3"), library)
                    .map_err(std::io::Error::other)?;
            let configured_library = store.library_path().map_err(std::io::Error::other)?;
            catalog::guidance::create_missing_guidance(&configured_library)
                .map_err(std::io::Error::other)?;
            let catalog = Arc::new(
                CatalogService::new(configured_library.clone()).map_err(std::io::Error::other)?,
            );
            let sync = Arc::new(
                SyncEngine::open(
                    &store.database_path().map_err(std::io::Error::other)?,
                    &configured_library,
                )
                .map_err(std::io::Error::other)?,
            );
            let app_handle = app.handle().clone();
            let watcher = sync_watcher::SyncWatcher::start(
                configured_library.clone(),
                Arc::clone(&sync),
                move |report| {
                    let _ = app_handle.emit("local-sync-updated", report);
                },
            )
            .map_err(std::io::Error::other)?;
            app.manage(AppState {
                store,
                sync,
                catalog,
                watcher,
                canvas_operation: Mutex::new(()),
                session: Mutex::new(None),
                active_imports: Mutex::new(HashMap::new()),
                active_captures: Mutex::new(HashMap::new()),
            });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            canvas_status,
            open_lyceum,
            lyceum_status,
            refresh_lyceum_catalog,
            set_chromium_path,
            set_library_path,
            sync_local_catalog,
            local_sync_conflicts,
            repository_status,
            edit_synced_document,
            resolve_local_sync_conflict,
            local_catalog_snapshot,
            refresh_local_catalog,
            read_local_document,
            read_local_asset,
            search_local_library,
            academic_catalog,
            connect_canvas,
            disconnect_canvas,
            list_courses,
            load_course,
            open_module_item,
            start_canvas_capture,
            resume_canvas_capture,
            cancel_canvas_capture,
            canvas_capture_status,
            canvas_capture_items,
            retry_canvas_capture_item,
            get_canvas_capture_limits,
            set_canvas_capture_limits,
            list_calendar_events,
            sync_calendar,
            import_material,
            cancel_import,
            list_materials,
            begin_lesson_session,
            save_lesson_progress,
            build_lesson_activity,
            assess_lesson_answer,
            save_lesson_interaction,
            list_lesson_interactions,
            get_lesson_interaction,
            get_lesson_session,
            list_study_history,
            overview,
            get_lesson,
            start_practice,
            submit_practice,
            study_summaries::get_subject_summary_snapshot,
            study_summaries::generate_subject_summaries
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
        archive::{self, import_pdf},
        browser::run_browser_smoke,
        canvas::{
            Course, CourseKind, discover_material_links, group_module_items, parse_courses,
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
            Some("2026.2"),
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
            Some("2026.2"),
            "https://ead.uva.br/aula.pdf?download_token=secret",
            &bytes,
            &pages,
        )
        .unwrap();
        assert_eq!(first.material.id, again.material.id);
        assert_eq!(first.material.semester.as_deref(), Some("2026.2"));
        assert!(
            std::fs::read_to_string(course_dir.join("README.md"))
                .unwrap()
                .contains("período 2026.2")
        );
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
    fn existing_material_can_be_reused_by_source_url_without_pdf_bytes() {
        let temporary = tempfile::tempdir().unwrap();
        let library = temporary.path().join("Faculdade");
        let store =
            crate::storage::AcademicStore::open(&temporary.path().join("app/db.sqlite"), library)
                .unwrap();
        let course = Course {
            id: "8765".into(),
            title: "Desenvolvimento Web".into(),
            url: "https://uva.instructure.com/courses/8765".into(),
            kind: CourseKind::Current,
        };
        let source = "https://ead.uva.br/aula.pdf?download_token=segredo";
        let bytes = two_page_pdf();
        let pages = split_pdf_pages(&bytes).unwrap();
        let imported = archive::import_pdf(
            &store,
            &course,
            "Unidade 1",
            "E-book U1",
            Some("2026.1"),
            source,
            &bytes,
            &pages,
        )
        .unwrap();

        let reused = archive::reuse_existing_material(
            &store,
            &course,
            Some("2026.2"),
            "https://ead.uva.br/aula.pdf?download_token=rotacionado",
        )
        .unwrap()
        .expect("material local deveria ser encontrado antes de uma nova transferência");

        assert_eq!(reused.material.id, imported.material.id);
        assert_eq!(reused.material.semester.as_deref(), Some("2026.2"));
        assert_eq!(reused.material.origin, "https://ead.uva.br/aula.pdf");
        assert_eq!(store.list_materials().unwrap().len(), 1);
        assert_eq!(reused.lesson_markdown, imported.lesson_markdown);
        let course_index = std::path::Path::new(&reused.material.path)
            .parent()
            .unwrap()
            .parent()
            .unwrap()
            .join("README.md");
        assert!(
            std::fs::read_to_string(course_index)
                .unwrap()
                .contains("período 2026.2")
        );
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
            None,
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
