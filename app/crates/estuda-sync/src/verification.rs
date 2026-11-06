//! Estados de verificação e checagem das atividades AVA 1 e AVA 2.
//!
//! Regra central: **ausência de informação não é ausência de entrega**. `NotFound` só é
//! emitido quando a fonte foi lida por completo e nada correspondente existe; leitura
//! parcial ou não feita resulta em `Unknown`.

use serde::{Deserialize, Serialize};

use crate::source::SourceRef;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Verification {
    Confirmed,
    NotFound,
    Unknown,
    Conflicting,
    NotApplicable,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Coverage {
    /// A fonte foi lida inteira dentro dos limites.
    Complete,
    /// Truncada por limite, erro ou layout alterado.
    Partial,
    NotRead,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LinkConfidence {
    /// Associação explícita curso ↔ disciplina.
    Explicit,
    /// Só semelhança de título/posição: não basta para confirmar.
    Probable,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AvaKind {
    Ava1,
    Ava2,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SubmissionState {
    Submitted,
    NotSubmitted,
    Graded,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Field<T> {
    pub state: Verification,
    pub value: Option<T>,
}

impl<T> Field<T> {
    pub fn unknown() -> Self {
        Self {
            state: Verification::Unknown,
            value: None,
        }
    }
}

/// Reconcilia observações de um campo: iguais → confirmado; diferentes → conflitante e
/// **sem valor** (nenhuma versão vence); nenhuma → desconhecido.
pub fn reconcile_field<T: PartialEq + Clone>(values: &[T]) -> Field<T> {
    match values.split_first() {
        None => Field::unknown(),
        Some((first, rest)) if rest.iter().all(|v| v == first) => Field {
            state: Verification::Confirmed,
            value: Some(first.clone()),
        },
        Some(_) => Field {
            state: Verification::Conflicting,
            value: None,
        },
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AvaObservation {
    pub kind: AvaKind,
    pub assessment_type: Option<String>,
    pub due_at: Option<String>,
    pub submission: Option<SubmissionState>,
    /// Nota em centésimos (8,50 → 850); só entra se `grade_published`.
    pub grade_hundredths: Option<u32>,
    pub grade_published: bool,
    pub feedback: Option<String>,
    pub source: SourceRef,
    pub link: LinkConfidence,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Applicability {
    Applies,
    /// Regra explícita (ex.: disciplina especial sem AVA). Nunca inferida por ausência.
    DoesNotApply,
    Unknown,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AvaCheck {
    pub kind: AvaKind,
    pub existence: Verification,
    pub assessment_type: Field<String>,
    pub due_at: Field<String>,
    pub submission: Field<SubmissionState>,
    pub grade_hundredths: Field<u32>,
    pub feedback: Field<String>,
    pub sources: Vec<SourceRef>,
    /// Melhor nível de vínculo entre as observações; `None` sem observações.
    pub link_confidence: Option<LinkConfidence>,
}

pub fn check_ava(
    kind: AvaKind,
    applicability: Applicability,
    observations: &[AvaObservation],
    coverage: Coverage,
) -> AvaCheck {
    let mine: Vec<&AvaObservation> = observations.iter().filter(|o| o.kind == kind).collect();
    let empty = AvaCheck {
        kind,
        existence: Verification::Unknown,
        assessment_type: Field::unknown(),
        due_at: Field::unknown(),
        submission: Field::unknown(),
        grade_hundredths: Field::unknown(),
        feedback: Field::unknown(),
        sources: Vec::new(),
        link_confidence: None,
    };
    match applicability {
        Applicability::DoesNotApply => {
            return AvaCheck {
                existence: Verification::NotApplicable,
                ..empty
            };
        }
        Applicability::Unknown => return empty,
        Applicability::Applies => {}
    }
    if mine.is_empty() {
        let existence = if coverage == Coverage::Complete {
            Verification::NotFound
        } else {
            Verification::Unknown
        };
        return AvaCheck { existence, ..empty };
    }
    let link_confidence = if mine.iter().any(|o| o.link == LinkConfidence::Explicit) {
        LinkConfidence::Explicit
    } else {
        LinkConfidence::Probable
    };
    // Vínculo só provável (título parecido) não confirma existência.
    let existence = if link_confidence == LinkConfidence::Explicit {
        Verification::Confirmed
    } else {
        Verification::Unknown
    };
    let usable: Vec<&&AvaObservation> = mine
        .iter()
        .filter(|o| o.link == LinkConfidence::Explicit)
        .collect();
    let collect = |f: &dyn Fn(&AvaObservation) -> Option<String>| -> Vec<String> {
        usable.iter().filter_map(|o| f(o)).collect()
    };
    let grades: Vec<u32> = usable
        .iter()
        .filter(|o| o.grade_published)
        .filter_map(|o| o.grade_hundredths)
        .collect();
    let submissions: Vec<SubmissionState> =
        usable.iter().filter_map(|o| o.submission.clone()).collect();
    AvaCheck {
        kind,
        existence,
        assessment_type: reconcile_field(&collect(&|o| o.assessment_type.clone())),
        due_at: reconcile_field(&collect(&|o| o.due_at.clone())),
        submission: reconcile_field(&submissions),
        grade_hundredths: reconcile_field(&grades),
        feedback: reconcile_field(&collect(&|o| o.feedback.clone())),
        sources: mine.iter().map(|o| o.source.clone()).collect(),
        link_confidence: Some(link_confidence),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::source::SourceSystem;

    fn obs(kind: AvaKind, link: LinkConfidence) -> AvaObservation {
        AvaObservation {
            kind,
            assessment_type: Some("Fórum avaliativo".into()),
            due_at: Some("2026-11-20T23:59:00".into()),
            submission: None,
            grade_hundredths: None,
            grade_published: false,
            feedback: None,
            source: SourceRef::new(
                SourceSystem::Canvas,
                Some("https://uva.instructure.com/courses/1/assignments/2"),
                "2026-11-06T10:00:00Z",
            ),
            link,
        }
    }

    #[test]
    fn complete_read_without_match_is_not_found() {
        let c = check_ava(
            AvaKind::Ava2,
            Applicability::Applies,
            &[],
            Coverage::Complete,
        );
        assert_eq!(c.existence, Verification::NotFound);
    }

    #[test]
    fn partial_or_missing_read_is_unknown_never_not_found() {
        for cov in [Coverage::Partial, Coverage::NotRead] {
            let c = check_ava(AvaKind::Ava1, Applicability::Applies, &[], cov);
            assert_eq!(c.existence, Verification::Unknown);
        }
    }

    #[test]
    fn absent_submission_data_is_unknown_not_unsubmitted() {
        let c = check_ava(
            AvaKind::Ava1,
            Applicability::Applies,
            &[obs(AvaKind::Ava1, LinkConfidence::Explicit)],
            Coverage::Complete,
        );
        assert_eq!(c.existence, Verification::Confirmed);
        assert_eq!(c.submission, Field::unknown());
        assert_eq!(c.grade_hundredths, Field::unknown());
        assert_eq!(c.due_at.value.as_deref(), Some("2026-11-20T23:59:00"));
    }

    #[test]
    fn probable_link_does_not_confirm_existence() {
        let c = check_ava(
            AvaKind::Ava1,
            Applicability::Applies,
            &[obs(AvaKind::Ava1, LinkConfidence::Probable)],
            Coverage::Complete,
        );
        assert_eq!(c.existence, Verification::Unknown);
        assert_eq!(c.link_confidence, Some(LinkConfidence::Probable));
        assert_eq!(c.due_at, Field::unknown());
    }

    #[test]
    fn conflicting_deadlines_keep_no_winner() {
        let mut a = obs(AvaKind::Ava1, LinkConfidence::Explicit);
        let mut b = obs(AvaKind::Ava1, LinkConfidence::Explicit);
        a.due_at = Some("2026-11-20T23:59:00".into());
        b.due_at = Some("2026-11-27T23:59:00".into());
        let c = check_ava(
            AvaKind::Ava1,
            Applicability::Applies,
            &[a, b],
            Coverage::Complete,
        );
        assert_eq!(c.due_at.state, Verification::Conflicting);
        assert_eq!(c.due_at.value, None);
    }

    #[test]
    fn unpublished_grade_is_ignored_and_published_is_kept() {
        let mut a = obs(AvaKind::Ava1, LinkConfidence::Explicit);
        a.grade_hundredths = Some(850);
        let c = check_ava(
            AvaKind::Ava1,
            Applicability::Applies,
            &[a.clone()],
            Coverage::Complete,
        );
        assert_eq!(c.grade_hundredths, Field::unknown());
        a.grade_published = true;
        let c = check_ava(
            AvaKind::Ava1,
            Applicability::Applies,
            &[a],
            Coverage::Complete,
        );
        assert_eq!(c.grade_hundredths.value, Some(850));
        assert_eq!(c.grade_hundredths.state, Verification::Confirmed);
    }

    #[test]
    fn not_applicable_only_by_explicit_rule() {
        let c = check_ava(
            AvaKind::Ava2,
            Applicability::DoesNotApply,
            &[],
            Coverage::Complete,
        );
        assert_eq!(c.existence, Verification::NotApplicable);
        let c = check_ava(
            AvaKind::Ava2,
            Applicability::Unknown,
            &[],
            Coverage::Complete,
        );
        assert_eq!(c.existence, Verification::Unknown);
    }

    #[test]
    fn observations_of_other_kind_are_ignored() {
        let c = check_ava(
            AvaKind::Ava2,
            Applicability::Applies,
            &[obs(AvaKind::Ava1, LinkConfidence::Explicit)],
            Coverage::Complete,
        );
        assert_eq!(c.existence, Verification::NotFound);
    }
}
