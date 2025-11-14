use fusion_cut_core::app_state::AppState;
use gpui::{App, IntoElement, ParentElement, Render, Styled, div, rgb};
use gpui_component::{Root, StyledExt, Theme, ThemeRegistry};
use std::path::PathBuf;

use crate::{
    pages::edit_page::EditPage,
    theme::{BACKGROUND, TEXT_COLOR},
};

// Init GPUI Component
pub fn init_gpui_component(cx: &mut App) {
    gpui_component::init(cx);
    if let Err(err) = ThemeRegistry::watch_dir(PathBuf::from("./themes"), cx, move |cx| {
        if let Some(theme) = ThemeRegistry::global(cx).themes().get("Ayu Dark").cloned() {
            Theme::global_mut(cx).apply_config(&theme);
        }
    }) {
        eprintln!("Failed to watch themes directory: {}", err);
    }
}

// Root App
pub struct RootApp {}
impl RootApp {
    pub fn build(_window: &mut gpui::Window, _cx: &mut gpui::Context<Self>) -> Self {
        Self {}
    }
}
impl Render for RootApp {
    fn render(
        &mut self,
        window: &mut gpui::Window,
        cx: &mut gpui::Context<Self>,
    ) -> impl IntoElement {
        let notification_layer = Root::render_notification_layer(window, cx);
        let global: &AppState = cx.global();
        let _i18n = global.i18n.clone();
        div()
            .v_flex()
            .size_full()
            .justify_center()
            .items_center()
            .overflow_hidden()
            .bg(rgb(BACKGROUND))
            .text_color(rgb(TEXT_COLOR))
            .child(EditPage::view(window, cx))
            .children(notification_layer)
    }
}
