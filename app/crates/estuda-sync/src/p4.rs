//! Busca do identificador "P4". **O significado de P4 é desconhecido** e este módulo não o
//! define: só procura o token exato e diz em que contexto ele apareceu.
//!
//! Uma ocorrência isolada em material didático não comprova tipo de avaliação.

use crate::text::{fold, is_word_char};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextSourceKind {
    /// Critérios/regulamento de avaliação, calendário de provas, Sistema de Provas.
    AuthoritativeEvaluation,
    /// E-book, aula, página de conteúdo, fórum, documentação do próprio app.
    Other,
}

#[derive(Debug, Clone)]
pub struct TextSource {
    pub kind: TextSourceKind,
    pub label: String,
    pub text: String,
    /// `false` quando a leitura foi truncada ou falhou parcialmente.
    pub complete: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Hit {
    pub source_label: String,
    pub line_number: usize,
    pub line: String,
    pub in_evaluation_context: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IdentifiedEvaluation {
    pub name: String,
    pub origin: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Exclusivity {
    Yes,
    No,
    Unknown,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum P4Result {
    /// Todas as fontes foram lidas por completo e o token não aparece.
    NotFound,
    /// Sem fontes ou alguma incompleta, e sem ocorrência: não dá para concluir.
    Unknown { reason: &'static str },
    /// Ocorreu, mas só fora de contexto avaliativo autorizado.
    IsolatedMentions { hits: Vec<Hit> },
    /// Ocorreu em fonte avaliativa, numa linha que fala de avaliação.
    EvaluationEvidence {
        hits: Vec<Hit>,
        exclusivity: Exclusivity,
    },
}

/// Palavras inteiras que indicam que a linha fala de avaliação.
const EVALUATION_EXACT: &[&str] = &[
    "prova", "provas", "nota", "notas", "peso", "ava", "a1", "a2", "a3", "a4",
];
/// Prefixos (avaliação, avaliativa, critérios, substitutiva…).
const EVALUATION_PREFIXES: &[&str] = &["avaliac", "avaliativ", "criterio", "substitutiv"];

/// Ocorrências de `token` com fronteira de palavra, diferenciando maiúsculas.
fn find_token(text: &str, token: &str) -> Vec<(usize, String)> {
    let mut out = Vec::new();
    for (idx, line) in text.lines().enumerate() {
        let mut start = 0;
        while let Some(pos) = line[start..].find(token) {
            let at = start + pos;
            let before = line[..at].chars().next_back();
            let after = line[at + token.len()..].chars().next();
            if !before.is_some_and(is_word_char) && !after.is_some_and(is_word_char) {
                out.push((idx + 1, line.trim().to_owned()));
                break;
            }
            start = at + token.len();
        }
    }
    out
}

fn has_evaluation_words(line: &str) -> bool {
    let folded = fold(line);
    folded
        .split(|c: char| !c.is_alphanumeric())
        .filter(|w| !w.is_empty())
        .any(|w| {
            EVALUATION_EXACT.contains(&w) || EVALUATION_PREFIXES.iter().any(|p| w.starts_with(p))
        })
}

/// `evaluations` é a lista efetivamente identificada nas fontes oficiais da disciplina;
/// `evaluations_complete` diz se essa lista cobre a disciplina inteira.
pub fn search(
    token: &str,
    sources: &[TextSource],
    evaluations: &[IdentifiedEvaluation],
    evaluations_complete: bool,
) -> P4Result {
    let mut hits = Vec::new();
    for source in sources {
        for (line_number, line) in find_token(&source.text, token) {
            let in_context = source.kind == TextSourceKind::AuthoritativeEvaluation
                && has_evaluation_words(&line);
            hits.push(Hit {
                source_label: source.label.clone(),
                line_number,
                line,
                in_evaluation_context: in_context,
            });
        }
    }
    if hits.is_empty() {
        return if !sources.is_empty() && sources.iter().all(|s| s.complete) {
            P4Result::NotFound
        } else {
            P4Result::Unknown {
                reason: "fontes ausentes ou lidas parcialmente",
            }
        };
    }
    if !hits.iter().any(|h| h.in_evaluation_context) {
        return P4Result::IsolatedMentions { hits };
    }
    let exclusivity = if !evaluations_complete || evaluations.is_empty() {
        Exclusivity::Unknown
    } else if evaluations.iter().all(|e| e.name == token) {
        Exclusivity::Yes
    } else {
        Exclusivity::No
    };
    P4Result::EvaluationEvidence { hits, exclusivity }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn src(kind: TextSourceKind, text: &str, complete: bool) -> TextSource {
        TextSource {
            kind,
            label: "fonte".into(),
            text: text.into(),
            complete,
        }
    }
    fn eval(name: &str) -> IdentifiedEvaluation {
        IdentifiedEvaluation {
            name: name.into(),
            origin: "Critérios".into(),
        }
    }

    #[test]
    fn complete_sources_without_token_say_not_found() {
        let s = [src(
            TextSourceKind::AuthoritativeEvaluation,
            "A1 peso 4\nA2 peso 6",
            true,
        )];
        assert_eq!(search("P4", &s, &[], true), P4Result::NotFound);
    }

    #[test]
    fn incomplete_or_missing_sources_stay_unknown() {
        let s = [src(
            TextSourceKind::AuthoritativeEvaluation,
            "A1 peso 4",
            false,
        )];
        assert!(matches!(
            search("P4", &s, &[], false),
            P4Result::Unknown { .. }
        ));
        assert!(matches!(
            search("P4", &[], &[], false),
            P4Result::Unknown { .. }
        ));
    }

    #[test]
    fn isolated_mention_in_didactic_material_proves_nothing() {
        let s = [src(
            TextSourceKind::Other,
            "O pipeline P4 faz o resumo.\nPortanto a prova P4 ...",
            true,
        )];
        match search("P4", &s, &[], true) {
            P4Result::IsolatedMentions { hits } => assert_eq!(hits.len(), 2),
            other => panic!("{other:?}"),
        }
    }

    #[test]
    fn mention_in_authoritative_source_without_evaluation_words_is_isolated() {
        let s = [src(
            TextSourceKind::AuthoritativeEvaluation,
            "Sala P4 do prédio central",
            true,
        )];
        assert!(matches!(
            search("P4", &s, &[], true),
            P4Result::IsolatedMentions { .. }
        ));
    }

    #[test]
    fn token_requires_word_boundaries_and_exact_case() {
        let s = [src(
            TextSourceKind::AuthoritativeEvaluation,
            "prova P40\nprova UP4\nprova p4\nnota XP4X",
            true,
        )];
        assert_eq!(search("P4", &s, &[], true), P4Result::NotFound);
        let s = [src(
            TextSourceKind::AuthoritativeEvaluation,
            "A prova (P4), peso 2.",
            true,
        )];
        assert!(matches!(
            search("P4", &s, &[eval("P4")], true),
            P4Result::EvaluationEvidence { .. }
        ));
    }

    #[test]
    fn exclusivity_needs_a_complete_evaluation_list() {
        let s = [src(
            TextSourceKind::AuthoritativeEvaluation,
            "Prova P4 em 10/12",
            true,
        )];
        let ex = |evals: &[IdentifiedEvaluation], complete| match search("P4", &s, evals, complete)
        {
            P4Result::EvaluationEvidence { exclusivity, .. } => exclusivity,
            other => panic!("{other:?}"),
        };
        assert_eq!(ex(&[eval("P4")], true), Exclusivity::Yes);
        assert_eq!(ex(&[eval("P4"), eval("A1")], true), Exclusivity::No);
        assert_eq!(ex(&[eval("P4")], false), Exclusivity::Unknown);
        assert_eq!(ex(&[], true), Exclusivity::Unknown);
    }
}
