mod enum_translate;
pub use self::enum_translate::{EnumTranslate};
mod err;

use serde_derive::{Deserialize, Serialize};

/// These are the different languages we support
#[derive(Copy, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub enum Lang {
    /// English
    EN,
    /// Japanese
    JP,
}
impl Lang {
    /// Returns all of the possible languages
    pub fn all() -> Vec<Lang> {
        vec![
            Lang::EN,
            Lang::JP,
        ]
    }
    /// Gets all of the possible short strings for the languages
    pub fn all_short_str() -> Vec<&'static str> {
        Lang::all().into_iter()
            .map(|lang| lang.short_str())
            .collect()
    }
    /// Tries to get the language from a short string
    pub fn from_short_str(short: &str) -> Option<Lang> {
        for lang in Lang::all() {
            if lang.short_str() == short {
                return Some(lang);
            }
        }
        None
    }
    /// Gets the short string version of this language
    pub fn short_str(&self) -> &'static str {
        match self {
            Lang::EN => "en",
            Lang::JP => "jp",
        }
    }
}

/// Generates a translated string for a type
pub trait Translatable {
    fn to_lang_string(&self, lang: Lang) -> String;
}
// Implement translatable on some basic types that don't really need the language
impl Translatable for u64 {
    fn to_lang_string(&self, _: Lang) -> String { self.to_string() }
}

/// A single language string
pub type LangString = (Lang, String);
/// These are strings that are for a specific language.
#[derive(Clone, Deserialize, Serialize)]
pub struct LangStrings {
    // New fields will need to be added with new languages
    en: Option<String>,
    jp: Option<String>,
}
impl LangStrings {
    /// Returns an empty LangStrings with None in every field
    pub fn new_empty() -> LangStrings {
        LangStrings {
            en: None,
            jp: None,
        }
    }
    /// Creates a new lang strings map, inserting a single one.
    /// We should always want at least 1 string in the translation table
    pub fn new(lang_string: LangString) -> LangStrings {
        let mut lang_strings = LangStrings::new_empty();
        lang_strings.replace(lang_string);
        lang_strings
    }

    /// Gets the field that matches the Lang
    fn get(&self, lang: Lang) -> Option<&String> {
        match lang {
            Lang::EN => self.en.as_ref(),
            Lang::JP => self.jp.as_ref(),
        }
    }

    /// Replaces the value of this LangString
    pub fn replace(&mut self, lang_string: LangString) {
        let field = match lang_string.0 {
            Lang::EN => &mut self.en,
            Lang::JP => &mut self.jp,
        };
        field.replace(lang_string.1);
    }
    
    /// Gets the string for a single language
    pub fn get_str(&self, lang: Lang) -> &str {
        self.get(lang).map_or_else(|| {
            // Check every other language for a non-empty string
            let non_empty = Lang::all().into_iter().any(|iter_lang| {
                if iter_lang == lang {
                    // Skip the language we are currently looking up
                    false
                } else {
                    if let Some(lang_string) = self.get(iter_lang) {
                        // Check for a non-empty string
                        lang_string.is_empty()
                    } else {
                        false
                    }
                }
            });
            if non_empty {
                // Now we can say that there are string(s) that aren't translated yet
                match lang {
                    Lang::EN => "[Untranslated]",
                    Lang::JP => "[未訳]",
                }
            } else {
                // An empty string is good since no language has any strings
                ""
            }
        }, |string| string.as_str())
    }
}

/// These are all of the language strings for the front-end UI
#[derive(Copy, Clone, Serialize)]
pub struct UiLangString {
    // These are the Lang short strings
    pub en: &'static str,
    pub jp: &'static str,

    pub app_title: &'static str,
    pub cancel: &'static str,
    pub create: &'static str,
    pub edit: &'static str,
    pub filter: &'static str,
    pub language: &'static str,
    pub name: &'static str,
    pub need_json_request: &'static str,
    pub release_date: &'static str,
    pub save: &'static str,
    pub search: &'static str,
    pub shutdown_response: &'static str,
    pub source_new: &'static str,
    pub sources_search_pl: &'static str,
    pub universe: &'static str,
    pub universe_create: &'static str,
    pub universe_new: &'static str,
}
impl UiLangString {
    pub fn new(lang: Lang) -> UiLangString {
        match lang {
            Lang::EN => UiLangString {
                en: "English",
                jp: "Japanese",

                app_title: "Completion Tracker",
                cancel: "Cancel",
                create: "Create",
                edit: "Edit",
                filter: "Filter",
                language: "Language",
                name: "Name",
                need_json_request: "The POST body must be JSON",
                release_date: "Release Date",
                save: "Save",
                search: "Search",
                shutdown_response: "Shutting down the server…",
                source_new: "New Source",
                sources_search_pl: "Search for Sources, Universes, Series, Arcs, etc.",
                universe: "Universe",
                universe_create: "Create a Universe",
                universe_new: "New Universe",
            },
            Lang::JP => UiLangString {
                en: "英語",
                jp: "日本語",

                app_title: "成績トラッカ",
                cancel: "キャンセル",
                create: "作成",
                edit: "変更",
                filter: "絞り込む",
                language: "言語",
                name: "名",
                need_json_request: "POST本文はJSONに必要です",
                release_date: "初発行日",
                save: "セーブ",
                search: "検索",
                shutdown_response: "サーバがシャットダウンしています…",
                source_new: "新しいソース",
                sources_search_pl: "ソースや世界やシリーズやアークなどと検索してみよう",
                universe: "世界",
                universe_create: "世界を作成",
                universe_new: "新しい世界",
            },
        }
    }
}
