//! Estuda UVA nativo: interface GPUI com o layout do canvas "Estuda UVA".

mod assets;
mod portal;
mod services;
mod speech;
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
                    let view = cx.new(|cx| {
                        let root = services::default_library();
                        let (services, status) = match services::Services::open(root.clone()) {
                            Ok(services) => (Some(services), None),
                            Err(error) => (None, Some(error)),
                        };
                        let mut view = ui::AppView::new(services);
                        if let Some(status) = status {
                            view.library_status = status;
                        } else {
                            view.refresh_library(cx);
                            view.refresh_disciplines(cx);
                        }
                        view.portal = portal::Portal::new().ok().map(std::sync::Arc::new);
                        view.start_memory_monitor(cx);
                        view
                    });
                    cx.new(|cx| Root::new(view, window, cx))
                },
            )
            .expect("abrir janela principal");
            cx.activate(true);
        });
}
