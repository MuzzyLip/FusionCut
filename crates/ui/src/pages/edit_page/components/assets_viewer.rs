use gpui::{AppContext, Entity, ParentElement, Render, Styled, div};
use gpui_component::skeleton::Skeleton;

pub struct AssetsViewer {}

impl AssetsViewer {
    pub fn new() -> Self {
        Self {}
    }

    pub fn view(_: &mut gpui::Window, cx: &mut gpui::App) -> Entity<Self> {
        cx.new(|_| AssetsViewer::new())
    }
}

impl Render for AssetsViewer {
    fn render(
        &mut self,
        _: &mut gpui::Window,
        _: &mut gpui::Context<Self>,
    ) -> impl gpui::IntoElement {
        div()
            .w_2_5()
            .h_full()
            .child(Skeleton::new().secondary().size_full().rounded_md())
    }
}
