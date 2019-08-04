use std::collections::{BTreeMap};

use serde_derive::{Deserialize, Serialize};

/// The source lang defines the languages that can be used with sources
#[derive(Copy, Clone, PartialOrd, PartialEq, Ord, Eq, Deserialize, Serialize)]
pub enum SourceLang {
    EN,
    JP,
}
impl SourceLang {
    pub fn all() -> &'static [SourceLang] {
        &[
            SourceLang::EN,
            SourceLang::JP,
        ]
    }
    pub fn short_str(self) -> &'static str {
        match self {
            SourceLang::EN => "en",
            SourceLang::JP => "jp",
        }
    }

    pub fn from_short_str(short_str: &str) -> Option<SourceLang> {
        for lang in SourceLang::all().iter() {
            if lang.short_str() == short_str {
                return Some(*lang);
            }
        }
        None
    }
}

/// A single language string
pub type SourceLangString = (SourceLang, String);
/// These are strings that are for a specific language.
#[derive(Clone, Deserialize, Serialize)]
pub struct SourceLangStrings {
    /// The mapping of language to string
    // Flatten the structure so that the keys are inside the SourceLangStrings, not map: { ... }
    #[serde(flatten)]
    map: BTreeMap<SourceLang, String>,
}
impl SourceLangStrings {
    /// Returns an empty SourceLangStrings with None in every field
    pub fn new_empty() -> SourceLangStrings {
        SourceLangStrings { map: BTreeMap::new() }
    }
    /// Creates a new lang strings map, inserting a single one.
    /// We should always want at least 1 string in the translation table
    pub fn new(lang_string: SourceLangString) -> SourceLangStrings {
        let mut lang_strings = SourceLangStrings::new_empty();
        lang_strings.insert(lang_string);
        lang_strings
    }

    /// Replaces the value of this SourceLangString
    pub fn insert(&mut self, (lang, string): SourceLangString) {
        self.map.insert(lang, string);
    }
    
    // /// Gets the string for a single language
    // pub fn get_str(&self, lang: SourceLang) -> &str {
    //     // Just return an empty string if the entire map is empty
    //     if self.is_empty() {
    //         ""
    //     } else {
    //         self.map.get(&lang).map_or_else(|| {
    //             // Now we can say that there are string(s) that aren't translated yet
    //             match lang {
    //                 SourceLang::EN => "[Untranslated]",
    //                 SourceLang::JP => "[未訳]",
    //             }
    //         }, |string| string.as_str())
    //     }
    // }

    /// Checks to see if this map is empty. Also checks the actual value of every string so that
    /// there actually needs to be data in the map.
    pub fn is_empty(&self) -> bool {
        for lang in SourceLang::all().iter() {
            if let Some(string) = self.map.get(lang) {
                if !string.is_empty() {
                    // We actually have at least 1 string that isn't empty
                    return true;
                }
            }
        }
        false
    }
}
