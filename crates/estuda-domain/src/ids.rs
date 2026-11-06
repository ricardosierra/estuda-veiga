//! Identificadores estáveis. Não há chave estrangeira entre o banco privado e o público:
//! a ligação é sempre por estes identificadores, resolvidos pela camada de associação privada.

use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IdError {
    Empty,
    TooLong,
    InvalidChar(char),
    /// Esperado `<namespace>:<tipo>:<slug>` com o tipo exigido pelo identificador.
    WrongShape { expected_kind: &'static str },
    NotNumeric,
}

impl fmt::Display for IdError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            IdError::Empty => write!(f, "identificador vazio"),
            IdError::TooLong => write!(f, "identificador acima de {MAX_LEN} caracteres"),
            IdError::InvalidChar(c) => write!(f, "caractere inválido {c:?}"),
            IdError::WrongShape { expected_kind } => {
                write!(f, "esperado <namespace>:{expected_kind}:<slug>")
            }
            IdError::NotNumeric => write!(f, "identificador do LMS deve ser numérico"),
        }
    }
}

impl std::error::Error for IdError {}

const MAX_LEN: usize = 200;

fn check_charset(raw: &str) -> Result<(), IdError> {
    if raw.is_empty() {
        return Err(IdError::Empty);
    }
    if raw.len() > MAX_LEN {
        return Err(IdError::TooLong);
    }
    match raw
        .chars()
        .find(|c| !(c.is_ascii_lowercase() || c.is_ascii_digit() || matches!(c, ':' | '-' | '_' | '.')))
    {
        Some(c) => Err(IdError::InvalidChar(c)),
        None => Ok(()),
    }
}

fn check_kinded(raw: &str, kind: &'static str) -> Result<(), IdError> {
    check_charset(raw)?;
    let mut parts = raw.splitn(3, ':');
    match (parts.next(), parts.next(), parts.next()) {
        (Some(ns), Some(k), Some(slug)) if !ns.is_empty() && k == kind && !slug.is_empty() => Ok(()),
        _ => Err(IdError::WrongShape { expected_kind: kind }),
    }
}

macro_rules! kinded_id {
    ($(#[$meta:meta])* $name:ident, $kind:literal) => {
        $(#[$meta])*
        #[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
        pub struct $name(String);

        impl $name {
            pub fn parse(raw: &str) -> Result<Self, IdError> {
                check_kinded(raw, $kind)?;
                Ok(Self(raw.to_owned()))
            }
            pub fn as_str(&self) -> &str {
                &self.0
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.write_str(&self.0)
            }
        }
    };
}

kinded_id!(
    /// Curso de graduação do catálogo (`uva:program:<slug>`).
    DegreeProgramId, "program"
);
kinded_id!(
    /// Disciplina do catálogo (`uva:discipline:<slug>`). Nunca derivada só do nome.
    SubjectId, "discipline"
);
kinded_id!(
    /// Período letivo (`uva:term:2026.2`).
    AcademicTermId, "term"
);

/// Curso/ambiente do LMS (Canvas). Tipo próprio para que um curso de apoio jamais seja
/// aceito como disciplina: só vira `Subject` por associação explícita no domínio privado.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct LmsCourseId {
    provider: &'static str,
    id: String,
}

impl LmsCourseId {
    pub fn canvas(raw: &str) -> Result<Self, IdError> {
        if raw.is_empty() {
            return Err(IdError::Empty);
        }
        if raw.len() > 20 || !raw.bytes().all(|b| b.is_ascii_digit()) {
            return Err(IdError::NotNumeric);
        }
        Ok(Self { provider: "canvas", id: raw.to_owned() })
    }
    pub fn provider(&self) -> &'static str {
        self.provider
    }
    pub fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_existing_acervo_ids() {
        assert!(SubjectId::parse("uva:discipline:bases-de-programacao").is_ok());
        assert!(DegreeProgramId::parse("uva:program:ciencia-da-computacao").is_ok());
        assert!(AcademicTermId::parse("uva:term:2026.2").is_ok());
    }

    #[test]
    fn kind_is_enforced() {
        assert_eq!(
            SubjectId::parse("uva:program:ciencia-da-computacao"),
            Err(IdError::WrongShape { expected_kind: "discipline" })
        );
        assert!(DegreeProgramId::parse("uva:discipline:x").is_err());
    }

    #[test]
    fn rejects_empty_slug_uppercase_spaces_and_traversal() {
        assert!(SubjectId::parse("uva:discipline:").is_err());
        assert!(SubjectId::parse("uva:discipline:Calculo").is_err());
        assert!(SubjectId::parse("uva:discipline:a b").is_err());
        assert!(SubjectId::parse("uva:discipline:../x").is_err());
        assert_eq!(SubjectId::parse(""), Err(IdError::Empty));
        assert_eq!(SubjectId::parse(&format!("u:discipline:{}", "a".repeat(300))), Err(IdError::TooLong));
    }

    #[test]
    fn lms_course_id_is_numeric_and_distinct_from_subject() {
        let c = LmsCourseId::canvas("53418").unwrap();
        assert_eq!((c.provider(), c.id()), ("canvas", "53418"));
        assert_eq!(LmsCourseId::canvas("53x"), Err(IdError::NotNumeric));
        assert_eq!(LmsCourseId::canvas(""), Err(IdError::Empty));
    }
}
