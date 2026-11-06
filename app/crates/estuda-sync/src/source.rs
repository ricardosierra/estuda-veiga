//! Identificação de sistemas de origem e da proveniência de cada dado.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum SourceSystem {
    /// Portal do Aluno (`portalaluno.uva.br`), página de entrada.
    Portal,
    /// Sistema do Aluno (Lyceum): boletim, frequência, histórico.
    Lyceum,
    /// Plataforma Virtual (Canvas).
    Canvas,
    /// Sistema de Provas (SGP/Avalia). Host **a validar**: ainda não observado.
    ExamSystem,
    /// Calendário acadêmico oficial publicado.
    AcademicCalendar,
    /// Sistemas complementares (Solicitações, Universo UVA, Carreiras UVA).
    Complementary,
    /// Material local do acervo (não é fonte oficial de situação acadêmica).
    LocalMaterial,
}

impl SourceSystem {
    pub fn label(self) -> &'static str {
        match self {
            SourceSystem::Portal => "Portal do Aluno",
            SourceSystem::Lyceum => "Sistema do Aluno (Lyceum)",
            SourceSystem::Canvas => "Plataforma Virtual (Canvas)",
            SourceSystem::ExamSystem => "Sistema de Provas",
            SourceSystem::AcademicCalendar => "Calendário Acadêmico",
            SourceSystem::Complementary => "Sistema complementar",
            SourceSystem::LocalMaterial => "Material local",
        }
    }

    /// Fontes que podem confirmar situação acadêmica (nota, frequência, data de prova).
    /// Material didático e Canvas livre (texto de aula) nunca confirmam.
    pub fn is_official_for_status(self) -> bool {
        matches!(
            self,
            SourceSystem::Lyceum | SourceSystem::ExamSystem | SourceSystem::AcademicCalendar
        )
    }
}

/// Proveniência: de onde e quando. `url` já deve estar sem query/fragmento.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SourceRef {
    pub system: SourceSystem,
    pub url: Option<String>,
    /// ISO-8601 informado por quem leu; este crate não consulta relógio para dados.
    pub read_at: String,
}

impl SourceRef {
    pub fn new(system: SourceSystem, url: Option<&str>, read_at: &str) -> Self {
        Self {
            system,
            url: url.map(strip_query_and_fragment),
            read_at: read_at.to_owned(),
        }
    }
}

/// Remove `?query` e `#fragmento` (podem carregar token/SSO). Idempotente.
pub fn strip_query_and_fragment(url: &str) -> String {
    let end = url.find(['?', '#']).unwrap_or(url.len());
    url[..end].to_owned()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn source_ref_never_keeps_query_or_fragment() {
        let s = SourceRef::new(
            SourceSystem::Canvas,
            Some("https://uva.instructure.com/courses/1?token=abc#x"),
            "2026-11-06T10:00:00Z",
        );
        assert_eq!(
            s.url.as_deref(),
            Some("https://uva.instructure.com/courses/1")
        );
    }

    #[test]
    fn only_official_systems_confirm_status() {
        assert!(SourceSystem::Lyceum.is_official_for_status());
        assert!(!SourceSystem::Canvas.is_official_for_status());
        assert!(!SourceSystem::LocalMaterial.is_official_for_status());
    }
}
