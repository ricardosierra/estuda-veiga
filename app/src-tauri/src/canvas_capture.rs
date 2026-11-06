use serde::{Deserialize, Serialize};
use sha2::Digest;
use std::collections::HashMap;

use crate::canvas::{
    CanvasCourseInventory, Course, CourseKind, ModuleItem, content_key, module_item_url,
    reference_key, sanitize_source_url,
};
use crate::catalog::academic::AcademicCatalog;
use unicode_normalization::{UnicodeNormalization, char::is_combining_mark};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CaptureRunStatus {
    Running,
    WaitingForLogin,
    Cancelled,
    Partial,
    Completed,
    Failed,
}

impl CaptureRunStatus {
    pub(crate) fn as_db(self) -> &'static str {
        match self {
            Self::Running => "running",
            Self::WaitingForLogin => "waiting_for_login",
            Self::Cancelled => "cancelled",
            Self::Partial => "partial",
            Self::Completed => "completed",
            Self::Failed => "failed",
        }
    }

    pub(crate) fn from_db(value: &str) -> Option<Self> {
        match value {
            "running" => Some(Self::Running),
            "waiting_for_login" => Some(Self::WaitingForLogin),
            "cancelled" => Some(Self::Cancelled),
            "partial" => Some(Self::Partial),
            "completed" => Some(Self::Completed),
            "failed" => Some(Self::Failed),
            _ => None,
        }
    }

    pub(crate) fn is_incomplete(self) -> bool {
        matches!(
            self,
            Self::Running | Self::WaitingForLogin | Self::Cancelled | Self::Partial | Self::Failed
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CaptureItemStatus {
    Pending,
    Running,
    Captured,
    Reused,
    Ignored,
    Failed,
    NeedsReview,
}

impl CaptureItemStatus {
    pub(crate) fn as_db(self) -> &'static str {
        match self {
            Self::Pending => "pending",
            Self::Running => "running",
            Self::Captured => "captured",
            Self::Reused => "reused",
            Self::Ignored => "ignored",
            Self::Failed => "failed",
            Self::NeedsReview => "needs_review",
        }
    }

    pub(crate) fn from_db(value: &str) -> Option<Self> {
        match value {
            "pending" => Some(Self::Pending),
            "running" => Some(Self::Running),
            "captured" => Some(Self::Captured),
            "reused" => Some(Self::Reused),
            "ignored" => Some(Self::Ignored),
            "failed" => Some(Self::Failed),
            "needs_review" => Some(Self::NeedsReview),
            _ => None,
        }
    }

    pub(crate) fn is_terminal(self) -> bool {
        !matches!(self, Self::Pending | Self::Running)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CaptureCounts {
    pub courses_total: usize,
    pub courses_done: usize,
    pub items_total: usize,
    pub captured: usize,
    pub reused: usize,
    pub ignored: usize,
    pub failed: usize,
    pub needs_review: usize,
    pub pending: usize,
}

impl Default for CaptureCounts {
    fn default() -> Self {
        Self {
            courses_total: 0,
            courses_done: 0,
            items_total: 0,
            captured: 0,
            reused: 0,
            ignored: 0,
            failed: 0,
            needs_review: 0,
            pending: 0,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CanvasCaptureLimits {
    pub max_course_pages: usize,
    pub max_courses: usize,
    pub max_items: usize,
    pub max_pdf_bytes: usize,
    pub max_redirects: usize,
    pub item_timeout_seconds: usize,
}

impl CanvasCaptureLimits {
    pub fn validate(&self) -> Result<(), &'static str> {
        if !(1..=100).contains(&self.max_course_pages) {
            return Err("maxCoursePages deve ficar entre 1 e 100");
        }
        if !(1..=500).contains(&self.max_courses) {
            return Err("maxCourses deve ficar entre 1 e 500");
        }
        if !(1..=20_000).contains(&self.max_items) {
            return Err("maxItems deve ficar entre 1 e 20000");
        }
        if !(1..=60 * 1024 * 1024).contains(&self.max_pdf_bytes) {
            return Err("maxPdfBytes deve ficar entre 1 e 60 MiB");
        }
        if self.max_redirects > 4 {
            return Err("maxRedirects deve ficar entre 0 e 4");
        }
        if !(5..=120).contains(&self.item_timeout_seconds) {
            return Err("itemTimeoutSeconds deve ficar entre 5 e 120");
        }
        Ok(())
    }
}

impl Default for CanvasCaptureLimits {
    fn default() -> Self {
        Self {
            max_course_pages: 100,
            max_courses: 200,
            max_items: 10_000,
            max_pdf_bytes: 60 * 1024 * 1024,
            max_redirects: 4,
            item_timeout_seconds: 45,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CaptureRun {
    pub id: String,
    pub scope: String,
    pub limits_snapshot: CanvasCaptureLimits,
    pub status: CaptureRunStatus,
    pub counts: CaptureCounts,
    pub current_course_id: Option<String>,
    pub current_course_title: Option<String>,
    pub current_module_title: Option<String>,
    pub current_item_title: Option<String>,
    pub warning: Option<String>,
    pub started_at: String,
    pub updated_at: String,
    pub finished_at: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CaptureItem {
    pub position: usize,
    pub reference_key: String,
    pub content_key: Option<String>,
    pub canvas_item_id: Option<String>,
    pub course_id: String,
    pub course_title: String,
    pub module_title: String,
    pub title: String,
    pub source_kind: String,
    pub source_url: String,
    pub status: CaptureItemStatus,
    pub retryable: bool,
    pub attempts: usize,
    pub material_id: Option<String>,
    pub message: Option<String>,
    pub updated_at: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MaterialAssociation {
    pub material_id: String,
    pub course_id: String,
    pub course_title: String,
    pub module_title: String,
    pub item_title: String,
    pub semester: Option<String>,
}

pub struct CourseCaptureInventory {
    pub course: Course,
    pub inventory: CanvasCourseInventory,
}

pub fn is_support_course(course: &Course) -> bool {
    let title = course.title.to_lowercase();
    title.contains("espaço curso") || title.contains("espaco curso")
}

pub fn confirmed_current_semester(course: &Course, catalog: &AcademicCatalog) -> Option<String> {
    if course.kind != CourseKind::Current {
        return None;
    }
    let wanted_title = normalized_course_title(&course.title);
    if wanted_title.is_empty() {
        return None;
    }
    catalog.trajectories.iter().find_map(|trajectory| {
        if trajectory.get("status")?.as_str()? != "active" {
            return None;
        }
        let trajectory_semester = trajectory
            .get("semester")
            .and_then(serde_json::Value::as_str);
        trajectory
            .get("currentEnrollments")?
            .as_array()?
            .iter()
            .find(|enrollment| {
                enrollment
                    .get("title")
                    .and_then(serde_json::Value::as_str)
                    .is_some_and(|title| normalized_course_title(title) == wanted_title)
            })
            .and_then(|enrollment| {
                enrollment
                    .get("semester")
                    .and_then(serde_json::Value::as_str)
                    .or(trajectory_semester)
            })
            .map(str::to_owned)
    })
}

fn normalized_course_title(value: &str) -> String {
    let trimmed = value.trim();
    let title_without_code = if trimmed.ends_with(')') {
        trimmed
            .rfind('(')
            .map(|opening| trimmed[..opening].trim())
            .unwrap_or(trimmed)
    } else {
        trimmed
    };
    title_without_code
        .nfd()
        .filter(|character| !is_combining_mark(*character))
        .flat_map(char::to_lowercase)
        .fold(String::new(), |mut normalized, character| {
            if character.is_ascii_alphanumeric() {
                normalized.push(character);
            } else if !normalized.ends_with(' ') {
                normalized.push(' ');
            }
            normalized
        })
        .trim()
        .to_owned()
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CapturePlan {
    pub items: Vec<CaptureItem>,
    /// Signed links exist only in memory for the immediate HTTP request.
    pub raw_urls: HashMap<String, String>,
    pub courses_total: usize,
    pub complete: bool,
    pub warning: Option<String>,
}

pub fn build_capture_plan(inventories: &[CourseCaptureInventory], max_items: usize) -> CapturePlan {
    let mut items = Vec::new();
    let mut raw_urls = HashMap::new();
    let mut warnings = Vec::new();
    let limit = max_items.clamp(1, 20_000);
    let mut truncated = false;
    for entry in inventories {
        if !entry.inventory.complete {
            if let Some(warning) = entry.inventory.warning.as_deref() {
                warnings.push(warning.to_owned());
            }
        }
        for module in &entry.inventory.modules {
            for module_item in &module.items {
                if items.len() >= limit {
                    truncated = true;
                    break;
                }
                if let Some((mut item, raw_url)) =
                    capture_item_from_module(&entry.course, module_item)
                {
                    item.position = items.len();
                    raw_urls.insert(item.reference_key.clone(), raw_url);
                    items.push(item);
                }
            }
            if truncated {
                break;
            }
        }
        if truncated {
            break;
        }
    }
    if truncated {
        warnings.push(format!("A captura foi limitada a {limit} itens."));
    }
    let mut unique_warnings = Vec::new();
    for warning in warnings {
        if !unique_warnings.contains(&warning) {
            unique_warnings.push(warning);
        }
    }
    CapturePlan {
        items,
        raw_urls,
        courses_total: inventories.len(),
        complete: unique_warnings.is_empty(),
        warning: (!unique_warnings.is_empty()).then(|| unique_warnings.join(" ")),
    }
}

fn capture_item_from_module(course: &Course, item: &ModuleItem) -> Option<(CaptureItem, String)> {
    let url = module_item_url(&course.id, &item.href).or_else(|| {
        let parsed = url::Url::parse(&item.href).ok()?;
        let path = parsed.path().to_ascii_lowercase();
        (crate::download::validate_material_url(&item.href).is_ok()
            && (path.ends_with(".pdf") || path.contains("/files/")))
        .then_some(parsed)
    })?;
    let path = url.path().to_ascii_lowercase();
    let source_kind =
        if item.item_type == "file" || path.contains("/files/") || path.ends_with(".pdf") {
            "pdf"
        } else {
            "canvas_page"
        };
    let source_url = sanitize_source_url(url.as_str())?;
    let reference_key =
        reference_key(&course.id, &item.module_title, &url.to_string()).or_else(|| {
            external_pdf_reference_key(&course.id, &item.module_title, &item.title, &url)
        })?;
    let canvas_item_id = canvas_id_from_url(&url);
    let content_key = content_key(
        if source_kind == "pdf" {
            canvas_item_id.as_deref()
        } else {
            None
        },
        &url.to_string(),
        source_kind,
    );
    let capture_item = CaptureItem {
        position: 0,
        reference_key,
        content_key,
        canvas_item_id,
        course_id: course.id.clone(),
        course_title: course.title.clone(),
        module_title: item.module_title.clone(),
        title: item.title.clone(),
        source_kind: source_kind.to_owned(),
        source_url,
        status: CaptureItemStatus::Pending,
        retryable: true,
        attempts: 0,
        material_id: None,
        message: None,
        updated_at: String::new(),
    };
    Some((capture_item, url.to_string()))
}

pub fn capture_pdf_link(
    course: &Course,
    module_title: &str,
    title: &str,
    href: &str,
) -> Option<(CaptureItem, String)> {
    capture_item_from_module(
        course,
        &ModuleItem {
            module_title: module_title.to_owned(),
            title: title.to_owned(),
            href: href.to_owned(),
            item_type: "file".into(),
        },
    )
    .filter(|(item, _)| item.source_kind == "pdf")
}

fn external_pdf_reference_key(
    course_id: &str,
    module_title: &str,
    item_title: &str,
    url: &url::Url,
) -> Option<String> {
    if !crate::download::validate_material_url(url.as_str()).is_ok()
        || !url.path().to_ascii_lowercase().ends_with(".pdf")
    {
        return None;
    }
    let mut canonical = url.clone();
    canonical.set_query(None);
    canonical.set_fragment(None);
    let identity = format!("{course_id}\0{module_title}\0{item_title}\0{canonical}");
    let digest = sha2::Sha256::digest(identity.as_bytes());
    Some(format!("ref:{digest:x}"))
}

fn canvas_id_from_url(url: &url::Url) -> Option<String> {
    url.path_segments()?
        .collect::<Vec<_>>()
        .windows(2)
        .find(|parts| parts[0] == "files")
        .map(|parts| parts[1].to_owned())
        .filter(|id| !id.is_empty() && id.bytes().all(|byte| byte.is_ascii_digit()))
}

pub fn resumable_items(items: &[CaptureItem], recover_stale_running: bool) -> Vec<&CaptureItem> {
    items
        .iter()
        .filter(|item| {
            matches!(item.status, CaptureItemStatus::Pending)
                || (item.status == CaptureItemStatus::Failed && item.retryable)
                || (recover_stale_running && item.status == CaptureItemStatus::Running)
        })
        .collect()
}

pub fn terminal_run_status(
    waiting_for_login: bool,
    cancelled: bool,
    partial: bool,
    fatal: bool,
) -> CaptureRunStatus {
    if waiting_for_login {
        CaptureRunStatus::WaitingForLogin
    } else if cancelled {
        CaptureRunStatus::Cancelled
    } else if fatal {
        CaptureRunStatus::Failed
    } else if partial {
        CaptureRunStatus::Partial
    } else {
        CaptureRunStatus::Completed
    }
}

pub fn recover_interrupted_capture_run(run: &mut CaptureRun) -> bool {
    if run.status != CaptureRunStatus::Running {
        return false;
    }
    run.status = CaptureRunStatus::Partial;
    run.finished_at = None;
    run.warning = Some(
        "A execução anterior foi interrompida; os itens pendentes podem ser retomados.".into(),
    );
    true
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::canvas::{CourseKind, ModuleGroup};

    fn course(id: &str, title: &str) -> Course {
        Course {
            id: id.into(),
            title: title.into(),
            url: format!("https://uva.instructure.com/courses/{id}"),
            kind: CourseKind::Past,
        }
    }

    fn inventory(course: Course, title: &str, href: &str) -> CourseCaptureInventory {
        CourseCaptureInventory {
            course,
            inventory: CanvasCourseInventory {
                modules: vec![ModuleGroup {
                    title: "Unidade 1".into(),
                    items: vec![ModuleItem {
                        module_title: "Unidade 1".into(),
                        title: title.into(),
                        href: href.into(),
                        item_type: "file".into(),
                    }],
                }],
                complete: true,
                warning: None,
            },
        }
    }

    #[test]
    fn capture_plan_deduplicates_content_and_keeps_module_references() {
        let plan = build_capture_plan(
            &[
                inventory(
                    course("41", "Bases de Programação"),
                    "Apostila",
                    "https://ead.uva.br/aula.pdf?signature=temporary",
                ),
                inventory(
                    course("42", "Cálculo"),
                    "Apostila compartilhada",
                    "https://ead.uva.br/aula.pdf?signature=rotated",
                ),
            ],
            100,
        );

        assert_eq!(plan.items.len(), 2);
        assert_eq!(plan.items[0].content_key, plan.items[1].content_key);
        assert_ne!(plan.items[0].reference_key, plan.items[1].reference_key);
        assert!(!plan.items[0].source_url.contains("signature"));
        assert!(plan.complete);
    }

    #[test]
    fn support_course_is_excluded_from_the_subject_capture_scope() {
        assert!(is_support_course(&course(
            "210062",
            "ESPAÇO CURSO - CIÊNCIA DA COMPUTAÇÃO"
        )));
        assert!(is_support_course(&course("210062", "Espaco Curso")));
        assert!(!is_support_course(&course(
            "57904",
            "Linguagens Formais e Autômatos"
        )));
    }

    #[test]
    fn current_canvas_course_receives_only_a_confirmed_active_semester() {
        let catalog = crate::catalog::academic::AcademicCatalog {
            curriculums: Vec::new(),
            disciplines: Vec::new(),
            relations: Vec::new(),
            trajectories: vec![serde_json::json!({
                "status": "active",
                "semester": "2026.2",
                "currentEnrollments": [{
                    "title": "Cálculo de Uma Variável",
                    "semester": "2026.2"
                }]
            })],
        };
        let current = Course {
            id: "57897".into(),
            title: "CALCULO DE UMA VARIÁVEL (E10100002)".into(),
            url: "https://uva.instructure.com/courses/57897".into(),
            kind: CourseKind::Current,
        };
        assert_eq!(
            confirmed_current_semester(&current, &catalog).as_deref(),
            Some("2026.2")
        );

        let past = Course {
            kind: CourseKind::Past,
            ..current.clone()
        };
        assert_eq!(confirmed_current_semester(&past, &catalog), None);
        let unrelated = Course {
            title: "Estatística para Computação".into(),
            ..current
        };
        assert_eq!(confirmed_current_semester(&unrelated, &catalog), None);
    }

    #[test]
    fn interrupted_capture_run_is_recoverable_after_app_restart() {
        let mut run = CaptureRun {
            id: "capture-1".into(),
            scope: "all_visible".into(),
            limits_snapshot: CanvasCaptureLimits::default(),
            status: CaptureRunStatus::Running,
            counts: CaptureCounts::default(),
            current_course_id: None,
            current_course_title: None,
            current_module_title: None,
            current_item_title: None,
            warning: None,
            started_at: "2026-10-15T12:00:00Z".into(),
            updated_at: "2026-10-15T12:01:00Z".into(),
            finished_at: None,
        };

        assert!(recover_interrupted_capture_run(&mut run));
        assert_eq!(run.status, CaptureRunStatus::Partial);
        assert!(
            run.warning
                .as_deref()
                .unwrap_or_default()
                .contains("retomados")
        );
        assert!(!recover_interrupted_capture_run(&mut run));
    }

    #[test]
    fn capture_state_pauses_or_cancels_without_losing_completed_items() {
        let completed = CaptureItemStatus::Captured;
        let pending = CaptureItemStatus::Pending;
        let cancelled = terminal_run_status(false, true, false, false);
        let waiting = terminal_run_status(true, false, false, false);

        assert_eq!(cancelled, CaptureRunStatus::Cancelled);
        assert_eq!(waiting, CaptureRunStatus::WaitingForLogin);
        assert_eq!(completed, CaptureItemStatus::Captured);
        assert_eq!(pending, CaptureItemStatus::Pending);
    }

    #[test]
    fn capture_run_resumes_only_pending_or_retryable_items() {
        let mut items = vec![
            capture_item_for_test("pending", CaptureItemStatus::Pending, false),
            capture_item_for_test("retryable", CaptureItemStatus::Failed, true),
            capture_item_for_test("failed", CaptureItemStatus::Failed, false),
            capture_item_for_test("captured", CaptureItemStatus::Captured, false),
            capture_item_for_test("running", CaptureItemStatus::Running, false),
        ];

        assert_eq!(resumable_items(&items, false).len(), 2);
        let after_restart = resumable_items(&items, true);
        assert_eq!(after_restart.len(), 3);
        assert!(
            after_restart
                .iter()
                .any(|item| item.reference_key == "running")
        );
        items[0].status = CaptureItemStatus::Captured;
        assert_eq!(resumable_items(&items, false).len(), 1);
    }

    fn capture_item_for_test(
        reference_key: &str,
        status: CaptureItemStatus,
        retryable: bool,
    ) -> CaptureItem {
        CaptureItem {
            position: 0,
            reference_key: reference_key.into(),
            content_key: None,
            canvas_item_id: None,
            course_id: "41".into(),
            course_title: "Bases de Programação".into(),
            module_title: "Unidade 1".into(),
            title: reference_key.into(),
            source_kind: "canvas_page".into(),
            source_url: "https://uva.instructure.com/courses/41/pages/aula".into(),
            status,
            retryable,
            attempts: 0,
            material_id: None,
            message: None,
            updated_at: String::new(),
        }
    }
}
