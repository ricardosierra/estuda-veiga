//! Fronteira privado → compartilhável.
//!
//! Fluxo (cada etapa é um tipo; não há atalho para o próximo):
//!
//! ```text
//! Discovered → OriginVerified → Sanitized → IntegrityChecked → (revisão humana) → PublicRecord
//! ```
//!
//! Só [`PublicRecord`] pode entrar em [`crate::store::PublicCache`]. Uma resposta individual
//! enviada ao Canvas **nunca** vira solução pública automaticamente, e prova restrita ou
//! atividade ainda não liberada é barrada na primeira etapa.

use std::collections::HashSet;

use estuda_domain::publication::{PublicationPolicy, Violation, check_publishable};
use thiserror::Error;

use crate::source::strip_query_and_fragment;
use crate::text::{fold, sha256_hex};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PrivateFinding {
    Cpf,
    Email,
    /// Rótulo de matrícula/RA/nome do aluno/CPF seguido de valor.
    IdentityLabel,
    /// "Nota: 8,5", "Frequência: 100%" e similares: dado individual.
    PersonalResult,
    /// Cookie, Authorization ou Bearer.
    Credential,
    /// URL com query que parece carregar token/assinatura/SSO.
    TokenInUrl,
}

const MAX_TEXT_BYTES: usize = 5 * 1024 * 1024;
const IDENTITY_LABELS: &[&str] = &[
    "matricula",
    "ra",
    "registro academico",
    "cpf",
    "rg",
    "nome do aluno",
    "nome do estudante",
    "aluno",
    "estudante",
    "nome completo",
];
const RESULT_LABELS: &[&str] = &[
    "nota",
    "nota final",
    "media final",
    "minha nota",
    "sua nota",
    "frequencia",
    "faltas",
    "cr",
    "coeficiente de rendimento",
];
const TOKEN_KEYS: &[&str] = &[
    "token",
    "access_token",
    "signature",
    "sig",
    "key",
    "auth",
    "session",
    "jwt",
    "ticket",
    "sso",
    "x-amz-signature",
    "x-amz-credential",
    "verifier",
    "download_token",
];

// ---------------------------------------------------------------------------------------
// Varredura
// ---------------------------------------------------------------------------------------

fn cpf_spans(text: &str) -> Vec<(usize, usize)> {
    let chars: Vec<(usize, char)> = text.char_indices().collect();
    let mut spans = Vec::new();
    let mut i = 0;
    while i < chars.len() {
        if !chars[i].1.is_ascii_digit() || (i > 0 && chars[i - 1].1.is_ascii_digit()) {
            i += 1;
            continue;
        }
        let mut run = 0;
        while i + run < chars.len() && chars[i + run].1.is_ascii_digit() {
            run += 1;
        }
        let end_of = |n: usize| chars.get(i + n).map_or(text.len(), |c| c.0);
        let formatted = |n: usize| -> bool {
            const SHAPE: &str = "ddd.ddd.ddd-dd";
            n >= 14
                && SHAPE.chars().enumerate().all(|(k, s)| {
                    let c = chars.get(i + k).map(|c| c.1).unwrap_or(' ');
                    if s == 'd' { c.is_ascii_digit() } else { c == s }
                })
                && !chars.get(i + 14).is_some_and(|c| c.1.is_ascii_digit())
        };
        if run == 3 && formatted(14) {
            spans.push((chars[i].0, end_of(14)));
            i += 14;
        } else if run == 11 {
            spans.push((chars[i].0, end_of(11)));
            i += 11;
        } else {
            i += run;
        }
    }
    spans
}

/// Tokens separados por espaço, com deslocamento em bytes.
fn tokens(text: &str) -> Vec<(usize, &str)> {
    let mut out = Vec::new();
    let mut start: Option<usize> = None;
    for (idx, ch) in text.char_indices() {
        match (ch.is_whitespace(), start) {
            (false, None) => start = Some(idx),
            (true, Some(s)) => {
                out.push((s, &text[s..idx]));
                start = None;
            }
            _ => {}
        }
    }
    if let Some(s) = start {
        out.push((s, &text[s..]));
    }
    out
}

const TRIM: &[char] = &['<', '>', '(', ')', '[', ']', ',', ';', ':', '"', '\'', '.'];

fn email_spans(text: &str) -> Vec<(usize, usize)> {
    tokens(text)
        .into_iter()
        .filter_map(|(offset, token)| {
            let lead = token.len() - token.trim_start_matches(TRIM).len();
            let inner = token.trim_matches(TRIM);
            let (local, domain) = inner.split_once('@')?;
            let ok = !local.is_empty()
                && !domain.contains('@')
                && domain.contains('.')
                && !domain.starts_with('.')
                && !domain.ends_with('.')
                && !token.starts_with("http");
            ok.then_some((offset + lead, offset + lead + inner.len()))
        })
        .collect()
}

/// Posição do rótulo (em `folded`) quando é palavra inteira seguida de `:` ou `=`.
fn label_with_value<'a>(folded: &'a str, labels: &[&str]) -> Option<&'a str> {
    for label in labels {
        let mut from = 0;
        while let Some(pos) = folded[from..].find(label) {
            let at = from + pos;
            let before_ok = !folded[..at]
                .chars()
                .next_back()
                .is_some_and(char::is_alphanumeric);
            let rest = &folded[at + label.len()..];
            let after_ok = !rest.chars().next().is_some_and(char::is_alphanumeric);
            if before_ok && after_ok {
                let rest = rest.trim_start();
                if let Some(value) = rest.strip_prefix(':').or_else(|| rest.strip_prefix('=')) {
                    return Some(value.trim_start());
                }
            }
            from = at + label.len();
        }
    }
    None
}

fn url_ranges(text: &str) -> Vec<(usize, usize)> {
    let mut out = Vec::new();
    let mut from = 0;
    while let Some(pos) = text[from..].find("http") {
        let start = from + pos;
        let rest = &text[start..];
        if !(rest.starts_with("http://") || rest.starts_with("https://")) {
            from = start + 4;
            continue;
        }
        let len = rest
            .find(|c: char| c.is_whitespace() || matches!(c, ')' | ']' | '>' | '"' | '\''))
            .unwrap_or(rest.len());
        out.push((start, start + len));
        from = start + len;
    }
    out
}

fn url_has_token_query(url: &str) -> bool {
    let Some((_, query)) = url.split_once('?') else {
        return false;
    };
    let query = query.split('#').next().unwrap_or("");
    query
        .split('&')
        .filter_map(|pair| pair.split('=').next())
        .any(|key| TOKEN_KEYS.contains(&key.to_ascii_lowercase().as_str()))
}

pub fn scan(text: &str) -> Vec<PrivateFinding> {
    let mut found: HashSet<PrivateFinding> = HashSet::new();
    if !cpf_spans(text).is_empty() {
        found.insert(PrivateFinding::Cpf);
    }
    if !email_spans(text).is_empty() {
        found.insert(PrivateFinding::Email);
    }
    if url_ranges(text)
        .iter()
        .any(|(s, e)| url_has_token_query(&text[*s..*e]))
    {
        found.insert(PrivateFinding::TokenInUrl);
    }
    for line in text.lines() {
        let folded = fold(line);
        if is_credential_line(&folded) {
            found.insert(PrivateFinding::Credential);
        }
        if label_with_value(&folded, IDENTITY_LABELS).is_some_and(|v| !v.is_empty()) {
            found.insert(PrivateFinding::IdentityLabel);
        }
        if label_with_value(&folded, RESULT_LABELS)
            .is_some_and(|v| v.chars().next().is_some_and(|c| c.is_ascii_digit()))
        {
            found.insert(PrivateFinding::PersonalResult);
        }
    }
    let mut v: Vec<_> = found.into_iter().collect();
    v.sort_by_key(|f| format!("{f:?}"));
    v
}

fn is_credential_line(folded_line: &str) -> bool {
    folded_line.contains("cookie:")
        || folded_line.contains("set-cookie")
        || folded_line.contains("authorization:")
        || folded_line.contains("bearer ")
}

/// Remoção **mecânica**: CPF, e-mail, query/fragmento de URL e linhas de credencial.
/// Rótulos de identidade e resultados pessoais não são removidos às cegas: bloqueiam.
pub fn redact_mechanical(text: &str) -> (String, usize) {
    let mut count = 0;
    let kept: Vec<&str> = text
        .lines()
        .filter(|l| {
            let drop = is_credential_line(&fold(l));
            count += usize::from(drop);
            !drop
        })
        .collect();
    let mut out = kept.join("\n");

    let mut edits: Vec<(usize, usize, &str)> = Vec::new();
    edits.extend(
        cpf_spans(&out)
            .into_iter()
            .map(|(s, e)| (s, e, "[CPF removido]")),
    );
    edits.extend(
        email_spans(&out)
            .into_iter()
            .map(|(s, e)| (s, e, "[e-mail removido]")),
    );
    for (s, e) in url_ranges(&out) {
        let url = &out[s..e];
        let cut = strip_query_and_fragment(url).len();
        if cut < url.len() {
            edits.push((s + cut, e, ""));
        }
    }
    edits.sort_by_key(|e| std::cmp::Reverse(e.0));
    edits.dedup_by(|a, b| a.0 < b.1 && a.1 > b.0); // sobreposição: mantém a de maior início
    for (s, e, with) in edits {
        out.replace_range(s..e, with);
        count += 1;
    }
    (out, count)
}

// ---------------------------------------------------------------------------------------
// Pipeline de publicação
// ---------------------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ContentOrigin {
    /// Publicação oficial liberada para distribuição.
    OfficialPublication,
    CourseMaterial,
    /// Resposta individual enviada pelo aluno ao Canvas.
    StudentSubmission,
    /// Prova restrita ou atividade avaliativa ainda não liberada.
    RestrictedAssessment,
    /// Solução oferecida voluntariamente por um aluno; exige consentimento registrado.
    VoluntarySolution {
        consent: bool,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum License {
    Unverified,
    Verified { note: String },
    Denied,
}

#[derive(Debug, Clone)]
pub struct Candidate {
    pub id: String,
    pub title: String,
    pub origin: ContentOrigin,
    pub license: License,
    pub source_url: Option<String>,
    pub text: String,
    /// Hash do texto no momento da extração, para detectar alteração antes da revisão.
    pub declared_hash: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum PublishBlock {
    #[error("resposta individual do aluno não é publicável")]
    StudentSubmission,
    #[error("prova restrita ou atividade ainda não liberada")]
    RestrictedAssessment,
    #[error("solução voluntária sem consentimento registrado")]
    NoConsent,
    #[error("licença/condições de uso não verificadas")]
    LicenseUnverified,
    #[error("licença negada")]
    LicenseDenied,
    #[error("origem sem URL de fonte")]
    MissingSource,
    #[error("conteúdo diferente do hash declarado na extração")]
    IntegrityMismatch,
    #[error("conteúdo vazio, grande demais ou com byte nulo")]
    InvalidContent,
    #[error("dados pessoais exigem redação manual: {0:?}")]
    NeedsManualRedaction(Vec<PrivateFinding>),
    #[error("revisor não identificado")]
    NoReviewer,
    #[error("dados privados ainda presentes na etapa final: {0:?}")]
    PrivateDataRemains(Vec<PrivateFinding>),
}

pub struct Discovered(Candidate);
pub struct OriginVerified(Candidate);
pub struct Sanitized {
    candidate: Candidate,
    redactions: usize,
}
pub struct IntegrityChecked {
    candidate: Candidate,
    content_hash: String,
}

impl Discovered {
    pub fn new(candidate: Candidate) -> Self {
        Self(candidate)
    }

    pub fn verify_origin(self) -> Result<OriginVerified, PublishBlock> {
        let c = self.0;
        match c.origin {
            ContentOrigin::StudentSubmission => return Err(PublishBlock::StudentSubmission),
            ContentOrigin::RestrictedAssessment => return Err(PublishBlock::RestrictedAssessment),
            ContentOrigin::VoluntarySolution { consent: false } => {
                return Err(PublishBlock::NoConsent);
            }
            _ => {}
        }
        match c.license {
            License::Unverified => return Err(PublishBlock::LicenseUnverified),
            License::Denied => return Err(PublishBlock::LicenseDenied),
            License::Verified { .. } => {}
        }
        if c.source_url.as_deref().is_none_or(|u| u.trim().is_empty()) {
            return Err(PublishBlock::MissingSource);
        }
        Ok(OriginVerified(c))
    }
}

impl OriginVerified {
    pub fn remove_personal_data(self) -> Result<Sanitized, PublishBlock> {
        let mut c = self.0;
        if let Some(expected) = &c.declared_hash
            && *expected != sha256_hex(c.text.as_bytes())
        {
            return Err(PublishBlock::IntegrityMismatch);
        }
        let (text, mut redactions) = redact_mechanical(&c.text);
        let (title, title_redactions) = redact_mechanical(&c.title);
        redactions += title_redactions;
        c.text = text;
        c.title = title;
        c.source_url = c.source_url.map(|u| strip_query_and_fragment(&u));
        let remaining: Vec<PrivateFinding> = scan(&c.text)
            .into_iter()
            .chain(scan(&c.title))
            .collect::<HashSet<_>>()
            .into_iter()
            .collect();
        if !remaining.is_empty() {
            return Err(PublishBlock::NeedsManualRedaction(remaining));
        }
        Ok(Sanitized {
            candidate: c,
            redactions,
        })
    }
}

impl Sanitized {
    pub fn redactions(&self) -> usize {
        self.redactions
    }

    pub fn check_integrity(self) -> Result<IntegrityChecked, PublishBlock> {
        let c = self.candidate;
        if c.text.trim().is_empty() || c.text.len() > MAX_TEXT_BYTES || c.text.contains('\0') {
            return Err(PublishBlock::InvalidContent);
        }
        let content_hash = sha256_hex(c.text.as_bytes());
        Ok(IntegrityChecked {
            candidate: c,
            content_hash,
        })
    }
}

impl IntegrityChecked {
    pub fn content_hash(&self) -> &str {
        &self.content_hash
    }

    /// Revisão humana: sem revisor identificado não há publicação. Reexecuta a varredura
    /// como última defesa.
    pub fn approve(self, reviewer: &str) -> Result<PublicRecord, PublishBlock> {
        if reviewer.trim().is_empty() {
            return Err(PublishBlock::NoReviewer);
        }
        let c = self.candidate;
        let leaks = scan(&c.text);
        if !leaks.is_empty() {
            return Err(PublishBlock::PrivateDataRemains(leaks));
        }
        Ok(PublicRecord {
            id: c.id,
            title: c.title,
            source_url: c.source_url.unwrap_or_default(),
            text: c.text,
            content_hash: self.content_hash,
            reviewed_by: reviewer.trim().to_owned(),
        })
    }
}

/// Registro aprovado para a biblioteca compartilhável. Campos privados: só o pipeline cria.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PublicRecord {
    id: String,
    title: String,
    source_url: String,
    text: String,
    content_hash: String,
    reviewed_by: String,
}

impl PublicRecord {
    pub fn id(&self) -> &str {
        &self.id
    }
    pub fn title(&self) -> &str {
        &self.title
    }
    pub fn source_url(&self) -> &str {
        &self.source_url
    }
    pub fn text(&self) -> &str {
        &self.text
    }
    pub fn content_hash(&self) -> &str {
        &self.content_hash
    }
    pub fn reviewed_by(&self) -> &str {
        &self.reviewed_by
    }
}

/// Barreira no nível de caminho, reaproveitando a classificação do domínio. Use
/// `PublicationPolicy::default()` para o catálogo público (pessoal não sai); `OWNER` só vale
/// para a publicação da camada pessoal, separada, que o dono do acervo autorizou. Banco,
/// perfil de navegador, segredos e binários originais nunca passam, em nenhuma política.
pub fn guard_paths(
    paths: &[&str],
    reviewed: &HashSet<String>,
    policy: PublicationPolicy,
) -> Result<(), Vec<Violation>> {
    let v = check_publishable(paths, reviewed, policy);
    if v.is_empty() { Ok(()) } else { Err(v) }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn candidate(text: &str) -> Candidate {
        Candidate {
            id: "mat-1".into(),
            title: "Aula 1".into(),
            origin: ContentOrigin::OfficialPublication,
            license: License::Verified {
                note: "material institucional liberado".into(),
            },
            source_url: Some("https://ead.uva.br/aula-1?token=segredo".into()),
            text: text.into(),
            declared_hash: None,
        }
    }

    fn run(c: Candidate) -> Result<PublicRecord, PublishBlock> {
        Discovered::new(c)
            .verify_origin()?
            .remove_personal_data()?
            .check_integrity()?
            .approve("revisor")
    }

    #[test]
    fn happy_path_strips_query_from_source_and_text() {
        let r = run(candidate(
            "Veja https://ead.uva.br/x.pdf?download_token=abc#p3 para detalhes.",
        ))
        .unwrap();
        assert_eq!(r.source_url(), "https://ead.uva.br/aula-1");
        assert_eq!(r.text(), "Veja https://ead.uva.br/x.pdf para detalhes.");
        assert!(!r.text().contains("token"));
        assert_eq!(r.reviewed_by(), "revisor");
    }

    #[test]
    fn student_submission_never_becomes_public() {
        let mut c = candidate("Minha resposta ao trabalho.");
        c.origin = ContentOrigin::StudentSubmission;
        assert_eq!(run(c).unwrap_err(), PublishBlock::StudentSubmission);
    }

    #[test]
    fn restricted_exams_and_unconsented_solutions_are_blocked() {
        let mut c = candidate("Prova A2");
        c.origin = ContentOrigin::RestrictedAssessment;
        assert_eq!(run(c).unwrap_err(), PublishBlock::RestrictedAssessment);
        let mut c = candidate("Solução");
        c.origin = ContentOrigin::VoluntarySolution { consent: false };
        assert_eq!(run(c).unwrap_err(), PublishBlock::NoConsent);
        let mut c = candidate("Solução");
        c.origin = ContentOrigin::VoluntarySolution { consent: true };
        assert!(run(c).is_ok());
    }

    #[test]
    fn license_and_source_are_required() {
        let mut c = candidate("x");
        c.license = License::Unverified;
        assert_eq!(run(c).unwrap_err(), PublishBlock::LicenseUnverified);
        let mut c = candidate("x");
        c.license = License::Denied;
        assert_eq!(run(c).unwrap_err(), PublishBlock::LicenseDenied);
        let mut c = candidate("x");
        c.source_url = None;
        assert_eq!(run(c).unwrap_err(), PublishBlock::MissingSource);
    }

    #[test]
    fn cpf_and_email_are_redacted_mechanically() {
        let r = run(candidate(
            "Contato: aluno@example.com e CPF 123.456.789-09 ou 12345678909.",
        ))
        .unwrap();
        assert_eq!(
            r.text(),
            "Contato: [e-mail removido] e CPF [CPF removido] ou [CPF removido]."
        );
    }

    #[test]
    fn credential_lines_are_dropped() {
        let r = run(candidate("Texto\nCookie: sessao=abc\nFim")).unwrap();
        assert_eq!(r.text(), "Texto\nFim");
    }

    #[test]
    fn identity_labels_and_personal_results_block_instead_of_guessing() {
        for text in [
            "Matrícula: 202610001",
            "Nome do aluno: Fulano",
            "Nota final: 8,5",
            "Frequência: 100%",
            "RA = 123",
        ] {
            assert!(
                matches!(
                    run(candidate(text)).unwrap_err(),
                    PublishBlock::NeedsManualRedaction(_)
                ),
                "{text}"
            );
        }
        // Mencionar a palavra sem valor individual não bloqueia.
        assert!(
            run(candidate(
                "A nota mínima é definida nos critérios. Frequência mínima é regra geral."
            ))
            .is_ok()
        );
    }

    #[test]
    fn content_changed_after_extraction_is_rejected() {
        let mut c = candidate("original");
        c.declared_hash = Some(sha256_hex(b"outro texto"));
        assert_eq!(run(c).unwrap_err(), PublishBlock::IntegrityMismatch);
        let mut c = candidate("original");
        c.declared_hash = Some(sha256_hex(b"original"));
        assert!(run(c).is_ok());
    }

    #[test]
    fn empty_content_and_missing_reviewer_are_rejected() {
        assert_eq!(
            run(candidate("   \n ")).unwrap_err(),
            PublishBlock::InvalidContent
        );
        let checked = Discovered::new(candidate("ok"))
            .verify_origin()
            .unwrap()
            .remove_personal_data()
            .unwrap()
            .check_integrity()
            .unwrap();
        assert_eq!(checked.approve("  ").unwrap_err(), PublishBlock::NoReviewer);
    }

    #[test]
    fn scan_reports_each_kind() {
        let s = scan("https://x.com/a?X-Amz-Signature=1\nBearer abc\nCPF: 111.222.333-44\nNota: 9");
        for f in [
            PrivateFinding::TokenInUrl,
            PrivateFinding::Credential,
            PrivateFinding::Cpf,
            PrivateFinding::IdentityLabel,
            PrivateFinding::PersonalResult,
        ] {
            assert!(s.contains(&f), "{f:?} em {s:?}");
        }
        assert!(scan("Texto de aula sem dados pessoais. 12345 e 2026.2").is_empty());
    }

    #[test]
    fn digits_that_are_not_cpf_are_left_alone() {
        assert!(scan("ISBN 9788575228074 e 123456789012").is_empty());
    }

    #[test]
    fn path_guard_reuses_domain_classification() {
        let reviewed = HashSet::new();
        let default = PublicationPolicy::default();
        assert!(guard_paths(&["docs/arquitetura/README.md"], &reviewed, default).is_ok());
        let personal = ["dados/notas/2026.2/x.json", "privado/PLANEJAMENTO.md"];
        assert_eq!(
            guard_paths(&personal, &reviewed, default)
                .unwrap_err()
                .len(),
            2
        );
        // A política do dono libera a camada pessoal separada, mas nunca banco/segredo.
        assert!(guard_paths(&personal, &reviewed, PublicationPolicy::OWNER).is_ok());
        let local = [
            "materias/catalogo/x.sqlite3",
            ".env",
            "app/perfil.sqlite3-wal",
        ];
        assert_eq!(
            guard_paths(&local, &reviewed, PublicationPolicy::OWNER)
                .unwrap_err()
                .len(),
            3
        );
    }
}
