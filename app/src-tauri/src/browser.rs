use crate::{
    calendar::CanvasAssignment,
    canvas::{
        Course, ModuleGroup, discover_material_links, group_module_items, parse_courses,
        parse_module_items,
    },
    download::validate_material_url,
};
use chromiumoxide::Page;
use chromiumoxide::{Browser, BrowserConfig};
use futures_util::StreamExt;
use std::{collections::HashMap, path::Path};
use thiserror::Error;
use tokio::{
    task::JoinHandle,
    time::{Duration, timeout},
};

const BROWSER_CLOSE_TIMEOUT: Duration = Duration::from_secs(5);
const BROWSER_EXIT_TIMEOUT: Duration = Duration::from_secs(5);
const BROWSER_KILL_TIMEOUT: Duration = Duration::from_secs(5);

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CanvasState {
    Disconnected,
    WaitingForLogin,
    Authenticated,
    SessionExpired,
    BrowserError,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CanvasStatus {
    pub state: CanvasState,
    pub message: String,
    pub current_url: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MaterialChoice {
    pub id: String,
    pub title: String,
    pub host: String,
    pub embedded: bool,
    pub allowed: bool,
}

#[derive(Debug, Error)]
pub enum BrowserError {
    #[error("não foi possível preparar o perfil do Chromium")]
    Profile,
    #[error("falha ao iniciar Chromium: {0}")]
    Launch(String),
    #[error("falha ao inspecionar a página local: {0}")]
    Page(String),
    #[error("a página de verificação não carregou o conteúdo esperado")]
    Content,
}

pub struct CanvasSession {
    browser: Browser,
    page: Page,
    handler_task: JoinHandle<()>,
    discovered_materials: HashMap<String, String>,
}

impl CanvasSession {
    pub async fn launch(executable: &str, profile: &Path) -> Result<Self, BrowserError> {
        std::fs::create_dir_all(profile).map_err(|_| BrowserError::Profile)?;
        let config = BrowserConfig::builder()
            .chrome_executable(executable)
            .user_data_dir(profile)
            .with_head()
            .build()
            .map_err(|error| BrowserError::Launch(error.to_string()))?;
        let (browser, mut handler) = timeout(Duration::from_secs(20), Browser::launch(config))
            .await
            .map_err(|_| BrowserError::Launch("Chromium excedeu o limite de inicialização".into()))?
            .map_err(|error| BrowserError::Launch(error.to_string()))?;
        let handler_task = tokio::spawn(async move {
            while let Some(result) = handler.next().await {
                if result.is_err() {
                    break;
                }
            }
        });
        let page = match timeout(
            Duration::from_secs(20),
            browser.new_page("https://uva.instructure.com/courses"),
        )
        .await
        {
            Ok(Ok(page)) => page,
            Ok(Err(_)) | Err(_) => {
                let mut browser = browser;
                let _ = shutdown_browser(&mut browser).await;
                handler_task.abort();
                return Err(BrowserError::Page(
                    "Chromium não abriu a página de navegação no tempo esperado".into(),
                ));
            }
        };
        Ok(Self {
            browser,
            page,
            handler_task,
            discovered_materials: HashMap::new(),
        })
    }

    pub async fn status(&self) -> CanvasStatus {
        let url = self.page.url().await.ok().flatten();
        let content = self.page.content().await.unwrap_or_default().to_lowercase();
        let login = url
            .as_deref()
            .is_some_and(|value| value.contains("/login") || value.contains("/sso"))
            || content.contains("id=\"login_form\"")
            || content.contains("type=\"password\"")
            || content.contains("name=\"password\"");
        let expired = content.contains("session has expired")
            || content.contains("sua sessão expirou")
            || content.contains("session expired");
        let authenticated = url
            .as_deref()
            .and_then(|value| url::Url::parse(value).ok())
            .is_some_and(|url| {
                url.host_str() == Some("uva.instructure.com") && url.path().starts_with("/courses")
            });
        let (state, message) = if expired {
            (
                CanvasState::SessionExpired,
                "Sua sessão do Canvas expirou. Entre novamente na janela do Chromium.",
            )
        } else if login {
            (
                CanvasState::WaitingForLogin,
                "Entre no Canvas manualmente na janela Chromium. O Estuda UVA não solicita nem registra sua senha.",
            )
        } else if authenticated {
            (
                CanvasState::Authenticated,
                "Canvas aberto. Você pode atualizar a lista de disciplinas.",
            )
        } else {
            (
                CanvasState::WaitingForLogin,
                "Conclua o acesso ao Canvas na janela Chromium para continuar.",
            )
        };
        let safe_url = url.and_then(|value| {
            url::Url::parse(&value).ok().map(|mut parsed| {
                parsed.set_query(None);
                parsed.set_fragment(None);
                parsed.to_string()
            })
        });
        CanvasStatus {
            state,
            message: message.to_owned(),
            current_url: safe_url,
        }
    }

    pub async fn list_courses(&self) -> Result<Vec<Course>, BrowserError> {
        self.page
            .goto("https://uva.instructure.com/courses")
            .await
            .map_err(|_| BrowserError::Page("não foi possível abrir a lista do Canvas".into()))?;
        let content =
            self.page.content().await.map_err(|_| {
                BrowserError::Page("não foi possível ler a página do Canvas".into())
            })?;
        Ok(parse_courses(&content))
    }

    pub async fn load_course(&self, course_id: &str) -> Result<Vec<ModuleGroup>, BrowserError> {
        if course_id.is_empty() || !course_id.bytes().all(|byte| byte.is_ascii_digit()) {
            return Err(BrowserError::Content);
        }
        self.page
            .goto(format!("https://uva.instructure.com/courses/{course_id}"))
            .await
            .map_err(|_| BrowserError::Page("não foi possível abrir a disciplina".into()))?;
        let content = self.page.content().await.map_err(|_| {
            BrowserError::Page("não foi possível ler os módulos da disciplina".into())
        })?;
        Ok(group_module_items(parse_module_items(&content)))
    }

    pub async fn list_assignment_events(
        &self,
        course: &Course,
    ) -> Result<Vec<CanvasAssignment>, BrowserError> {
        if course.id.is_empty() || !course.id.bytes().all(|byte| byte.is_ascii_digit()) {
            return Err(BrowserError::Content);
        }
        self.page
            .goto(format!(
                "https://uva.instructure.com/courses/{}/assignments",
                course.id
            ))
            .await
            .map_err(|_| {
                BrowserError::Page("não foi possível abrir os prazos da disciplina".into())
            })?;
        if !matches!(self.status().await.state, CanvasState::Authenticated) {
            return Err(BrowserError::Page(
                "a sessão do Canvas expirou durante a consulta de prazos".into(),
            ));
        }
        let expression = format!(
            r#"(async () => {{
            let url = new URL("/api/v1/courses/{}/assignments?per_page=100&override_assignment_dates=true", location.origin);
            const rows = [];
            for (let page = 0; url && page < 20; page += 1) {{
                if (url.origin !== location.origin) throw new Error("origem do Canvas inválida");
                const response = await fetch(url, {{ credentials: "same-origin", headers: {{ Accept: "application/json" }} }});
                if (!response.ok) throw new Error("Canvas retornou HTTP " + response.status);
                const assignments = await response.json();
                rows.push(...assignments.map(item => ({{ id: item.id, name: item.name, due_at: item.due_at, html_url: item.html_url, published: item.published }})));
                const next = (response.headers.get("Link") || "").split(",").find(link => /rel="next"/.test(link));
                const match = next && next.match(/<([^>]+)>/);
                url = match ? new URL(match[1], location.origin) : null;
            }}
            return JSON.stringify(rows);
        }})()"#,
            course.id
        );
        let result = self.page.evaluate(expression).await.map_err(|_| {
            BrowserError::Page("não foi possível ler os prazos publicados no Canvas".into())
        })?;
        let json = result.into_value::<String>().map_err(|_| {
            BrowserError::Page("o Canvas retornou dados de prazo em formato inesperado".into())
        })?;
        serde_json::from_str(&json).map_err(|_| {
            BrowserError::Page("não foi possível interpretar os prazos do Canvas".into())
        })
    }

    pub async fn open_module_item(
        &mut self,
        course_id: &str,
        href: &str,
    ) -> Result<Vec<MaterialChoice>, BrowserError> {
        let base = url::Url::parse(&format!("https://uva.instructure.com/courses/{course_id}/"))
            .map_err(|_| BrowserError::Content)?;
        let url = base.join(href).map_err(|_| BrowserError::Content)?;
        if url.scheme() != "https" || url.host_str() != Some("uva.instructure.com") {
            return Err(BrowserError::Content);
        }
        self.page
            .goto(url.as_str())
            .await
            .map_err(|_| BrowserError::Page("não foi possível abrir este item do módulo".into()))?;
        let content =
            self.page.content().await.map_err(|_| {
                BrowserError::Page("não foi possível ler o material do módulo".into())
            })?;
        self.discovered_materials.clear();
        Ok(discover_material_links(&content, url.as_str())
            .into_iter()
            .map(|link| {
                let id = uuid::Uuid::new_v4().to_string();
                let host = url::Url::parse(&link.href)
                    .ok()
                    .and_then(|url| url.host_str().map(str::to_owned))
                    .unwrap_or_default();
                let allowed = validate_material_url(&link.href).is_ok();
                self.discovered_materials.insert(id.clone(), link.href);
                MaterialChoice {
                    id,
                    title: link.title,
                    host,
                    embedded: link.embedded,
                    allowed,
                }
            })
            .collect())
    }

    pub fn material_url(&self, id: &str) -> Result<String, BrowserError> {
        self.discovered_materials
            .get(id)
            .cloned()
            .ok_or(BrowserError::Content)
    }

    pub async fn cookies_for_host(&self, host: &str) -> Result<String, BrowserError> {
        let cookies = self.browser.get_cookies().await.map_err(|_| {
            BrowserError::Page("não foi possível consultar a sessão do navegador".into())
        })?;
        let normalized_host = host.to_ascii_lowercase();
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs_f64();
        let pairs = cookies
            .into_iter()
            .filter(|cookie| {
                let domain = cookie.domain.trim_start_matches('.').to_ascii_lowercase();
                normalized_host == domain || normalized_host.ends_with(&format!(".{domain}"))
            })
            .filter(|cookie| cookie.expires < 0.0 || cookie.expires > now)
            .filter(|cookie| {
                !cookie.name.is_empty()
                    && !cookie.name.contains([';', '=', '\r', '\n'])
                    && !cookie.value.contains([';', '\r', '\n'])
            })
            .map(|cookie| format!("{}={}", cookie.name, cookie.value))
            .collect::<Vec<_>>();
        Ok(pairs.join("; "))
    }

    pub async fn current_url(&self) -> Result<Option<String>, BrowserError> {
        self.page
            .url()
            .await
            .map_err(|_| BrowserError::Page("não foi possível consultar a página atual".into()))
    }

    pub async fn close(mut self) {
        let _ = timeout(BROWSER_CLOSE_TIMEOUT, self.page.close()).await;
        let _ = shutdown_browser(&mut self.browser).await;
        self.handler_task.abort();
    }
}

async fn shutdown_browser(browser: &mut Browser) -> Result<(), BrowserError> {
    let close_acknowledged = matches!(
        timeout(BROWSER_CLOSE_TIMEOUT, browser.close()).await,
        Ok(Ok(_))
    );
    if close_acknowledged
        && matches!(
            timeout(BROWSER_EXIT_TIMEOUT, browser.wait()).await,
            Ok(Ok(_))
        )
    {
        return Ok(());
    }

    match timeout(BROWSER_KILL_TIMEOUT, browser.kill()).await {
        Ok(Some(Ok(()))) | Ok(None) => Ok(()),
        Ok(Some(Err(_))) | Err(_) => Err(BrowserError::Page(
            "não foi possível encerrar o processo Chromium".into(),
        )),
    }
}

pub async fn run_browser_smoke(
    executable: &str,
    profile: &Path,
    headless: bool,
) -> Result<(), BrowserError> {
    std::fs::create_dir_all(profile).map_err(|_| BrowserError::Profile)?;
    let builder = BrowserConfig::builder()
        .chrome_executable(executable)
        .user_data_dir(profile);
    let config = if headless {
        builder.new_headless_mode()
    } else {
        builder.with_head()
    }
    .build()
    .map_err(|error| BrowserError::Launch(error.to_string()))?;
    let (browser, mut handler) = timeout(Duration::from_secs(20), Browser::launch(config))
        .await
        .map_err(|_| BrowserError::Launch("Chromium excedeu o limite de inicialização".into()))?
        .map_err(|error| BrowserError::Launch(error.to_string()))?;
    let handler_task = tokio::spawn(async move {
        while let Some(result) = handler.next().await {
            if result.is_err() {
                break;
            }
        }
    });
    let result = timeout(Duration::from_secs(20), async {
        let page = browser.new_page("data:text/html,%3C!doctype%20html%3E%3Ch1%3EEstuda%20UVA%20navega%20em%20Rust%3C%2Fh1%3E").await
            .map_err(|error| BrowserError::Page(error.to_string()))?;
        let content = page.content().await.map_err(|error| BrowserError::Page(error.to_string()))?;
        if !content.contains("Estuda UVA navega em Rust") { return Err(BrowserError::Content); }
        page.close().await.map_err(|error| BrowserError::Page(error.to_string()))?;
        Ok(())
    }).await.unwrap_or_else(|_| Err(BrowserError::Page("Chromium não respondeu ao teste local no tempo esperado".into())));
    let mut browser = browser;
    let shutdown = shutdown_browser(&mut browser).await;
    handler_task.abort();
    match result {
        Err(error) => Err(error),
        Ok(()) => shutdown,
    }
}
