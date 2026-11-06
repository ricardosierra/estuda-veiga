//! Cruzamento Portal × Canvas. A localização de um curso (Favoritos, inscrições anteriores)
//! e a semelhança de título **não** provam matrícula nem identificam o material: só uma
//! associação explícita confirma. O resto vira achado para revisão humana.

use std::collections::{BTreeMap, BTreeSet};

use estuda_domain::ids::{AcademicTermId, LmsCourseId, SubjectId};

use crate::text::fold;

#[derive(Debug, Clone)]
pub struct PortalEnrollment {
    pub subject: SubjectId,
    pub subject_name: String,
    pub term: AcademicTermId,
}

#[derive(Debug, Clone)]
pub struct CanvasCourse {
    pub id: LmsCourseId,
    pub title: String,
}

#[derive(Debug, Clone)]
pub struct ExplicitAssociation {
    pub course: LmsCourseId,
    pub subject: SubjectId,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CourseRole {
    /// Associação explícita com uma disciplina.
    Subject(SubjectId),
    /// Ambiente de apoio (ex.: Espaço Curso): nunca é disciplina.
    SupportEnvironment,
    /// Sem associação explícita.
    Unlinked,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Finding {
    /// Título coincide com matrícula, mas sem associação explícita: revisar.
    TitleMatchNeedsReview {
        course: LmsCourseId,
        subject: SubjectId,
    },
    /// Curso do Canvas sem matrícula nem associação (ex.: inscrição anterior, favorito).
    CanvasCourseWithoutEnrollment { course: LmsCourseId },
    /// Matrícula do Portal sem nenhum curso do Canvas associado.
    EnrollmentWithoutCanvasCourse { subject: SubjectId },
    /// Associação explícita aponta para disciplina sem matrícula no período.
    AssociationWithoutEnrollment {
        course: LmsCourseId,
        subject: SubjectId,
    },
    /// Associação explícita e título apontam para disciplinas diferentes.
    TitleDisagreesWithAssociation {
        course: LmsCourseId,
        associated: SubjectId,
        title_suggests: SubjectId,
    },
    /// Mais de uma disciplina associada ao mesmo curso.
    ConflictingAssociations {
        course: LmsCourseId,
        subjects: Vec<SubjectId>,
    },
}

#[derive(Debug, Default)]
pub struct CrossCheck {
    pub roles: BTreeMap<LmsCourseId, CourseRole>,
    pub findings: Vec<Finding>,
}

const SUPPORT_MARKERS: &[&str] = &["espaco curso", "ambiente de apoio"];

pub fn is_support_environment(title: &str) -> bool {
    let folded = fold(title);
    SUPPORT_MARKERS.iter().any(|m| folded.contains(m))
}

pub fn cross_check(
    enrollments: &[PortalEnrollment],
    courses: &[CanvasCourse],
    associations: &[ExplicitAssociation],
) -> CrossCheck {
    let mut out = CrossCheck::default();
    let enrolled: BTreeSet<&SubjectId> = enrollments.iter().map(|e| &e.subject).collect();
    let mut covered: BTreeSet<SubjectId> = BTreeSet::new();

    for course in courses {
        let linked: Vec<&SubjectId> = {
            let mut v: Vec<&SubjectId> = associations
                .iter()
                .filter(|a| a.course == course.id)
                .map(|a| &a.subject)
                .collect();
            v.sort();
            v.dedup();
            v
        };
        let title_candidates: Vec<&PortalEnrollment> = enrollments
            .iter()
            .filter(|e| fold(&e.subject_name) == fold(&course.title))
            .collect();

        if is_support_environment(&course.title) {
            out.roles
                .insert(course.id.clone(), CourseRole::SupportEnvironment);
            continue;
        }
        match linked.as_slice() {
            [] => {
                out.roles.insert(course.id.clone(), CourseRole::Unlinked);
                if let Some(candidate) = title_candidates.first() {
                    out.findings.push(Finding::TitleMatchNeedsReview {
                        course: course.id.clone(),
                        subject: candidate.subject.clone(),
                    });
                } else {
                    out.findings.push(Finding::CanvasCourseWithoutEnrollment {
                        course: course.id.clone(),
                    });
                }
            }
            [one] => {
                out.roles
                    .insert(course.id.clone(), CourseRole::Subject((*one).clone()));
                covered.insert((*one).clone());
                if !enrolled.contains(one) {
                    out.findings.push(Finding::AssociationWithoutEnrollment {
                        course: course.id.clone(),
                        subject: (*one).clone(),
                    });
                }
                if let Some(c) = title_candidates.iter().find(|c| c.subject != **one) {
                    out.findings.push(Finding::TitleDisagreesWithAssociation {
                        course: course.id.clone(),
                        associated: (*one).clone(),
                        title_suggests: c.subject.clone(),
                    });
                }
            }
            many => {
                out.roles.insert(course.id.clone(), CourseRole::Unlinked);
                out.findings.push(Finding::ConflictingAssociations {
                    course: course.id.clone(),
                    subjects: many.iter().map(|s| (*s).clone()).collect(),
                });
            }
        }
    }
    for subject in enrolled {
        if !covered.contains(subject) {
            out.findings.push(Finding::EnrollmentWithoutCanvasCourse {
                subject: subject.clone(),
            });
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn subj(s: &str) -> SubjectId {
        SubjectId::parse(&format!("uva:discipline:{s}")).unwrap()
    }
    fn course(id: &str) -> LmsCourseId {
        LmsCourseId::canvas(id).unwrap()
    }
    fn enr(s: &str, name: &str) -> PortalEnrollment {
        PortalEnrollment {
            subject: subj(s),
            subject_name: name.into(),
            term: AcademicTermId::parse("uva:term:2026.2").unwrap(),
        }
    }
    fn cc(id: &str, title: &str) -> CanvasCourse {
        CanvasCourse {
            id: course(id),
            title: title.into(),
        }
    }

    #[test]
    fn support_course_is_not_a_subject() {
        let r = cross_check(&[], &[cc("1", "Espaço Curso – Ciência da Computação")], &[]);
        assert_eq!(r.roles[&course("1")], CourseRole::SupportEnvironment);
        assert!(r.findings.is_empty());
    }

    #[test]
    fn title_match_alone_never_confirms() {
        let r = cross_check(
            &[enr("algoritmos", "Algoritmos")],
            &[cc("2", "Algoritmos")],
            &[],
        );
        assert_eq!(r.roles[&course("2")], CourseRole::Unlinked);
        assert!(r.findings.contains(&Finding::TitleMatchNeedsReview {
            course: course("2"),
            subject: subj("algoritmos")
        }));
        assert!(
            r.findings
                .contains(&Finding::EnrollmentWithoutCanvasCourse {
                    subject: subj("algoritmos")
                })
        );
    }

    #[test]
    fn explicit_association_confirms_and_flags_divergent_title() {
        let r = cross_check(
            &[
                enr("algoritmos", "Algoritmos"),
                enr("calculo-1", "Cálculo I"),
            ],
            &[cc("3", "Cálculo I")],
            &[ExplicitAssociation {
                course: course("3"),
                subject: subj("algoritmos"),
            }],
        );
        assert_eq!(
            r.roles[&course("3")],
            CourseRole::Subject(subj("algoritmos"))
        );
        assert!(
            r.findings
                .contains(&Finding::TitleDisagreesWithAssociation {
                    course: course("3"),
                    associated: subj("algoritmos"),
                    title_suggests: subj("calculo-1"),
                })
        );
        assert!(
            r.findings
                .contains(&Finding::EnrollmentWithoutCanvasCourse {
                    subject: subj("calculo-1")
                })
        );
    }

    #[test]
    fn canvas_course_without_enrollment_is_reported_not_trusted() {
        let r = cross_check(&[], &[cc("4", "Disciplina Antiga")], &[]);
        assert_eq!(
            r.findings,
            vec![Finding::CanvasCourseWithoutEnrollment {
                course: course("4")
            }]
        );
    }

    #[test]
    fn association_to_unenrolled_subject_and_double_association_are_flagged() {
        let r = cross_check(
            &[],
            &[cc("5", "X"), cc("6", "Y")],
            &[
                ExplicitAssociation {
                    course: course("5"),
                    subject: subj("a"),
                },
                ExplicitAssociation {
                    course: course("6"),
                    subject: subj("b"),
                },
                ExplicitAssociation {
                    course: course("6"),
                    subject: subj("c"),
                },
            ],
        );
        assert!(r.findings.contains(&Finding::AssociationWithoutEnrollment {
            course: course("5"),
            subject: subj("a")
        }));
        assert!(r.findings.iter().any(
            |f| matches!(f, Finding::ConflictingAssociations { course: c, .. } if *c == course("6"))
        ));
    }
}
