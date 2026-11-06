//! Tela Estudo: plano semanal, flashcards e resumos gerados do acervo.

use gpui_kit::gpui::{
    ClickEvent, Context, Div, FontWeight, InteractiveElement, ParentElement, Styled, div,
    prelude::*, px,
};

use super::AppView;
use super::data::{CARDS, SUMMARIES, WEEK};
use super::kit::{self, Size, Variant, card, segment, segmented};
use crate::theme::{AZUL_PALIDO, PETROLEO, PETROLEO_DEEP, Palette, hex};

const TABS: [&str; 3] = ["Plano semanal", "Flashcards", "Resumos"];

pub fn render(view: &AppView, p: &Palette, cx: &mut Context<AppView>) -> Div {
    let mut tabs = segmented(p).self_start();
    for (i, label) in TABS.iter().enumerate() {
        tabs = tabs.child(segment(p, ("estudo-tab", i), label, view.estudo_tab == i).on_click(
            cx.listener(move |this, _: &ClickEvent, _, cx| {
                this.estudo_tab = i;
                cx.notify();
            }),
        ));
    }

    let body = match view.estudo_tab {
        0 => plano(p),
        1 => flashcards(view, p, cx),
        _ => resumos(p),
    };

    div().flex().flex_col().gap(px(18.)).child(tabs).child(body)
}

fn plano(p: &Palette) -> Div {
    let mut row = div().flex().gap(px(10.));
    for (day, blocks) in WEEK {
        let mut col = div()
            .flex_1()
            .min_w_0()
            .flex()
            .flex_col()
            .gap(px(8.))
            .min_h(px(280.))
            .p(px(12.))
            .rounded(px(14.))
            .bg(p.card)
            .border_1()
            .border_color(p.border)
            .child(
                div()
                    .text_size(px(12.))
                    .font_weight(FontWeight::BOLD)
                    .text_color(p.muted)
                    .child(day),
            );
        for (time, subject) in blocks.iter() {
            col = col.child(
                div()
                    .flex()
                    .flex_col()
                    .gap(px(2.))
                    .p(px(8.))
                    .rounded(px(8.))
                    .bg(hex(AZUL_PALIDO))
                    .text_color(hex(PETROLEO_DEEP))
                    .text_size(px(12.))
                    .child(
                        div()
                            .font_family(kit::FONT_NUMERIC)
                            .text_size(px(11.))
                            .child(*time),
                    )
                    .child(
                        div()
                            .font_weight(FontWeight::SEMIBOLD)
                            .line_height(px(16.))
                            .child(*subject),
                    ),
            );
        }
        row = row.child(col);
    }
    row
}

fn flashcards(view: &AppView, p: &Palette, cx: &mut Context<AppView>) -> Div {
    let (question, answer, deck) = CARDS[view.card_idx % CARDS.len()];
    let pos = format!("{deck} · cartão {} de {}", view.card_idx + 1, CARDS.len());

    let face = if view.flipped {
        div()
            .id("flashcard-face")
            .absolute()
            .inset_0()
            .flex()
            .flex_col()
            .items_center()
            .justify_center()
            .gap(px(12.))
            .p(px(32.))
            .rounded(px(20.))
            .bg(hex(PETROLEO))
            .text_color(hex(0xffffff))
            .child(
                div()
                    .text_size(px(11.))
                    .font_weight(FontWeight::BOLD)
                    .child("RESPOSTA"),
            )
            .child(
                div()
                    .text_size(px(18.))
                    .line_height(px(27.))
                    .text_align(gpui_kit::gpui::TextAlign::Center)
                    .child(answer),
            )
    } else {
        div()
            .id("flashcard-face")
            .absolute()
            .inset_0()
            .flex()
            .flex_col()
            .items_center()
            .justify_center()
            .gap(px(12.))
            .p(px(32.))
            .rounded(px(20.))
            .bg(p.card)
            .border_1()
            .border_color(p.border)
            .text_color(p.text)
            .child(
                div()
                    .text_size(px(11.))
                    .font_weight(FontWeight::BOLD)
                    .text_color(p.muted)
                    .child("PERGUNTA"),
            )
            .child(
                div()
                    .text_size(px(22.))
                    .font_weight(FontWeight::BOLD)
                    .line_height(px(30.))
                    .text_align(gpui_kit::gpui::TextAlign::Center)
                    .child(question),
            )
    };

    let card_area = div()
        .id("flashcard")
        .relative()
        .w(px(560.))
        .h(px(300.))
        .cursor_pointer()
        .child(face)
        .on_click(cx.listener(|this, _: &ClickEvent, _, cx| {
            this.flipped = !this.flipped;
            cx.notify();
        }));

    let controls = div()
        .flex()
        .gap(px(10.))
        .child(
            kit::button_ex(p, "card-prev", "Anterior", Variant::Outline, Size::Md, Some("chevronLeft"), None)
                .on_click(cx.listener(|this, _: &ClickEvent, _, cx| {
                    this.card_idx = (this.card_idx + CARDS.len() - 1) % CARDS.len();
                    this.flipped = false;
                    cx.notify();
                })),
        )
        .child(
            kit::button(p, "card-flip", "Virar", Variant::Secondary, Size::Md, None).on_click(
                cx.listener(|this, _: &ClickEvent, _, cx| {
                    this.flipped = !this.flipped;
                    cx.notify();
                }),
            ),
        )
        .child(
            kit::button_ex(p, "card-next", "Próximo", Variant::Outline, Size::Md, None, Some("chevronRight"))
                .on_click(cx.listener(|this, _: &ClickEvent, _, cx| {
                    this.card_idx = (this.card_idx + 1) % CARDS.len();
                    this.flipped = false;
                    cx.notify();
                })),
        );

    div()
        .flex()
        .flex_col()
        .items_center()
        .gap(px(18.))
        .py(px(12.))
        .child(
            div()
                .text_size(px(12.))
                .text_color(p.muted)
                .child(pos),
        )
        .child(card_area)
        .child(controls)
}

fn resumos(p: &Palette) -> Div {
    let mut grid = div().flex().flex_wrap().gap(px(14.));
    for (title, body, src) in SUMMARIES {
        grid = grid.child(
            card(p)
                .flex_1()
                .min_w(px(300.))
                .p(px(18.))
                .gap(px(10.))
                .child(
                    div()
                        .text_size(px(15.))
                        .font_weight(FontWeight::BOLD)
                        .child(title),
                )
                .child(
                    div()
                        .text_size(px(13.))
                        .line_height(px(20.))
                        .text_color(p.muted)
                        .child(body),
                )
                .child(
                    div()
                        .text_size(px(11.))
                        .text_color(p.muted)
                        .child(format!("Gerado de {src} · material didático, não fonte oficial")),
                ),
        );
    }
    grid
}
