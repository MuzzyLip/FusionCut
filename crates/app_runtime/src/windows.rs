use fusion_cut_ui::{RootApp, init_gpui_component};
use gpui::{AppContext, Application, Pixels, Size, px, size};
use gpui_component::Root;

use crate::option::DefaultWindowOptions;

#[derive(Clone, Copy, Debug)]
pub enum WindowsName {
    Main,
}

impl WindowsName {
    pub fn size(&self) -> Size<Pixels> {
        match self {
            WindowsName::Main => size(px(1280.0), px(720.0)),
        }
    }
}

pub fn create_gpui_window() {
    let app = Application::new();
    app.run(move |cx| {
        init_gpui_component(cx);
        let options = DefaultWindowOptions::build(WindowsName::Main, cx);
        cx.spawn(async move |cx| {
            cx.open_window(options, |window, cx| {
                let view = cx.new(|cx| RootApp::build(window, cx));
                cx.new(|cx| Root::new(view.into(), window, cx))
            })?;

            Ok::<_, anyhow::Error>(())
        })
        .detach()
    });
}
