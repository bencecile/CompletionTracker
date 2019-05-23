use crate::api::data::{DataError};
use crate::lang::{Lang, Translatable};
use crate::source::{SourceItem, SourceItemIDPair};
use crate::source::err::{LinkError, SourceError};
use crate::types::{Date, DateError};

impl Translatable for DataError {
    fn to_lang_string(&self, lang: Lang) -> String {
        match self {
            DataError::BadLang(lang_str) => match lang {
                Lang::EN => format!("'{}' is not a language short form", lang_str),
                Lang::JP => format!("「{}」は略称した言語ではありません", lang_str)
            },
            DataError::BadLink(link_error) => link_error.to_lang_string(lang),
            DataError::EmptyName => match lang {
                Lang::EN => "A name cannot be empty".to_string(),
                Lang::JP => "虚しい名とは良くありません".to_string(),
            },
            DataError::MissingId => match lang {
                Lang::EN => "The ID is missing".to_string(),
                Lang::JP => "IDがは見つけません".to_string(),
            },
            DataError::MissingLanguage => match lang {
                Lang::EN => "The language is missing".to_string(),
                Lang::JP => "言語が見つけません".to_string(),
            },
            DataError::MissingName => match lang {
                Lang::EN => "The name is missing".to_string(),
                Lang::JP => "名が見つけません".to_string(),
            },
            DataError::MissingUrl => match lang {
                Lang::EN => "A URL (key, value) is missing".to_string(),
                Lang::JP => "URLの(key, value)が見つけません".to_string(),
            },
            DataError::SourceError(source_err) => source_err.to_lang_string(lang),
        }
    }
}

impl Translatable for DateError {
    fn to_lang_string(&self, lang: Lang) -> String {
        // Get the date format that we will return in the message to be more helpful
        let date_format = Date::date_format(lang, true);
        match self {
            DateError::BadFormat => match lang {
                Lang::EN => format!("Invalid format. The format is '{}'", date_format),
                Lang::JP => format!("不適な形式。正しくのは「{}」", date_format),
            },
            DateError::InvalidDay => match lang {
                Lang::EN => format!("Invalid day. The format is '{}'", date_format),
                Lang::JP => format!("不適な日。正しくのは「{}」", date_format),
            },
            DateError::InvalidMonth => match lang {
                Lang::EN => format!("Invalid month. The format is '{}'", date_format),
                Lang::JP => format!("不適な月。正しくのは「{}」", date_format),
            },
        }
    }
}

impl Translatable for LinkError {
    fn to_lang_string(&self, lang: Lang) -> String {
        match self {
            LinkError::MalformedUrl(url) => match lang {
                Lang::EN => format!("This URL '{}' doesn't look like a URL", url),
                Lang::JP => format!("このURL「{}」の見た目はURLみたいにありません", url),
            },
            LinkError::UnknownLink(url) => match lang {
                Lang::EN => format!("This URL '{}' doesn't belong to a known site", url),
                Lang::JP => format!("このURL「{}」は知るサイトに所属していません", url),
            },
        }
    }
}

impl Translatable for SourceError {
    fn to_lang_string(&self, lang: Lang) -> String {
        // Create a helper function to translate a source item pair
        let translate_err_type = |err_type: &SourceItemIDPair| {
            let id = err_type.1;
            match err_type.0 {
                SourceItem::Source => match lang {
                    Lang::EN => format!("A Source '{}'", id),
                    Lang::JP => format!("ソース「{}」", id),
                },
                SourceItem::Universe => match lang {
                    Lang::EN => format!("A Universe '{}'", id),
                    Lang::JP => format!("世界「{}」", id),
                },
                SourceItem::Series(u_id) => match lang {
                    Lang::EN => format!("A Series '{}' in Universe '{}'", id, u_id),
                    Lang::JP => format!("シリーズ「{}」は世界「{}」で", id, u_id),
                },
                SourceItem::Arc(u_id, s_id) => match lang {
                    Lang::EN => format!("An Arc '{}' in Universe '{}'/Series '{}'", id, u_id, s_id),
                    Lang::JP => format!("アーク「{}」は世界「{}」／シリーズ「{}」で", id, u_id, s_id),
                },
                SourceItem::Character => match lang {
                    Lang::EN => format!("A Character '{}'", id),
                    Lang::JP => format!("キャラ「{}」", id),
                },
                SourceItem::Person => match lang {
                    Lang::EN => format!("A Person '{}'", id),
                    Lang::JP => format!("人「{}」", id),
                },
                SourceItem::Company => match lang {
                    Lang::EN => format!("A Company '{}'", id),
                    Lang::JP => format!("会社「{}」", id),
                },
            }
        };

        match self {
            SourceError::IDNotMatching(err_type) => match lang {
                Lang::EN => format!("{} doesn't match its index", translate_err_type(err_type)),
                Lang::JP => format!("{}は指数に合いません", translate_err_type(err_type)),
            },
            SourceError::ImageBadBase64 => match lang {
                Lang::EN => "An image has a bad Base64 encoding".to_string(),
                Lang::JP => "画像のBase64エンコーディングは不適です".to_string(),
            },
            SourceError::ImageBadData => match lang {
                Lang::EN => "The image data cannot be read".to_string(),
                Lang::JP => "画像のデータが読み込めません".to_string(),
            },
            SourceError::ImageMissing(err_type) => match lang {
                Lang::EN => format!("{} has a missing image", translate_err_type(err_type)),
                Lang::JP => format!("{}の画像が見つけません", translate_err_type(err_type)),
            },
            SourceError::ImageWriteFailed(image_path) => match lang {
                Lang::EN => format!("Failed to write an image to '{}'", image_path.display()),
                Lang::JP => format!("画像「{}」の書き込みが失敗しました", image_path.display()),
            },
            SourceError::MissingSeries => match lang {
                Lang::EN => r#"This source is missing a series."
                    " One is needed for an arc"#.to_string(),
                Lang::JP => "このソースでシリーズはありません。アークとして必要です".to_string(),
            },
            SourceError::MissingUniverse => match lang {
                Lang::EN => r#"This source is missing a universe."
                    " One is needed for a series"#.to_string(),
                Lang::JP => "このソースで世界はありません。シリーズとして必要です".to_string(),
            },
            SourceError::NotFound(err_type) => match lang {
                Lang::EN => format!("{} can't be found", translate_err_type(err_type)),
                Lang::JP => format!("{}は見つけません", translate_err_type(err_type)),
            },
            SourceError::NotFoundRelation(err_type, related_id) => match lang {
                Lang::EN => format!("{} with a relation to '{}' can't be found",
                    translate_err_type(err_type), translate_err_type( &(err_type.0, *related_id) )),
                Lang::JP => format!("関係した{}と「{}」が見つけません",
                    translate_err_type(err_type), translate_err_type( &(err_type.0, *related_id) )),
            },
        }
    }
}
