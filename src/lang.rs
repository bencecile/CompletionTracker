mod ui;
mod lang_impl;

pub use self::ui::{UIStrings};

use serde_derive::{Serialize};

/// These are the different languages we support
#[derive(Copy, Clone)]
pub enum UILang {
    /// English
    EN,
    /// Japanese
    JP,
}
impl UILang {
    /// Returns all of the possible languages
    pub fn all() -> &'static [UILang] {
        &[
            UILang::EN,
            UILang::JP,
        ]
    }
    /// Gets all of the possible short strings for the languages
    pub fn all_short_str() -> Vec<&'static str> {
        UILang::all().iter()
            .map(|lang| lang.short_str())
            .collect()
    }
    /// Tries to get the language from a short string
    pub fn from_short_str(short: &str) -> Option<UILang> {
        for lang in UILang::all().iter() {
            if lang.short_str() == short {
                return Some(*lang);
            }
        }
        None
    }
    /// Gets the short string version of this language
    pub fn short_str(&self) -> &'static str {
        match self {
            UILang::EN => "en",
            UILang::JP => "jp",
        }
    }
}

/// A simple struct to keep the different translations of something.
/// Make sure that the UILang.short_str has the same value as these fields
/// (for client side matching).
#[derive(Serialize)]
pub struct UILangStrings {
    pub en: String,
    pub jp: String,
}

/// Generates a translated string for a type
pub trait Translatable {
    fn to_lang_strings(&self) -> UILangStrings;
}

/// This trait can guarantee that enums have translations
pub trait EnumTranslate where Self: Sized + Clone + 'static {
    /// Returns all of the variations of this enum
    fn all() -> &'static [Self];
    /// Gives the translation string for the variation
    fn lang_str(&self) -> UILangStrings;

    // /// Returns a vector with the translations for all variations
    // fn all_lang_str(lang: UILang) -> Vec<UILangStrings> {
    //     Self::all().iter()
    //         .map(|variation| variation.lang_str())
    //         .collect()
    // }
}
impl <T> Translatable for T
where T: EnumTranslate {
    fn to_lang_strings(&self) -> UILangStrings { self.lang_str() }
}
