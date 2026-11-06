//! Tela Biblioteca: filtros por origem e cartões dos documentos convertidos.

use gpui_kit::gpui::{
    ClickEvent, Context, Div, FontWeight, InteractiveElement, ParentElement, Styled, div,
    prelude::*, px,
};

use super::AppView;
use super::data::DOCS;
use super::kit::{self};
use crate::theme::{AZUL_PALIDO, PETROLEO, Palette, hex};

/// Rótulos dos filtros de origem, na ordem do layout.
pub const FILTERS: [&str; 4] = ["Todos", "Portal", "Canvas", "Pessoal"];

pub fn render(view: &AppView, p: &Palette, cx: &mut Context<AppView>) -> Div {
    let mut filters = div().flex().flex_wrap().gap(px(8.));
    for (i, label) in FILTERS.iter().enumerate() {
        let active = view.filter == i;
        filters = filters.child(
            kit::pill(p, ("filter", i), label, active, true).on_click(cx.listener(
                move |this, _: &ClickEvent, _, cx| {
                    this.filter = i;
                    cx.notify();
                },
            )),
        );
    }

    let query = view.query.trim().to_lowercase();
    let mut grid = div().flex().flex_wrap().gap(px(14.));
    let mut shown = 0;
    for (i, (kind, title, subj, src, state, origin)) in DOCS.iter().enumerate() {
        let by_origin = view.filter == 0 || FILTERS[view.filter] == *origin;
        let by_query = query.is_empty()
            || format!("{title} {subj}").to_lowercase().contains(&query);
        if !(by_origin && by_query) {
            continue;
        }
        shown += 1;
        grid = grid.child(
            div()
                .id(("doc", i))
                .flex_1()
                .min_w(px(240.))
                .flex()
                .flex_col()
                .gap(px(10.))
                .p(px(16.))
                .rounded(px(14.))
                .border_1()
                .border_color(p.border)
                .bg(p.card)
                .cursor_pointer()
                .hover(|s| s.bg(p.bg_alt))
                .child(
                    div()
                        .flex()
                        .items_center()
                        .justify_between()
                        .child(
                            div()
                                .px(px(8.))
                                .py(px(3.))
                                .rounded(px(6.))
                                .bg(hex(AZUL_PALIDO))
                                .text_color(hex(PETROLEO))
                                .text_size(px(11.))
                                .font_weight(FontWeight::BOLD)
                                .child(*kind),
                        )
                        .child(
                            div()
                                .text_size(px(11.))
                                .text_color(p.muted)
                                .child(*state),
                        ),
                )
                .child(
                    div()
                        .text_size(px(14.))
                        .font_weight(FontWeight::SEMIBOLD)
                        .line_height(px(19.))
                        .child(*title),
                )
                .child(
                    div()
                        .text_size(px(12.))
                        .text_color(p.muted)
                        .child(*subj),
                )
                .child(
                    div()
                        .text_size(px(11.))
                        .text_color(p.muted)
                        .child(format!("Fonte: {src}")),
                )
                .on_click(cx.listener(move |this, _: &ClickEvent, _, cx| {
                    this.open_doc(i, cx);
                })),
        );
    }

    let mut body = div().flex().flex_col().gap(px(16.)).child(filters).child(grid);
    if shown == 0 {
        body = body.child(
            div()
                .text_size(px(13.))
                .text_color(p.muted)
                .child(format!("Nada encontrado para “{}”.", view.query)),
        );
    }
    body
}
