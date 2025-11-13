use gpui::Global;
use gpui_component::ThemeRegistry;
use std::{path::PathBuf, rc::Rc};

use crate::i18n::{I18n, Language};

#[derive(Debug)]
pub struct AppState {
    pub i18n: Rc<I18n>,
    pub theme_registry: ThemeRegistry,
    pub video_path: Option<PathBuf>,
}

impl AppState {
    pub fn new() -> Self {
        let theme_registry = ThemeRegistry::default();
        Self {
            i18n: Rc::new(I18n::new()),
            theme_registry,
            video_path: None,
        }
    }

    pub fn set_lang(&mut self, lang: Language) {
        let mut i18n = self.i18n.clone();
        Rc::make_mut(&mut i18n).set_lang(lang);
        self.i18n = i18n;
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self::new()
    }
}

impl Global for AppState {}
