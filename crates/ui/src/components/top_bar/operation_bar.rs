use gpui::{AppContext, IntoElement, ParentElement, Render, Styled, div};
use gpui_component::{
    Icon, IconName, StyledExt,
    button::{Button, ButtonCustomVariant, ButtonVariants},
};
use std::sync::Arc;

use crate::theme::{BACKGROUND, ColorExt, TEXT_COLOR};
pub enum OperationBarActions {
    Minimize,
    Maximize,
    Close,
}

pub struct OperationBar {
    on_click_actions:
        Arc<dyn Fn(&mut gpui::Window, &mut gpui::App, OperationBarActions) + Send + Sync>,
}

impl OperationBar {
    pub fn new() -> Self {
        let on_click_actions =
            |window: &mut gpui::Window, cx: &mut gpui::App, action: OperationBarActions| {
                match action {
                    OperationBarActions::Minimize => window.minimize_window(),
                    OperationBarActions::Maximize => window.toggle_fullscreen(),
                    OperationBarActions::Close => cx.quit(),
                }
            };
        Self {
            on_click_actions: Arc::new(on_click_actions),
        }
    }

    pub fn view(_: &mut gpui::Window, cx: &mut gpui::App) -> impl IntoElement {
        cx.new(|_| Self::new())
    }
}

impl Render for OperationBar {
    fn render(&mut self, _: &mut gpui::Window, cx: &mut gpui::Context<Self>) -> impl IntoElement {
        if !cfg!(target_os = "macos") {
            // clone the Arc handler so closures don't borrow `self`
            let handler = self.on_click_actions.clone();

            let variant = ButtonCustomVariant::new(cx)
                .color(TEXT_COLOR.to_hsla())
                .hover(TEXT_COLOR.to_hsla_alpha(0.1));
            div()
                .absolute()
                .h_flex()
                .h_full()
                .right_4()
                .items_center()
                .justify_center()
                .gap_4()
                .children([
                    Button::new("minus")
                        .bg(BACKGROUND.to_rgba())
                        .icon(
                            Icon::new(IconName::Minus)
                                .text_color(TEXT_COLOR.to_rgba())
                                .size_6(),
                        )
                        .custom(variant)
                        .on_click({
                            let h = handler.clone();
                            move |_, window, cx| (h)(window, cx, OperationBarActions::Minimize)
                        }),
                    Button::new("maximizes")
                        .bg(BACKGROUND.to_rgba())
                        .icon(
                            Icon::new(IconName::WindowMaximize)
                                .text_color(TEXT_COLOR.to_rgba())
                                .size_6(),
                        )
                        .custom(variant)
                        .on_click({
                            let h = handler.clone();
                            move |_, window, cx| (h)(window, cx, OperationBarActions::Maximize)
                        }),
                    Button::new("close")
                        .bg(BACKGROUND.to_rgba())
                        .icon(
                            Icon::new(IconName::Close)
                                .text_color(TEXT_COLOR.to_rgba())
                                .size_6(),
                        )
                        .custom(variant)
                        .on_click({
                            let h = handler.clone();
                            move |_, window, cx| (h)(window, cx, OperationBarActions::Close)
                        }),
                ])
        } else {
            div()
        }
    }
}
