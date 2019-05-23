use crate::lang::{Lang};
use crate::source::{
    LinkType, MediaType,
    Role, CharacterRole, CompanyRole,
};

/// This trait can guarantee that enums have translations similarilly
pub trait EnumTranslate where Self: Sized {
    /// Returns all of the variations of this enum
    fn all() -> Vec<Self>;
    /// Gives the translation string for the variation
    fn lang_str(&self, lang: Lang) -> &'static str;

    /// Returns a vector with the translations for all variations
    fn all_lang_str(lang: Lang) -> Vec<&'static str> {
        Self::all().into_iter()
            .map(|variation| variation.lang_str(lang))
            .collect()
    }
    /// Tries to convert the given translation string into a variant
    fn from_lang_str(lang_str: &str) -> Option<Self> {
        // Go through each language
        for lang in Lang::all() {
            // Try each role for a match
            for variation in Self::all() {
                if variation.lang_str(lang) == lang_str {
                    return Some(variation);
                }
            }
        }
        None
    }
}

impl EnumTranslate for MediaType {
    fn all() -> Vec<Self> {
        vec![
            MediaType::Book_WebNovel,
        ]
    }
    fn lang_str(&self, lang: Lang) -> &'static str {
        match self {
            MediaType::Book_WebNovel => match lang {
                Lang::EN => "Book - Web Novel",
                Lang::JP => "本 - Web小説",
            },
        }
    }
}

impl EnumTranslate for Role {
    /// Gets a list of all possible roles
    fn all() -> Vec<Self> {
        vec![
            Role::Writer,
            Role::VoiceActor(0),
        ]
    }
    fn lang_str(&self, lang: Lang) -> &'static str {
        match self {
            Role::Writer => match lang {
                Lang::EN => "Writer",
                Lang::JP => "著者",
            },
            Role::VoiceActor(_) => match lang {
                Lang::EN => "Voice Actor",
                Lang::JP => "声優",
            },
        }
    }
}

impl EnumTranslate for CharacterRole {
    fn all() -> Vec<Self> {
        vec![
            CharacterRole::MainCharacter,
        ]
    }
    fn lang_str(&self, lang: Lang) -> &'static str {
        match self {
            CharacterRole::MainCharacter => match lang {
                Lang::EN => "Main Character",
                Lang::JP => "主人公",
            },
        }
    }
}

impl EnumTranslate for CompanyRole {
    fn all() -> Vec<Self> {
        vec![
            CompanyRole::Developer,
            CompanyRole::Publisher,
        ]
    }
    fn lang_str(&self, lang: Lang) -> &'static str {
        match self {
            CompanyRole::Developer => match lang {
                Lang::EN => "Developer",
                Lang::JP => "開発元",
            },
            CompanyRole::Publisher => match lang {
                Lang::EN => "Publisher",
                Lang::JP => "発売元",
            },
        }
    }
}

impl EnumTranslate for LinkType {
    fn all() -> Vec<Self> {
        vec![
            LinkType::ShousetsukaNarou,
            LinkType::Wikipedia,
        ]
    }
    fn lang_str(&self, lang: Lang) -> &'static str {
        match self {
            LinkType::ShousetsukaNarou => match lang {
                Lang::EN => "Shousetsuka ni Narou",
                Lang::JP => "小説家になろう",
            },
            LinkType::Wikipedia => match lang {
                Lang::EN => "Wikipedia",
                Lang::JP => "ウィキペディア",
            },
        }
    }
}
