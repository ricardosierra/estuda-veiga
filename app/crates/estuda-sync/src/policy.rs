//! Política **somente leitura**. Toda requisição e todo clique passam por aqui antes de sair.
//!
//! Se uma ação parecer necessária para continuar, o chamador deve parar e registrar o
//! [`Blocker`], nunca contornar. A lista de hosts permitidos só inclui o que já foi
//! observado (ver `docs/NAVEGACAO_PORTAL_UVA.md`); Sistema de Provas, Zeev e Pagamento
//! entram com [`ReadOnlyPolicy::with_host`] depois da primeira observação registrada.

use thiserror::Error;
use url::Url;

use crate::text::fold;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HttpMethod {
    Get,
    Head,
    Post,
    Put,
    Patch,
    Delete,
}

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum Blocker {
    #[error("método {0:?} não é de leitura")]
    MutatingMethod(HttpMethod),
    #[error("recurso de autenticação não é acessado automaticamente")]
    AuthResource,
    #[error("ação de escrita bloqueada: {0}")]
    MutatingAction(String),
    #[error("host fora da lista permitida: {0}")]
    HostNotAllowed(String),
    #[error("somente HTTPS")]
    NotHttps,
    #[error("URL inválida")]
    InvalidUrl,
    #[error("leitura com efeito colateral (marca como lido): {0}")]
    SideEffectingRead(&'static str),
}

/// Papel do elemento clicado. Menus que só abrem uma tela de consulta são navegação.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ElementRole {
    Navigation,
    Button,
}

const AUTH_SEGMENTS: &[&str] = &[
    "login",
    "logout",
    "sso",
    "autologin",
    "oauth",
    "oauth2",
    "saml",
    "signin",
    "sign-in",
    "token",
    "authorize",
];

/// Rótulos (já sem acento e minúsculos) que nunca podem ser acionados.
const MUTATING_LABELS: &[&str] = &[
    "enviar",
    "entregar",
    "responder",
    "postar",
    "publicar",
    "presenca",
    "agendar",
    "inscrever",
    "cancelar",
    "confirmar",
    "salvar",
    "aceitar",
    "concordo",
    "matricular",
    "rematricula",
    "trancar",
    "solicitar",
    "nova solicitacao",
    "iniciar prova",
    "iniciar tarefa",
    "carregar arquivo",
    "excluir",
    "apagar",
    "alterar",
    "editar",
    "pagar",
    "boleto",
    "negociar",
    "sair",
    "finalizar",
    "assinar",
    "autorizar",
];

/// Telas de consulta cujo nome contém um verbo de escrita mas que apenas exibem dados
/// quando abertas por menu (`docs/NAVEGACAO_PORTAL_UVA.md`, P2). Valem só como navegação;
/// os botões dentro delas continuam bloqueados.
const READ_ONLY_NAVIGATION_EXCEPTIONS: &[&str] = &["agendar provas", "provas realizadas"];

#[derive(Debug, Clone)]
pub struct ReadOnlyPolicy {
    allowed_hosts: Vec<String>,
}

impl ReadOnlyPolicy {
    /// Hosts já observados no acervo.
    pub fn uva_default() -> Self {
        Self {
            allowed_hosts: [
                "portalaluno.uva.br",
                "sis.lyceum.com.br",
                "uva.instructure.com",
                "ead.uva.br",
            ]
            .iter()
            .map(|h| (*h).to_owned())
            .collect(),
        }
    }

    pub fn with_host(mut self, host: &str) -> Self {
        self.allowed_hosts.push(host.to_ascii_lowercase());
        self
    }

    fn host_allowed(&self, host: &str) -> bool {
        let host = host.to_ascii_lowercase();
        self.allowed_hosts
            .iter()
            .any(|allowed| host == *allowed || host.ends_with(&format!(".{allowed}")))
    }

    pub fn check_request(&self, method: HttpMethod, raw_url: &str) -> Result<(), Blocker> {
        if !matches!(method, HttpMethod::Get | HttpMethod::Head) {
            return Err(Blocker::MutatingMethod(method));
        }
        let url = Url::parse(raw_url).map_err(|_| Blocker::InvalidUrl)?;
        if url.scheme() != "https" {
            return Err(Blocker::NotHttps);
        }
        if !url.username().is_empty() || url.password().is_some() || url.port().is_some() {
            return Err(Blocker::HostNotAllowed(url.authority().to_owned()));
        }
        let host = url.host_str().ok_or(Blocker::InvalidUrl)?;
        if !self.host_allowed(host) {
            return Err(Blocker::HostNotAllowed(host.to_owned()));
        }
        if is_auth_path(url.path()) {
            return Err(Blocker::AuthResource);
        }
        check_side_effects(&url)
    }

    pub fn check_action(&self, label: &str, role: ElementRole) -> Result<(), Blocker> {
        let folded = fold(label);
        if role == ElementRole::Navigation
            && READ_ONLY_NAVIGATION_EXCEPTIONS.contains(&folded.as_str())
        {
            return Ok(());
        }
        let words: Vec<&str> = folded
            .split(|c: char| !c.is_alphanumeric())
            .filter(|w| !w.is_empty())
            .collect();
        for banned in MUTATING_LABELS {
            let hit = if banned.contains(' ') {
                folded.contains(banned)
            } else {
                words.iter().any(|w| w == banned)
            };
            if hit {
                return Err(Blocker::MutatingAction(label.trim().to_owned()));
            }
        }
        Ok(())
    }
}

fn is_auth_path(path: &str) -> bool {
    path.split('/')
        .map(str::to_ascii_lowercase)
        .any(|seg| AUTH_SEGMENTS.contains(&seg.as_str()))
}

/// Canvas marca a conversa como lida ao abri-la, a menos que `auto_mark_as_read=false`.
fn check_side_effects(url: &Url) -> Result<(), Blocker> {
    let segments: Vec<&str> = url.path().trim_matches('/').split('/').collect();
    let is_conversation_detail = segments.len() == 4
        && segments[..3] == ["api", "v1", "conversations"]
        && segments[3].bytes().all(|b| b.is_ascii_digit());
    if is_conversation_detail {
        let safe = url
            .query_pairs()
            .any(|(k, v)| k == "auto_mark_as_read" && v == "false");
        if !safe {
            return Err(Blocker::SideEffectingRead(
                "conversa sem auto_mark_as_read=false",
            ));
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn policy() -> ReadOnlyPolicy {
        ReadOnlyPolicy::uva_default()
    }

    #[test]
    fn only_get_and_head_pass() {
        let url = "https://uva.instructure.com/api/v1/courses";
        assert!(policy().check_request(HttpMethod::Get, url).is_ok());
        assert!(policy().check_request(HttpMethod::Head, url).is_ok());
        for m in [
            HttpMethod::Post,
            HttpMethod::Put,
            HttpMethod::Patch,
            HttpMethod::Delete,
        ] {
            assert_eq!(
                policy().check_request(m, url),
                Err(Blocker::MutatingMethod(m))
            );
        }
    }

    #[test]
    fn auth_routes_are_never_automated() {
        for path in [
            "/login",
            "/sso/start",
            "/AutoLogin",
            "/a/b/logout",
            "/login/oauth2/auth",
        ] {
            let url = format!("https://sis.lyceum.com.br{path}");
            assert_eq!(
                policy().check_request(HttpMethod::Get, &url),
                Err(Blocker::AuthResource),
                "{url}"
            );
        }
    }

    #[test]
    fn host_boundaries_are_strict() {
        for bad in [
            "http://ead.uva.br/a",
            "https://ead.uva.br.attacker.example/a",
            "https://evil-ead.uva.br.example/a",
            "https://user@ead.uva.br/a",
            "https://ead.uva.br:8080/a",
            "https://provas.uva.br/a", // host ainda não observado
        ] {
            assert!(
                policy().check_request(HttpMethod::Get, bad).is_err(),
                "{bad}"
            );
        }
        assert!(
            policy()
                .check_request(HttpMethod::Get, "https://ead.uva.br/a.pdf")
                .is_ok()
        );
        let extended = policy().with_host("provas.uva.br");
        assert!(
            extended
                .check_request(HttpMethod::Get, "https://provas.uva.br/a")
                .is_ok()
        );
    }

    #[test]
    fn inbox_read_requires_auto_mark_as_read_false() {
        let base = "https://uva.instructure.com/api/v1/conversations/42";
        assert!(matches!(
            policy().check_request(HttpMethod::Get, base),
            Err(Blocker::SideEffectingRead(_))
        ));
        assert!(
            policy()
                .check_request(HttpMethod::Get, &format!("{base}?auto_mark_as_read=false"))
                .is_ok()
        );
        assert!(
            policy()
                .check_request(
                    HttpMethod::Get,
                    "https://uva.instructure.com/api/v1/conversations?scope=inbox"
                )
                .is_ok()
        );
    }

    #[test]
    fn mutating_buttons_are_blocked_in_every_spelling() {
        for label in [
            "Inscrever",
            "Salvar",
            "Cancelar Inscrição",
            "Enviar Tarefa",
            "Iniciar Prova Online",
            "Confirmar",
            "Aceitar termos",
            "Nova Solicitação",
            "Carregar Arquivo",
            "Sair",
            "Marcar presença",
            "Concordo",
        ] {
            assert!(
                policy().check_action(label, ElementRole::Button).is_err(),
                "{label}"
            );
        }
    }

    #[test]
    fn read_only_labels_and_lookalikes_pass() {
        for label in [
            "Boletim",
            "Frequência",
            "Histórico do Aluno",
            "Mensagens enviadas",
            "Respondidas",
            "Notas",
        ] {
            assert!(
                policy().check_action(label, ElementRole::Button).is_ok(),
                "{label}"
            );
        }
    }

    #[test]
    fn schedule_screen_is_navigation_only() {
        assert!(
            policy()
                .check_action("Agendar Provas", ElementRole::Navigation)
                .is_ok()
        );
        assert!(
            policy()
                .check_action("Agendar Provas", ElementRole::Button)
                .is_err()
        );
        assert!(
            policy()
                .check_action("Agendar", ElementRole::Navigation)
                .is_err()
        );
    }
}
