use serde::{Deserialize, Serialize};

use super::{Lang};
use crate::{
    impl_sql_simple_enum,
    simple_enum::{SimpleEnum},
};

/// The role that a company can have in the creation of a source
#[derive(Copy, Clone, Deserialize, Serialize)]
pub enum CompanyRole {
    Developer,
    Publisher,
    Editor,
}
impl SimpleEnum for CompanyRole {
    fn all() -> &'static [CompanyRole] {
        &[
            Self::Developer,
            Self::Publisher,
            Self::Editor,
        ]
    }
    fn as_str(&self) -> &'static str {
        match self {
            Self::Developer => "Developer",
            Self::Publisher => "Publisher",
            Self::Editor => "Editor",
        }
    }
}
impl_sql_simple_enum!(CompanyRole);

/// The role a person can have in the creation of a source
#[derive(Copy, Clone, Deserialize, Serialize)]
pub enum PersonRole {
    Writer, // 著者
    Illustrator, // イラスト
    ComicArtist, // 漫画家 (絵だけ)
    /// A character's ID with their spoken language
    VoiceActor(u64, Lang), // 声優
}
