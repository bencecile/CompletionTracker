use super::endpoints::data::{DataError};
use crate::lang::{Translatable, UILangStrings};

impl Translatable for DataError {
    fn to_lang_strings(&self) -> UILangStrings {
        match self {
            DataError::BadJson(json_err_string) => UILangStrings {
                en: format!("Bad JSON format: '{}'", &json_err_string),
                jp: format!("JSON形式は不適です: 「{}」", &json_err_string),
            },
            DataError::BadLang(lang_str) => UILangStrings {
                en: format!("'{}' is not a language short form", lang_str),
                jp: format!("「{}」は略称した言語ではありません", lang_str)
            },
            DataError::BadLink(link_error) => link_error.to_lang_strings(),
            DataError::EmptyName => UILangStrings {
                en: "A name cannot be empty".to_string(),
                jp: "虚しい名とは良くありません".to_string(),
            },
            DataError::MissingId => UILangStrings {
                en: "The ID is missing".to_string(),
                jp: "IDがは見つけません".to_string(),
            },
            DataError::MissingLanguage => UILangStrings {
                en: "The language is missing".to_string(),
                jp: "言語が見つけません".to_string(),
            },
            DataError::MissingName => UILangStrings {
                en: "The name is missing".to_string(),
                jp: "名が見つけません".to_string(),
            },
            DataError::MissingUrl => UILangStrings {
                en: "A URL (key, value) is missing".to_string(),
                jp: "URLの(key, value)が見つけません".to_string(),
            },
            DataError::SourceError(source_err) => source_err.to_lang_strings(),
        }
    }
}
