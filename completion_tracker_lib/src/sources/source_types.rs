mod roles;
pub use roles::{CompanyRole, PersonRole};

use std::collections::{BTreeMap};

use serde::{
    Deserialize, Serialize,
    ser::{SerializeStruct},
};
use url::{Url, Host};

use crate::{
    impl_sql_simple_enum,
    simple_enum::{SimpleEnum},
};

#[derive(Copy, Clone, Deserialize, Serialize, Eq, Ord, PartialEq, PartialOrd)]
pub enum Lang {
    English,
    Japanese,
}
pub type LangMap = BTreeMap<Lang, String>;
pub type LangMapList = BTreeMap<Lang, Vec<String>>;

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


#[derive(Copy, Clone, Deserialize, Serialize)]
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
impl RelatedLink {
    pub fn new(url: &str, descriptions: LangMap) -> Result<RelatedLink, String> {
        let url = Url::parse(url).map_err(|e| e.to_string())?;
        let host = if let Some(host) = url.host() {
            match host {
                Host::Domain(string) => string,
                Host::Ipv4(_) | Host::Ipv6(_) =>
                    return Err(format!("The URL cannot be an IP address: {}", url.as_str())),
            }
        } else {
            return Err(format!("Failed to find a host for: {}", url.as_str()));
        };
        let link_type = if let Some(link_type) = LinkType::from_host(host) {
            link_type
        } else {
            return Err(format!("Failed to find a link type for: {}", url.as_str()));
        };
        Ok(RelatedLink {
            url,
            link_type,
            descriptions,
        })
    }
    pub fn url(&self) -> &Url { &self.url }
    pub fn link_type(&self) -> LinkType { self.link_type }
    pub fn descriptions(&self) -> &LangMap { &self.descriptions }
}
/// Implemented because URL is not serializable
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
    fn from_host(host: &str) -> Option<LinkType> {
        for (host_part, link_type) in LinkType::host_pairs().iter() {
            if host.contains(host_part) { return Some(*link_type); }
        }
        None
    }
}

#[derive(Copy, Clone, Deserialize, Serialize)]
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
impl_sql_simple_enum!(SourceType);

#[derive(Copy, Clone, Deserialize, Serialize)]
pub enum Country {
    Japan,
}
impl SimpleEnum for Country {
    fn all() -> &'static [Country] {
        &[
            Self::Japan,
        ]
    }
    fn as_str(&self) -> &'static str {
        match self {
            Self::Japan => "Japan",
        }
    }
}
impl_sql_simple_enum!(Country);
