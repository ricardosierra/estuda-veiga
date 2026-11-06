//! Tela Leitor PDF→MD: pré-visualização do PDF e do Markdown convertido.

use gpui_kit::gpui::{
    ClickEvent, Context, Div, FontWeight, Hsla, InteractiveElement, ParentElement, Styled, div,
    prelude::*, px,
};

use super::AppView;
use super::data::DOCS;
use super::kit::{self, Size, Variant};
use crate::theme::{Palette, hexa};

/// Linhas do Markdown gerado: (texto, tamanho, peso, cor).
fn markdown_lines(view: &AppView, p: &Palette) -> Vec<(String, f32, FontWeight, Hsla)> {
    let (title, _, _, src, _, _) = DOCS[view.doc_idx.min(DOCS.len() - 1)];
    let lines = vec![
        (format!("# {title}"), 20., FontWeight::BOLD, p.text),
        (format!("> Fonte: {src} · lido em [data da leitura]"), 13., FontWeight::NORMAL, p.muted),
        ("## Conteúdo".to_string(), 16., FontWeight::BOLD, p.text),
        ("Texto extraído página a página e normalizado em Markdown.".to_string(), 14., FontWeight::NORMAL, p.text),
        ("## Avaliações citadas".to_string(), 16., FontWeight::BOLD, p.text),
        ("- Campos sem fonte oficial ficam como “desconhecido”.".to_string(), 14., FontWeight::NORMAL, p.text),
        ("- Estimativas e material didático ficam separados do dado confirmado.".to_string(), 14., FontWeight::NORMAL, p.text),
    ];
    let shown = (lines.len() * view.conv as usize).div_ceil(100);
    lines.into_iter().take(shown).collect()
}

pub fn render(view: &AppView, p: &Palette, cx: &mut Context<AppView>) -> Div {
    let (title, _, subj, src, _, _) = DOCS[view.doc_idx.min(DOCS.len() - 1)];
    let converting = view.conv < 100;
    let conv_label = if converting {
        format!("convertendo {}%", view.conv)
    } else {
        "pronto".to_string()
    };

    // Esqueleto do PDF: barras de texto como no layout original.
    let bars: [(f32, f32, bool); 8] = [
        (14., 60., false),
        (8., 92., true),
        (8., 86., true),
        (8., 90., true),
        (10., 40., false),
        (8., 88., true),
        (8., 72., true),
        (8., 80., true),
    ];
    let mut pdf = div()
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
                .child("PDF · extraído em memória, descartado após conversão"),
        );
    for (h, w, faded) in bars {
        pdf = pdf.child(
            div()
                .h(px(h))
                .w(gpui_kit::gpui::relative(w / 100.))
                .rounded(px(4.))
                .bg(p.border)
                .opacity(if faded { 0.7 } else { 1.0 }),
        );
    }
    if converting {
        pdf = pdf.child(
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
    for (text, size, weight, color) in markdown_lines(view, p) {
        md = md.child(
            div()
                .text_size(px(size))
                .font_weight(weight)
                .text_color(color)
                .child(text),
        );
    }

    div()
        .flex()
        .flex_col()
        .gap(px(16.))
        .child(
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
                        .child(format!("{subj} · {src}")),
                )
                .child(div().flex_1())
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
                ),
        )
        .child(
            div()
                .flex()
                .gap(px(16.))
                .min_h(px(460.))
                .child(div().flex_1().min_w_0().child(pdf))
                .child(div().flex_1().min_w_0().child(md)),
        )
}
