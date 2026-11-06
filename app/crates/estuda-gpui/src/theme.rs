//! Tokens do design system do canvas "Estuda UVA" (`:root` e `.dark` do `_ds_bundle.css`).

use gpui_kit::gpui::{Hsla, rgb, rgba};

/// Cor opaca a partir de `0xRRGGBB`.
pub fn hex(value: u32) -> Hsla {
    rgb(value).into()
}

/// Cor com alpha a partir de `0xRRGGBBAA`.
pub fn hexa(value: u32) -> Hsla {
    rgba(value).into()
}

pub const PETROLEO: u32 = 0x1b4f72;
pub const PETROLEO_DEEP: u32 = 0x0d2d42;
pub const AZUL_MEDIO: u32 = 0x2e7ba8;
pub const AZUL_PALIDO: u32 = 0xd6eaf8;
pub const LARANJA: u32 = 0xf4841a;
pub const LARANJA_LIGHT: u32 = 0xf89b45;

/// Paleta do tema ativo. `dark` segue o alternador do cabeçalho.
#[derive(Clone, Copy)]
pub struct Palette {
    pub dark: bool,
    pub bg: Hsla,
    pub bg_alt: Hsla,
    pub nav: Hsla,
    pub nav_border: Hsla,
    pub text: Hsla,
    pub muted: Hsla,
    pub border: Hsla,
    pub surface: Hsla,
    pub card: Hsla,
    pub card_alt: Hsla,
}

impl Palette {
    pub fn new(dark: bool) -> Self {
        if dark {
            Self {
                dark,
                bg: hex(0x0d2d42),
                bg_alt: hex(0x0a2333),
                nav: hexa(0x0d2d42e0),
                nav_border: hexa(0xffffff0f),
                text: hex(0xeef6fb),
                muted: hex(0x9dc4de),
                border: hexa(0xffffff17),
                surface: hex(0x0c2c40),
                card: hex(0x103451),
                card_alt: hex(0x0b2a3e),
            }
        } else {
            Self {
                dark,
                bg: hex(0xffffff),
                bg_alt: hex(0xf4f8fb),
                nav: hexa(0xffffffF2),
                nav_border: hexa(0x1b4f721a),
                text: hex(PETROLEO),
                muted: hex(0x4a6f88),
                border: hexa(0x1b4f7224),
                surface: hex(0xf8fbfe),
                card: hex(0xffffff),
                card_alt: hex(0xeef6fc),
            }
        }
    }
}
