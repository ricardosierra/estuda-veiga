//! Componentes do design system `DMConecta` (Card, SectionTitle, Button, Icon, Avatar, chips).
//! Cada função devolve elementos GPUI; o estado e os cliques ficam com a tela que chama.

use gpui_kit::gpui::{
    Div, ElementId, FontWeight, Hsla, InteractiveElement, ParentElement, Stateful,
    Styled, Svg, div, linear_color_stop, linear_gradient, prelude::*, px, svg,
};

use crate::theme::{AZUL_MEDIO, LARANJA, LARANJA_LIGHT, PETROLEO, PETROLEO_DEEP, Palette, hex, hexa};

/// Fonte do design system: DM Sans para texto e Inter para números.
pub const FONT_SANS: &str = "DM Sans";
pub const FONT_NUMERIC: &str = "Inter";

/// Ícone monocromático (traço), tingido pela cor do texto.
pub fn icon(name: &str, size: f32, color: Hsla) -> Svg {
    svg()
        .path(format!("icons/{name}.svg"))
        .size(px(size))
        .flex_none()
        .text_color(color)
}

/// `DMConecta.Card`: fundo do cartão, borda do tema e raio de 16px.
pub fn card(p: &Palette) -> Div {
    div()
        .flex()
        .flex_col()
        .rounded(px(16.))
        .bg(p.card)
        .border_1()
        .border_color(p.border)
        .p(px(20.))
}

/// `DMConecta.SectionTitle`: chip do ícone + título em caixa-alta.
pub fn section_title(p: &Palette, icon_name: &str, title: &str) -> Div {
    div()
        .flex()
        .items_center()
        .gap(px(8.))
        .mb(px(12.))
        .child(
            div()
                .flex()
                .items_center()
                .justify_center()
                .size(px(28.))
                .rounded(px(8.))
                .bg(hexa(0x2e7ba81f))
                .child(icon(icon_name, 16., hex(AZUL_MEDIO))),
        )
        .child(
            div()
                .text_size(px(14.))
                .font_weight(FontWeight::BOLD)
                .text_color(p.text)
                .child(title.to_uppercase()),
        )
}

#[derive(Clone, Copy, PartialEq)]
pub enum Variant {
    Primary,
    Secondary,
    Outline,
    Ghost,
}

#[derive(Clone, Copy, PartialEq)]
pub enum Size {
    Sm,
    Md,
}

/// `DMConecta.Button` com ícone à esquerda. O chamador anexa `on_click`.
pub fn button(
    p: &Palette,
    id: impl Into<ElementId>,
    label: &str,
    variant: Variant,
    size: Size,
    icon_name: Option<&str>,
) -> Stateful<Div> {
    button_ex(p, id, label, variant, size, icon_name, None)
}

/// `DMConecta.Button` com ícones opcionais à esquerda e à direita.
pub fn button_ex(
    p: &Palette,
    id: impl Into<ElementId>,
    label: &str,
    variant: Variant,
    size: Size,
    icon_name: Option<&str>,
    icon_right: Option<&str>,
) -> Stateful<Div> {
    let (height, pad_x, text, radius) = match size {
        Size::Sm => (36., 14., 13., 8.),
        Size::Md => (44., 20., 14., 12.),
    };
    let base = div()
        .id(id)
        .flex()
        .flex_none()
        .items_center()
        .justify_center()
        .gap(px(6.))
        .h(px(height))
        .px(px(pad_x))
        .rounded(px(radius))
        .text_size(px(text))
        .font_weight(FontWeight::SEMIBOLD)
        .cursor_pointer();
    let styled = match variant {
        Variant::Primary => base.text_color(hex(0xffffff)).bg(linear_gradient(
            135.,
            linear_color_stop(hex(LARANJA), 0.),
            linear_color_stop(hex(LARANJA_LIGHT), 1.),
        )),
        Variant::Secondary => base.text_color(hex(0xffffff)).bg(linear_gradient(
            135.,
            linear_color_stop(hex(PETROLEO_DEEP), 0.),
            linear_color_stop(hex(PETROLEO), 1.),
        )),
        Variant::Outline => base
            .border_1()
            .border_color(p.border)
            .bg(p.card)
            .text_color(p.text),
        Variant::Ghost => base.text_color(p.muted),
    };
    let styled = styled.hover(|s| s.opacity(0.92));
    let icon_px = if size == Size::Sm { 16. } else { 18. };
    styled
        .when_some(icon_name, |d, name| d.child(icon(name, icon_px, hex(0xffffff))))
        .child(label.to_string())
        .when_some(icon_right, |d, name| d.child(icon(name, icon_px, hex(0xffffff))))
}

/// Chip de texto pequeno (ex.: "AVA 1 · desconhecido").
pub fn chip(p: &Palette, text: &str) -> Div {
    div()
        .px(px(8.))
        .py(px(3.))
        .rounded_full()
        .bg(p.bg_alt)
        .text_size(px(11.))
        .text_color(p.muted)
        .child(text.to_string())
}

/// Botão de pílula usado em abas, filtros e segmentos (ativo = petróleo).
pub fn pill(
    p: &Palette,
    id: impl Into<ElementId>,
    label: &str,
    active: bool,
    compact: bool,
) -> Stateful<Div> {
    let (pad_x, pad_y) = if compact { (14., 7.) } else { (16., 8.) };
    let (bg, fg) = if active {
        (hex(PETROLEO), hex(0xffffff))
    } else {
        (p.card, p.text)
    };
    div()
        .id(id)
        .px(px(pad_x))
        .py(px(pad_y))
        .rounded_full()
        .border_1()
        .border_color(p.border)
        .bg(bg)
        .text_color(fg)
        .text_size(px(13.))
        .font_weight(FontWeight::SEMIBOLD)
        .cursor_pointer()
        .child(label.to_string())
}

/// Grupo de abas em cápsula (`bg card` com borda), como o seletor de Estudo e Auxílio.
pub fn segmented(p: &Palette) -> Div {
    div()
        .flex()
        .gap(px(4.))
        .p(px(4.))
        .rounded(px(12.))
        .bg(p.card)
        .border_1()
        .border_color(p.border)
}

/// Aba dentro de `segmented`.
pub fn segment(
    p: &Palette,
    id: impl Into<ElementId>,
    label: &str,
    active: bool,
) -> Stateful<Div> {
    let (bg, fg) = if active {
        (hex(PETROLEO), hex(0xffffff))
    } else {
        (hexa(0x00000000), p.muted)
    };
    div()
        .id(id)
        .px(px(16.))
        .py(px(8.))
        .rounded(px(9.))
        .bg(bg)
        .text_color(fg)
        .text_size(px(13.))
        .font_weight(FontWeight::SEMIBOLD)
        .cursor_pointer()
        .child(label.to_string())
}

/// `input type=checkbox` de privacidade: pista de 40x22 com botão de 16px.
pub fn switch(p: &Palette, on: bool, locked: bool) -> Div {
    let track = if on { hex(AZUL_MEDIO) } else { p.border };
    let _ = locked;
    div()
        .relative()
        .flex_none()
        .w(px(40.))
        .h(px(22.))
        .rounded_full()
        .bg(track)
        .child(
            div()
                .absolute()
                .top(px(3.))
                .left(px(if on { 21. } else { 3. }))
                .size(px(16.))
                .rounded_full()
                .bg(hex(0xffffff)),
        )
}

/// Barra de progresso fina (`height 4px`).
pub fn progress(p: &Palette, percent: f32, color: Hsla) -> Div {
    div()
        .w_full()
        .h(px(4.))
        .rounded_full()
        .bg(p.border)
        .overflow_hidden()
        .child(
            div()
                .h_full()
                .w(gpui_kit::gpui::relative((percent / 100.).clamp(0., 1.)))
                .rounded_full()
                .bg(color),
        )
}

/// Círculo de iniciais (`DMConecta.Avatar`).
pub fn avatar(initials: &str, size: f32) -> Div {
    div()
        .flex()
        .flex_none()
        .items_center()
        .justify_center()
        .size(px(size))
        .rounded_full()
        .bg(hex(PETROLEO))
        .text_color(hex(0xffffff))
        .text_size(px(size * 0.4))
        .font_weight(FontWeight::SEMIBOLD)
        .child(initials.to_string())
}

/// Ponto de status de 7–8px.
pub fn dot(color: Hsla, size: f32) -> Div {
    div().flex_none().size(px(size)).rounded_full().bg(color)
}

pub fn font_sans() -> &'static str {
    FONT_SANS
}

pub fn font_numeric() -> &'static str {
    FONT_NUMERIC
}

/// Linha de texto com trechos em negrito, preservando a quebra natural.
pub fn rich_row() -> Div {
    div().flex().flex_wrap().items_baseline()
}

pub fn rich_bold(text: &str) -> Div {
    div()
        .font_weight(FontWeight::BOLD)
        .child(text.to_string())
}
