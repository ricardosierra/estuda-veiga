//! Tela Usuário: perfil, matrículas do Portal, privacidade e limite de memória.

use gpui_kit::gpui::{
    ClickEvent, Context, Div, FontWeight, InteractiveElement, ParentElement, Styled, div,
    prelude::*, px,
};

use super::AppView;
use super::data::PREFS;
use super::kit::{self, avatar, card, icon, section_title, switch};
use crate::theme::{AZUL_MEDIO, Palette, hex};

/// Limite de memória do índice em RAM (MB): mínimo, máximo e passo do layout.
pub const RAM_MIN: u32 = 64;
pub const RAM_MAX: u32 = 1024;
pub const RAM_STEP: u32 = 32;

pub fn render(view: &AppView, p: &Palette, cx: &mut Context<AppView>) -> Div {
    let profile = card(p).child(
        div()
            .flex()
            .items_center()
            .gap(px(16.))
            .child(avatar("AU", 56.))
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap(px(2.))
                    .child(
                        div()
                            .text_size(px(18.))
                            .font_weight(FontWeight::BOLD)
                            .child("[Seu nome]"),
                    )
                    .child(
                        div()
                            .text_size(px(13.))
                            .text_color(p.muted)
                            .child("Universidade Veiga de Almeida · período 2026.2"),
                    ),
            ),
    );

    let mut linhas = div().flex().flex_col().gap(px(10.));
    if view.enrollments.is_empty() {
        linhas = linhas.child(
            div()
                .text_size(px(13.))
                .text_color(p.muted)
                .child("Nenhuma matrícula sincronizada ainda."),
        );
    }
    for enrollment in &view.enrollments {
        let (border, opacity) = if enrollment.active {
            (hex(AZUL_MEDIO), 1.0)
        } else {
            (p.border, 0.65)
        };
        let note = if enrollment.active {
            "usada pelo app".to_string()
        } else {
            "trajetória separada, ignorada".to_string()
        };
        let semester = if enrollment.semester.is_empty() {
            String::new()
        } else {
            format!(" · semestre {}", enrollment.semester)
        };
        linhas = linhas.child(
            div()
                .flex()
                .items_center()
                .gap(px(12.))
                .p(px(12.))
                .rounded(px(10.))
                .border_1()
                .border_color(border)
                .bg(if enrollment.active { p.surface } else { p.card })
                .opacity(opacity)
                .child(
                    div()
                        .flex_1()
                        .flex()
                        .flex_col()
                        .child(
                            div()
                                .text_size(px(14.))
                                .font_weight(FontWeight::SEMIBOLD)
                                .child(enrollment.program.clone()),
                        )
                        .child(
                            div()
                                .text_size(px(12.))
                                .text_color(p.muted)
                                .child(format!("Situação: {} · {}{}", enrollment.status, note, semester)),
                        ),
                )
                .when(enrollment.active, |d| d.child(icon("check", 18., p.text))),
        );
    }
    let matriculas = card(p)
        .child(section_title(p, "school", "Matrículas no Portal"))
        .child(linhas);

    let left = div()
        .flex_1()
        .min_w(px(360.))
        .flex()
        .flex_col()
        .gap(px(20.))
        .child(profile)
        .child(matriculas);

    let mut prefs = div().flex().flex_col();
    for (i, (title, desc, locked)) in PREFS.iter().enumerate() {
        let on = view.prefs[i];
        let label = if *locked {
            format!("{title} · fixo")
        } else {
            title.to_string()
        };
        let mut row = div()
            .id(("pref", i))
            .flex()
            .items_center()
            .gap(px(12.))
            .py(px(12.))
            .border_t_1()
            .border_color(p.border)
            .text_color(p.text)
            .child(
                div()
                    .flex_1()
                    .flex()
                    .flex_col()
                    .gap(px(2.))
                    .child(
                        div()
                            .text_size(px(14.))
                            .font_weight(FontWeight::SEMIBOLD)
                            .child(label),
                    )
                    .child(
                        div()
                            .text_size(px(12.))
                            .line_height(px(17.))
                            .text_color(p.muted)
                            .child(*desc),
                    ),
            )
            .child(switch(p, on, *locked));
        if !*locked {
            row = row.cursor_pointer().on_click(cx.listener(move |this, _: &ClickEvent, _, cx| {
                this.prefs[i] = !this.prefs[i];
                cx.notify();
            }));
        }
        prefs = prefs.child(row);
    }

    let privacidade = card(p)
        .child(section_title(p, "shield", "Privacidade e dados"))
        .child(prefs);

    let fraction = (view.ram_limit - RAM_MIN) as f32 / (RAM_MAX - RAM_MIN) as f32;
    let memoria = card(p)
        .child(section_title(p, "bolt", "Limite de memória"))
        .child(
            div()
                .flex()
                .items_center()
                .gap(px(14.))
                .child(
                    kit::button(p, "ram-minus", "−", kit::Variant::Outline, kit::Size::Sm, None)
                        .w(px(36.))
                        .on_click(cx.listener(|this, _: &ClickEvent, _, cx| {
                            this.ram_limit = this.ram_limit.saturating_sub(RAM_STEP).max(RAM_MIN);
                            cx.notify();
                        })),
                )
                .child(
                    div()
                        .flex_1()
                        .h(px(4.))
                        .rounded_full()
                        .bg(p.border)
                        .overflow_hidden()
                        .child(
                            div()
                                .h_full()
                                .w(gpui_kit::gpui::relative(fraction))
                                .rounded_full()
                                .bg(hex(AZUL_MEDIO)),
                        ),
                )
                .child(
                    kit::button(p, "ram-plus", "+", kit::Variant::Outline, kit::Size::Sm, None)
                        .w(px(36.))
                        .on_click(cx.listener(|this, _: &ClickEvent, _, cx| {
                            this.ram_limit = (this.ram_limit + RAM_STEP).min(RAM_MAX);
                            cx.notify();
                        })),
                )
                .child(
                    div()
                        .min_w(px(72.))
                        .text_right()
                        .font_family(kit::FONT_NUMERIC)
                        .font_weight(FontWeight::BOLD)
                        .child(format!("{} MB", view.ram_limit)),
                ),
        )
        .child(
            div()
                .mt(px(8.))
                .text_size(px(12.))
                .text_color(p.muted)
                .child("Índice em RAM primeiro; gravação em lote no SQLite."),
        );

    let right = div()
        .flex_1()
        .min_w(px(360.))
        .flex()
        .flex_col()
        .gap(px(20.))
        .child(privacidade)
        .child(memoria);

    div().flex().flex_wrap().items_start().gap(px(20.)).child(left).child(right)
}
