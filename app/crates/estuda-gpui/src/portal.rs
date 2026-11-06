//! Navegação no Portal do Aluno e no Lyceum pelo Chromium do app (CDP, sem Playwright).
//!
//! Esta etapa só abre a janela do Portal e consulta o estado da sessão. O aluno digita o
//! login e o MFA na janela, nunca no app. A captura que grava arquivos e o SQLite fica
//! para uma etapa com aprovação prévia do dono.

use std::path::PathBuf;

use estuda_core::browser::CanvasSession;
use estuda_core::lyceum::PortalStatus;

/// Mesmo perfil do app original (`app_data_dir` do Tauri no macOS). Assim a sessão do
/// Canvas e do Portal feita antes continua válida. Fica fora do repositório.
fn profile_dir() -> Option<PathBuf> {
    let home = std::env::var_os("HOME")?;
    Some(PathBuf::from(home).join("Library/Application Support/br.edu.uva.estuda/chromium-profile"))
}

/// Executáveis de Chromium na ordem do app original.
fn chromium_executable() -> Option<PathBuf> {
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

/// Sessão do Portal. O chromiumoxide exige um runtime do tokio, que roda aqui dentro.
pub struct Portal {
    runtime: tokio::runtime::Runtime,
    session: tokio::sync::Mutex<Option<CanvasSession>>,
}

impl Portal {
    pub fn new() -> Result<Self, String> {
        let runtime = tokio::runtime::Builder::new_multi_thread()
            .worker_threads(2)
            .enable_all()
            .build()
            .map_err(|error| error.to_string())?;
        Ok(Self {
            runtime,
            session: tokio::sync::Mutex::new(None),
        })
    }

    /// Abre (ou reaproveita) o Chromium do app e leva à página do Lyceum.
    /// Deve ser chamada fora da thread da interface.
    pub fn open_lyceum(&self) -> Result<PortalStatus, String> {
        self.runtime.block_on(async {
            let mut slot = self.session.lock().await;
            if slot.is_none() {
                let executable = chromium_executable().ok_or_else(|| {
                    "Chromium não encontrado. Instale o Google Chrome ou o Chromium.".to_string()
                })?;
                let profile = profile_dir().ok_or_else(|| "HOME não definido.".to_string())?;
                let session = CanvasSession::launch(&executable.to_string_lossy(), &profile)
                    .await
                    .map_err(|error| error.to_string())?;
                *slot = Some(session);
            }
            let session = slot
                .as_mut()
                .ok_or_else(|| "Não foi possível abrir a janela do portal UVA.".to_string())?;
            session
                .open_lyceum()
                .await
                .map_err(|error| error.to_string())
        })
    }

    /// Estado atual da página do Portal, sem navegar.
    pub fn lyceum_status(&self) -> Option<PortalStatus> {
        self.runtime.block_on(async {
            let slot = self.session.lock().await;
            match slot.as_ref() {
                Some(session) => Some(session.lyceum_status().await),
                None => None,
            }
        })
    }
}
