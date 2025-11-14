use std::path::PathBuf;

use fusion_cut_core::app_state::AppState;
use gpui::{
    AppContext, Entity, InteractiveElement, MouseButton, ParentElement, PathPromptOptions, Render,
    Styled, div,
};
use gpui_component::{
    Icon, Sizable, StyledExt, WindowExt, notification::Notification, skeleton::Skeleton,
};
use gpui_util::ResultExt;
use gpui_video_player::{Video, video};
use url::Url;

const VIDEO_EXTS: &[&str] = &[
    "mp4", "mov", "avi", "mkv", "flv", "wmv", "webm", "rmvb", "mpeg", "mpg", "m4v",
];

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
                let player = cx.new(|_| VideoPlayer::new(video_path));
                div().size_full().child(player)
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
            .on_mouse_down(MouseButton::Left, |_, window, cx| {
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
                let window_handle = window.window_handle();
                cx.spawn(async move |cx| {
                    if let Ok(Some(paths)) = path.await.anyhow().and_then(|res| res)
                        && let Some(path) = paths.first()
                    {
                        let path = path.clone();
                        if let Some(ext) = path.extension() {
                            let ext_lower = ext.to_ascii_lowercase();
                            let ext_lower = ext_lower.to_str().unwrap();
                            println!("Check ext {:?}", &ext_lower);
                            if VIDEO_EXTS.contains(&ext_lower) {
                                cx.update(|cx| {
                                    let global: &mut AppState = cx.global_mut();
                                    global.video_path = Some(path);
                                })
                                .ok();
                            } else {
                                window_handle
                                    .update(cx, |_, window, cx| {
                                        let i18n = {
                                            let global: &AppState = cx.global();
                                            global.i18n.clone()
                                        };
                                        let message = i18n.t("video-ext-support");
                                        window.push_notification(Notification::error(message), cx)
                                    })
                                    .ok();
                            }
                        } else {
                            window_handle
                                .update(cx, |_, window, cx| {
                                    let i18n = {
                                        let global: &AppState = cx.global();
                                        global.i18n.clone()
                                    };
                                    let message = i18n.t("get-ext-error");
                                    window.push_notification(Notification::error(message), cx)
                                })
                                .ok();
                        }
                    }
                })
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

struct VideoPlayer {
    video: Video,
}

impl VideoPlayer {
    pub fn new(video_path: PathBuf) -> Self {
        let video = Video::new(&Url::from_file_path(video_path).expect("Invalid File Path"))
            .expect("Failed To Create Video");
        Self { video }
    }
}

impl Render for VideoPlayer {
    fn render(
        &mut self,
        _: &mut gpui::Window,
        _: &mut gpui::Context<Self>,
    ) -> impl gpui::IntoElement {
        video(self.video.clone())
            .id("main-video")
            .buffer_capacity(30)
    }
}
