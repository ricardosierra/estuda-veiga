//! Tela Painel: lista de disciplinas e cartões de confirmação (padrão) ou quadro por raias.

use gpui_kit::gpui::{
    ClickEvent, Context, FontWeight, InteractiveElement, ParentElement, Styled, div, prelude::*,
    px,
};

use super::data::{LANES, OPEN_QUESTIONS, SUBJECTS};
use super::kit::{self, Size, Variant, card, chip, section_title};
use super::{AppView, Screen};
use crate::theme::{LARANJA, Palette, hex};

pub fn render(view: &AppView, p: &Palette, cx: &mut Context<AppView>) -> gpui_kit::gpui::Div {
    if view.quadro {
        board(p, cx)
    } else {
        list(p, cx)
    }
}

fn list(p: &Palette, cx: &mut Context<AppView>) -> gpui_kit::gpui::Div {
    div()
        .flex()
        .flex_wrap()
        .items_start()
        .gap(px(20.))
        .child(
            div()
                .flex_1()
                .min_w(px(380.))
                .child(subjects_card(p, cx)),
        )
        .child(
            div()
                .flex_1()
                .min_w(px(380.))
                .flex()
                .flex_col()
                .gap(px(20.))
                .child(questions_card(p))
                .child(exams_card(p, cx)),
        )
}

fn subjects_card(p: &Palette, cx: &mut Context<AppView>) -> gpui_kit::gpui::Div {
    let mut rows = div().flex().flex_col();
    for (i, (name, short, note)) in SUBJECTS.iter().enumerate() {
        let _ = short;
        let note = if note.is_empty() { "Sem observações" } else { note };
        rows = rows.child(
            div()
                .id(("subject", i))
                .flex()
                .flex_wrap()
                .items_center()
                .gap(px(8.))
                .px(px(6.))
                .py(px(12.))
                .border_t_1()
                .border_color(p.border)
                .cursor_pointer()
                .hover(|s| s.bg(p.bg_alt))
                .child(
                    div()
                        .font_family(kit::FONT_NUMERIC)
                        .text_size(px(12.))
                        .text_color(p.muted)
                        .child(format!("{:02}", i + 1)),
                )
                .child(
                    div()
                        .flex_1()
                        .min_w(px(220.))
                        .flex()
                        .flex_col()
                        .gap(px(2.))
                        .child(
                            div()
                                .text_size(px(14.))
                                .font_weight(FontWeight::SEMIBOLD)
                                .child(*name),
                        )
                        .child(
                            div()
                                .text_size(px(12.))
                                .text_color(p.muted)
                                .child(note),
                        ),
                )
                .child(
                    div()
                        .flex()
                        .flex_wrap()
                        .gap(px(6.))
                        .text_size(px(11.))
                        .child(chip(p, "AVA 1 · desconhecido"))
                        .child(chip(p, "AVA 2 · desconhecido")),
                )
                .on_click(cx.listener(move |this, _: &ClickEvent, _, cx| {
                    this.sel = i;
                    this.go(Screen::Academico);
                    cx.notify();
                })),
        );
    }
    card(p)
        .child(section_title(p, "school", "Disciplinas 2026.2"))
        .child(rows)
}

fn questions_card(p: &Palette) -> gpui_kit::gpui::Div {
    let mut items = div().flex().flex_col().gap(px(12.));
    for (title, desc) in OPEN_QUESTIONS {
        items = items.child(
            div()
                .flex()
                .gap(px(10.))
                .text_size(px(13.))
                .line_height(px(19.))
                .child(
                    div()
                        .flex_none()
                        .mt(px(7.))
                        .size(px(6.))
                        .rounded_full()
                        .bg(hex(LARANJA)),
                )
                .child(
                    div()
                        .flex()
                        .flex_col()
                        .gap(px(2.))
                        .child(
                            div()
                                .font_weight(FontWeight::SEMIBOLD)
                                .child(title),
                        )
                        .child(div().text_color(p.muted).child(desc)),
                ),
        );
    }
    card(p)
        .child(section_title(p, "info", "Em aberto para confirmar"))
        .child(items)
}

fn exams_card(p: &Palette, cx: &mut Context<AppView>) -> gpui_kit::gpui::Div {
    card(p)
        .child(section_title(p, "calendar", "Próximas provas"))
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.))
                .text_size(px(13.))
                .line_height(px(20.))
                .text_color(p.muted)
                .child(
                    div()
                        .flex()
                        .flex_wrap()
                        .items_baseline()
                        .child("Nenhuma data oficial lida. Avaliação 2 (regular) e Avaliação 3 (substituta) ficam ")
                        .child(
                            div()
                                .font_weight(FontWeight::BOLD)
                                .text_color(p.text)
                                .child("a confirmar"),
                        )
                        .child(" até a leitura do Sistema de Provas e do Calendário Acadêmico."),
                )
                .child(
                    kit::button(
                        p,
                        "exams-sync",
                        "Abrir sincronização",
                        Variant::Outline,
                        Size::Sm,
                        Some("bolt"),
                    )
                    .w(px(180.))
                    .on_click(cx.listener(|this, _: &ClickEvent, _, cx| {
                        this.go(Screen::Sync);
                        cx.notify();
                    })),
                ),
        )
}

fn board(p: &Palette, cx: &mut Context<AppView>) -> gpui_kit::gpui::Div {
    let mut row = div().flex().flex_wrap().gap(px(12.));
    for (i, (title, src, desc, state)) in LANES.iter().enumerate() {
        let mut lane = div()
            .flex_1()
            .min_w(px(200.))
            .flex()
            .flex_col()
            .gap(px(10.))
            .p(px(14.))
            .rounded(px(16.))
            .bg(p.card)
            .border_1()
            .border_color(p.border)
            .child(
                div()
                    .flex()
                    .items_center()
                    .justify_between()
                    .gap(px(8.))
                    .child(
                        div()
                            .text_size(px(13.))
                            .font_weight(FontWeight::BOLD)
                            .child(*title),
                    )
                    .child(
                        div()
                            .text_size(px(11.))
                            .text_color(p.muted)
                            .child(*src),
                    ),
            )
            .child(
                div()
                    .text_size(px(12.))
                    .line_height(px(17.))
                    .text_color(p.muted)
                    .child(*desc),
            );
        for (j, (_, short, _)) in SUBJECTS.iter().enumerate() {
            lane = lane.child(
                div()
                    .id(("lane", i * 10 + j))
                    .flex()
                    .flex_col()
                    .gap(px(4.))
                    .p(px(10.))
                    .rounded(px(10.))
                    .border_1()
                    .border_color(p.border)
                    .bg(p.surface)
                    .cursor_pointer()
                    .child(
                        div()
                            .text_size(px(12.))
                            .font_weight(FontWeight::SEMIBOLD)
                            .line_height(px(16.))
                            .child(*short),
                    )
                    .child(
                        div()
                            .text_size(px(11.))
                            .text_color(p.muted)
                            .child(*state),
                    )
                    .on_click(cx.listener(move |this, _: &ClickEvent, _, cx| {
                        this.sel = j;
                        this.go(Screen::Academico);
                        cx.notify();
                    })),
            );
        }
        row = row.child(lane);
    }
    div().flex().flex_col().gap(px(20.)).child(row)
}
