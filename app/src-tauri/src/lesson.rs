use crate::pdf::escape_markdown_text;
use serde::{Deserialize, Serialize};
use unicode_normalization::{UnicodeNormalization, char::is_combining_mark};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LessonQuestion {
    pub prompt: String,
    pub answer: String,
    pub source_page: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PracticePrompt {
    pub position: usize,
    pub prompt: String,
    pub source_page: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GradedAnswer {
    pub position: usize,
    pub correct: bool,
    pub expected: String,
    pub source_page: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PracticeResult {
    pub correct: u32,
    pub total: u32,
    pub percent: u8,
    pub answers: Vec<GradedAnswer>,
}

pub fn build_lesson(course_title: &str, pages: &[(u32, &str)]) -> String {
    generate_lesson(course_title, pages).0
}

pub fn generate_lesson(course_title: &str, pages: &[(u32, &str)]) -> (String, Vec<LessonQuestion>) {
    let mut output = format!(
        "# Aula guiada — {}\n\n> Roteiro determinístico, montado localmente a partir do texto extraído. Não é uma explicação gerada por IA. Confira cada ponto no material oficial.\n\n## Mapa do material\n\n",
        escape_markdown_text(course_title)
    );
    let mut questions = Vec::new();
    for (page_number, text) in pages {
        let lines = text
            .lines()
            .map(str::trim)
            .filter(|line| !line.is_empty())
            .collect::<Vec<_>>();
        let heading = lines
            .iter()
            .find(|line| line.starts_with('#'))
            .map(|line| line.trim_start_matches('#').trim())
            .or_else(|| lines.first().copied())
            .unwrap_or("Página sem texto");
        output.push_str(&format!(
            "- **Página {page_number}:** {}\n",
            escape_markdown_text(heading)
        ));
        let paragraph = lines
            .iter()
            .find(|line| line.len() > 35 && !line.starts_with('#'))
            .copied();
        if let Some(paragraph) = paragraph {
            output.push_str(&format!(
                "\n## Leitura guiada — página {page_number}\n\n{}\n\n",
                escape_markdown_text(paragraph)
            ));
        }
        for line in &lines {
            if let Some((term, definition)) = definition_in(line) {
                if !questions.iter().any(|question: &LessonQuestion| {
                    question.prompt == format!("O que o material chama de {}?", term)
                }) {
                    questions.push(LessonQuestion {
                        prompt: format!("O que o material chama de {}?", term),
                        answer: definition.to_owned(),
                        source_page: *page_number,
                    });
                }
            }
        }
    }
    output.push_str("\n## Prática de recuperação\n\n");
    if questions.is_empty() {
        output.push_str("O texto extraído não contém definições claras o bastante para perguntas automáticas. Faça uma leitura guiada: escolha um título do mapa, localize a definição na página correspondente e explique-a com suas palavras.\n");
    } else {
        for (index, question) in questions.iter().enumerate() {
            output.push_str(&format!(
                "{}. {} (página {})\n\n   **Resposta:** {}\n\n   **Fonte:** página {}.\n\n",
                index + 1,
                escape_markdown_text(&question.prompt),
                question.source_page,
                escape_markdown_text(&question.answer),
                question.source_page
            ));
        }
    }
    output.push_str("\n## Roteiro de estudo\n\n1. Leia as páginas na ordem do mapa.\n2. Feche o texto e tente recuperar os conceitos sem consultar.\n3. Compare sua resposta com a página indicada e registre dúvidas.\n");
    (output, questions)
}

pub fn score_answer(answer: &str, expected: &str) -> u8 {
    u8::from(normalize(answer) == normalize(expected) && !normalize(expected).is_empty())
}

pub fn score_answers(answers: &[String], expected: &[String]) -> (u32, u32, u8) {
    let total = expected.len().min(u32::MAX as usize) as u32;
    let earned = answers
        .iter()
        .zip(expected)
        .map(|(answer, expected)| score_answer(answer, expected) as u32)
        .sum::<u32>();
    let percent = if total == 0 {
        0
    } else {
        ((earned * 100) / total) as u8
    };
    (earned, total, percent)
}

pub fn grade_answers(answers: &[String], questions: &[LessonQuestion]) -> PracticeResult {
    let expected = questions
        .iter()
        .map(|question| question.answer.clone())
        .collect::<Vec<_>>();
    let (correct, total, percent) = score_answers(answers, &expected);
    let grades = questions
        .iter()
        .enumerate()
        .map(|(position, question)| GradedAnswer {
            position,
            correct: answers
                .get(position)
                .is_some_and(|answer| score_answer(answer, &question.answer) == 1),
            expected: question.answer.clone(),
            source_page: question.source_page,
        })
        .collect();
    PracticeResult {
        correct,
        total,
        percent,
        answers: grades,
    }
}

fn normalize(value: &str) -> String {
    value
        .nfd()
        .filter(|character| !is_combining_mark(*character))
        .flat_map(char::to_lowercase)
        .filter(|character| character.is_alphanumeric())
        .collect::<String>()
}

pub(crate) fn definition_in(line: &str) -> Option<(&str, &str)> {
    let text = line.trim().trim_start_matches(['#', '-', '*', ' ']);
    for marker in [
        " significa ",
        " é definido como ",
        " é definida como ",
        ": ",
    ] {
        if let Some((term, definition)) = text.split_once(marker) {
            let term = term.trim().trim_end_matches(['.', ':']);
            let definition = definition
                .split_once(". ")
                .map(|(first_sentence, _)| first_sentence)
                .unwrap_or(definition)
                .trim()
                .trim_end_matches(['.', ';']);
            if term.len() >= 2 && definition.len() >= 3 {
                return Some((term, definition));
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::build_lesson;

    #[test]
    fn source_markdown_links_remain_plain_text_in_the_lesson() {
        let lesson = build_lesson(
            "Compiladores [material](https://example.test/course)",
            &[(
                1,
                "Conceito significa [abrir](https://example.test/material). Esta segunda frase completa o trecho.",
            )],
        );

        assert!(!lesson.contains("](https://example.test/"));
        assert!(!lesson.contains("![abrir]"));
    }
}
