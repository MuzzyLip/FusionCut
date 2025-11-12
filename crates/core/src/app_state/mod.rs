use gpui::Global;
use std::rc::Rc;

use crate::i18n::{I18n, Language};

#[derive(Debug, Clone)]
pub struct AppState {
    pub i18n: Rc<I18n>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            i18n: Rc::new(I18n::new()),
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
