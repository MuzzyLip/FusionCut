use locale_config::Locale;
use std::{cell::RefCell, collections::HashMap, fs, rc::Rc};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Language {
    En,
    ZhCN,
}

impl Language {
    pub fn code(&self) -> &'static str {
        match self {
            Language::En => "en",
            Language::ZhCN => "zh-cn",
        }
    }

    #[allow(clippy::if_same_then_else)]
    pub fn from(locale: Locale) -> Self {
        let lang_code = locale.to_string().to_lowercase();
        if lang_code.starts_with("zh-cn") {
            Language::ZhCN
        } else if lang_code.starts_with("en") {
            Language::En
        } else {
            Language::En
        }
    }
}

#[derive(Debug, Clone)]
pub struct I18n {
    lang: Rc<RefCell<Language>>,
    dict: HashMap<String, String>,
}

impl I18n {
    pub fn new() -> Self {
        let locale = Locale::user_default();
        let lang = Language::from(locale);
        let dict = Self::load_language(lang);
        Self {
            lang: Rc::new(RefCell::new(lang)),
            dict,
        }
    }

    pub fn lang(&self) -> Language {
        *self.lang.borrow()
    }

    pub fn set_lang(&mut self, lang: Language) {
        *self.lang.borrow_mut() = lang;
        self.dict = Self::load_language(lang);
    }

    pub fn t(&self, key: &str) -> String {
        self.dict.get(key).cloned().unwrap_or_default()
    }

    fn load_language(lang: Language) -> HashMap<String, String> {
        let file = format!("crates/core/src/i18n/{}.toml", lang.code());
        let content = fs::read_to_string(&file)
            .unwrap_or_else(|_| panic!("Missing translation file: {}", file));

        toml::from_str(&content).expect("Invalid TOML format")
    }
}

impl Default for I18n {
    fn default() -> Self {
        Self::new()
    }
}
