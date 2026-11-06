use crate::lesson::{definition_in, score_answer};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

pub fn hash_section_markdown(markdown: &str) -> String {
    format!("{:x}", Sha256::digest(markdown.as_bytes()))
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TutorSectionInput {
    pub index: usize,
    pub title: String,
    pub markdown: String,
    pub source_page: Option<u32>,
}

impl TutorSectionInput {
    pub fn into_lesson_section(self) -> LessonSection {
        let source_sha256 = hash_section_markdown(&self.markdown);
        let source_excerpt = self.markdown.chars().take(600).collect();
        LessonSection {
            index: self.index,
            title: self.title.clone(),
            markdown: self.markdown,
            source_page: self.source_page,
            source_heading: self.title,
            source_excerpt,
            source_sha256,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct LessonSection {
    pub index: usize,
    pub title: String,
    pub markdown: String,
    pub source_page: Option<u32>,
    pub source_heading: String,
    pub source_excerpt: String,
    pub source_sha256: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum TutorActivityKind {
    Verifiable,
    SelfAssessment,
}

impl TutorActivityKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Verifiable => "verifiable",
            Self::SelfAssessment => "self_assessment",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct TutorActivity {
    pub id: String,
    pub session_id: String,
    pub section_index: usize,
    pub kind: TutorActivityKind,
    pub prompt: String,
    pub reference_answer: Option<String>,
    pub source_page: Option<u32>,
    pub source_heading: String,
    pub source_excerpt: String,
    pub source_sha256: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum TutorAssessmentStatus {
    Correct,
    SelfAssessed,
    Pending,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct TutorAssessment {
    pub status: TutorAssessmentStatus,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub correct: Option<bool>,
}

pub fn build_tutor_activity(section: &LessonSection, session_id: &str) -> TutorActivity {
    let found_definition = section
        .markdown
        .lines()
        .filter(|line| !line.trim_start().starts_with("```"))
        .find_map(definition_in_teaching_line);
    let (kind, prompt, reference_answer) = match found_definition {
        Some((term, answer)) => (
            TutorActivityKind::Verifiable,
            format!("O que o material chama de {}?", term.trim()),
            Some(answer.trim().to_owned()),
        ),
        None => (
            TutorActivityKind::SelfAssessment,
            "Explique a ideia principal deste trecho com suas palavras.".to_owned(),
            None,
        ),
    };

    TutorActivity {
        id: format!("{}:{}", session_id, section.index),
        session_id: session_id.to_owned(),
        section_index: section.index,
        kind,
        prompt,
        reference_answer,
        source_page: section.source_page,
        source_heading: section.source_heading.clone(),
        source_excerpt: section.source_excerpt.chars().take(600).collect(),
        source_sha256: section.source_sha256.clone(),
    }
}

fn definition_in_teaching_line(line: &str) -> Option<(&str, &str)> {
    let text = line.trim().trim_start_matches('>').trim();
    if is_source_metadata_line(text) {
        return None;
    }
    definition_in(text)
}

fn is_source_metadata_line(line: &str) -> bool {
    let text = line.trim().trim_start_matches(['#', '-', '*', ' ']).trim();
    let Some((label, _)) = text.split_once(':') else {
        return false;
    };
    let label = label
        .trim()
        .trim_matches(['*', '_'])
        .trim()
        .to_ascii_lowercase();
    matches!(
        label.as_str(),
        "disciplina"
            | "canvas"
            | "pdf oficial"
            | "fonte oficial"
            | "baixado em"
            | "paginas do pdf"
            | "páginas do pdf"
            | "paginas no pdf"
            | "páginas no pdf"
            | "paginas pdf"
            | "páginas pdf"
            | "pdf original"
            | "original preservado"
    )
}

pub fn assess_tutor_answer(activity: &TutorActivity, response: &str) -> TutorAssessment {
    if response.trim().is_empty() {
        return TutorAssessment {
            status: TutorAssessmentStatus::Pending,
            correct: None,
        };
    }

    if activity
        .reference_answer
        .as_deref()
        .is_some_and(|expected| score_answer(response, expected) == 1)
    {
        return TutorAssessment {
            status: TutorAssessmentStatus::Correct,
            correct: Some(true),
        };
    }

    TutorAssessment {
        status: TutorAssessmentStatus::SelfAssessed,
        correct: None,
    }
}

#[cfg(test)]
mod tests {
    use super::{LessonSection, TutorAssessmentStatus, assess_tutor_answer, build_tutor_activity};

    fn section(markdown: &str, page: Option<u32>) -> LessonSection {
        LessonSection {
            index: 2,
            title: "Página 4".into(),
            markdown: markdown.into(),
            source_page: page,
            source_heading: "Definição".into(),
            source_excerpt: markdown.into(),
            source_sha256: super::hash_section_markdown(markdown),
        }
    }

    #[test]
    fn builds_verifiable_activity_from_an_explicit_definition() {
        let activity = build_tutor_activity(
            &section(
                "Entropia: medida da incerteza de uma variável aleatória.",
                Some(4),
            ),
            "session-1",
        );

        assert_eq!(activity.session_id, "session-1");
        assert_eq!(activity.section_index, 2);
        assert_eq!(activity.kind.as_str(), "verifiable");
        assert_eq!(activity.prompt, "O que o material chama de Entropia?");
        assert_eq!(
            activity.reference_answer.as_deref(),
            Some("medida da incerteza de uma variável aleatória")
        );
        assert_eq!(activity.source_page, Some(4));
        assert_eq!(activity.source_sha256.len(), 64);
    }

    #[test]
    fn uses_open_self_assessment_when_no_trustworthy_answer_exists() {
        let activity = build_tutor_activity(
            &section("O gráfico mostra uma tendência crescente.", None),
            "s",
        );

        assert_eq!(activity.kind.as_str(), "self_assessment");
        assert!(activity.prompt.to_lowercase().contains("suas palavras"));
        assert!(activity.reference_answer.is_none());
        assert_eq!(activity.source_page, None);
    }

    #[test]
    fn does_not_turn_course_source_metadata_into_a_question() {
        let activity = build_tutor_activity(
            &section(
                "> Disciplina: Estatística para Computação\n> PDF oficial: Unidade 2\n> Fonte oficial: Canvas\n> Páginas no PDF: 19",
                Some(1),
            ),
            "s",
        );

        assert_eq!(activity.kind.as_str(), "self_assessment");
        assert!(!activity.prompt.contains("Disciplina"));
        assert!(!activity.prompt.contains("PDF oficial"));
    }

    #[test]
    fn exact_normalized_definition_answer_is_confirmed() {
        let activity = build_tutor_activity(
            &section(
                "Entropia: medida da incerteza de uma variável aleatória.",
                Some(4),
            ),
            "s",
        );
        let result =
            assess_tutor_answer(&activity, "Medida da incerteza de uma variável aleatoria!");

        assert_eq!(result.status, TutorAssessmentStatus::Correct);
        assert_eq!(result.correct, Some(true));
    }

    #[test]
    fn paraphrase_is_not_automatically_marked_incorrect() {
        let activity = build_tutor_activity(
            &section(
                "Entropia: medida da incerteza de uma variável aleatória.",
                Some(4),
            ),
            "s",
        );
        let result = assess_tutor_answer(
            &activity,
            "Ela representa o quanto os resultados podem variar.",
        );

        assert_eq!(result.status, TutorAssessmentStatus::SelfAssessed);
        assert_eq!(result.correct, None);
    }

    #[test]
    fn blank_response_remains_pending() {
        let activity = build_tutor_activity(
            &section(
                "Entropia: medida da incerteza de uma variável aleatória.",
                Some(4),
            ),
            "s",
        );
        let result = assess_tutor_answer(&activity, "  \n ");

        assert_eq!(result.status, TutorAssessmentStatus::Pending);
        assert_eq!(result.correct, None);
    }

    #[test]
    fn activity_retains_source_as_data_without_rendering_markdown() {
        let activity = build_tutor_activity(
            &section(
                "Conceito: <script>alert(1)</script> e [link](https://example.test)",
                None,
            ),
            "s",
        );

        assert_eq!(
            activity.source_excerpt,
            "Conceito: <script>alert(1)</script> e [link](https://example.test)"
        );
        assert!(!activity.prompt.contains("<script>"));
    }

    #[test]
    fn source_revision_hashes_exact_markdown_bytes() {
        assert_eq!(
            super::hash_section_markdown("ação\n"),
            "ada323d45a144db50e2fbb47c5744fc4cbcb9ab393e42f63ce91913eeea74489"
        );
    }

    #[test]
    fn serializes_frontend_contract_with_camel_case_and_snake_case_values() {
        let activity = build_tutor_activity(
            &section("Entropia: medida da incerteza.", Some(4)),
            "session-1",
        );
        let value = serde_json::to_value(activity).unwrap();

        assert_eq!(value["sessionId"], "session-1");
        assert_eq!(value["sectionIndex"], 2);
        assert_eq!(value["sourcePage"], 4);
        assert_eq!(value["kind"], "verifiable");
    }

    #[test]
    fn section_input_derives_excerpt_revision_and_heading_from_markdown() {
        let markdown = "## Página 9\n".to_owned() + &"x".repeat(700);
        let section = super::TutorSectionInput {
            index: 0,
            title: "Página 9".into(),
            markdown: markdown.clone(),
            source_page: Some(9),
        }
        .into_lesson_section();

        assert_eq!(section.source_heading, "Página 9");
        assert_eq!(
            section.source_excerpt,
            markdown.chars().take(600).collect::<String>()
        );
        assert_eq!(
            section.source_sha256,
            super::hash_section_markdown(&markdown)
        );
    }

    #[test]
    fn activity_id_stays_stable_when_markdown_revision_changes() {
        let first = build_tutor_activity(&section("Conceito: texto inicial.", Some(4)), "session");
        let second =
            build_tutor_activity(&section("Conceito: texto alterado.", Some(4)), "session");

        assert_eq!(first.id, second.id);
        assert_ne!(first.source_sha256, second.source_sha256);
    }
}
