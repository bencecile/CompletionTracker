use std::collections::{BTreeMap};

use serde::ser::{SerializeStruct};
use serde_derive::{Serialize};
use url::{Url};

use super::simple_enum::{SimpleEnum};
use crate::{impl_sql_simple_enum};

#[derive(Copy, Clone, Serialize, Eq, Ord, PartialEq, PartialOrd)]
pub enum Lang {
    English,
    Japanese,
}
pub type LangMap = BTreeMap<Lang, String>;

impl SimpleEnum for Lang {
    fn all() -> &'static [Lang] {
        &[
            Self::English,
            Self::Japanese,
        ]
    }
    fn as_str(&self) -> &'static str {
        match self {
            Self::English => "English",
            Self::Japanese => "Japanese",
        }
    }
}
impl_sql_simple_enum!(Lang);


#[derive(Copy, Clone, Serialize)]
pub enum Relation {
    Before,
    After,
    Alternate,
}
impl SimpleEnum for Relation {
    fn all() -> &'static [Relation] {
        &[
            Self::Before,
            Self::After,
            Self::Alternate,
        ]
    }
    fn as_str(&self) -> &'static str {
        match self {
            Self::Before => "Before",
            Self::After => "After",
            Self::Alternate => "Alternate",
        }
    }
}
impl_sql_simple_enum!(Relation);

pub struct RelatedLink {
    url: Url,
    link_type: LinkType,
    descriptions: LangMap,
}
impl serde::Serialize for RelatedLink {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut s = serializer.serialize_struct("RelatedLink", 3)?;
        s.serialize_field("url", self.url.as_str())?;
        s.serialize_field("link_type", &self.link_type)?;
        s.serialize_field("descriptions", &self.descriptions)?;
        s.end()
    }
}
#[derive(Copy, Clone, Serialize)]
pub enum LinkType {
    /// This is for https://syosetu.com/ and the novels that it contains
    ShousetsukaNarou,
    /// Any part of wikipedia.org
    Wikipedia,
}
impl SimpleEnum for LinkType {
    fn all() -> &'static [LinkType] {
        &[
            Self::ShousetsukaNarou,
            Self::Wikipedia,
        ]
    }
    fn as_str(&self) -> &'static str {
        match self {
            Self::ShousetsukaNarou => "ShousetsukaNarou",
            Self::Wikipedia => "Wikipedia",
        }
    }
}
impl LinkType {
    /// Pairs with a URL and the matching link type
    fn host_pairs() -> &'static [(&'static str, LinkType)] {
        &[
            ("syosetu.com", Self::ShousetsukaNarou),
            ("wikipedia.org", Self::Wikipedia),
        ]
    }
    pub fn from_host(host: &str) -> Option<LinkType> {
        for (host_part, link_type) in LinkType::host_pairs().iter() {
            if host.contains(host_part) { return Some(*link_type); }
        }
        None
    }
}

#[derive(Copy, Clone, Serialize)]
pub enum SourceType {
    Comic, // 漫画
    ComicRunning, // 連載の漫画話 (原点は関係のないこと)
    Novel, // 小説
    WebNovel, // Web小説
    TVShow, // アニメやドラマ
    Movie, // 映画や劇場版
}
impl SimpleEnum for SourceType {
    fn all() -> &'static [SourceType] {
        &[
            Self::Comic,
            Self::ComicRunning,
            Self::Novel,
            Self::WebNovel,
            Self::TVShow,
            Self::Movie,
        ]
    }

    fn as_str(&self) -> &'static str {
        match self {
            Self::Comic => "Comic",
            Self::ComicRunning => "ComicRunning",
            Self::Novel => "Novel",
            Self::WebNovel => "WebNovel",
            Self::TVShow => "TVShow",
            Self::Movie => "Movie",
        }
    }
}

#[derive(Copy, Clone, Serialize)]
pub enum SourceRole {
    Writer, // 著者
    Illustrator, // イラスト
    ComicArtist, // 漫画家 (絵だけ)
    /// A character's ID
    VoiceActor(u64), // 声優
}
