//! Contratos dos leitores por sistema (Fase B) e detecção de mudança de layout.
//!
//! Os leitores concretos (CDP/`fetch`) ficam em `estuda-core`/GPUI e implementam estes traits.
//! Nenhum leitor pode escrever: a API aqui só devolve dados lidos, cobertura e bloqueios.
//! Os parsers de HTML do Sistema de Provas, Zeev e Calendário **não** foram escritos porque
//! essas telas ainda não foram observadas numa sessão autenticada (ver
//! `docs/NAVEGACAO_PORTAL_UVA.md`, "a validar"): inventar seletores seria inventar dados.

use serde::{Deserialize, Serialize};

use crate::exam::ExamObservation;
use crate::link::AcademicLink;
use crate::policy::Blocker;
use crate::source::SourceRef;
use crate::text::{fold, sha256_hex};
use crate::verification::{AvaObservation, Coverage};

/// Resultado de uma leitura: dados, quão completa foi e o que impediu de ir adiante.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReadResult<T> {
    pub items: Vec<T>,
    pub coverage: Coverage,
    pub blockers: Vec<Blocker>,
    pub layout_changes: Vec<LayoutChanged>,
    pub source: SourceRef,
}

impl<T> ReadResult<T> {
    pub fn empty(source: SourceRef) -> Self {
        Self {
            items: vec![],
            coverage: Coverage::NotRead,
            blockers: vec![],
            layout_changes: vec![],
            source,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EnrollmentRow {
    pub link_key: String,
    pub subject_name: String,
    pub term: String,
    pub official_status: Option<String>,
    pub attendance_percent_hundredths: Option<u32>,
    pub grade_hundredths: Option<u32>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CalendarEntry {
    pub term: String,
    pub label: String,
    pub starts_on: Option<String>,
    pub ends_on: Option<String>,
    pub source: SourceRef,
}

pub trait PortalReader {
    fn read_links(&mut self) -> ReadResult<AcademicLink>;
    fn read_enrollments(&mut self, link: &AcademicLink) -> ReadResult<EnrollmentRow>;
}

pub trait CanvasReader {
    fn read_assessments(&mut self, course_id: &str) -> ReadResult<AvaObservation>;
}

pub trait ExamReader {
    fn read_exams(&mut self) -> ReadResult<ExamObservation>;
}

pub trait CalendarReader {
    fn read_calendar(&mut self, term: &str) -> ReadResult<CalendarEntry>;
}

// ---------------------------------------------------------------------------------------
// Mudança de layout
// ---------------------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LayoutChanged {
    pub table: String,
    pub missing_columns: Vec<String>,
    pub found_columns: Vec<String>,
}

/// Mapeia colunas esperadas (por nome, sem acento/caixa) para o índice real. Se faltar
/// alguma, é mudança de layout: o chamador deve marcar a leitura como `Partial` e **não**
/// usar posições antigas.
pub fn map_columns(
    table: &str,
    expected: &[&str],
    header: &[String],
) -> Result<Vec<usize>, LayoutChanged> {
    let folded: Vec<String> = header.iter().map(|h| fold(h)).collect();
    let mut indices = Vec::with_capacity(expected.len());
    let mut missing = Vec::new();
    for name in expected {
        match folded.iter().position(|h| *h == fold(name)) {
            Some(i) => indices.push(i),
            None => missing.push((*name).to_owned()),
        }
    }
    if missing.is_empty() {
        Ok(indices)
    } else {
        Err(LayoutChanged {
            table: table.to_owned(),
            missing_columns: missing,
            found_columns: header.to_vec(),
        })
    }
}

/// Impressão digital da estrutura (nomes de colunas ordenados), para comparar entre leituras.
pub fn layout_fingerprint(header: &[String]) -> String {
    let mut cols: Vec<String> = header.iter().map(|h| fold(h)).collect();
    cols.sort();
    sha256_hex(cols.join("\u{1f}").as_bytes())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn h(cols: &[&str]) -> Vec<String> {
        cols.iter().map(|c| (*c).to_owned()).collect()
    }

    #[test]
    fn columns_are_found_by_name_not_by_position() {
        let idx = map_columns(
            "boletim",
            &["Disciplina", "Nota"],
            &h(&["Situação", "NOTA", "Disciplina"]),
        )
        .unwrap();
        assert_eq!(idx, [2, 1]);
    }

    #[test]
    fn renamed_column_is_reported_as_layout_change() {
        let err = map_columns(
            "frequência",
            &["Disciplina", "Faltas"],
            &h(&["Disciplina", "Ausências"]),
        )
        .unwrap_err();
        assert_eq!(err.missing_columns, ["Faltas"]);
        assert_eq!(err.found_columns, ["Disciplina", "Ausências"]);
    }

    #[test]
    fn fingerprint_ignores_order_and_accents_but_not_new_columns() {
        assert_eq!(
            layout_fingerprint(&h(&["Nota", "Disciplina"])),
            layout_fingerprint(&h(&["disciplina", "NOTA"]))
        );
        assert_ne!(
            layout_fingerprint(&h(&["Nota"])),
            layout_fingerprint(&h(&["Nota", "Peso"]))
        );
    }
}
