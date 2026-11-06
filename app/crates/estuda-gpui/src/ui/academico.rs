//! Tela Acadêmico: tabela de disciplinas e detalhe da disciplina selecionada.

use gpui_kit::gpui::{
    ClickEvent, Context, Div, FontWeight, InteractiveElement, ParentElement, Styled, div,
    prelude::*, px, relative,
};

use super::AppView;
use super::data::SUBJECTS;
use super::kit::card;
use crate::theme::{Palette, hexa};

/// Coluna com proporção fixa dentro de uma linha (equivale a `minmax(0, Nfr)`).
fn col(fraction: f32) -> Div {
    div()
        .flex_grow(1.)
        .flex_shrink(1.)
        .flex_basis(relative(fraction))
        .min_w_0()
}

pub fn render(view: &AppView, p: &Palette, cx: &mut Context<AppView>) -> Div {
    div()
        .flex()
        .flex_wrap()
        .items_start()
        .gap(px(20.))
        .child(table(view, p, cx))
        .child(detail(view, p))
}

fn table(view: &AppView, p: &Palette, cx: &mut Context<AppView>) -> Div {
    let header = div()
        .flex()
        .items_center()
        .gap(px(8.))
        .px(px(16.))
        .py(px(12.))
        .bg(p.surface)
        .border_b_1()
        .border_color(p.border)
        .text_size(px(11.))
        .font_weight(FontWeight::SEMIBOLD)
        .text_color(p.muted)
        .child(col(2.).child("DISCIPLINA"))
        .child(col(1.).child("AVA 1"))
        .child(col(1.).child("AVA 2"))
        .child(col(1.).child("AVAL. 2"))
        .child(col(1.).child("P4"));

    let mut rows = div().flex().flex_col();
    for (i, (name, _, _)) in SUBJECTS.iter().enumerate() {
        let selected = view.sel == i;
        rows = rows.child(
            div()
                .id(("academico-row", i))
                .flex()
                .items_center()
                .gap(px(8.))
                .w_full()
                .px(px(16.))
                .py(px(13.))
                .border_b_1()
                .border_color(p.border)
                .bg(if selected { p.bg_alt } else { hexa(0x00000000) })
                .text_size(px(12.))
                .text_color(p.muted)
                .cursor_pointer()
                .child(
                    col(2.).child(
                        div()
                            .text_size(px(14.))
                            .font_weight(FontWeight::SEMIBOLD)
                            .text_color(p.text)
                            .child(*name),
                    ),
                )
                .child(col(1.).child("desconhecido"))
                .child(col(1.).child("desconhecido"))
                .child(col(1.).child("a confirmar"))
                .child(col(1.).child("não encontrado"))
                .on_click(cx.listener(move |this, _: &ClickEvent, _, cx| {
                    this.sel = i;
                    cx.notify();
                })),
        );
    }

    div()
        .flex_1()
        .min_w(px(460.))
        .flex()
        .flex_col()
        .rounded(px(16.))
        .border_1()
        .border_color(p.border)
        .bg(p.card)
        .overflow_hidden()
        .child(header)
        .child(rows)
}

fn detail(view: &AppView, p: &Palette) -> Div {
    let (name, _, note) = SUBJECTS[view.sel.min(SUBJECTS.len() - 1)];
    let note = if note.is_empty() { "Sem observações" } else { note };
    let number = view.sel + 1;

    let mut avas = div().flex().flex_col().gap(px(8.));
    for k in ["AVA 1", "AVA 2"] {
        avas = avas.child(
            div()
                .flex()
                .items_center()
                .gap(px(8.))
                .px(px(12.))
                .py(px(10.))
                .rounded(px(10.))
                .bg(p.bg_alt)
                .text_size(px(12.))
                .child(
                    div()
                        .w(px(64.))
                        .font_weight(FontWeight::BOLD)
                        .child(k),
                )
                .child(labelled(p, "Prazo", "desconhecido").flex_1())
                .child(labelled(p, "Entrega", "desconhecido").flex_1())
                .child(labelled(p, "Nota", "não exibida").flex_1()),
        );
    }

    let mut provas = div().flex().flex_col().gap(px(8.));
    for k in ["Avaliação 1", "Avaliação 2 · regular", "Avaliação 3 · substituta"] {
        provas = provas.child(
            div()
                .flex()
                .items_center()
                .gap(px(8.))
                .px(px(12.))
                .py(px(10.))
                .rounded(px(10.))
                .border_1()
                .border_color(p.border)
                .text_size(px(12.))
                .child(
                    div()
                        .flex_1()
                        .font_weight(FontWeight::SEMIBOLD)
                        .child(k),
                )
                .child(labelled(p, "Data", "a confirmar").flex_1())
                .child(labelled(p, "Local", "a confirmar").flex_1())
                .child(labelled(p, "Status", "não lido").flex_1()),
        );
    }

    card(p)
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(4.))
                .child(
                    div()
                        .text_size(px(12.))
                        .text_color(p.muted)
                        .child(format!("Disciplina {number} de 7 · 2026.2")),
                )
                .child(
                    div()
                        .text_size(px(18.))
                        .font_weight(FontWeight::BOLD)
                        .line_height(px(24.))
                        .child(name),
                )
                .child(
                    div()
                        .text_size(px(13.))
                        .text_color(p.muted)
                        .child(note),
                ),
        )
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.))
                .mt(px(16.))
                .child(section_label(p, "Atividades (Canvas)"))
                .child(avas),
        )
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.))
                .mt(px(16.))
                .child(section_label(p, "Provas (Sistema de Provas · Calendário)"))
                .child(provas),
        )
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(6.))
                .mt(px(16.))
                .p(px(12.))
                .rounded(px(10.))
                .bg(p.bg_alt)
                .text_size(px(12.))
                .line_height(px(18.))
                .child(
                    div()
                        .font_weight(FontWeight::BOLD)
                        .child("Tipos de avaliação encontrados"),
                )
                .child(
                    div()
                        .text_color(p.muted)
                        .child("Portal: aguardando leitura · Canvas: aguardando leitura · P4: não encontrado nas fontes"),
                ),
        )
}

fn section_label(p: &Palette, text: &str) -> Div {
    div()
        .text_size(px(12.))
        .font_weight(FontWeight::BOLD)
        .text_color(p.muted)
        .child(text.to_uppercase())
}

fn labelled(p: &Palette, label: &str, value: &str) -> Div {
    div()
        .flex()
        .flex_col()
        .child(div().text_color(p.muted).child(label.to_string()))
        .child(value.to_string())
}
