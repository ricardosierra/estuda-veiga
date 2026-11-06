//! Provas: data, horário, local, agendamento e situação. Somente leitura.
//!
//! Sem confirmação oficial a data é exibida como **"a confirmar"**. Nunca se infere data por
//! semelhança com outro período letivo.

use std::collections::BTreeMap;

use estuda_domain::ids::{AcademicTermId, SubjectId};
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::source::SourceRef;
use crate::verification::{Field, Verification, reconcile_field};

pub const TO_CONFIRM: &str = "a confirmar";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Scheduling {
    Scheduled,
    NotScheduled,
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ExamStatus {
    NotYetTaken,
    Taken,
    Unknown,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExamObservation {
    pub subject: SubjectId,
    pub term: AcademicTermId,
    /// Nome exatamente como a fonte publica (ex.: "A2"); não é normalizado para P4/A4/etc.
    pub name: String,
    pub date: Option<String>,
    pub time: Option<String>,
    pub location: Option<String>,
    pub scheduling: Scheduling,
    pub status: ExamStatus,
    pub source: SourceRef,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExamRecord {
    pub subject: SubjectId,
    pub term: AcademicTermId,
    pub name: String,
    pub date: Field<String>,
    pub time: Field<String>,
    pub location: Field<String>,
    pub scheduling: Field<Scheduling>,
    pub status: Field<ExamStatus>,
    pub sources: Vec<SourceRef>,
}

impl ExamRecord {
    pub fn date_display(&self) -> String {
        match (&self.date.state, &self.date.value) {
            (Verification::Confirmed, Some(d)) => d.clone(),
            (Verification::Conflicting, _) => "conflitante: revisar fontes".to_owned(),
            _ => TO_CONFIRM.to_owned(),
        }
    }
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum ExamError {
    #[error("observação de outro período ({found}) misturada com {expected}")]
    WrongTerm { expected: String, found: String },
    #[error("data inválida: {0}")]
    InvalidDate(String),
    #[error("horário inválido: {0}")]
    InvalidTime(String),
}

pub fn parse_iso_date(s: &str) -> bool {
    let parts: Vec<&str> = s.split('-').collect();
    let [y, m, d] = parts.as_slice() else {
        return false;
    };
    let (Ok(y), Ok(m), Ok(d)) = (y.parse::<u32>(), m.parse::<u32>(), d.parse::<u32>()) else {
        return false;
    };
    if y.to_string().len() != 4
        || m.to_string().len() > 2
        || y < 2000
        || !(1..=12).contains(&m)
        || d == 0
    {
        return false;
    }
    let leap = (y % 4 == 0 && y % 100 != 0) || y % 400 == 0;
    let days = match m {
        2 if leap => 29,
        2 => 28,
        4 | 6 | 9 | 11 => 30,
        _ => 31,
    };
    d <= days && s.len() == 10
}

pub fn parse_hh_mm(s: &str) -> bool {
    let Some((h, m)) = s.split_once(':') else {
        return false;
    };
    s.len() == 5 && h.parse::<u32>().is_ok_and(|h| h < 24) && m.parse::<u32>().is_ok_and(|m| m < 60)
}

/// Constrói um registro por (disciplina, nome) para `term`. Só observações **oficiais**
/// (Lyceum, Sistema de Provas, Calendário) definem data/horário/local; as demais são
/// descartadas desses campos.
pub fn build_records(
    term: &AcademicTermId,
    observations: &[ExamObservation],
) -> Result<Vec<ExamRecord>, ExamError> {
    let mut groups: BTreeMap<(SubjectId, String), Vec<&ExamObservation>> = BTreeMap::new();
    for o in observations {
        if &o.term != term {
            return Err(ExamError::WrongTerm {
                expected: term.to_string(),
                found: o.term.to_string(),
            });
        }
        if let Some(d) = &o.date
            && !parse_iso_date(d)
        {
            return Err(ExamError::InvalidDate(d.clone()));
        }
        if let Some(t) = &o.time
            && !parse_hh_mm(t)
        {
            return Err(ExamError::InvalidTime(t.clone()));
        }
        groups
            .entry((o.subject.clone(), o.name.clone()))
            .or_default()
            .push(o);
    }
    Ok(groups
        .into_iter()
        .map(|((subject, name), group)| {
            let official: Vec<&&ExamObservation> = group
                .iter()
                .filter(|o| o.source.system.is_official_for_status())
                .collect();
            let pick = |f: &dyn Fn(&ExamObservation) -> Option<String>| -> Vec<String> {
                official.iter().filter_map(|o| f(o)).collect()
            };
            ExamRecord {
                subject,
                term: term.clone(),
                name,
                date: reconcile_field(&pick(&|o| o.date.clone())),
                time: reconcile_field(&pick(&|o| o.time.clone())),
                location: reconcile_field(&pick(&|o| o.location.clone())),
                scheduling: reconcile_field(
                    &official
                        .iter()
                        .map(|o| o.scheduling)
                        .filter(|s| *s != Scheduling::Unknown)
                        .collect::<Vec<_>>(),
                ),
                status: reconcile_field(
                    &official
                        .iter()
                        .map(|o| o.status)
                        .filter(|s| *s != ExamStatus::Unknown)
                        .collect::<Vec<_>>(),
                ),
                sources: group.iter().map(|o| o.source.clone()).collect(),
            }
        })
        .collect())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::source::SourceSystem;

    fn term(t: &str) -> AcademicTermId {
        AcademicTermId::parse(&format!("uva:term:{t}")).unwrap()
    }
    fn obs(system: SourceSystem, date: Option<&str>, t: &str) -> ExamObservation {
        ExamObservation {
            subject: SubjectId::parse("uva:discipline:algoritmos").unwrap(),
            term: term(t),
            name: "A2".into(),
            date: date.map(Into::into),
            time: None,
            location: None,
            scheduling: Scheduling::Unknown,
            status: ExamStatus::Unknown,
            source: SourceRef::new(system, None, "2026-11-06T10:00:00Z"),
        }
    }

    #[test]
    fn without_official_confirmation_date_is_to_confirm() {
        let r = build_records(
            &term("2026.2"),
            &[obs(SourceSystem::Canvas, Some("2026-12-10"), "2026.2")],
        )
        .unwrap();
        assert_eq!(r[0].date_display(), TO_CONFIRM);
        let r = build_records(
            &term("2026.2"),
            &[obs(SourceSystem::ExamSystem, None, "2026.2")],
        )
        .unwrap();
        assert_eq!(r[0].date_display(), TO_CONFIRM);
    }

    #[test]
    fn official_date_is_confirmed() {
        let r = build_records(
            &term("2026.2"),
            &[obs(SourceSystem::ExamSystem, Some("2026-12-10"), "2026.2")],
        )
        .unwrap();
        assert_eq!(r[0].date_display(), "2026-12-10");
    }

    #[test]
    fn contradictory_official_dates_are_conflicting_with_no_winner() {
        let r = build_records(
            &term("2026.2"),
            &[
                obs(SourceSystem::ExamSystem, Some("2026-12-10"), "2026.2"),
                obs(SourceSystem::AcademicCalendar, Some("2026-12-12"), "2026.2"),
            ],
        )
        .unwrap();
        assert_eq!(r[0].date.state, Verification::Conflicting);
        assert!(r[0].date_display().starts_with("conflitante"));
    }

    #[test]
    fn dates_from_another_term_are_never_borrowed() {
        let err = build_records(
            &term("2026.2"),
            &[obs(SourceSystem::ExamSystem, Some("2026-06-10"), "2026.1")],
        )
        .unwrap_err();
        assert!(matches!(err, ExamError::WrongTerm { .. }));
    }

    #[test]
    fn invalid_dates_and_times_are_rejected() {
        for d in [
            "2026-02-30",
            "2026-13-01",
            "10/12/2026",
            "2026-1-5",
            "2025-02-29",
        ] {
            assert!(!parse_iso_date(d), "{d}");
        }
        assert!(parse_iso_date("2028-02-29"));
        assert!(parse_hh_mm("08:30") && !parse_hh_mm("24:00") && !parse_hh_mm("8:30"));
        let bad = obs(SourceSystem::ExamSystem, Some("2026-02-30"), "2026.2");
        assert!(matches!(
            build_records(&term("2026.2"), &[bad]),
            Err(ExamError::InvalidDate(_))
        ));
    }

    #[test]
    fn scheduling_unknown_does_not_become_not_scheduled() {
        let r = build_records(
            &term("2026.2"),
            &[obs(SourceSystem::ExamSystem, None, "2026.2")],
        )
        .unwrap();
        assert_eq!(r[0].scheduling, Field::unknown());
        assert_eq!(r[0].status, Field::unknown());
    }
}
