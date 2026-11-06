use serde::{Deserialize, Serialize};
use std::path::Path;
use unicode_normalization::{UnicodeNormalization, char::is_combining_mark};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PathClassification {
    pub course: Option<String>,
    pub semester: Option<String>,
    pub category: String,
}

impl PathClassification {
    pub fn matches_course_filter(&self, filter: &str) -> bool {
        self.course
            .as_deref()
            .is_some_and(|course| fold(course).contains(&fold(filter)))
    }
}

pub fn classify_path(path: &Path) -> PathClassification {
    let normalized = path.to_string_lossy().replace('\\', "/");
    let parts = normalized.split('/').collect::<Vec<_>>();
    let course = parts
        .iter()
        .find_map(|part| known_course(part).map(str::to_owned));
    let semester = normalized
        .split('/')
        .find_map(find_explicit_semester)
        .map(str::to_owned);
    let category = classify_category(&parts);

    PathClassification {
        course,
        semester,
        category,
    }
}

fn known_course(folder: &str) -> Option<&'static str> {
    match fold(folder).replace('_', "-").as_str() {
        "bases-de-programacao" => Some("Bases de Programação"),
        "desenvolvimento-web" | "desenvolvimento-de-aplicacoes-para-web" => {
            Some("Desenvolvimento de Aplicações para Web")
        }
        "paradigmas-de-programacao" | "paradigmas-de-linguagens-de-programacao" => {
            Some("Paradigmas de Programação")
        }
        "laboratorio-de-gestao" => Some("Laboratório de Gestão"),
        "leitura-e-producao-academica" => Some("Leitura e Produção Acadêmica"),
        "sistemas-computacionais" => Some("Sistemas Computacionais"),
        _ => None,
    }
}

fn find_explicit_semester(segment: &str) -> Option<&str> {
    let bytes = segment.as_bytes();
    if bytes.len() < 6 {
        return None;
    }
    for start in 0..=bytes.len() - 6 {
        let candidate = &bytes[start..start + 6];
        let has_boundary_before = start == 0 || !bytes[start - 1].is_ascii_digit();
        let has_boundary_after = start + 6 == bytes.len() || !bytes[start + 6].is_ascii_digit();
        if has_boundary_before
            && has_boundary_after
            && candidate[..4].iter().all(u8::is_ascii_digit)
            && candidate[4] == b'.'
            && matches!(candidate[5], b'1' | b'2')
        {
            return std::str::from_utf8(candidate).ok();
        }
    }
    None
}

fn classify_category(parts: &[&str]) -> String {
    let file_name = parts.last().map(|part| fold(part)).unwrap_or_default();
    match parts.first().map(|part| fold(part)).as_deref() {
        Some("materias") if is_official_record_file(&file_name) => "registro acadêmico".into(),
        Some("materias") if parts.iter().any(|part| known_course(part).is_some()) => {
            "material acadêmico".into()
        }
        Some("materias") if parts.get(1).is_some_and(|part| fold(part) == "catalogo") => {
            "catálogo curricular".into()
        }
        Some("materias")
            if parts
                .iter()
                .any(|part| matches!(fold(part).as_str(), "apoio" | "institucional")) =>
        {
            "documentação acadêmica".into()
        }
        Some("materias") => "não classificado".into(),
        Some("fontes-uva") if is_official_record_file(&file_name) => "registro acadêmico".into(),
        Some("fontes-uva")
            if parts
                .iter()
                .any(|part| matches!(fold(part).as_str(), "apoio" | "institucional")) =>
        {
            "documentação acadêmica".into()
        }
        Some("fontes-uva") => "material acadêmico".into(),
        Some("cursos") => "histórico de programa".into(),
        Some("periodos") => match file_name.trim_end_matches(".md") {
            "readme" => "registro de cursos".into(),
            "materiais" => "índice de materiais".into(),
            "calendario" => "calendário acadêmico".into(),
            "notas-de-estudo" => "notas de estudo".into(),
            "futuros" => "planejamento acadêmico".into(),
            _ => "não classificado".into(),
        },
        Some("originais") => "original acadêmico".into(),
        Some("docs") => "documentação".into(),
        _ => {
            if is_official_record_file(&file_name) {
                "registro acadêmico".into()
            } else if file_name.starts_with("cursos_") {
                "registro de cursos".into()
            } else if file_name.starts_with("materiais_") {
                "índice de materiais".into()
            } else if file_name.starts_with("calendario_") {
                "calendário acadêmico".into()
            } else if file_name.starts_with("notas_de_estudo_") {
                "notas de estudo".into()
            } else if file_name.starts_with("futuros_semestres")
                || file_name.starts_with("planejamento")
            {
                "planejamento acadêmico".into()
            } else if file_name.starts_with("engenharia_de_computacao") {
                "histórico de programa".into()
            } else {
                "não classificado".into()
            }
        }
    }
}

fn is_official_record_file(file_name: &str) -> bool {
    let normalized = file_name.replace('_', "-");
    [
        "registro-academico",
        "registro-de-notas",
        "notas-oficiais",
        "boletim-escolar",
        "historico-escolar",
        "historico-academico",
        "registro-de-frequencia",
        "registro-de-presenca",
    ]
    .iter()
    .any(|prefix| normalized.starts_with(prefix))
}

fn fold(value: &str) -> String {
    value
        .nfd()
        .filter(|character| !is_combining_mark(*character))
        .flat_map(char::to_lowercase)
        .collect()
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use super::classify_path;

    #[test]
    fn classifies_known_course_and_only_explicit_semester() {
        let classification = classify_path(Path::new(
            "fontes-uva/2026.2/bases-de-programacao/capitulo-01.txt",
        ));

        assert_eq!(
            classification.course.as_deref(),
            Some("Bases de Programação")
        );
        assert_eq!(classification.semester.as_deref(), Some("2026.2"));
        assert_eq!(classification.category, "material acadêmico");
    }

    #[test]
    fn unknown_paths_remain_unclassified_instead_of_guessing_course_or_semester() {
        let classification = classify_path(Path::new("materias/curso-desconhecido/nota.md"));

        assert_eq!(classification.course, None);
        assert_eq!(classification.semester, None);
        assert_eq!(classification.category, "não classificado");
    }

    #[test]
    fn keeps_official_academic_records_out_of_materials_inside_subject_folders() {
        for file_name in [
            "registro_academico.md",
            "registro-de-notas.md",
            "notas-oficiais.md",
            "boletim-escolar.md",
            "histórico-escolar.md",
            "histórico-acadêmico.md",
            "registro-de-frequencia.md",
            "registro-de-presenca.md",
        ] {
            let path = format!("fontes-uva/2026.2/bases-de-programacao/{file_name}");
            assert_eq!(
                classify_path(Path::new(&path)).category,
                "registro acadêmico",
                "{file_name} must not enter material-based study summaries",
            );
        }
    }

    #[test]
    fn classifies_root_academic_register_without_claiming_course_enrollment() {
        let classification = classify_path(Path::new("REGISTRO_ACADEMICO.md"));

        assert_eq!(classification.course, None);
        assert_eq!(classification.semester, None);
        assert_eq!(classification.category, "registro acadêmico");
    }

    #[test]
    fn course_filter_ignores_case_and_diacritics() {
        let classification =
            classify_path(Path::new("materias/paradigmas-de-programacao/trabalho.md"));

        assert!(classification.matches_course_filter("PARADIGMAS DE PROGRAMACAO"));
    }
}
