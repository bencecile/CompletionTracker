use std::collections::{HashMap};

use serde_derive::{Deserialize, Serialize};

/// These are the different languages we support
#[derive(Copy, Clone, Hash, PartialEq, Eq, Deserialize, Serialize)]
pub enum LangSelect {
    /// English
    EN,
    /// Japanese
    JP,
}
impl LangSelect {
    /// Returns all of the possible languages
    pub fn all_langs() -> Vec<LangSelect> {
        vec![
            LangSelect::EN,
            LangSelect::JP,
        ]
    }

    /// Gets the short string version of this language
    pub fn short_str(&self) -> &'static str {
        match self {
            LangSelect::EN => "en",
            LangSelect::JP => "jp",
        }
    }
    /// Gets all of the possible short strings for the languages
    pub fn all_short_str() -> Vec<&'static str> {
        LangSelect::all_langs().into_iter()
            .map(|lang| lang.short_str())
            .collect()
    }
    /// Tries to get the language from a short string
    pub fn from_short_str(short: &str) -> Option<LangSelect> {
        for (lang_str, lang) in LangSelect::all_short_str().into_iter()
        .zip(LangSelect::all_langs()) {
            if lang_str == short {
                return Some(lang);
            }
        }

        None
    }
}

/// A single language string
pub type LangString = (LangSelect, String);
/// These are strings that are for a specific language
pub type LangStrings = HashMap<LangSelect, String>;
/// Creates a new lang strings map, inserting the single one
pub fn new_lang_strings(lang_string: LangString) -> LangStrings {
    let mut map = HashMap::new();
    map.insert(lang_string.0, lang_string.1);
    map
}

#[derive(Copy, Clone)]
pub struct Lang {
    lang: LangSelect,
    ui: UILang,
}
impl Lang {
    /// Creates a new language object
    pub fn new(lang: LangSelect) -> Lang {
        Lang {
            lang,
            ui: UILang::new(lang),
        }
    }
    pub fn lang(&self) -> LangSelect { self.lang }
    pub fn ui(&self) -> UILang { self.ui }

    pub fn date_err(&self, date: &str) -> String {
        match self.lang {
            LangSelect::EN => format!("The date format is YYYY-MM-DD. We got '{}'", date),
            LangSelect::JP => format!("TODO {}", date),
        }
    }
    pub fn media_type_err_missing(&self, name: &str) -> String {
        match self.lang {
            LangSelect::EN => format!("The media type '{}' doesn't exist", name),
            LangSelect::JP => format!("TODO {}", name),
        }
    }
    pub fn source_err_bad_id_relation(&self, id: u64) -> String {
        match self.lang {
            LangSelect::EN => format!("The ID '{}' cannot be found in a source relation", id),
            LangSelect::JP => format!("TODO {}", id)
        }
    }
    pub fn source_err_id_not_matching(&self, id: u64) -> String {
        match self.lang {
            LangSelect::EN => format!("A source '{}' doesn't match its index", id),
            LangSelect::JP => format!("TODO {}", id),
        }
    }
    pub fn source_err_missing_series(&self) -> String {
        match self.lang {
            LangSelect::EN =>
                "This source is missing a series. One is needed for an arc".to_string(),
            LangSelect::JP => "TODO".to_string(),
        }
    }
    pub fn source_err_missing_universe(&self) -> String {
        match self.lang {
            LangSelect::EN =>
                "This source is missing a universe. One is needed for a series".to_string(),
            LangSelect::JP => "TODO".to_string(),
        }
    }
    pub fn source_err_not_found(&self, id: u64) -> String {
        match self.lang {
            LangSelect::EN => format!("A source with an ID of '{}' couldn't be found", id),
            LangSelect::JP => format!("TODO {}", id),
        }
    }
    pub fn universe_err_non_matching_id(&self, u_id: u64) -> String {
        match self.lang {
            LangSelect::EN => format!("A universe '{}' didn't match its index", u_id),
            LangSelect::JP => format!("TODO {}", u_id),
        }
    }
    pub fn universe_err_non_matching_id_arc(&self, u_id: u64, s_id: u64, a_id: u64) -> String {
        match self.lang {
            LangSelect::EN => format!(
                "An arc '{}' in series '{}', universe '{}' didn't match its index",
                a_id, s_id, u_id),
            LangSelect::JP => format!("TODO {} {} {}", u_id, s_id, a_id),
        }
    }
    pub fn universe_err_non_matching_id_series(&self, u_id: u64, s_id: u64) -> String {
        match self.lang {
            LangSelect::EN => format!(
                "A series '{}' in universe '{}' didn't match its index", s_id, u_id),
            LangSelect::JP => format!("TODO {} {}", u_id, s_id),
        }
    }
    pub fn universe_err_not_found(&self, u_id: u64) -> String {
        match self.lang {
            LangSelect::EN => format!("A universe with an ID of '{}' was not found", u_id),
            LangSelect::JP => format!("TODO {}", u_id),
        }
    }
    pub fn universe_err_not_found_arc(&self, u_id: u64, s_id: u64, a_id: u64) -> String {
        match self.lang {
            LangSelect::EN => format!("An arc '{}' in series '{}', universe '{}' was not found",
                a_id, s_id, u_id),
            LangSelect::JP => format!("TODO {} {} {}", u_id, s_id, a_id),
        }
    }
    pub fn universe_err_not_found_series(&self, u_id: u64, s_id: u64) -> String {
        match self.lang {
            LangSelect::EN => format!("A series '{}' in universe '{}' was not found", s_id, u_id),
            LangSelect::JP => format!("TODO {} {}", u_id, s_id),
        }
    }
}

/// These are all of the language strings for the front-end UI
#[derive(Copy, Clone, Serialize)]
pub struct UILang {
    // These are the LangSelect short strings
    en: &'static str,
    jp: &'static str,

    cancel: &'static str,
    create: &'static str,
    edit: &'static str,
    filter: &'static str,
    lang: &'static str,
    name: &'static str,
    release_date: &'static str,
    save: &'static str,
    search: &'static str,
    source_new: &'static str,
    sources_search_pl: &'static str,
    universe: &'static str,
    universe_create: &'static str,
    universe_new: &'static str,
}
impl UILang {
    fn new(lang: LangSelect) -> UILang {
        match lang {
            LangSelect::EN => UILang {
                en: "English",
                jp: "Japanese",

                cancel: "Cancel",
                create: "Create",
                edit: "Edit",
                filter: "Filter",
                lang: "Language",
                name: "Name",
                release_date: "Release Date",
                save: "Save",
                search: "Search",
                source_new: "New Source",
                sources_search_pl: "Search for Sources, Universes, Series, Arcs, etc.",
                universe: "Universe",
                universe_create: "Create a Universe",
                universe_new: "New Universe",
            },
            LangSelect::JP => UILang {
                en: "英語",
                jp: "日本語",

                cancel: "キャンセル",
                create: "作成",
                edit: "変更",
                filter: "絞り込む",
                lang: "言語"
                name: "名",
                release_date: "初発行日",
                save: "セーブ",
                search: "検索",
                source_new: "新しいソース",
                sources_search_pl: "ソースや世界やシリーズやアークなどと検索してみ",
                universe: "世界",
                universe_create: "世界を作成",
                universe_new: "新しい世界",
            },
        }
    }
}
