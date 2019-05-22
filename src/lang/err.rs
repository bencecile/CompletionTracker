use crate::api::data::{DataError};
use crate::lang::{Lang, Translatable};
use crate::source::{SourceItem};
use crate::source::err::{LinkError, SourceError, SourceErrorType};
use crate::types::{Date, DateError};

impl Translatable for DataError {
    fn to_lang_string(&self, lang: Lang) -> String {
        match self {
            DataError::MissingId => match lang {
                Lang::EN => "The ID is missing".to_string(),
                Lang::JP => "IDはありません".to_string(),
            },
            DataError::MissingLanguage => match lang {
                Lang::EN => "The language is missing".to_string(),
                Lang::JP => "言語はありません".to_string(),
            },
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
        match self {
            SourceError::IDNotMatching(err_type) => match lang {
                Lang::EN => format!("{} doesn't match its index", err_type.to_lang_string(lang)),
                Lang::JP => format!("{}は指数に合いません", err_type.to_lang_string(lang)),
            },
            SourceError::MissingSeries => match lang {
                Lang::EN => r#"This source is missing a series."
                    " One is needed for an arc"#.to_string(),
                Lang::JP => "このソースがシリーズはありません。アークとして必要です".to_string(),
            },
            SourceError::MissingUniverse => match lang {
                Lang::EN => r#"This source is missing a universe."
                    " One is needed for a series"#.to_string(),
                Lang::JP => "このソースが世界はありません。シリーズとして必要です".to_string(),
            },
            SourceError::NotFound(err_type) => match lang {
                Lang::EN => format!("{} can't be found", err_type.to_lang_string(lang)),
                Lang::JP => format!("{}は見つけません", err_type.to_lang_string(lang)),
            },
            SourceError::NotFoundSourceRelation(err_type, source_id) => match lang {
                Lang::EN => format!("{} with a source ID '{}' in a relation can't be found",
                    err_type.to_lang_string(lang), source_id),
                Lang::JP => format!("{}のソースID「{}」は関係で見つけません",
                    err_type.to_lang_string(lang), source_id),
            },
        }
    }
}
impl Translatable for SourceErrorType {
    fn to_lang_string(&self, lang: Lang) -> String {
        match self {
            SourceErrorType::Arc(u_id, s_id, a_id) => match lang {
                Lang::EN => format!("An Arc '{}' in Universe '{}'/Series '{}'", a_id, u_id, s_id),
                Lang::JP => format!("アーク「{}」は世界「{}」／シリーズ「{}」で", a_id, u_id, s_id),
            },
            SourceErrorType::Character(id) => match lang {
                Lang::EN => format!("A Character '{}'", id),
                Lang::JP => format!("キャラ「{}」", id),
            },
            SourceErrorType::Company(id) => match lang {
                Lang::EN => format!("A Company '{}'", id),
                Lang::JP => format!("会社「{}」", id),
            },
            SourceErrorType::Person(id) => match lang {
                Lang::EN => format!("A Person '{}'", id),
                Lang::JP => format!("人「{}」", id),
            },
            SourceErrorType::Series(u_id, s_id) => match lang {
                Lang::EN => format!("A Series '{}' in Universe '{}'", s_id, u_id),
                Lang::JP => format!("シリーズ「{}」は世界「{}」で", s_id, u_id),
            },
            SourceErrorType::Source(id) => match lang {
                Lang::EN => format!("A Source '{}'", id),
                Lang::JP => format!("ソース「{}」", id),
            },
            SourceErrorType::Universe(id) => match lang {
                Lang::EN => format!("A Universe '{}'", id),
                Lang::JP => format!("世界「{}」", id),
            },
        }
    }
}
