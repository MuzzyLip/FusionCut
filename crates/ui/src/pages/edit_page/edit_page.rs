use gpui::{App, AppContext, Entity, ParentElement, Render, Styled, div};

use crate::components::TopBar;

pub struct EditPage {}

impl EditPage {
    fn new() -> Self {
        Self {}
    }

    pub fn view(_: &mut gpui::Window, cx: &mut App) -> Entity<Self> {
        cx.new(|_| EditPage::new())
    }
}

impl Render for EditPage {
    fn render(
        &mut self,
        window: &mut gpui::Window,
        cx: &mut gpui::Context<Self>,
    ) -> impl gpui::IntoElement {
        div().size_full().child(TopBar::view(window, cx))
    }
}
