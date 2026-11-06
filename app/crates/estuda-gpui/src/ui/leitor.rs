//! Tela Leitor PDF→MD: mostra a origem do documento e o Markdown lido do acervo.

use gpui_kit::gpui::{
    ClickEvent, Context, Div, FontWeight, Hsla, ParentElement, Styled, div, prelude::*, px,
};

use super::AppView;
use super::kit::{self, Size, Variant};
use crate::theme::{Palette, hexa};

/// Limite de linhas renderizadas (memória): o restante fica no arquivo.
const MAX_LINES: usize = 400;

/// Uma linha do Markdown já classificada para exibição.
struct Line {
    text: String,
    size: f32,
    weight: FontWeight,
    color: Hsla,
}

/// Classificação simples de Markdown em linhas de exibição (títulos, citações, listas).
fn parse_markdown(markdown: &str, p: &Palette) -> (Vec<Line>, bool) {
    let mut lines = Vec::new();
    let mut truncated = false;
    for raw in markdown.lines() {
        let trimmed = raw.trim_end();
        if trimmed.trim().is_empty() {
            continue;
        }
        if lines.len() >= MAX_LINES {
            truncated = true;
            break;
        }
        let line = if let Some(rest) = trimmed.strip_prefix("### ") {
            Line { text: rest.into(), size: 15., weight: FontWeight::BOLD, color: p.text }
        } else if let Some(rest) = trimmed.strip_prefix("## ") {
            Line { text: rest.into(), size: 16., weight: FontWeight::BOLD, color: p.text }
        } else if let Some(rest) = trimmed.strip_prefix("# ") {
            Line { text: rest.into(), size: 20., weight: FontWeight::BOLD, color: p.text }
        } else if let Some(rest) = trimmed.strip_prefix("> ") {
            Line { text: rest.into(), size: 13., weight: FontWeight::NORMAL, color: p.muted }
        } else if let Some(rest) = trimmed.strip_prefix("- ").or_else(|| trimmed.strip_prefix("* ")) {
            Line { text: format!("•  {rest}"), size: 14., weight: FontWeight::NORMAL, color: p.text }
        } else {
            Line { text: trimmed.into(), size: 14., weight: FontWeight::NORMAL, color: p.text }
        };
        lines.push(line);
    }
    (lines, truncated)
}

pub fn render(view: &AppView, p: &Palette, cx: &mut Context<AppView>) -> Div {
    let Some(doc) = view.doc.as_ref() else {
        return empty(view, p);
    };
    let document = &doc.document;
    let subject = document.course.clone().unwrap_or_else(|| "Sem disciplina identificada".into());
    let converting = view.conv < 100;
    let narrating = view.narrator.as_ref().is_some_and(|n| n.is_speaking());
    let conv_label = if converting {
        format!("convertendo {}%", view.conv)
    } else {
        "pronto".to_string()
    };

    let (lines, truncated) = parse_markdown(&doc.markdown, p);
    let total = lines.len();
    let shown = (total * view.conv as usize).div_ceil(100);

    // Painel da fonte: formato, tamanho, caminho relativo e avisos da leitura.
    let mut source = div()
        .relative()
        .overflow_hidden()
        .flex()
        .flex_col()
        .gap(px(10.))
        .p(px(28.))
        .rounded(px(14.))
        .bg(p.surface)
        .border_1()
        .border_color(p.border)
        .child(
            div()
                .text_size(px(11.))
                .font_weight(FontWeight::BOLD)
                .text_color(p.muted)
                .child(format!(
                    "{} · {} bytes · lido do acervo",
                    document.format.to_uppercase(),
                    document.size_bytes
                )),
        )
        .child(
            div()
                .text_size(px(13.))
                .text_color(p.text)
                .child(document.relative_path.clone()),
        );
    for warning in &doc.warnings {
        source = source.child(
            div()
                .text_size(px(12.))
                .line_height(px(18.))
                .text_color(p.muted)
                .child(warning.clone()),
        );
    }
    if converting {
        source = source.child(
            div()
                .absolute()
                .left(px(0.))
                .right(px(0.))
                .top(px(0.))
                .h(gpui_kit::gpui::relative(0.25))
                .bg(hexa(0x2e7ba82e)),
        );
    }

    let mut md = div()
        .flex()
        .flex_col()
        .gap(px(10.))
        .p(px(24.))
        .px(px(28.))
        .rounded(px(14.))
        .bg(p.card)
        .border_1()
        .border_color(p.border)
        .text_size(px(14.))
        .line_height(px(22.))
        .child(
            div()
                .text_size(px(11.))
                .font_weight(FontWeight::BOLD)
                .text_color(p.muted)
                .child(format!("Markdown · {conv_label}")),
        );
    for line in lines.into_iter().take(shown) {
        md = md.child(
            div()
                .text_size(px(line.size))
                .font_weight(line.weight)
                .text_color(line.color)
                .child(line.text),
        );
    }
    if truncated && shown == total {
        md = md.child(
            div()
                .text_size(px(12.))
                .text_color(p.muted)
                .child(format!("Exibindo as primeiras {MAX_LINES} linhas; o restante está no arquivo.")),
        );
    }

    div()
        .flex()
        .flex_col()
        .gap(px(16.))
        .child(toolbar(p, cx, document.title.clone(), format!("{subject} · {}", document.category), narrating, view.narration_note.clone()))
        .child(
            div()
                .flex()
                .gap(px(16.))
                .min_h(px(460.))
                .child(div().flex_1().min_w_0().child(source))
                .child(div().flex_1().min_w_0().child(md)),
        )
}

fn toolbar(p: &Palette, cx: &mut Context<AppView>, title: String, meta: String, narrating: bool, note: Option<String>) -> Div {
    div()
        .flex()
        .items_center()
        .gap(px(12.))
        .flex_wrap()
        .child(
            div()
                .text_size(px(15.))
                .font_weight(FontWeight::BOLD)
                .child(title),
        )
        .child(
            div()
                .text_size(px(12.))
                .text_color(p.muted)
                .child(meta),
        )
        .child(div().flex_1())
        .child(narration_controls(p, cx, narrating, note.clone()))
        .child(
            kit::button_ex(
                p,
                "convert-again",
                "Converter de novo",
                Variant::Outline,
                Size::Sm,
                Some("bolt"),
                None,
            )
            .on_click(cx.listener(|this, _: &ClickEvent, _, cx| {
                this.convert(cx);
            })),
        )
}

/// Estado sem documento aberto: orienta o aluno a escolher um item da Biblioteca.
fn empty(view: &AppView, p: &Palette) -> Div {
    div()
        .flex()
        .flex_col()
        .gap(px(10.))
        .p(px(28.))
        .rounded(px(14.))
        .border_1()
        .border_color(p.border)
        .child(
            div()
                .text_size(px(15.))
                .font_weight(FontWeight::BOLD)
                .child("Nenhum documento aberto"),
        )
        .child(
            div()
                .text_size(px(13.))
                .text_color(p.muted)
                .child("Abra um item em Biblioteca para ler o Markdown do acervo."),
        )
        .child(
            div()
                .text_size(px(12.))
                .text_color(p.muted)
                .child(view.library_status.clone()),
        )
}

/// Controles de narração: Ouvir, Pausar/Continuar e Parar, com o aviso do motor se houver.
fn narration_controls(p: &Palette, cx: &mut Context<AppView>, narrating: bool, note: Option<String>) -> Div {
    let mut row = div().flex().items_center().gap(px(8.));
    row = row.child(
        kit::button(p, "narrate", "Ouvir", Variant::Outline, Size::Sm, None).on_click(cx.listener(
            |this, _: &ClickEvent, _, cx| {
                this.narrate_doc();
                cx.notify();
            },
        )),
    );
    row = row.child(
        kit::button(p, "narrate-pause", "Pausar / continuar", Variant::Outline, Size::Sm, None)
            .on_click(cx.listener(|this, _: &ClickEvent, _, cx| {
                this.toggle_narration_pause();
                cx.notify();
            })),
    );
    row = row.child(
        kit::button(p, "narrate-stop", "Parar", Variant::Outline, Size::Sm, None).on_click(
            cx.listener(|this, _: &ClickEvent, _, cx| {
                this.stop_narration();
                cx.notify();
            }),
        ),
    );
    if narrating {
        row = row.child(div().text_size(px(12.)).text_color(p.muted).child("lendo…"));
    }
    if let Some(note) = note {
        row = row.child(div().text_size(px(12.)).text_color(p.muted).child(note));
    }
    row
}
