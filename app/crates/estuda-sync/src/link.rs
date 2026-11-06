//! Seleção do vínculo acadêmico. O estado (ativo/evadido/…) vem **da fonte**; nada aqui
//! presume qual curso está ativo. Trajetórias de vínculos distintos nunca se misturam.

use std::collections::BTreeMap;

use estuda_domain::ids::DegreeProgramId;
use serde::{Deserialize, Serialize};

use crate::source::SourceRef;
use crate::text::fold;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum LinkStatus {
    Active,
    /// Texto original da fonte (ex.: "Evadido"), para o relatório.
    Inactive(String),
    /// Texto que não foi reconhecido: exige revisão humana.
    Unknown(String),
}

impl LinkStatus {
    /// Interpreta o texto de situação exibido pela fonte. Inativo é testado antes de ativo
    /// porque "inativo" contém "ativo".
    pub fn parse(source_text: &str) -> Self {
        let folded = fold(source_text);
        const INACTIVE: &[&str] = &[
            "evadid",
            "inativ",
            "trancad",
            "formad",
            "cancelad",
            "jubilad",
            "transferid",
            "desligad",
        ];
        const ACTIVE: &[&str] = &["ativo", "regular", "matriculad", "cursando"];
        if INACTIVE.iter().any(|k| folded.contains(k)) {
            LinkStatus::Inactive(source_text.trim().to_owned())
        } else if ACTIVE.iter().any(|k| folded.contains(k)) {
            LinkStatus::Active
        } else {
            LinkStatus::Unknown(source_text.trim().to_owned())
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AcademicLink {
    /// Chave opaca do vínculo na fonte (não é matrícula legível; fica no domínio privado).
    pub link_key: String,
    pub program_name: String,
    pub program: Option<DegreeProgramId>,
    pub status: LinkStatus,
    pub source: SourceRef,
}

/// Programa desejado. `aliases` são nomes aceitos quando o catálogo não mapeou o id.
#[derive(Debug, Clone)]
pub struct Criteria {
    pub program: DegreeProgramId,
    pub aliases: Vec<String>,
}

impl Criteria {
    fn matches(&self, link: &AcademicLink) -> bool {
        if let Some(id) = &link.program {
            return *id == self.program;
        }
        let name = fold(&link.program_name);
        self.aliases.iter().any(|a| fold(a) == name)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum LinkSelection<'a> {
    Selected(&'a AcademicLink),
    /// O programa existe, mas nenhum vínculo dele está ativo na fonte.
    NoActiveLink {
        inactive: Vec<&'a AcademicLink>,
    },
    /// Mais de um vínculo ativo para o mesmo programa: revisão humana.
    Ambiguous(Vec<&'a AcademicLink>),
    /// A fonte não mostrou o status de forma reconhecível.
    NeedsReview(Vec<&'a AcademicLink>),
    NotFound,
}

pub fn select_active_link<'a>(links: &'a [AcademicLink], criteria: &Criteria) -> LinkSelection<'a> {
    let matching: Vec<&AcademicLink> = links.iter().filter(|l| criteria.matches(l)).collect();
    if matching.is_empty() {
        return LinkSelection::NotFound;
    }
    let active: Vec<&AcademicLink> = matching
        .iter()
        .copied()
        .filter(|l| l.status == LinkStatus::Active)
        .collect();
    match active.len() {
        1 => LinkSelection::Selected(active[0]),
        n if n > 1 => LinkSelection::Ambiguous(active),
        _ => {
            let unknown: Vec<&AcademicLink> = matching
                .iter()
                .copied()
                .filter(|l| matches!(l.status, LinkStatus::Unknown(_)))
                .collect();
            if unknown.is_empty() {
                LinkSelection::NoActiveLink { inactive: matching }
            } else {
                LinkSelection::NeedsReview(unknown)
            }
        }
    }
}

/// Item qualquer lido sob um vínculo (matrícula, resultado, frequência).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnderLink<T> {
    pub link_key: String,
    pub item: T,
}

/// Agrupa por vínculo. Cada trajetória é processada isoladamente; não há função que una grupos.
pub fn split_trajectories<T: Clone>(items: &[UnderLink<T>]) -> BTreeMap<String, Vec<T>> {
    let mut out: BTreeMap<String, Vec<T>> = BTreeMap::new();
    for entry in items {
        out.entry(entry.link_key.clone())
            .or_default()
            .push(entry.item.clone());
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::source::SourceSystem;

    fn link(key: &str, name: &str, status: &str, id: Option<&str>) -> AcademicLink {
        AcademicLink {
            link_key: key.into(),
            program_name: name.into(),
            program: id.map(|i| DegreeProgramId::parse(i).unwrap()),
            status: LinkStatus::parse(status),
            source: SourceRef::new(SourceSystem::Lyceum, None, "2026-11-06T10:00:00Z"),
        }
    }

    fn cs() -> Criteria {
        Criteria {
            program: DegreeProgramId::parse("uva:program:ciencia-da-computacao").unwrap(),
            aliases: vec!["Ciência da Computação".into()],
        }
    }

    #[test]
    fn status_parser_tests_inactive_before_active() {
        assert_eq!(LinkStatus::parse("Ativo"), LinkStatus::Active);
        assert!(matches!(
            LinkStatus::parse("Inativo"),
            LinkStatus::Inactive(_)
        ));
        assert!(matches!(
            LinkStatus::parse("Evadido"),
            LinkStatus::Inactive(_)
        ));
        assert!(matches!(LinkStatus::parse("???"), LinkStatus::Unknown(_)));
    }

    #[test]
    fn selects_active_cs_link_and_ignores_evaded_engineering() {
        let links = vec![
            link(
                "v1",
                "Engenharia de Computação",
                "Evadido",
                Some("uva:program:engenharia-de-computacao"),
            ),
            link(
                "v2",
                "Ciência da Computação",
                "Ativo",
                Some("uva:program:ciencia-da-computacao"),
            ),
        ];
        match select_active_link(&links, &cs()) {
            LinkSelection::Selected(l) => assert_eq!(l.link_key, "v2"),
            other => panic!("{other:?}"),
        }
    }

    #[test]
    fn status_is_read_from_source_not_assumed() {
        // Se a fonte disser que CC está evadido, não há vínculo ativo — não cai para Engenharia.
        let links = vec![
            link(
                "v1",
                "Engenharia de Computação",
                "Ativo",
                Some("uva:program:engenharia-de-computacao"),
            ),
            link(
                "v2",
                "Ciência da Computação",
                "Evadido",
                Some("uva:program:ciencia-da-computacao"),
            ),
        ];
        assert!(matches!(
            select_active_link(&links, &cs()),
            LinkSelection::NoActiveLink { .. }
        ));
    }

    #[test]
    fn matches_by_alias_when_catalog_id_is_missing() {
        let links = vec![link("v2", "CIÊNCIA DA COMPUTAÇÃO", "Ativo", None)];
        assert!(matches!(
            select_active_link(&links, &cs()),
            LinkSelection::Selected(_)
        ));
    }

    #[test]
    fn ambiguous_unknown_and_missing_need_human_review() {
        let two = vec![
            link("a", "Ciência da Computação", "Ativo", None),
            link("b", "Ciência da Computação", "Regular", None),
        ];
        assert!(matches!(
            select_active_link(&two, &cs()),
            LinkSelection::Ambiguous(_)
        ));
        let unk = vec![link("a", "Ciência da Computação", "Situação X", None)];
        assert!(matches!(
            select_active_link(&unk, &cs()),
            LinkSelection::NeedsReview(_)
        ));
        assert_eq!(select_active_link(&[], &cs()), LinkSelection::NotFound);
    }

    #[test]
    fn trajectories_stay_separated_by_link() {
        let items = vec![
            UnderLink {
                link_key: "v1".into(),
                item: "Cálculo I",
            },
            UnderLink {
                link_key: "v2".into(),
                item: "Algoritmos",
            },
            UnderLink {
                link_key: "v1".into(),
                item: "Física",
            },
        ];
        let groups = split_trajectories(&items);
        assert_eq!(groups["v1"], ["Cálculo I", "Física"]);
        assert_eq!(groups["v2"], ["Algoritmos"]);
    }
}
