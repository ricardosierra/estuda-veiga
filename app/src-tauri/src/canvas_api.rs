//! Leitura acadêmica do Canvas pela API REST (`/api/v1`), sem IA.
//!
//! Este módulo é puro: monta e valida caminhos de API, converte o JSON do Canvas em registros
//! locais e gera o Markdown da captura. A requisição em si roda em `browser.rs`, dentro da sessão
//! autenticada do Chromium do app. Procedimentos documentados em `docs/NAVEGACAO_PORTAL_UVA.md`
//! (C4 notas e feedback, C5 fóruns, C6 avisos, C7 Caixa de Entrada).

use crate::canvas::{Course, CourseKind};
use scraper::{Html, Node};
use serde::{Deserialize, Serialize};
use std::path::Path;

pub const CANVAS_ORIGIN: &str = "https://uva.instructure.com";
pub const MAX_COURSES: usize = 40;
pub const MAX_LIST_PAGES: usize = 10;
pub const MAX_COMMENT_LOOKUPS_PER_COURSE: usize = 40;
pub const MAX_TOPICS_PER_COURSE: usize = 60;
pub const MAX_ENTRIES_PER_TOPIC: usize = 2_000;
pub const MAX_CONVERSATIONS: usize = 100;
const MAX_TEXT_CHARS: usize = 20_000;

// ---------------------------------------------------------------------------
// Caminhos de API
// ---------------------------------------------------------------------------

fn numeric_id(value: &str) -> Result<&str, String> {
    if value.is_empty() || value.len() > 20 || !value.bytes().all(|byte| byte.is_ascii_digit()) {
        return Err("Identificador do Canvas inválido.".into());
    }
    Ok(value)
}

pub fn assignments_path(course_id: &str) -> Result<String, String> {
    Ok(format!(
        "/api/v1/courses/{}/assignments?include[]=submission&per_page=100",
        numeric_id(course_id)?
    ))
}

pub fn submission_comments_path(course_id: &str, assignment_id: u64) -> Result<String, String> {
    Ok(format!(
        "/api/v1/courses/{}/assignments/{assignment_id}/submissions/self?include[]=submission_comments",
        numeric_id(course_id)?
    ))
}

pub fn discussion_topics_path(course_id: &str) -> Result<String, String> {
    Ok(format!(
        "/api/v1/courses/{}/discussion_topics?per_page=100",
        numeric_id(course_id)?
    ))
}

pub fn discussion_view_path(course_id: &str, topic_id: u64) -> Result<String, String> {
    Ok(format!(
        "/api/v1/courses/{}/discussion_topics/{topic_id}/view",
        numeric_id(course_id)?
    ))
}

pub fn announcements_path(course_id: &str) -> Result<String, String> {
    Ok(format!(
        "/api/v1/courses/{}/discussion_topics?only_announcements=true&per_page=50",
        numeric_id(course_id)?
    ))
}

pub fn conversations_path() -> String {
    "/api/v1/conversations?scope=inbox&per_page=50".into()
}

/// Sem `auto_mark_as_read=false` o Canvas marca a conversa como lida ao abri-la.
pub fn conversation_path(conversation_id: u64) -> String {
    format!("/api/v1/conversations/{conversation_id}?auto_mark_as_read=false")
}

/// Aceita só caminhos relativos de leitura em `/api/v1/` e recusa a leitura de conversa que
/// marcaria a mensagem como lida.
pub fn validate_api_path(path: &str) -> Result<(), String> {
    let (route, query) = path.split_once('?').unwrap_or((path, ""));
    let valid_route = route.starts_with("/api/v1/")
        && !route.contains("//")
        && !route.contains("..")
        && route
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'/' | b'_' | b'-'));
    let valid_query = query.bytes().all(|byte| {
        byte.is_ascii_alphanumeric() || matches!(byte, b'=' | b'&' | b'_' | b'[' | b']' | b'-')
    });
    if !valid_route || !valid_query {
        return Err("Caminho de API do Canvas recusado.".into());
    }
    let segments = route
        .trim_start_matches("/api/v1/")
        .split('/')
        .collect::<Vec<_>>();
    if segments.first() == Some(&"conversations")
        && segments.len() > 1
        && !query
            .split('&')
            .any(|pair| pair == "auto_mark_as_read=false")
    {
        return Err("A leitura de conversa precisa de auto_mark_as_read=false.".into());
    }
    Ok(())
}

// ---------------------------------------------------------------------------
// JSON do Canvas (somente os campos usados)
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Deserialize)]
pub struct ApiAssignment {
    pub id: u64,
    pub name: Option<String>,
    pub points_possible: Option<f64>,
    pub due_at: Option<String>,
    pub html_url: Option<String>,
    pub published: Option<bool>,
    pub submission: Option<ApiSubmission>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ApiSubmission {
    pub score: Option<f64>,
    pub grade: Option<String>,
    pub workflow_state: Option<String>,
    pub submitted_at: Option<String>,
    pub graded_at: Option<String>,
    pub late: Option<bool>,
    pub missing: Option<bool>,
    #[serde(default)]
    pub submission_comments: Vec<ApiComment>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ApiComment {
    pub comment: Option<String>,
    pub created_at: Option<String>,
    pub author_name: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ApiTopic {
    pub id: u64,
    pub title: Option<String>,
    pub message: Option<String>,
    pub posted_at: Option<String>,
    pub last_reply_at: Option<String>,
    pub discussion_subentry_count: Option<u64>,
    pub assignment_id: Option<u64>,
    pub locked: Option<bool>,
    pub html_url: Option<String>,
    pub user_name: Option<String>,
    pub author: Option<ApiAuthor>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ApiAuthor {
    pub display_name: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ApiDiscussionView {
    #[serde(default)]
    pub participants: Vec<ApiParticipant>,
    #[serde(default)]
    pub view: Vec<ApiEntry>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ApiParticipant {
    pub id: u64,
    pub display_name: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ApiEntry {
    pub user_id: Option<u64>,
    pub message: Option<String>,
    pub created_at: Option<String>,
    #[serde(default)]
    pub deleted: bool,
    #[serde(default)]
    pub replies: Vec<ApiEntry>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ApiConversation {
    pub id: u64,
    pub subject: Option<String>,
    pub context_name: Option<String>,
    pub last_message_at: Option<String>,
    pub workflow_state: Option<String>,
    #[serde(default)]
    pub participants: Vec<ApiConversationParticipant>,
    #[serde(default)]
    pub messages: Vec<ApiMessage>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ApiConversationParticipant {
    pub id: u64,
    pub name: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ApiMessage {
    pub created_at: Option<String>,
    pub body: Option<String>,
    pub author_id: Option<u64>,
}

// ---------------------------------------------------------------------------
// Registro local
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CanvasAcademicOptions {
    pub grades: bool,
    pub forums: bool,
    pub announcements: bool,
    pub inbox: bool,
}

impl CanvasAcademicOptions {
    pub fn any(&self) -> bool {
        self.grades || self.forums || self.announcements || self.inbox
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CanvasAcademicCapture {
    pub captured_at: String,
    pub options: CanvasAcademicOptions,
    pub courses: Vec<CourseAcademicCapture>,
    pub conversations: Vec<ConversationCapture>,
    pub warnings: Vec<String>,
    pub complete: bool,
    pub json_path: Option<String>,
    pub markdown_path: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CourseAcademicCapture {
    pub course_id: String,
    pub title: String,
    pub kind: CourseKind,
    pub source_url: String,
    pub grades: Vec<GradeItem>,
    pub forums: Vec<ForumTopicCapture>,
    pub announcements: Vec<AnnouncementCapture>,
    pub warnings: Vec<String>,
}

impl CourseAcademicCapture {
    pub fn new(course: &Course) -> Self {
        Self {
            course_id: course.id.clone(),
            title: course.title.clone(),
            kind: course.kind,
            source_url: format!("{CANVAS_ORIGIN}/courses/{}", course.id),
            grades: Vec::new(),
            forums: Vec::new(),
            announcements: Vec::new(),
            warnings: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GradeItem {
    pub assignment_id: u64,
    pub name: String,
    pub points_possible: Option<f64>,
    pub due_at: Option<String>,
    pub score: Option<f64>,
    pub grade: Option<String>,
    pub workflow_state: Option<String>,
    pub submitted_at: Option<String>,
    pub graded_at: Option<String>,
    pub late: bool,
    pub missing: bool,
    pub comments: Vec<TextEntry>,
    /// `false` quando a busca do feedback não foi feita ou falhou.
    pub comments_checked: bool,
    pub source_url: Option<String>,
}

impl GradeItem {
    /// Só vale buscar feedback quando há entrega ou correção.
    pub fn has_submission(&self) -> bool {
        self.submitted_at.is_some()
            || self.graded_at.is_some()
            || self.score.is_some()
            || self
                .workflow_state
                .as_deref()
                .is_some_and(|state| state != "unsubmitted")
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TextEntry {
    pub author: Option<String>,
    pub created_at: Option<String>,
    pub text: String,
    /// Profundidade da resposta no fórum (0 = mensagem de primeiro nível).
    #[serde(default)]
    pub depth: usize,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ForumTopicCapture {
    pub topic_id: u64,
    pub title: String,
    pub author: Option<String>,
    pub posted_at: Option<String>,
    pub last_reply_at: Option<String>,
    pub reply_count: Option<u64>,
    /// Fórum avaliativo (ligado a uma tarefa da A1).
    pub graded: bool,
    pub locked: bool,
    pub opening: String,
    pub entries: Vec<TextEntry>,
    /// `true` quando a árvore de respostas foi lida por inteiro.
    pub entries_complete: bool,
    pub source_url: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AnnouncementCapture {
    pub topic_id: u64,
    pub title: String,
    pub author: Option<String>,
    pub posted_at: Option<String>,
    pub text: String,
    pub source_url: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConversationCapture {
    pub conversation_id: u64,
    pub subject: String,
    pub context: Option<String>,
    pub last_message_at: Option<String>,
    pub workflow_state: Option<String>,
    pub participants: Vec<String>,
    pub messages: Vec<TextEntry>,
    pub messages_checked: bool,
}

// ---------------------------------------------------------------------------
// Conversões
// ---------------------------------------------------------------------------

/// URL do Canvas sem query e fragmento; outros hosts são descartados.
fn safe_canvas_url(value: Option<&str>) -> Option<String> {
    let mut url = url::Url::parse(value?).ok()?;
    if url.scheme() != "https" || url.host_str() != Some("uva.instructure.com") {
        return None;
    }
    url.set_query(None);
    url.set_fragment(None);
    Some(url.to_string())
}

fn title_or(value: Option<String>, fallback: &str) -> String {
    value
        .map(|title| clip(title.trim()))
        .filter(|title| !title.is_empty())
        .unwrap_or_else(|| fallback.to_owned())
}

fn clip(value: &str) -> String {
    if value.chars().count() <= MAX_TEXT_CHARS {
        return value.to_owned();
    }
    let mut clipped = value.chars().take(MAX_TEXT_CHARS).collect::<String>();
    clipped.push_str(" […]");
    clipped
}

pub fn grades_from_assignments(assignments: Vec<ApiAssignment>) -> Vec<GradeItem> {
    assignments
        .into_iter()
        .filter(|assignment| assignment.published != Some(false))
        .map(|assignment| {
            let submission = assignment.submission;
            GradeItem {
                assignment_id: assignment.id,
                name: title_or(assignment.name, "Atividade sem título"),
                points_possible: assignment.points_possible,
                due_at: assignment.due_at,
                score: submission.as_ref().and_then(|value| value.score),
                grade: submission.as_ref().and_then(|value| value.grade.clone()),
                workflow_state: submission
                    .as_ref()
                    .and_then(|value| value.workflow_state.clone()),
                submitted_at: submission
                    .as_ref()
                    .and_then(|value| value.submitted_at.clone()),
                graded_at: submission
                    .as_ref()
                    .and_then(|value| value.graded_at.clone()),
                late: submission
                    .as_ref()
                    .and_then(|value| value.late)
                    .unwrap_or(false),
                missing: submission
                    .as_ref()
                    .and_then(|value| value.missing)
                    .unwrap_or(false),
                comments: Vec::new(),
                comments_checked: false,
                source_url: safe_canvas_url(assignment.html_url.as_deref()),
            }
        })
        .collect()
}

pub fn attach_comments(item: &mut GradeItem, submission: ApiSubmission) {
    item.comments = submission
        .submission_comments
        .into_iter()
        .filter_map(|comment| {
            let text = clip(comment.comment?.trim());
            (!text.is_empty()).then_some(TextEntry {
                author: comment.author_name,
                created_at: comment.created_at,
                text,
                depth: 0,
            })
        })
        .collect();
    item.comments_checked = true;
}

fn topic_author(topic: &ApiTopic) -> Option<String> {
    topic
        .author
        .as_ref()
        .and_then(|author| author.display_name.clone())
        .or_else(|| topic.user_name.clone())
}

pub fn forum_topic(topic: ApiTopic) -> ForumTopicCapture {
    ForumTopicCapture {
        topic_id: topic.id,
        author: topic_author(&topic),
        posted_at: topic.posted_at.clone(),
        last_reply_at: topic.last_reply_at.clone(),
        reply_count: topic.discussion_subentry_count,
        graded: topic.assignment_id.is_some(),
        locked: topic.locked.unwrap_or(false),
        opening: clip(&html_to_text(topic.message.as_deref().unwrap_or(""))),
        source_url: safe_canvas_url(topic.html_url.as_deref()),
        title: title_or(topic.title, "Fórum sem título"),
        entries: Vec::new(),
        entries_complete: false,
    }
}

pub fn apply_discussion_view(topic: &mut ForumTopicCapture, view: ApiDiscussionView) {
    let names = view
        .participants
        .into_iter()
        .filter_map(|participant| Some((participant.id, participant.display_name?)))
        .collect::<std::collections::HashMap<_, _>>();
    let mut entries = Vec::new();
    let mut truncated = false;
    let mut stack = view
        .view
        .into_iter()
        .rev()
        .map(|entry| (entry, 0usize))
        .collect::<Vec<_>>();
    while let Some((entry, depth)) = stack.pop() {
        if entries.len() >= MAX_ENTRIES_PER_TOPIC {
            truncated = true;
            break;
        }
        let ApiEntry {
            user_id,
            message,
            created_at,
            deleted,
            replies,
        } = entry;
        if !deleted {
            let text = clip(&html_to_text(message.as_deref().unwrap_or("")));
            if !text.is_empty() {
                entries.push(TextEntry {
                    author: user_id.and_then(|id| names.get(&id).cloned()),
                    created_at,
                    text,
                    depth,
                });
            }
        }
        stack.extend(replies.into_iter().rev().map(|reply| (reply, depth + 1)));
    }
    topic.entries = entries;
    topic.entries_complete = !truncated;
}

pub fn announcement(topic: ApiTopic) -> AnnouncementCapture {
    AnnouncementCapture {
        topic_id: topic.id,
        author: topic_author(&topic),
        posted_at: topic.posted_at.clone(),
        text: clip(&html_to_text(topic.message.as_deref().unwrap_or(""))),
        source_url: safe_canvas_url(topic.html_url.as_deref()),
        title: title_or(topic.title, "Aviso sem título"),
    }
}

pub fn conversation_summary(conversation: ApiConversation) -> ConversationCapture {
    ConversationCapture {
        conversation_id: conversation.id,
        subject: title_or(conversation.subject, "(sem assunto)"),
        context: conversation.context_name,
        last_message_at: conversation.last_message_at,
        workflow_state: conversation.workflow_state,
        participants: conversation
            .participants
            .into_iter()
            .filter_map(|participant| participant.name)
            .collect(),
        messages: Vec::new(),
        messages_checked: false,
    }
}

pub fn apply_conversation_detail(target: &mut ConversationCapture, detail: ApiConversation) {
    let names = detail
        .participants
        .into_iter()
        .filter_map(|participant| Some((participant.id, participant.name?)))
        .collect::<std::collections::HashMap<_, _>>();
    // O Canvas devolve as mensagens da mais nova para a mais antiga; o registro fica cronológico.
    target.messages = detail
        .messages
        .into_iter()
        .rev()
        .filter_map(|message| {
            let text = clip(message.body?.trim());
            (!text.is_empty()).then(|| TextEntry {
                author: message.author_id.and_then(|id| names.get(&id).cloned()),
                created_at: message.created_at,
                text,
                depth: 0,
            })
        })
        .collect();
    target.messages_checked = true;
}

/// Texto legível de um trecho HTML do Canvas: blocos viram linhas, itens de lista ganham "- ".
pub fn html_to_text(html: &str) -> String {
    fn walk(node: ego_tree::NodeRef<'_, Node>, out: &mut String, depth: usize) {
        if depth > 200 {
            return;
        }
        match node.value() {
            Node::Text(text) => out.push_str(text),
            Node::Element(element) => {
                let name = element.name();
                if matches!(name, "script" | "style" | "template") {
                    return;
                }
                let block = matches!(
                    name,
                    "p" | "div"
                        | "h1"
                        | "h2"
                        | "h3"
                        | "h4"
                        | "h5"
                        | "h6"
                        | "blockquote"
                        | "pre"
                        | "section"
                        | "article"
                        | "table"
                        | "ul"
                        | "ol"
                );
                match name {
                    "br" => out.push('\n'),
                    "li" => out.push_str("\n- "),
                    "tr" => out.push('\n'),
                    _ if block => out.push_str("\n\n"),
                    _ => {}
                }
                for child in node.children() {
                    walk(child, out, depth + 1);
                }
                match name {
                    "td" | "th" => out.push_str(" | "),
                    _ if block => out.push_str("\n\n"),
                    _ => {}
                }
            }
            _ => {
                for child in node.children() {
                    walk(child, out, depth + 1);
                }
            }
        }
    }
    let fragment = Html::parse_fragment(html);
    let mut raw = String::new();
    walk(fragment.tree.root(), &mut raw, 0);
    let mut lines = Vec::<String>::new();
    for line in raw.lines() {
        let line = line.split_whitespace().collect::<Vec<_>>().join(" ");
        let line = line.trim_matches(|character| character == '|' || character == ' ');
        if line.is_empty() || line == "-" {
            if lines.last().is_some_and(|last| !last.is_empty()) {
                lines.push(String::new());
            }
        } else {
            lines.push(line.to_owned());
        }
    }
    while lines.last().is_some_and(String::is_empty) {
        lines.pop();
    }
    lines.join("\n")
}

// ---------------------------------------------------------------------------
// Markdown e gravação
// ---------------------------------------------------------------------------

fn md_inline(value: &str) -> String {
    value
        .replace('\\', "\\\\")
        .replace('|', "\\|")
        .replace(['\n', '\r'], " ")
}

fn md_block(value: &str, indent: &str) -> String {
    value
        .lines()
        .map(|line| {
            if line.is_empty() {
                indent.trim_end().to_owned()
            } else {
                format!("{indent}{line}")
            }
        })
        .collect::<Vec<_>>()
        .join("\n")
}

fn optional(value: Option<&str>) -> String {
    value.map(md_inline).unwrap_or_else(|| "—".into())
}

fn number(value: Option<f64>) -> String {
    value
        .map(|value| {
            let text = format!("{value:.2}");
            let text = text.trim_end_matches('0').trim_end_matches('.');
            text.replace('.', ",")
        })
        .unwrap_or_else(|| "—".into())
}

fn entry_heading(entry: &TextEntry) -> String {
    format!(
        "**{}** · {}",
        md_inline(entry.author.as_deref().unwrap_or("Autor não identificado")),
        optional(entry.created_at.as_deref())
    )
}

pub fn render_markdown(capture: &CanvasAcademicCapture) -> String {
    let mut parts = Vec::new();
    for (enabled, label) in [
        (capture.options.grades, "notas e feedback"),
        (capture.options.forums, "fóruns"),
        (capture.options.announcements, "avisos"),
        (capture.options.inbox, "Caixa de Entrada"),
    ] {
        if enabled {
            parts.push(label);
        }
    }
    let mut out = format!(
        "# Captura acadêmica do Canvas\n\n- Capturada em: {}\n- Partes lidas: {}\n- Cobertura: {}\n- Fonte: API do Canvas (`{CANVAS_ORIGIN}/api/v1`), leitura sem alterar estado de leitura.\n- Notas do Canvas são das atividades online (A1). A nota oficial é a do boletim do Lyceum.\n",
        capture.captured_at,
        parts.join(", "),
        if capture.complete {
            "completa"
        } else {
            "parcial"
        }
    );
    for warning in &capture.warnings {
        out.push_str(&format!("- Aviso: {}\n", md_inline(warning)));
    }

    for course in &capture.courses {
        out.push_str(&format!(
            "\n## {} (Canvas {})\n\nOrigem: {}\n",
            md_inline(&course.title),
            course.course_id,
            course.source_url
        ));
        for warning in &course.warnings {
            out.push_str(&format!("\n> Aviso: {}\n", md_inline(warning)));
        }
        if capture.options.grades {
            out.push_str("\n### Notas das atividades\n\n");
            if course.grades.is_empty() {
                out.push_str("Nenhuma atividade publicada encontrada.\n");
            } else {
                out.push_str("| Atividade | Nota | Pontos | Situação | Prazo | Entregue em | Corrigida em |\n|---|---:|---:|---|---|---|---|\n");
                for item in &course.grades {
                    let mut state = optional(item.workflow_state.as_deref());
                    if item.late {
                        state.push_str(" · atrasada");
                    }
                    if item.missing {
                        state.push_str(" · pendente");
                    }
                    out.push_str(&format!(
                        "| {} | {} | {} | {} | {} | {} | {} |\n",
                        md_inline(&item.name),
                        item.grade
                            .as_deref()
                            .map(md_inline)
                            .unwrap_or_else(|| number(item.score)),
                        number(item.points_possible),
                        state,
                        optional(item.due_at.as_deref()),
                        optional(item.submitted_at.as_deref()),
                        optional(item.graded_at.as_deref()),
                    ));
                }
                for item in course
                    .grades
                    .iter()
                    .filter(|item| !item.comments.is_empty())
                {
                    out.push_str(&format!("\n#### Feedback: {}\n\n", md_inline(&item.name)));
                    for comment in &item.comments {
                        out.push_str(&format!(
                            "- {}\n\n{}\n\n",
                            entry_heading(comment),
                            md_block(&comment.text, "  ")
                        ));
                    }
                }
            }
        }
        if capture.options.forums {
            out.push_str("\n### Fóruns\n\n");
            if course.forums.is_empty() {
                out.push_str("Nenhum fórum visível.\n");
            }
            for topic in &course.forums {
                out.push_str(&format!(
                    "#### {}\n\n- Tipo: {}{}\n- Aberto por: {} · {}\n- Última resposta: {}\n- Respostas: {}\n",
                    md_inline(&topic.title),
                    if topic.graded { "fórum avaliativo (A1)" } else { "fórum" },
                    if topic.locked { " · fechado" } else { "" },
                    optional(topic.author.as_deref()),
                    optional(topic.posted_at.as_deref()),
                    optional(topic.last_reply_at.as_deref()),
                    topic
                        .reply_count
                        .map(|count| count.to_string())
                        .unwrap_or_else(|| "—".into()),
                ));
                if let Some(url) = &topic.source_url {
                    out.push_str(&format!("- Origem: {url}\n"));
                }
                if !topic.opening.is_empty() {
                    out.push_str(&format!("\n{}\n", md_block(&topic.opening, "> ")));
                }
                if !topic.entries_complete {
                    out.push_str("\n_Respostas não lidas por inteiro._\n");
                }
                for entry in &topic.entries {
                    let indent = "  ".repeat(entry.depth);
                    out.push_str(&format!(
                        "\n{indent}- {}\n\n{}\n",
                        entry_heading(entry),
                        md_block(&entry.text, &format!("{indent}  "))
                    ));
                }
                out.push('\n');
            }
        }
        if capture.options.announcements {
            out.push_str("\n### Avisos\n\n");
            if course.announcements.is_empty() {
                out.push_str("Nenhum aviso visível.\n");
            }
            for item in &course.announcements {
                out.push_str(&format!(
                    "#### {}\n\n- Publicado por: {} · {}\n",
                    md_inline(&item.title),
                    optional(item.author.as_deref()),
                    optional(item.posted_at.as_deref())
                ));
                if let Some(url) = &item.source_url {
                    out.push_str(&format!("- Origem: {url}\n"));
                }
                if !item.text.is_empty() {
                    out.push_str(&format!("\n{}\n", md_block(&item.text, "> ")));
                }
                out.push('\n');
            }
        }
    }

    if capture.options.inbox {
        out.push_str("\n## Caixa de Entrada\n\n");
        if capture.conversations.is_empty() {
            out.push_str("Nenhuma conversa encontrada.\n");
        }
        for conversation in &capture.conversations {
            out.push_str(&format!(
                "### {}\n\n- Contexto: {}\n- Participantes: {}\n- Última mensagem: {}\n- Estado no Canvas: {}\n",
                md_inline(&conversation.subject),
                optional(conversation.context.as_deref()),
                if conversation.participants.is_empty() {
                    "—".to_owned()
                } else {
                    md_inline(&conversation.participants.join(", "))
                },
                optional(conversation.last_message_at.as_deref()),
                optional(conversation.workflow_state.as_deref()),
            ));
            if !conversation.messages_checked {
                out.push_str("\n_Mensagens não lidas nesta captura._\n");
            }
            for message in &conversation.messages {
                out.push_str(&format!(
                    "\n- {}\n\n{}\n",
                    entry_heading(message),
                    md_block(&message.text, "  ")
                ));
            }
            out.push('\n');
        }
    }
    out
}

pub fn persist_capture(
    root: &Path,
    sync: &crate::sync::SyncEngine,
    mut capture: CanvasAcademicCapture,
) -> Result<CanvasAcademicCapture, String> {
    let base = format!("materias/catalogo/capturas/canvas-{}", uuid::Uuid::new_v4());
    let json_path = format!("{base}.json");
    let markdown_path = format!("{base}.md");
    capture.json_path = Some(json_path.clone());
    capture.markdown_path = Some(markdown_path.clone());
    let json = serde_json::to_vec_pretty(&capture).map_err(|error| error.to_string())?;
    let markdown = render_markdown(&capture);
    if json.len() > 64 * 1024 * 1024 || markdown.len() > 64 * 1024 * 1024 {
        return Err("A captura excedeu o limite local de 64 MB e não foi salva.".into());
    }
    crate::lyceum::write_new_file_beneath(root, &json_path, &json)?;
    crate::lyceum::write_new_file_beneath(root, &markdown_path, markdown.as_bytes()).map_err(
        |error| {
            format!(
                "O JSON da captura foi preservado em {json_path}; o Markdown ficou pendente: {error}"
            )
        },
    )?;
    let report = sync.reconcile_all()?;
    let errors = report
        .errors
        .iter()
        .filter(|error| error.starts_with(&json_path) || error.starts_with(&markdown_path))
        .cloned()
        .collect::<Vec<_>>();
    if !errors.is_empty() {
        return Err(format!(
            "JSON e Markdown foram preservados; a sincronização local ficou pendente: {}",
            errors.join("; ")
        ));
    }
    Ok(capture)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn api_paths_are_read_only_and_validated() {
        let paths = [
            assignments_path("57897").unwrap(),
            submission_comments_path("57897", 12).unwrap(),
            discussion_topics_path("57897").unwrap(),
            discussion_view_path("57897", 4).unwrap(),
            announcements_path("57897").unwrap(),
            conversations_path(),
            conversation_path(99),
        ];
        for path in &paths {
            assert!(validate_api_path(path).is_ok(), "{path}");
        }
        assert!(assignments_path("57897/../../users").is_err());
        assert!(assignments_path("").is_err());
        assert!(validate_api_path("https://attacker.example/api/v1/courses").is_err());
        assert!(validate_api_path("//attacker.example/api/v1/x").is_err());
        assert!(validate_api_path("/api/v1/../login").is_err());
        assert!(validate_api_path("/courses/1").is_err());
        assert!(validate_api_path("/api/v1/courses?x=<script>").is_err());
    }

    #[test]
    fn conversation_detail_requires_disabling_auto_read() {
        assert!(validate_api_path("/api/v1/conversations/99").is_err());
        assert!(validate_api_path("/api/v1/conversations/99?auto_mark_as_read=true").is_err());
        assert!(conversation_path(99).ends_with("auto_mark_as_read=false"));
        assert!(validate_api_path(&conversations_path()).is_ok());
    }

    #[test]
    fn grades_keep_score_state_and_feedback() {
        let assignments: Vec<ApiAssignment> = serde_json::from_str(
            r#"[
              {"id": 1, "name": "AVA 1 – Trabalho", "points_possible": 100, "due_at": "2026-09-20T02:59:59Z",
               "html_url": "https://uva.instructure.com/courses/5/assignments/1?module_item_id=8",
               "published": true,
               "submission": {"score": 95, "grade": "95", "workflow_state": "graded",
                              "submitted_at": "2026-09-18T10:00:00Z", "graded_at": "2026-09-25T10:00:00Z",
                              "late": false, "missing": false}},
              {"id": 2, "name": "AVA 2 – Fórum", "points_possible": 100, "published": true,
               "submission": {"workflow_state": "unsubmitted", "missing": true}},
              {"id": 3, "name": "Rascunho", "published": false},
              {"id": 4, "name": "Externo", "html_url": "https://attacker.example/a"}
            ]"#,
        )
        .unwrap();
        let mut grades = grades_from_assignments(assignments);
        assert_eq!(grades.len(), 3);
        assert_eq!(grades[0].score, Some(95.0));
        assert_eq!(
            grades[0].source_url.as_deref(),
            Some("https://uva.instructure.com/courses/5/assignments/1")
        );
        assert!(grades[0].has_submission());
        assert!(!grades[1].has_submission());
        assert!(grades[1].missing);
        assert_eq!(grades[2].source_url, None);

        let submission: ApiSubmission = serde_json::from_str(
            r#"{"submission_comments": [
                {"comment": "Bom trabalho.\nRevise a seção 2.", "created_at": "2026-09-25T10:00:00Z", "author_name": "Tutor"},
                {"comment": "   ", "author_name": "Vazio"}
            ]}"#,
        )
        .unwrap();
        attach_comments(&mut grades[0], submission);
        assert!(grades[0].comments_checked);
        assert_eq!(grades[0].comments.len(), 1);
        assert_eq!(grades[0].comments[0].author.as_deref(), Some("Tutor"));
    }

    #[test]
    fn discussion_view_is_flattened_in_thread_order_with_names() {
        let topic: ApiTopic = serde_json::from_str(
            r#"{"id": 7, "title": "Fórum de Dúvidas", "message": "<p>Poste suas <b>dúvidas</b>.</p>",
                "assignment_id": null, "discussion_subentry_count": 3,
                "author": {"display_name": "Tutor"},
                "html_url": "https://uva.instructure.com/courses/5/discussion_topics/7?x=1"}"#,
        )
        .unwrap();
        let mut forum = forum_topic(topic);
        assert!(!forum.graded);
        assert_eq!(forum.opening, "Poste suas dúvidas.");
        let view: ApiDiscussionView = serde_json::from_str(
            r#"{"participants": [{"id": 1, "display_name": "Ana"}, {"id": 2, "display_name": "Tutor"}],
                "view": [
                  {"id": 10, "user_id": 1, "message": "<p>Primeira</p>", "created_at": "a",
                   "replies": [{"id": 11, "user_id": 2, "message": "<p>Resposta</p>", "created_at": "b",
                                "replies": [{"id": 12, "user_id": 1, "message": "<p>Obrigada</p>"}]}]},
                  {"id": 13, "deleted": true},
                  {"id": 14, "user_id": 9, "message": "<p>Segunda</p>"}
                ]}"#,
        )
        .unwrap();
        apply_discussion_view(&mut forum, view);
        let summary = forum
            .entries
            .iter()
            .map(|entry| (entry.text.as_str(), entry.depth, entry.author.as_deref()))
            .collect::<Vec<_>>();
        assert_eq!(
            summary,
            vec![
                ("Primeira", 0, Some("Ana")),
                ("Resposta", 1, Some("Tutor")),
                ("Obrigada", 2, Some("Ana")),
                ("Segunda", 0, None),
            ]
        );
        assert!(forum.entries_complete);
    }

    #[test]
    fn graded_forum_is_flagged() {
        let topic: ApiTopic = serde_json::from_str(
            r#"{"id": 8, "title": "AVA 2", "assignment_id": 22, "locked": true}"#,
        )
        .unwrap();
        let forum = forum_topic(topic);
        assert!(forum.graded);
        assert!(forum.locked);
    }

    #[test]
    fn conversation_messages_are_chronological_and_named() {
        let summary: ApiConversation = serde_json::from_str(
            r#"{"id": 3, "subject": "Prova A2", "context_name": "Espaço Curso", "workflow_state": "unread",
                "participants": [{"id": 1, "name": "Monitoria"}, {"id": 2, "name": "Estudante"}]}"#,
        )
        .unwrap();
        let mut conversation = conversation_summary(summary);
        assert!(!conversation.messages_checked);
        let detail: ApiConversation = serde_json::from_str(
            r#"{"id": 3, "participants": [{"id": 1, "name": "Monitoria"}],
                "messages": [{"body": "Segunda", "author_id": 2}, {"body": "Primeira", "author_id": 1}]}"#,
        )
        .unwrap();
        apply_conversation_detail(&mut conversation, detail);
        assert_eq!(conversation.messages[0].text, "Primeira");
        assert_eq!(
            conversation.messages[0].author.as_deref(),
            Some("Monitoria")
        );
        assert_eq!(conversation.messages[1].author, None);
        assert_eq!(conversation.workflow_state.as_deref(), Some("unread"));
    }

    #[test]
    fn html_to_text_keeps_structure_without_markup() {
        let text = html_to_text(
            "<p>Olá,&nbsp;turma</p><ul><li>Item 1</li><li>Item 2</li></ul><script>alert(1)</script><p>Fim<br>linha</p>",
        );
        assert_eq!(text, "Olá, turma\n\n- Item 1\n- Item 2\n\nFim\nlinha");
    }

    #[test]
    fn markdown_escapes_tables_and_marks_partial_reads() {
        let course = Course {
            id: "5".into(),
            title: "Bases | Programação".into(),
            url: "https://uva.instructure.com/courses/5".into(),
            kind: CourseKind::Current,
        };
        let mut record = CourseAcademicCapture::new(&course);
        record.grades = grades_from_assignments(
            serde_json::from_str(r#"[{"id": 1, "name": "AVA | 1", "points_possible": 100, "submission": {"score": 87.5}}]"#)
                .unwrap(),
        );
        record.forums.push(forum_topic(
            serde_json::from_str(r#"{"id": 2, "title": "Dúvidas"}"#).unwrap(),
        ));
        let capture = CanvasAcademicCapture {
            captured_at: "2026-11-06T12:00:00Z".into(),
            options: CanvasAcademicOptions {
                grades: true,
                forums: true,
                announcements: false,
                inbox: true,
            },
            courses: vec![record],
            conversations: Vec::new(),
            warnings: vec!["Um fórum não abriu.".into()],
            complete: false,
            json_path: None,
            markdown_path: None,
        };
        let markdown = render_markdown(&capture);
        assert!(markdown.contains("## Bases \\| Programação (Canvas 5)"));
        assert!(markdown.contains("| AVA \\| 1 | 87,5 | 100 |"));
        assert!(markdown.contains("Cobertura: parcial"));
        assert!(markdown.contains("_Respostas não lidas por inteiro._"));
        assert!(markdown.contains("Nenhuma conversa encontrada."));
        assert!(!markdown.contains("### Avisos"));
    }
}
