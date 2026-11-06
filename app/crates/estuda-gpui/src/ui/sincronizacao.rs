//! Tela Sincronização: fontes somente leitura, login manual e registro da sessão.

use gpui_kit::gpui::{
    ClickEvent, Context, Div, FontWeight, Hsla, ParentElement, Styled, div, prelude::*, px,
};

use super::AppView;
use super::data::SOURCES;
use super::kit::{self, Size, Variant, card, icon, section_title};
use super::{SrcState, SyncPhase};
use crate::theme::{AZUL_MEDIO, AZUL_PALIDO, LARANJA, PETROLEO, PETROLEO_DEEP, Palette, hex};

/// Rótulo, cor do texto/borda e progresso de cada estado (`stLabel` do layout).
fn state_style(p: &Palette, state: SrcState) -> (&'static str, Hsla, Hsla, f32) {
    match state {
        SrcState::Idle => ("Parado", p.muted, p.border, 0.),
        SrcState::Waiting => ("Aguardando você", hex(LARANJA), hex(LARANJA), 15.),
        SrcState::Done => ("Lido agora", hex(PETROLEO), p.border, 100.),
    }
}

pub fn render(view: &AppView, p: &Palette, cx: &mut Context<AppView>) -> Div {
    let mut sources = div().flex().flex_col().gap(px(10.));
    for (i, (name, scope, icon_name)) in SOURCES.iter().enumerate() {
        let (label, fg, border, bar) = state_style(p, view.src_state[i]);
        sources = sources.child(
            div()
                .relative()
                .overflow_hidden()
                .flex()
                .items_center()
                .gap(px(14.))
                .px(px(16.))
                .py(px(14.))
                .rounded(px(14.))
                .border_1()
                .border_color(border)
                .bg(p.card)
                .child(
                    div()
                        .flex_none()
                        .flex()
                        .items_center()
                        .justify_center()
                        .size(px(40.))
                        .rounded(px(10.))
                        .bg(p.bg_alt)
                        .child(icon(icon_name, 20., hex(PETROLEO))),
                )
                .child(
                    div()
                        .flex_1()
                        .min_w_0()
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
                                .child(*scope),
                        ),
                )
                .child(
                    div()
                        .flex_none()
                        .flex()
                        .items_center()
                        .gap(px(6.))
                        .text_size(px(12.))
                        .font_weight(FontWeight::SEMIBOLD)
                        .text_color(fg)
                        .child(kit::dot(fg, 7.))
                        .child(label),
                )
                .child(
                    div()
                        .absolute()
                        .left(px(0.))
                        .bottom(px(0.))
                        .h(px(2.))
                        .w(gpui_kit::gpui::relative(bar / 100.))
                        .bg(hex(AZUL_MEDIO)),
                ),
        );
    }

    if view.sync_phase == SyncPhase::Login {
        sources = sources.child(
            div()
                .flex()
                .items_center()
                .gap(px(14.))
                .px(px(16.))
                .py(px(14.))
                .rounded(px(14.))
                .bg(hex(AZUL_PALIDO))
                .text_color(hex(PETROLEO_DEEP))
                .child(
                    div()
                        .flex_1()
                        .text_size(px(13.))
                        .line_height(px(19.))
                        .child("Faça o login no Portal (CPF, senha e MFA) e escolha a matrícula ")
                        .child(
                            div()
                                .font_weight(FontWeight::BOLD)
                                .child("Ativo · Ciência da Computação"),
                        )
                        .child(". O app não digita nem guarda credenciais."),
                )
                .child(
                    kit::button(
                        p,
                        "login-done",
                        "Concluí o login",
                        Variant::Primary,
                        Size::Md,
                        Some("check"),
                    )
                    .on_click(cx.listener(|this, _: &ClickEvent, _, cx| {
                        this.login_done(cx);
                    })),
                ),
        );
    }

    let stat = |value: u32, label: &str, accent: bool| {
        div()
            .flex()
            .flex_col()
            .gap(px(2.))
            .p(px(12.))
            .rounded(px(10.))
            .bg(p.bg_alt)
            .child(
                div()
                    .font_family(kit::FONT_NUMERIC)
                    .text_size(px(28.))
                    .font_weight(FontWeight::BOLD)
                    .text_color(if accent { hex(AZUL_MEDIO) } else { p.text })
                    .child(value.to_string()),
            )
            .child(
                div()
                    .text_size(px(12.))
                    .text_color(p.muted)
                    .child(label.to_string()),
            )
    };

    let internet = card(p)
        .child(section_title(p, "globe", "Internet"))
        .child(
            div()
                .flex()
                .gap(px(12.))
                .child(div().flex_1().child(stat(view.reqs, "requisições feitas", false)))
                .child(div().flex_1().child(stat(view.avoided, "evitadas (cache/ETag)", true))),
        )
        .child(
            div()
                .mt(px(12.))
                .text_size(px(12.))
                .line_height(px(18.))
                .text_color(p.muted)
                .child("Um item por vez, com timeout. Sem vídeos ou anexos fora do escopo. URLs com token ou SSO são descartadas."),
        );

    let mut log = div().flex().flex_col().gap(px(8.)).font_family(kit::FONT_NUMERIC).text_size(px(12.));
    for (time, message) in &view.log {
        log = log.child(
            div()
                .flex()
                .gap(px(10.))
                .child(div().text_color(p.muted).child(time.clone()))
                .child(div().text_color(p.text).child(message.clone())),
        );
    }
    let registro = card(p)
        .child(section_title(p, "clock", "Registro"))
        .child(log);

    div()
        .flex()
        .flex_wrap()
        .items_start()
        .gap(px(20.))
        .child(
            div()
                .flex_1()
                .min_w(px(380.))
                .flex()
                .flex_col()
                .gap(px(10.))
                .child(sources),
        )
        .child(
            div()
                .flex_1()
                .min_w(px(380.))
                .flex()
                .flex_col()
                .gap(px(20.))
                .child(internet)
                .child(registro),
        )
}
