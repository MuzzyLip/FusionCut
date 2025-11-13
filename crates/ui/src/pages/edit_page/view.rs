use gpui::{App, AppContext, Entity, ParentElement, Render, Styled, div};
use gpui_component::StyledExt;

use crate::{
    components::TopBar,
    pages::edit_page::components::{AssetsViewer, VideoViewer},
};

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
        div()
            .size_full()
            .v_flex()
            .child(TopBar::view(window, cx))
            .child(
                div()
                    .w_full()
                    .px_4()
                    .h_1_2()
                    .h_flex()
                    .mt_4()
                    .gap_4()
                    .child(AssetsViewer::view(window, cx))
                    .child(VideoViewer::view(window, cx)),
            )
    }
}
