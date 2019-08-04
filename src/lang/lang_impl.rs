use crate::lang::{UILang, UILangStrings, EnumTranslate, Translatable};
use crate::source::{
    Country,
    LinkType, MediaType,
    Role, CharacterRole, CompanyRole,
    SourceLang,
    SourceItem, SourceItemIDPair,
};
use crate::source::err::{LinkError, SourceError};
use crate::types::{Date, DateError};

impl EnumTranslate for CharacterRole {
    fn all() -> &'static [Self] {
        &[
            CharacterRole::MainCharacter,
        ]
    }
    fn lang_str(&self) -> UILangStrings {
        match self {
            CharacterRole::MainCharacter => UILangStrings {
                en: "Main Character".to_string(),
                jp: "主人公".to_string(),
            },
        }
    }
}
impl EnumTranslate for CompanyRole {
    fn all() -> &'static [Self] {
        &[
            CompanyRole::Developer,
            CompanyRole::Publisher,
        ]
    }
    fn lang_str(&self) -> UILangStrings {
        match self {
            CompanyRole::Developer => UILangStrings {
                en: "Developer".to_string(),
                jp: "開発元".to_string(),
            },
            CompanyRole::Publisher => UILangStrings {
                en: "Publisher".to_string(),
                jp: "発売元".to_string(),
            },
        }
    }
}
impl EnumTranslate for Country {
    fn all() -> &'static [Self] {
        &[
            Country::Japan,
        ]
    }
    fn lang_str(&self) -> UILangStrings {
        match self {
            Country::Japan => UILangStrings {
                en: "Japan".to_string(),
                jp: "日本".to_string(),
            },
        }
    }
}
impl EnumTranslate for LinkType {
    fn all() -> &'static [Self] {
        &[
            LinkType::ShousetsukaNarou,
            LinkType::Wikipedia,
        ]
    }
    fn lang_str(&self) -> UILangStrings {
        match self {
            LinkType::ShousetsukaNarou => UILangStrings {
                en: "Shousetsuka ni Narou".to_string(),
                jp: "小説家になろう".to_string(),
            },
            LinkType::Wikipedia => UILangStrings {
                en: "Wikipedia".to_string(),
                jp: "ウィキペディア".to_string(),
            },
        }
    }
}
impl EnumTranslate for MediaType {
    fn all() -> &'static [Self] {
        &[
            MediaType::Book_WebNovel,
        ]
    }
    fn lang_str(&self) -> UILangStrings {
        match self {
            MediaType::Book_WebNovel => UILangStrings {
                en: "Book - Web Novel".to_string(),
                jp: "本 - Web小説".to_string(),
            },
        }
    }
}
impl EnumTranslate for Role {
    /// Gets a list of all possible roles
    fn all() -> &'static [Self] {
        &[
            Role::Writer,
            Role::VoiceActor(0, SourceLang::EN),
        ]
    }
    fn lang_str(&self) -> UILangStrings {
        match self {
            Role::Writer => UILangStrings {
                en: "Writer".to_string(),
                jp: "著者".to_string(),
            },
            Role::VoiceActor(_, _) => UILangStrings {
                en: "Voice Actor".to_string(),
                jp: "声優".to_string(),
            },
        }
    }
}

// ---------- Start the error translation ----------

impl Translatable for SourceError {
    fn to_lang_strings(&self) -> UILangStrings {
        // Create a helper function to translate a source item pair
        let translate_err_type = |err_type: &SourceItemIDPair| {
            let id = err_type.1;
            match err_type.0 {
                SourceItem::Source => UILangStrings {
                    en: format!("A Source '{}'", id),
                    jp: format!("ソース「{}」", id),
                },
                SourceItem::UniverseTag => UILangStrings {
                    en: format!("A Universe Tag '{}'", id),
                    jp: format!("世界ターグ「{}」", id),
                },
                SourceItem::Character => UILangStrings {
                    en: format!("A Character '{}'", id),
                    jp: format!("キャラ「{}」", id),
                },
                SourceItem::Person => UILangStrings {
                    en: format!("A Person '{}'", id),
                    jp: format!("人「{}」", id),
                },
                SourceItem::Company => UILangStrings {
                    en: format!("A Company '{}'", id),
                    jp: format!("会社「{}」", id),
                },
            }
        };

        match self {
            SourceError::IDNotMatching(err_type) => UILangStrings {
                en: format!("{} doesn't match its index", translate_err_type(err_type).en),
                jp: format!("{}は指数に合いません", translate_err_type(err_type).jp),
            },
            SourceError::ImageBadBase64 => UILangStrings {
                en: "An image has a bad Base64 encoding".to_string(),
                jp: "画像のBase64エンコーディングは不適です".to_string(),
            },
            SourceError::ImageBadData => UILangStrings {
                en: "The image data cannot be read".to_string(),
                jp: "画像のデータが読み込めません".to_string(),
            },
            SourceError::ImageMissing(err_type) => UILangStrings {
                en: format!("{} has a missing image", translate_err_type(err_type).en),
                jp: format!("{}の画像が見つけません", translate_err_type(err_type).jp),
            },
            SourceError::ImageWriteFailed(image_path) => UILangStrings {
                en: format!("Failed to write an image to '{}'", image_path.display()),
                jp: format!("画像「{}」の書き込みが失敗しました", image_path.display()),
            },
            SourceError::MissingSeries => UILangStrings {
                en: r#"This source is missing a series."
                    " One is needed for an arc"#.to_string(),
                jp: "このソースでシリーズはありません。アークとして必要です".to_string(),
            },
            SourceError::MissingUniverse => UILangStrings {
                en: r#"This source is missing a universe."
                    " One is needed for a series"#.to_string(),
                jp: "このソースで世界はありません。シリーズとして必要です".to_string(),
            },
            SourceError::NotFound(err_type) => UILangStrings {
                en: format!("{} can't be found", translate_err_type(err_type).en),
                jp: format!("{}は見つけません", translate_err_type(err_type).jp),
            },
            SourceError::NotFoundRelation(err_type, related_id) => UILangStrings {
                en: format!("{} with a relation to '{}' can't be found",
                    translate_err_type(err_type).en,
                    translate_err_type( &(err_type.0, *related_id) ).en),
                jp: format!("関係した{}と「{}」が見つけません",
                    translate_err_type(err_type).jp,
                    translate_err_type( &(err_type.0, *related_id) ).jp),
            },
            SourceError::FailedSave => UILangStrings {
                en: "Failed to save the data to disk".to_string(),
                jp: "データ書き込みが失敗しました".to_string(),
            },
        }
    }
}

impl Translatable for LinkError {
    fn to_lang_strings(&self) -> UILangStrings {
        match self {
            LinkError::MalformedUrl(url) => UILangStrings {
                en: format!("This URL '{}' doesn't look like a URL", url),
                jp: format!("このURL「{}」の見た目はURLみたいにありません", url),
            },
            LinkError::UnknownLink(url) => UILangStrings {
                en: format!("This URL '{}' doesn't belong to a known site", url),
                jp: format!("このURL「{}」は知るサイトに所属していません", url),
            },
        }
    }
}

impl Translatable for DateError {
    fn to_lang_strings(&self) -> UILangStrings {
        // Get the date format that we will return in the message to be more helpful
        let date_format_en = Date::date_format(UILang::EN);
        let date_format_jp = Date::date_format(UILang::JP);
        match self {
            DateError::BadFormat => UILangStrings {
                en: format!("Invalid format. The format is '{}'", date_format_en),
                jp: format!("不適な形式。正しくのは「{}」", date_format_jp),
            },
            DateError::InvalidDay => UILangStrings {
                en: format!("Invalid day. The format is '{}'", date_format_en),
                jp: format!("不適な日。正しくのは「{}」", date_format_jp),
            },
            DateError::InvalidMonth => UILangStrings {
                en: format!("Invalid month. The format is '{}'", date_format_en),
                jp: format!("不適な月。正しくのは「{}」", date_format_jp),
            },
            DateError::InvalidYear => UILangStrings {
                en: format!("Invalid year. The format is '{}'", date_format_en),
                jp: format!("不適な年。正しくのは「{}」", date_format_jp),
            },
        }
    }
}
