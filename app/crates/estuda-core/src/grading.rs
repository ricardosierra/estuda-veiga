//! Nota Final Ponderada (NFp) da graduação EAD da UVA.
//!
//! Regras de `materias/fontes-uva/2026.1/apoio/CRITERIOS_GRADUACAO_AVALIACAO_2024.md` e do tutorial
//! `Como_calcular_a_Media_Final_Ponderada.md`. Cálculo puro e local: sem rede e sem IA. O resultado
//! é sempre uma estimativa até o Lyceum publicar a média oficial.

use serde::{Deserialize, Serialize};

/// As notas são convertidas para centésimos inteiros. A NFp fica em "décimos de milésimo"
/// (`A1 × 40 + prova × 60`), o que torna a comparação com a média de aprovação exata.
const A1_WEIGHT: i64 = 40;
const EXAM_WEIGHT: i64 = 60;
const REGULAR_PASS_HUNDREDTHS: i64 = 600;
const SPECIAL_PASS_HUNDREDTHS: i64 = 700;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DisciplineKind {
    /// Disciplina regular: A1 (AVAs), A2 (prova) e A3 (substitutiva da A2).
    Regular,
    /// Estágio, TCC, monografia e afins: A1 e A4, sem A2/A3.
    Special,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GradeStatus {
    Approved,
    Failed,
    /// Faltam notas para concluir o cálculo.
    Incomplete,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FinalGradeInput {
    pub kind: DisciplineKind,
    pub a1: Option<f64>,
    pub a2: Option<f64>,
    pub a3: Option<f64>,
    pub a4: Option<f64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FinalGradeResult {
    pub status: GradeStatus,
    /// Nota final ponderada arredondada a duas casas, quando calculável.
    pub final_grade: Option<f64>,
    pub passing_grade: f64,
    /// Qual nota de prova entrou no cálculo: "A2", "A3" ou "A4".
    pub exam_used: Option<String>,
    /// Nota mínima necessária na prova restante para atingir a aprovação.
    pub needed_exam_grade: Option<f64>,
    pub formula: String,
    pub notes: Vec<String>,
}

pub fn final_grade(input: &FinalGradeInput) -> Result<FinalGradeResult, String> {
    for (label, value) in [
        ("A1", input.a1),
        ("A2", input.a2),
        ("A3", input.a3),
        ("A4", input.a4),
    ] {
        if let Some(value) = value {
            if !value.is_finite() || !(0.0..=10.0).contains(&value) {
                return Err(format!("{label} deve estar entre 0 e 10."));
            }
        }
    }
    match input.kind {
        DisciplineKind::Regular => {
            if input.a4.is_some() {
                return Err("Disciplina regular não usa A4; escolha disciplina especial.".into());
            }
            Ok(regular(input))
        }
        DisciplineKind::Special => {
            if input.a2.is_some() || input.a3.is_some() {
                return Err("Disciplina especial não tem A2/A3; use a A4.".into());
            }
            Ok(special(input))
        }
    }
}

fn regular(input: &FinalGradeInput) -> FinalGradeResult {
    let passing = REGULAR_PASS_HUNDREDTHS;
    let mut notes =
        vec!["Estimativa local. A média oficial é a publicada no boletim do Lyceum.".to_owned()];
    let best_exam = match (input.a2, input.a3) {
        (Some(a2), Some(a3)) if a3 > a2 => Some(("A3", a3)),
        (Some(a2), _) => Some(("A2", a2)),
        (None, Some(a3)) => Some(("A3", a3)),
        (None, None) => None,
    };
    let Some(a1) = input.a1 else {
        notes.push("Informe a A1 (média das atividades avaliativas online).".into());
        return FinalGradeResult {
            status: GradeStatus::Incomplete,
            final_grade: None,
            passing_grade: hundredths_to_grade(passing),
            exam_used: None,
            needed_exam_grade: None,
            formula: "NFp = A1 × 0,4 + maior(A2, A3) × 0,6".into(),
            notes,
        };
    };
    if a1 == 0.0 {
        notes.push(
            "A1 igual a 0,0: reprovação automática. A A1 não pode ser substituída e não tem 2ª chamada."
                .into(),
        );
        let final_grade =
            best_exam.map(|(_, exam)| scaled_to_grade(to_hundredths(exam) * EXAM_WEIGHT / 2));
        return FinalGradeResult {
            status: GradeStatus::Failed,
            final_grade,
            passing_grade: hundredths_to_grade(passing),
            exam_used: best_exam.map(|(label, _)| label.to_owned()),
            needed_exam_grade: None,
            formula: "NFp = (A1 × 0,4) + (A2 ou A3 × 0,6) / 2".into(),
            notes,
        };
    }
    let formula = "NFp = A1 × 0,4 + maior(A2, A3) × 0,6".to_owned();
    let Some((label, exam)) = best_exam else {
        let needed = needed_exam(a1, passing);
        match needed {
            Some(value) => notes.push(format!(
                "Para chegar a {:.1}, a A2 (ou a A3) precisa de pelo menos {:.2}.",
                hundredths_to_grade(passing),
                value
            )),
            None => notes.push("Nem nota 10 na prova alcança a média de aprovação.".into()),
        }
        return FinalGradeResult {
            status: GradeStatus::Incomplete,
            final_grade: None,
            passing_grade: hundredths_to_grade(passing),
            exam_used: None,
            needed_exam_grade: needed,
            formula,
            notes,
        };
    };
    let total = weighted(a1, exam);
    let status = if total >= passing * 100 {
        GradeStatus::Approved
    } else {
        GradeStatus::Failed
    };
    let mut needed_exam_grade = None;
    if status == GradeStatus::Failed && input.a3.is_none() {
        needed_exam_grade = needed_exam(a1, passing);
        match needed_exam_grade {
            Some(value) => notes.push(format!(
                "A A3 substitui a A2 se for maior. Para aprovação, a A3 precisa de pelo menos {value:.2}."
            )),
            None => notes.push("Nem nota 10 na A3 alcança a média de aprovação.".into()),
        }
    }
    FinalGradeResult {
        status,
        final_grade: Some(scaled_to_grade(total)),
        passing_grade: hundredths_to_grade(passing),
        exam_used: Some(label.to_owned()),
        needed_exam_grade,
        formula,
        notes,
    }
}

fn special(input: &FinalGradeInput) -> FinalGradeResult {
    let passing = SPECIAL_PASS_HUNDREDTHS;
    let formula = "NFp = A1 × 0,4 + A4 × 0,6".to_owned();
    let mut notes = vec![
        "Estimativa local. A média oficial é a publicada no boletim do Lyceum.".to_owned(),
        "Disciplinas especiais não têm segunda chamada nem recuperação.".to_owned(),
    ];
    let Some(a1) = input.a1 else {
        notes.push(
            "Sem A1 informada. Algumas disciplinas especiais não usam A1; nesse caso consulte os Critérios Avaliativos da disciplina, porque a fórmula não está nas fontes do acervo."
                .into(),
        );
        return FinalGradeResult {
            status: GradeStatus::Incomplete,
            final_grade: None,
            passing_grade: hundredths_to_grade(passing),
            exam_used: None,
            needed_exam_grade: None,
            formula,
            notes,
        };
    };
    let Some(a4) = input.a4 else {
        let needed = needed_exam(a1, passing);
        match needed {
            Some(value) => notes.push(format!(
                "Para chegar a {:.1}, a A4 precisa de pelo menos {value:.2}.",
                hundredths_to_grade(passing)
            )),
            None => notes.push("Nem nota 10 na A4 alcança a média de aprovação.".into()),
        }
        return FinalGradeResult {
            status: GradeStatus::Incomplete,
            final_grade: None,
            passing_grade: hundredths_to_grade(passing),
            exam_used: None,
            needed_exam_grade: needed,
            formula,
            notes,
        };
    };
    let total = weighted(a1, a4);
    FinalGradeResult {
        status: if total >= passing * 100 {
            GradeStatus::Approved
        } else {
            GradeStatus::Failed
        },
        final_grade: Some(scaled_to_grade(total)),
        passing_grade: hundredths_to_grade(passing),
        exam_used: Some("A4".into()),
        needed_exam_grade: None,
        formula,
        notes,
    }
}

fn to_hundredths(grade: f64) -> i64 {
    (grade * 100.0).round() as i64
}

/// NFp × 10.000, exata para notas com até duas casas decimais.
fn weighted(a1: f64, exam: f64) -> i64 {
    to_hundredths(a1) * A1_WEIGHT + to_hundredths(exam) * EXAM_WEIGHT
}

/// Menor nota de prova (múltiplo de 0,01) que leva a NFp ao mínimo de aprovação.
fn needed_exam(a1: f64, passing: i64) -> Option<f64> {
    let missing = (passing * 100 - to_hundredths(a1) * A1_WEIGHT).max(0);
    let needed = (missing + EXAM_WEIGHT - 1) / EXAM_WEIGHT;
    (needed <= 1000).then(|| hundredths_to_grade(needed))
}

/// Converte NFp × 10.000 em nota com duas casas, truncando: 5,996 aparece como 5,99,
/// coerente com a reprovação, e nunca como 6,00.
fn scaled_to_grade(value: i64) -> f64 {
    hundredths_to_grade(value / 100)
}

fn hundredths_to_grade(value: i64) -> f64 {
    value as f64 / 100.0
}

#[cfg(test)]
mod tests {
    use super::*;

    fn regular_input(a1: Option<f64>, a2: Option<f64>, a3: Option<f64>) -> FinalGradeInput {
        FinalGradeInput {
            kind: DisciplineKind::Regular,
            a1,
            a2,
            a3,
            a4: None,
        }
    }

    #[test]
    fn tutorial_examples_match_official_results() {
        let first = final_grade(&regular_input(Some(10.0), Some(9.0), Some(7.0))).unwrap();
        assert_eq!(first.final_grade, Some(9.4));
        assert_eq!(first.exam_used.as_deref(), Some("A2"));
        assert_eq!(first.status, GradeStatus::Approved);

        let second = final_grade(&regular_input(Some(10.0), Some(4.0), Some(8.0))).unwrap();
        assert_eq!(second.final_grade, Some(8.8));
        assert_eq!(second.exam_used.as_deref(), Some("A3"));

        let special = final_grade(&FinalGradeInput {
            kind: DisciplineKind::Special,
            a1: Some(10.0),
            a2: None,
            a3: None,
            a4: Some(9.0),
        })
        .unwrap();
        assert_eq!(special.final_grade, Some(9.4));
        assert_eq!(special.passing_grade, 7.0);
        assert_eq!(special.status, GradeStatus::Approved);
    }

    #[test]
    fn exact_threshold_is_approved_without_float_drift() {
        // 6,0 × 0,4 + 6,0 × 0,6 = 6,0 exatamente; em f64 dá 5,999…
        let result = final_grade(&regular_input(Some(6.0), Some(6.0), None)).unwrap();
        assert_eq!(result.final_grade, Some(6.0));
        assert_eq!(result.status, GradeStatus::Approved);
    }

    #[test]
    fn zero_a1_fails_automatically() {
        let result = final_grade(&regular_input(Some(0.0), Some(10.0), None)).unwrap();
        assert_eq!(result.status, GradeStatus::Failed);
        assert_eq!(result.final_grade, Some(3.0));
        assert_eq!(result.needed_exam_grade, None);
    }

    #[test]
    fn missing_exam_reports_needed_grade() {
        let result = final_grade(&regular_input(Some(8.1), None, None)).unwrap();
        assert_eq!(result.status, GradeStatus::Incomplete);
        // 8,1 × 0,4 = 3,24; faltam 2,76 → 2,76 / 0,6 = 4,6
        assert_eq!(result.needed_exam_grade, Some(4.6));
        let check = final_grade(&regular_input(Some(8.1), Some(4.6), None)).unwrap();
        assert_eq!(check.status, GradeStatus::Approved);
        let below = final_grade(&regular_input(Some(8.1), Some(4.59), None)).unwrap();
        assert_eq!(below.status, GradeStatus::Failed);
    }

    #[test]
    fn failed_a2_without_a3_points_to_substitute_exam() {
        let result = final_grade(&regular_input(Some(5.0), Some(3.0), None)).unwrap();
        assert_eq!(result.status, GradeStatus::Failed);
        assert_eq!(result.final_grade, Some(3.8));
        assert_eq!(result.needed_exam_grade, Some(6.67));
        // 5 × 0,4 + 6,66 × 0,6 = 5,996: reprovado e exibido como 5,99.
        let close = final_grade(&regular_input(Some(5.0), Some(3.0), Some(6.66))).unwrap();
        assert_eq!(close.status, GradeStatus::Failed);
        assert_eq!(close.final_grade, Some(5.99));
        let enough = final_grade(&regular_input(Some(5.0), Some(3.0), Some(6.67))).unwrap();
        assert_eq!(enough.status, GradeStatus::Approved);
    }

    #[test]
    fn unreachable_target_has_no_needed_grade() {
        let result = final_grade(&FinalGradeInput {
            kind: DisciplineKind::Special,
            a1: Some(1.0),
            a2: None,
            a3: None,
            a4: None,
        })
        .unwrap();
        // 1,0 × 0,4 + 10 × 0,6 = 6,4 < 7,0
        assert_eq!(result.needed_exam_grade, None);
    }

    #[test]
    fn invalid_inputs_are_rejected() {
        assert!(final_grade(&regular_input(Some(11.0), None, None)).is_err());
        assert!(final_grade(&regular_input(Some(f64::NAN), None, None)).is_err());
        assert!(
            final_grade(&FinalGradeInput {
                kind: DisciplineKind::Regular,
                a1: Some(8.0),
                a2: None,
                a3: None,
                a4: Some(8.0),
            })
            .is_err()
        );
        assert!(
            final_grade(&FinalGradeInput {
                kind: DisciplineKind::Special,
                a1: Some(8.0),
                a2: Some(8.0),
                a3: None,
                a4: None,
            })
            .is_err()
        );
    }

    #[test]
    fn missing_a1_is_incomplete() {
        let result = final_grade(&regular_input(None, Some(9.0), None)).unwrap();
        assert_eq!(result.status, GradeStatus::Incomplete);
        assert_eq!(result.final_grade, None);
    }
}
