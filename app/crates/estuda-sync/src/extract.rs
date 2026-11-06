//! Extração determinística de HTML para Markdown estruturado.
//!
//! Preserva títulos, parágrafos, listas, tabelas, código, citações, links e fórmulas.
//! **Não gera imagens e não usa modelo de linguagem**: o que não pode ser representado como
//! texto vira uma *limitação* registrada no documento, nunca uma interpretação inventada.

use scraper::{ElementRef, Html, Selector};

use crate::source::strip_query_and_fragment;
use crate::text::sha256_hex;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Extracted {
    pub body: String,
    pub limitations: Vec<String>,
    /// Hash só do corpo (sem cabeçalho de fonte/data): estável para detectar mudança de conteúdo.
    pub body_hash: String,
}

const MAX_LIMITATIONS: usize = 50;

pub fn html_to_markdown(html: &str) -> Extracted {
    let doc = Html::parse_fragment(html);
    let mut r = Renderer::default();
    r.children(doc.root_element());
    let (body, limitations) = r.finish();
    let body_hash = sha256_hex(body.as_bytes());
    Extracted {
        body,
        limitations,
        body_hash,
    }
}

/// Documento completo com identificação da fonte e limitações. `section` é a página/seção de
/// origem quando houver (ex.: "Módulo 2 · Unidade 3").
pub fn render_document(
    title: &str,
    source_url: Option<&str>,
    section: Option<&str>,
    read_at: Option<&str>,
    extracted: &Extracted,
) -> String {
    let mut out = format!("# {}\n\n", title.trim());
    if let Some(url) = source_url {
        out.push_str(&format!("> Fonte: {}\n", strip_query_and_fragment(url)));
    }
    if let Some(s) = section {
        out.push_str(&format!("> Seção de origem: {s}\n"));
    }
    if let Some(d) = read_at {
        out.push_str(&format!("> Lido em: {d}\n"));
    }
    out.push_str("> Extração: conversão determinística; nenhum trecho foi gerado ou completado por modelo.\n\n");
    out.push_str(&extracted.body);
    out.push('\n');
    if !extracted.limitations.is_empty() {
        out.push_str("\n## Limitações da extração\n\n");
        for l in &extracted.limitations {
            out.push_str(&format!("- {l}\n"));
        }
    }
    out
}

#[derive(Default)]
struct Renderer {
    out: String,
    limitations: Vec<String>,
    lists: Vec<ListState>,
    in_pre: bool,
}

struct ListState {
    ordered: bool,
    counter: usize,
}

impl Renderer {
    fn finish(mut self) -> (String, Vec<String>) {
        let trimmed = self.out.trim().to_owned();
        self.limitations.dedup();
        (trimmed, self.limitations)
    }

    fn limit(&mut self, message: String) {
        if self.limitations.len() < MAX_LIMITATIONS && !self.limitations.contains(&message) {
            self.limitations.push(message);
        }
    }

    fn at_line_start(&self) -> bool {
        self.out.is_empty() || self.out.ends_with('\n')
    }

    fn line_break(&mut self) {
        while self.out.ends_with(' ') {
            self.out.pop();
        }
        if !self.out.is_empty() && !self.out.ends_with('\n') {
            self.out.push('\n');
        }
    }

    fn block_break(&mut self) {
        self.line_break();
        if !self.out.is_empty() && !self.out.ends_with("\n\n") {
            self.out.push('\n');
        }
    }

    fn push_text(&mut self, raw: &str) {
        if self.in_pre {
            self.out.push_str(raw);
            return;
        }
        for ch in raw.chars() {
            if ch.is_whitespace() {
                if !self.at_line_start() && !self.out.ends_with(' ') {
                    self.out.push(' ');
                }
            } else {
                self.out.push(ch);
            }
        }
    }

    /// Renderiza `el` num renderizador isolado e devolve o texto inline em uma linha.
    fn inline_of(&mut self, el: ElementRef<'_>) -> String {
        let mut sub = Renderer::default();
        sub.children(el);
        let (text, limits) = sub.finish();
        for l in limits {
            self.limit(l);
        }
        text.split_whitespace().collect::<Vec<_>>().join(" ")
    }

    fn children(&mut self, el: ElementRef<'_>) {
        for child in el.children() {
            if let Some(text) = child.value().as_text() {
                self.push_text(text);
            } else if let Some(e) = ElementRef::wrap(child) {
                self.element(e);
            }
        }
    }

    fn element(&mut self, el: ElementRef<'_>) {
        let name = el.value().name();
        match name {
            "script" | "style" | "noscript" | "head" | "template" => {}
            "iframe" | "video" | "audio" | "embed" | "object" | "svg" | "canvas" => {
                let label = el.value().attr("title").unwrap_or("sem título");
                self.limit(format!("conteúdo incorporado omitido: <{name}> ({label})"));
            }
            "h1" | "h2" | "h3" | "h4" | "h5" | "h6" => {
                let level = usize::from(name.as_bytes()[1] - b'0');
                let text = self.inline_of(el);
                if !text.is_empty() {
                    self.block_break();
                    self.out.push_str(&format!("{} {text}", "#".repeat(level)));
                    self.block_break();
                }
            }
            "p" | "div" | "section" | "article" | "main" | "header" | "footer" | "figure"
            | "figcaption" | "dl" | "dt" | "dd" | "body" | "html" | "address" | "details"
            | "summary" => {
                self.block_break();
                self.children(el);
                self.block_break();
            }
            "br" => self.line_break(),
            "hr" => {
                self.block_break();
                self.out.push_str("---");
                self.block_break();
            }
            "ul" | "ol" => {
                if self.lists.is_empty() {
                    self.block_break();
                } else {
                    self.line_break();
                }
                self.lists.push(ListState {
                    ordered: name == "ol",
                    counter: 0,
                });
                self.children(el);
                self.lists.pop();
                if self.lists.is_empty() {
                    self.block_break();
                } else {
                    self.line_break();
                }
            }
            "li" => {
                self.line_break();
                let depth = self.lists.len().saturating_sub(1);
                let marker = match self.lists.last_mut() {
                    Some(l) if l.ordered => {
                        l.counter += 1;
                        format!("{}. ", l.counter)
                    }
                    _ => "- ".to_owned(),
                };
                self.out.push_str(&"  ".repeat(depth));
                self.out.push_str(&marker);
                self.children(el);
                self.line_break();
            }
            "blockquote" => {
                let mut sub = Renderer::default();
                sub.children(el);
                let (text, limits) = sub.finish();
                for l in limits {
                    self.limit(l);
                }
                if !text.is_empty() {
                    self.block_break();
                    for line in text.lines() {
                        self.out.push_str(if line.is_empty() { ">" } else { "> " });
                        self.out.push_str(line);
                        self.out.push('\n');
                    }
                    self.block_break();
                }
            }
            "pre" => {
                let text: String = el.text().collect();
                self.block_break();
                self.out.push_str("```\n");
                self.out.push_str(text.trim_matches('\n'));
                self.out.push_str("\n```");
                self.block_break();
            }
            "code" | "kbd" | "samp" | "tt" => {
                let text = self.inline_of(el);
                if !text.is_empty() {
                    self.out.push_str(&format!("`{text}`"));
                }
            }
            "strong" | "b" => self.wrap_inline(el, "**"),
            "em" | "i" => self.wrap_inline(el, "*"),
            "sup" => self.script_inline(el, "^"),
            "sub" => self.script_inline(el, "_"),
            "a" => self.link(el),
            "img" => self.image(el),
            "math" => self.math(el),
            "table" => self.table(el),
            _ => self.children(el),
        }
    }

    fn wrap_inline(&mut self, el: ElementRef<'_>, mark: &str) {
        let text = self.inline_of(el);
        if text.is_empty() {
            return;
        }
        if !self.at_line_start() && !self.out.ends_with(' ') && !self.out.ends_with('(') {
            self.out.push(' ');
        }
        self.out.push_str(&format!("{mark}{text}{mark}"));
    }

    fn script_inline(&mut self, el: ElementRef<'_>, mark: &str) {
        let text = self.inline_of(el);
        if !text.is_empty() {
            self.out.push_str(&format!("{mark}{{{text}}}"));
        }
    }

    fn link(&mut self, el: ElementRef<'_>) {
        let text = self.inline_of(el);
        if text.is_empty() {
            return;
        }
        let href = el.value().attr("href").map(str::trim).unwrap_or("");
        let safe = href.starts_with("https://")
            || href.starts_with("http://")
            || href.starts_with('/')
            || (!href.contains(':') && !href.is_empty() && !href.starts_with('#'));
        if !self.at_line_start() && !self.out.ends_with(' ') && !self.out.ends_with('(') {
            self.out.push(' ');
        }
        if safe {
            self.out
                .push_str(&format!("[{text}]({})", strip_query_and_fragment(href)));
        } else {
            self.out.push_str(&text);
        }
    }

    fn image(&mut self, el: ElementRef<'_>) {
        let v = el.value();
        if let Some(latex) = v
            .attr("data-equation-content")
            .map(str::trim)
            .filter(|s| !s.is_empty())
        {
            self.out.push_str(&format!("${latex}$"));
            return;
        }
        match v.attr("alt").map(str::trim).filter(|s| !s.is_empty()) {
            Some(alt) => {
                self.out.push_str(&format!("[Imagem: {alt}]"));
                self.limit("imagem representada só pelo texto alternativo; o conteúdo visual não foi extraído".into());
            }
            None => self
                .limit("imagem sem texto alternativo omitida; conteúdo visual não extraído".into()),
        }
    }

    fn math(&mut self, el: ElementRef<'_>) {
        let tex = Selector::parse("annotation").ok().and_then(|sel| {
            el.select(&sel)
                .find(|a| {
                    a.value()
                        .attr("encoding")
                        .is_some_and(|e| e.to_ascii_lowercase().contains("tex"))
                })
                .map(|a| a.text().collect::<String>())
        });
        match tex.map(|t| t.trim().to_owned()).filter(|t| !t.is_empty()) {
            Some(t) => self.out.push_str(&format!("${t}$")),
            None => {
                let plain = el
                    .text()
                    .collect::<String>()
                    .split_whitespace()
                    .collect::<Vec<_>>()
                    .join(" ");
                if !plain.is_empty() {
                    self.out.push_str(&plain);
                }
                self.limit("fórmula MathML sem LaTeX: apenas o texto plano foi preservado".into());
            }
        }
    }

    fn table(&mut self, table: ElementRef<'_>) {
        let (Ok(tr), Ok(cell)) = (Selector::parse("tr"), Selector::parse("th, td")) else {
            return;
        };
        let mut rows: Vec<Vec<String>> = Vec::new();
        let mut header_from_th = false;
        for (i, row) in table.select(&tr).enumerate() {
            let mut cells = Vec::new();
            for c in row.select(&cell) {
                let v = c.value();
                let span = |a: &str| v.attr(a).and_then(|s| s.parse::<usize>().ok()).unwrap_or(1);
                if span("colspan") > 1 || span("rowspan") > 1 {
                    self.limit("tabela com células mescladas (colspan/rowspan): a estrutura foi aproximada".into());
                }
                if i == 0 && v.name() == "th" {
                    header_from_th = true;
                }
                cells.push(self.inline_of(c).replace('|', "\\|"));
            }
            if !cells.is_empty() {
                rows.push(cells);
            }
        }
        if rows.is_empty() {
            return;
        }
        let width = rows.iter().map(Vec::len).max().unwrap_or(0);
        for r in &mut rows {
            r.resize(width, String::new());
        }
        if !header_from_th {
            self.limit(
                "tabela sem linha de cabeçalho: a primeira linha foi usada como cabeçalho".into(),
            );
        }
        self.block_break();
        let line = |cells: &[String]| format!("| {} |", cells.join(" | "));
        self.out.push_str(&line(&rows[0]));
        self.out.push('\n');
        self.out
            .push_str(&format!("|{}|", vec![" --- "; width].join("|")));
        self.out.push('\n');
        for r in rows.iter().skip(1) {
            self.out.push_str(&line(r));
            self.out.push('\n');
        }
        self.block_break();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn md(html: &str) -> Extracted {
        html_to_markdown(html)
    }

    #[test]
    fn headings_paragraphs_and_inline_marks() {
        let e = md(
            "<h1>Aula 1</h1><p>Texto com <strong>negrito</strong> e <em>ênfase</em> e <code>x = 1</code>.</p><h2>Seção</h2><p>Outro.</p>",
        );
        assert_eq!(
            e.body,
            "# Aula 1\n\nTexto com **negrito** e *ênfase* e `x = 1`.\n\n## Seção\n\nOutro."
        );
        assert!(e.limitations.is_empty());
    }

    #[test]
    fn nested_and_ordered_lists_keep_hierarchy() {
        let e = md(
            "<ul><li>A<ul><li>A1</li><li>A2</li></ul></li><li>B</li></ul><ol><li>um</li><li>dois</li></ol>",
        );
        assert_eq!(e.body, "- A\n  - A1\n  - A2\n- B\n\n1. um\n2. dois");
    }

    #[test]
    fn tables_become_pipe_tables() {
        let e = md(
            "<table><tr><th>Prova</th><th>Peso</th></tr><tr><td>A1</td><td>4</td></tr><tr><td>A2 | A3</td><td>6</td></tr></table>",
        );
        assert_eq!(
            e.body,
            "| Prova | Peso |\n| --- | --- |\n| A1 | 4 |\n| A2 \\| A3 | 6 |"
        );
        assert!(e.limitations.is_empty());
    }

    #[test]
    fn merged_cells_and_missing_header_are_recorded_as_limitations() {
        let e =
            md("<table><tr><td colspan=\"2\">Todos</td></tr><tr><td>a</td><td>b</td></tr></table>");
        assert!(e.limitations.iter().any(|l| l.contains("mescladas")));
        assert!(
            e.limitations
                .iter()
                .any(|l| l.contains("sem linha de cabeçalho"))
        );
    }

    #[test]
    fn formulas_are_preserved_as_latex_when_available() {
        let e = md(
            r#"<p>Área: <img class="equation_image" data-equation-content="\pi r^2" alt="pi r ao quadrado"> e
            <math><semantics><mi>x</mi><annotation encoding="application/x-tex">x^2+1</annotation></semantics></math>.</p>"#,
        );
        assert_eq!(e.body, "Área: $\\pi r^2$ e $x^2+1$.");
        assert!(e.limitations.is_empty());
    }

    #[test]
    fn mathml_without_latex_and_plain_images_are_limitations_never_invented() {
        let e = md(
            "<p><math><mi>a</mi><mo>+</mo><mi>b</mi></math></p><img src='https://x/y.png?token=1'><img alt='Diagrama do fluxo' src='z.png'>",
        );
        assert!(e.body.contains("a + b") || e.body.contains("a+b") || e.body.contains("a b"));
        assert!(e.body.contains("[Imagem: Diagrama do fluxo]"));
        assert!(e.limitations.iter().any(|l| l.contains("MathML sem LaTeX")));
        assert!(
            e.limitations
                .iter()
                .any(|l| l.contains("sem texto alternativo"))
        );
        assert!(!e.body.contains("token"));
        assert!(!e.limitations.iter().any(|l| l.contains("token")));
    }

    #[test]
    fn links_lose_query_and_dangerous_schemes_are_dropped() {
        let e = md(
            r#"<p><a href="https://ead.uva.br/a.pdf?download_token=x#p2">PDF</a> <a href="javascript:alert(1)">mau</a></p>"#,
        );
        assert_eq!(e.body, "[PDF](https://ead.uva.br/a.pdf) mau");
    }

    #[test]
    fn code_blocks_and_quotes() {
        let e = md(
            "<pre>fn main() {\n    println!(\"oi\");\n}\n</pre><blockquote><p>Citação</p><p>Segunda</p></blockquote>",
        );
        assert_eq!(
            e.body,
            "```\nfn main() {\n    println!(\"oi\");\n}\n```\n\n> Citação\n>\n> Segunda"
        );
    }

    #[test]
    fn scripts_styles_and_embeds_are_dropped_with_limitation() {
        let e = md(
            "<p>Texto</p><script>evil()</script><style>p{}</style><iframe title=\"Vídeo da aula\" src=\"https://v/1\"></iframe>",
        );
        assert_eq!(e.body, "Texto");
        assert_eq!(
            e.limitations,
            ["conteúdo incorporado omitido: <iframe> (Vídeo da aula)"]
        );
    }

    #[test]
    fn whitespace_is_collapsed_and_extraction_is_deterministic() {
        let html = "<div>\n  <p>  muitos \n  espaços   aqui </p>\n</div>";
        let a = md(html);
        assert_eq!(a.body, "muitos espaços aqui");
        assert_eq!(a, md(html));
        assert_ne!(a.body_hash, md("<p>outro</p>").body_hash);
    }

    #[test]
    fn document_identifies_source_and_lists_limitations() {
        let e = md("<p>Oi</p><iframe title=\"v\"></iframe>");
        let doc = render_document(
            "Aula 1",
            Some("https://uva.instructure.com/courses/1/pages/a?x=1"),
            Some("Módulo 2"),
            Some("2026-11-06"),
            &e,
        );
        assert!(
            doc.starts_with("# Aula 1\n\n> Fonte: https://uva.instructure.com/courses/1/pages/a\n")
        );
        assert!(doc.contains("> Seção de origem: Módulo 2"));
        assert!(doc.contains("nenhum trecho foi gerado ou completado por modelo"));
        assert!(doc.contains("## Limitações da extração\n\n- conteúdo incorporado omitido"));
        assert!(!doc.contains("x=1"));
    }
}
