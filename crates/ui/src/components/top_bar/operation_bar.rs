use gpui::{AppContext, InteractiveElement, IntoElement, ParentElement, Render, Styled, div};
use gpui_component::{
    Icon, IconName, StyledExt,
    button::{Button, ButtonCustomVariant, ButtonVariants},
};

use crate::theme::{BACKGROUND, ColorExt, TEXT_COLOR};

pub enum OperationBarActions {
    Minimize,
    Maximize,
    Close,
}

pub struct OperationBar {}

impl OperationBar {
    pub fn new() -> Self {
        Self {}
    }

    pub fn view(_: &mut gpui::Window, cx: &mut gpui::App) -> impl IntoElement {
        cx.new(|_| Self::new())
    }

    pub fn on_click_actions(
        window: &mut gpui::Window,
        cx: &mut gpui::App,
        action: OperationBarActions,
    ) {
        match action {
            OperationBarActions::Minimize => window.minimize_window(),
            OperationBarActions::Maximize => window.toggle_fullscreen(),
            OperationBarActions::Close => cx.quit(),
        }
    }
}

impl Render for OperationBar {
    fn render(&mut self, _: &mut gpui::Window, cx: &mut gpui::Context<Self>) -> impl IntoElement {
        if !cfg!(target_os = "macos") {
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
                        .cursor_pointer()
                        .icon(
                            Icon::new(IconName::Minus)
                                .text_color(TEXT_COLOR.to_rgba())
                                .size_6(),
                        )
                        .custom(variant)
                        .on_mouse_down(gpui::MouseButton::Left, |_, window, cx| {
                            Self::on_click_actions(window, cx, OperationBarActions::Minimize)
                        }),
                    Button::new("maximizes")
                        .bg(BACKGROUND.to_rgba())
                        .cursor_pointer()
                        .icon(
                            Icon::new(IconName::WindowMaximize)
                                .text_color(TEXT_COLOR.to_rgba())
                                .size_6(),
                        )
                        .custom(variant)
                        .on_mouse_down(gpui::MouseButton::Left, |_, window, cx| {
                            Self::on_click_actions(window, cx, OperationBarActions::Maximize)
                        }),
                    Button::new("close")
                        .bg(BACKGROUND.to_rgba())
                        .cursor_pointer()
                        .icon(
                            Icon::new(IconName::Close)
                                .text_color(TEXT_COLOR.to_rgba())
                                .size_6(),
                        )
                        .custom(variant)
                        .on_mouse_down(gpui::MouseButton::Left, |_, window, cx| {
                            Self::on_click_actions(window, cx, OperationBarActions::Close)
                        }),
                ])
        } else {
            div()
        }
    }
}
