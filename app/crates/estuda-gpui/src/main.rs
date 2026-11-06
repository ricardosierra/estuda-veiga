//! Estuda UVA nativo: interface GPUI com o layout do canvas "Estuda UVA".

mod assets;
mod theme;
mod ui;

use gpui_kit::component::Root;
use gpui_kit::gpui::{App, AppContext, Bounds, WindowBounds, WindowOptions, px, size};

fn main() {
    gpui_kit::application()
        .with_assets(assets::Assets)
        .run(|cx: &mut App| {
            gpui_kit::component::init(cx);
            let bounds = Bounds::centered(None, size(px(1280.), px(820.)), cx);
            cx.open_window(
                WindowOptions {
                    window_bounds: Some(WindowBounds::Windowed(bounds)),
                    window_min_size: Some(size(px(960.), px(640.))),
                    titlebar: None,
                    ..Default::default()
                },
                |window, cx| {
                    let view = cx.new(|_| ui::AppView::new());
                    cx.new(|cx| Root::new(view, window, cx))
                },
            )
            .expect("abrir janela principal");
            cx.activate(true);
        });
}
