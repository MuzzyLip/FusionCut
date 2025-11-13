use fusion_cut_core::app_state::AppState;
use gpui::{
    AppContext, Entity, InteractiveElement, MouseButton, ParentElement, PathPromptOptions, Render,
    Styled, div,
};
use gpui_component::{Icon, Sizable, StyledExt, skeleton::Skeleton};
use gpui_util::ResultExt;

pub struct VideoViewer {}

impl VideoViewer {
    pub fn new() -> Self {
        Self {}
    }

    pub fn view(_: &mut gpui::Window, cx: &mut gpui::App) -> Entity<Self> {
        cx.new(|_| VideoViewer::new())
    }
}

impl Render for VideoViewer {
    fn render(
        &mut self,
        window: &mut gpui::Window,
        cx: &mut gpui::Context<Self>,
    ) -> impl gpui::IntoElement {
        let global: &AppState = cx.global();
        div().relative().w_3_5().h_full().child(
            if let Some(video_path) = global.video_path.clone() {
                div().size_full()
            } else {
                div().size_full().child(VideoViewerEmpty::view(window, cx))
            },
        )
    }
}

pub struct VideoViewerEmpty {}

impl VideoViewerEmpty {
    pub fn new() -> Self {
        Self {}
    }

    pub fn view(_: &mut gpui::Window, cx: &mut gpui::App) -> Entity<Self> {
        cx.new(|_| VideoViewerEmpty::new())
    }
}

impl Render for VideoViewerEmpty {
    fn render(
        &mut self,
        _: &mut gpui::Window,
        cx: &mut gpui::Context<Self>,
    ) -> impl gpui::IntoElement {
        let global: &AppState = cx.global();
        let i18n = global.i18n.clone();
        div()
            .size_full()
            .on_mouse_down(MouseButton::Left, |_, _, cx| {
                let i18n = {
                    let global: &AppState = cx.global();
                    global.i18n.clone()
                };
                let path = cx.prompt_for_paths(PathPromptOptions {
                    files: true,
                    directories: false,
                    multiple: false,
                    prompt: Some(i18n.t("choose-file").into()),
                });
                cx.spawn(
                    async move |cx| match path.await.anyhow().and_then(|res| res) {
                        Ok(Some(paths)) => {
                            if let Some(path) = paths.first() {
                                let path = path.clone();
                                cx.update(|cx| {
                                    let global: &mut AppState = cx.global_mut();
                                    global.video_path = Some(path);
                                })
                                .ok();
                            }
                        }
                        _ => {}
                    },
                )
                .detach();
            })
            .child(Skeleton::new().secondary().size_full().rounded_md())
            .child(
                div()
                    .absolute()
                    .v_flex()
                    .inset_0()
                    .size_full()
                    .items_center()
                    .justify_center()
                    .gap_2()
                    .rounded_md()
                    .child(Icon::new(Icon::empty().path("icons/upload.svg")).large())
                    .child(i18n.t("upload")),
            )
    }
}
