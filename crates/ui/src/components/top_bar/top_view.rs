use gpui::{
    App, AppContext, Entity, InteractiveElement, IntoElement, ParentElement, Render, Styled, div,
};
use gpui_component::StyledExt;

use crate::{
    components::top_bar::operation_bar::OperationBar,
    theme::{BACKGROUND, ColorExt},
};

pub struct TopBar {}

impl TopBar {
    pub fn new() -> Self {
        Self {}
    }

    pub fn view(_: &mut gpui::Window, cx: &mut App) -> Entity<Self> {
        cx.new(|_| TopBar::new())
    }
}

impl Render for TopBar {
    fn render(
        &mut self,
        window: &mut gpui::Window,
        cx: &mut gpui::Context<Self>,
    ) -> impl IntoElement {
        div()
            .v_flex()
            .relative()
            .w_full()
            .h_10()
            .window_control_area(gpui::WindowControlArea::Drag)
            .bg(BACKGROUND.to_rgba())
            .child(OperationBar::view(window, cx))
    }
}
