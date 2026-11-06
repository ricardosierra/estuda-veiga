//! Fronteira entre camadas e política de publicação. Nega por padrão o que é desconhecido.
//!
//! Decisão do dono do acervo (6/11/2026): todo conteúdo pode ir ao acervo público, inclusive
//! notas, matrículas e trabalhos, **desde que separado** em camada própria. Continuam fora,
//! sempre: credenciais, cookies, tokens, perfis de navegador, bancos SQLite e binários originais.
//!
//! A classificação é por caminho relativo à raiz do repositório e espelha
//! `docs/ARQUITETURA_ACERVO.md` §2 e `tools/acervo/src/separar-privado.mjs`.
//! Ela NÃO inspeciona conteúdo: segredo dentro de arquivo de texto é papel de
//! `tools/acervo/src/validar.mjs`.

use std::collections::HashSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Layer {
    /// Material que nunca sai da máquina: bancos, perfis de navegador, `.env`, binários originais.
    /// Pode conter cookies e tokens. Nunca publicável, por nenhuma política.
    LocalOnly,
    /// Dado de uma pessoa (notas, matrícula, trabalhos, capturas). Separado do catálogo.
    /// Publicável somente se a política permitir (`allow_personal`).
    Personal,
    /// Saída de gerador ou espelho; reconstruível, não é fonte. Nunca publicável.
    Generated,
    /// Código, documentação e catálogo sem dado de pessoa.
    Shareable,
    /// Depende do conteúdo (ex.: `dados/disciplinas/` mistura catálogo e disciplinas pessoais).
    /// Só publicável se estiver na lista de revisados.
    NeedsReview,
}

/// O que o dono do acervo autorizou publicar.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PublicationPolicy {
    pub allow_personal: bool,
}

impl Default for PublicationPolicy {
    /// Padrão conservador da biblioteca: pessoal não sai.
    fn default() -> Self {
        Self { allow_personal: false }
    }
}

impl PublicationPolicy {
    /// Política adotada para este acervo em 6/11/2026: dado pessoal pode ser público, separado.
    pub const OWNER: Self = Self { allow_personal: true };
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PathError {
    Empty,
    Absolute,
    Traversal,
    Backslash,
    Nul,
}

/// Aceita só caminho relativo, com `/`, sem `..`, sem NUL. Não toca no disco.
pub fn validate_relative_path(path: &str) -> Result<(), PathError> {
    if path.is_empty() {
        return Err(PathError::Empty);
    }
    if path.contains('\0') {
        return Err(PathError::Nul);
    }
    if path.contains('\\') {
        return Err(PathError::Backslash);
    }
    if path.starts_with('/') || path.as_bytes().get(1) == Some(&b':') {
        return Err(PathError::Absolute);
    }
    if path.split('/').any(|seg| seg == "..") {
        return Err(PathError::Traversal);
    }
    Ok(())
}

const LOCAL_ONLY_PREFIXES: &[&str] = &[".estuda-uva", "originais/", "app/src-tauri/chromium-profile/", "chromium-profile/"];
const LOCAL_ONLY_SUFFIXES: &[&str] = &[".sqlite3", ".sqlite3-wal", ".sqlite3-shm", ".sqlite3-journal", ".sqlite", ".db"];
const PERSONAL_PREFIXES: &[&str] = &[
    "privado/",
    "cursos/",
    "dados/matriculas/",
    "dados/notas/",
    "dados/trajetorias/",
    "dados/trabalhos/",
    "materias/catalogo/capturas/",
];
const PERSONAL_FILES: &[&str] = &["REGISTRO_ACADEMICO.md", "PLANEJAMENTO.md", "IDEIAS.md", "dados/meta.json"];
const GENERATED_PREFIXES: &[&str] = &["build/", "materias/catalogo/", "app/dist/", "app/node_modules/", "app/src-tauri/target/", "app/target/"];
const SHAREABLE_PREFIXES: &[&str] = &[
    "docs/arquitetura/",
    "crates/",
    "tools/",
    "schemas/",
    "biblioteca/",
    "fontes-uva/",
    "dados/instituicao/",
    "dados/programas/",
    "dados/ofertas/",
    "dados/relacoes/",
    "dados/periodos/",
];

/// Classifica um caminho relativo. Ordem importa: `LocalOnly` vence tudo, depois `Personal`.
pub fn classify(path: &str) -> Result<Layer, PathError> {
    validate_relative_path(path)?;
    let file_name = path.rsplit('/').next().unwrap_or(path);
    if LOCAL_ONLY_PREFIXES.iter().any(|p| path.starts_with(p))
        || LOCAL_ONLY_SUFFIXES.iter().any(|s| path.ends_with(s))
        || file_name == ".env"
        || file_name.starts_with(".env.")
    {
        return Ok(Layer::LocalOnly);
    }
    if PERSONAL_FILES.contains(&path) || PERSONAL_PREFIXES.iter().any(|p| path.starts_with(p)) {
        return Ok(Layer::Personal);
    }
    if GENERATED_PREFIXES.iter().any(|p| path.starts_with(p)) {
        return Ok(Layer::Generated);
    }
    if SHAREABLE_PREFIXES.iter().any(|p| path.starts_with(p)) {
        return Ok(Layer::Shareable);
    }
    Ok(Layer::NeedsReview)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Violation {
    pub path: String,
    pub reason: &'static str,
}

/// Verifica uma lista candidata a publicação. `reviewed` são caminhos que uma pessoa aprovou
/// explicitamente; a aprovação só vale para `NeedsReview`, nunca para `LocalOnly` ou `Generated`.
pub fn check_publishable(paths: &[&str], reviewed: &HashSet<String>, policy: PublicationPolicy) -> Vec<Violation> {
    let mut out = Vec::new();
    for &path in paths {
        let reason = match classify(path) {
            Err(_) => Some("caminho inválido (absoluto, com .. ou separador inesperado)"),
            Ok(Layer::LocalOnly) => Some("somente local: banco, perfil, segredo ou binário original"),
            Ok(Layer::Generated) => Some("arquivo gerado: reconstruível, não é fonte"),
            Ok(Layer::Personal) if !policy.allow_personal => Some("dado pessoal e a política não o autoriza"),
            Ok(Layer::NeedsReview) if !reviewed.contains(path) => Some("depende de revisão humana e não foi revisado"),
            Ok(_) => None,
        };
        if let Some(reason) = reason {
            out.push(Violation { path: path.to_owned(), reason });
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn none() -> HashSet<String> {
        HashSet::new()
    }

    #[test]
    fn rejects_unsafe_paths() {
        assert_eq!(validate_relative_path(""), Err(PathError::Empty));
        assert_eq!(validate_relative_path("/etc/passwd"), Err(PathError::Absolute));
        assert_eq!(validate_relative_path("C:/x"), Err(PathError::Absolute));
        assert_eq!(validate_relative_path("dados/../../x"), Err(PathError::Traversal));
        assert_eq!(validate_relative_path("a\\b"), Err(PathError::Backslash));
        assert_eq!(validate_relative_path("a\0b"), Err(PathError::Nul));
        assert!(validate_relative_path("docs/a..b/c.md").is_ok());
    }

    #[test]
    fn personal_records_are_a_separate_layer() {
        for p in [
            "dados/notas/2026.2/x.json",
            "dados/matriculas/2026.1/x.json",
            "dados/trajetorias/ciencia-da-computacao.json",
            "dados/trabalhos/t.json",
            "REGISTRO_ACADEMICO.md",
            "cursos/engenharia-de-computacao/README.md",
            "materias/catalogo/capturas/boletim.json",
            "privado/dados/notas/x.json",
        ] {
            assert_eq!(classify(p), Ok(Layer::Personal), "{p}");
        }
    }

    #[test]
    fn databases_profiles_secrets_and_originals_are_local_only() {
        for p in [
            ".estuda-uva.sqlite3",
            ".estuda-uva/mirrors/a.json",
            "originais/calculo-1/prova.pdf",
            "qualquer/lugar/perfil.sqlite3-wal",
            "app/src-tauri/chromium-profile/Default/Cookies",
            ".env",
            "app/.env.local",
        ] {
            assert_eq!(classify(p), Ok(Layer::LocalOnly), "{p}");
        }
    }

    #[test]
    fn capturas_are_personal_even_inside_generated_catalog() {
        assert_eq!(classify("materias/catalogo/disciplinas.json"), Ok(Layer::Generated));
        assert_eq!(classify("materias/catalogo/capturas/x.md"), Ok(Layer::Personal));
    }

    #[test]
    fn unknown_and_mixed_paths_need_review() {
        assert_eq!(classify("dados/disciplinas/calculo-1.json"), Ok(Layer::NeedsReview));
        assert_eq!(classify("novo-diretorio/x.md"), Ok(Layer::NeedsReview));
    }

    #[test]
    fn owner_policy_publishes_personal_but_default_policy_does_not() {
        let p = ["dados/notas/2026.2/x.json", "REGISTRO_ACADEMICO.md"];
        assert!(check_publishable(&p, &none(), PublicationPolicy::OWNER).is_empty());
        assert_eq!(check_publishable(&p, &none(), PublicationPolicy::default()).len(), 2);
    }

    #[test]
    fn no_policy_or_review_can_publish_local_only_or_generated() {
        let mut reviewed = none();
        for p in ["build/acervo.sqlite3", ".estuda-uva.sqlite3", "originais/a.pdf", ".env"] {
            reviewed.insert(p.into());
        }
        let v = check_publishable(
            &["build/acervo.sqlite3", ".estuda-uva.sqlite3", "originais/a.pdf", ".env", "docs/arquitetura/README.md"],
            &reviewed,
            PublicationPolicy::OWNER,
        );
        assert_eq!(v.len(), 4);
    }

    #[test]
    fn review_unlocks_needs_review_only() {
        let mut reviewed = none();
        reviewed.insert("dados/disciplinas/calculo-1.json".into());
        let v = check_publishable(&["dados/disciplinas/calculo-1.json", "dados/disciplinas/outra.json"], &reviewed, PublicationPolicy::OWNER);
        assert_eq!(v.iter().map(|x| x.path.as_str()).collect::<Vec<_>>(), ["dados/disciplinas/outra.json"]);
    }

    #[test]
    fn default_is_deny_for_invalid_paths() {
        assert_eq!(check_publishable(&["../fora.md"], &none(), PublicationPolicy::OWNER).len(), 1);
        assert!(check_publishable(&["fontes-uva/2026.2/bases-de-programacao/capitulo-01.md"], &none(), PublicationPolicy::OWNER).is_empty());
    }
}
