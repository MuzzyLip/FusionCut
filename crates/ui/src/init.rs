use gpui::{App, IntoElement, ParentElement, Render, Styled, div, rgb};
use gpui_component::StyledExt;

use crate::theme::BACKGROUND;

pub fn init_gpui_component(cx: &mut App) {
    gpui_component::init(cx)
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
        _window: &mut gpui::Window,
        _cx: &mut gpui::Context<Self>,
    ) -> impl IntoElement {
        div()
            .v_flex()
            .size_full()
            .justify_center()
            .items_center()
            .bg(rgb(BACKGROUND))
            .child("Hello FusionCut")
    }
}
