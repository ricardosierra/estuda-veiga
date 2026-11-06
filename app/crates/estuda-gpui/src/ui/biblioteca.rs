//! Tela Biblioteca: filtros por origem e cartões dos documentos convertidos.

use gpui_kit::gpui::{
    ClickEvent, Context, Div, FontWeight, InteractiveElement, ParentElement, Styled, div,
    prelude::*, px,
};

use super::AppView;
use super::kit::{self};
use crate::theme::{AZUL_PALIDO, PETROLEO, Palette, hex};

/// Filtros por categoria real do índice (`category` do acervo). O layout original tinha
/// "Portal/Canvas/Pessoal", mas o índice não tem origem; as categorias reais substituem esses rótulos.
/// Cada par é (rótulo, categoria em minúsculas); `None` mostra tudo.
pub const FILTERS: [(&str, Option<&str>); 4] = [
    ("Todos", None),
    ("Material acadêmico", Some("material acadêmico")),
    ("Documentação", Some("documentação")),
    ("Notas de estudo", Some("notas de estudo")),
];

/// Limite de cartões desenhados de uma vez (memória).
const MAX_CARDS: usize = 120;

pub fn render(view: &AppView, p: &Palette, cx: &mut Context<AppView>) -> Div {
    let mut filters = div().flex().flex_wrap().gap(px(8.));
    for (i, (label, _)) in FILTERS.iter().enumerate() {
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
    for (i, document) in view.library.iter().enumerate() {
        let by_origin = match FILTERS[view.filter].1 {
            None => true,
            Some(category) => document.category.to_lowercase() == category,
        };
        let subject = document.course.clone().unwrap_or_else(|| "Sem disciplina identificada".into());
        let by_query = query.is_empty()
            || format!("{} {}", document.title, subject).to_lowercase().contains(&query);
        if !(by_origin && by_query) {
            continue;
        }
        if shown >= MAX_CARDS {
            break;
        }
        shown += 1;
        let state = document.semester.clone().unwrap_or_else(|| "local".into());
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
                                .child(document.format.to_uppercase()),
                        )
                        .child(
                            div()
                                .text_size(px(11.))
                                .text_color(p.muted)
                                .child(state),
                        ),
                )
                .child(
                    div()
                        .text_size(px(14.))
                        .font_weight(FontWeight::SEMIBOLD)
                        .line_height(px(19.))
                        .child(document.title.clone()),
                )
                .child(
                    div()
                        .text_size(px(12.))
                        .text_color(p.muted)
                        .child(subject),
                )
                .child(
                    div()
                        .text_size(px(11.))
                        .text_color(p.muted)
                        .child(format!("Fonte: {}", document.category)),
                )
                .on_click(cx.listener(move |this, _: &ClickEvent, _, cx| {
                    this.open_doc(i, cx);
                })),
        );
    }

    let status = div()
        .text_size(px(12.))
        .text_color(p.muted)
        .child(view.library_status.clone());
    let mut body = div()
        .flex()
        .flex_col()
        .gap(px(16.))
        .child(filters)
        .child(status)
        .child(grid);
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
