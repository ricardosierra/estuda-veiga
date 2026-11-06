//! Ícones do design system (traço estilo Lucide), embutidos no binário.
//! Fonte: `_ds_bundle.js` do canvas "Estuda UVA" (componente `DMConecta.Icon`).

use std::borrow::Cow;

use gpui_kit::gpui::{AssetSource, Result, SharedString};

macro_rules! icons {
    ($($name:literal),* $(,)?) => {
        const ICONS: &[(&str, &[u8])] = &[
            $(
                (
                    concat!("icons/", $name, ".svg"),
                    include_bytes!(concat!("../assets/icons/", $name, ".svg")),
                ),
            )*
        ];
    };
}

icons!(
    "home",
    "school",
    "info",
    "calendar",
    "search",
    "moon",
    "sun",
    "bolt",
    "globe",
    "inventory",
    "eye",
    "leaf",
    "volunteer",
    "person",
    "shield",
    "clock",
    "link",
    "check",
    "send",
    "chevronLeft",
    "chevronRight",
);

pub struct Assets;

impl AssetSource for Assets {
    fn load(&self, path: &str) -> Result<Option<Cow<'static, [u8]>>> {
        Ok(ICONS
            .iter()
            .find(|(name, _)| *name == path)
            .map(|(_, bytes)| Cow::Borrowed(*bytes)))
    }

    fn list(&self, path: &str) -> Result<Vec<SharedString>> {
        Ok(ICONS
            .iter()
            .filter(|(name, _)| name.starts_with(path))
            .map(|(name, _)| SharedString::from(*name))
            .collect())
    }
}
