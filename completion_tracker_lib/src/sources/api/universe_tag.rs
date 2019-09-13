mod create;
pub use self::create::{create};

mod read;

use serde_derive::{Deserialize, Serialize};

use crate::sources::source_types::{LangMap, RelatedLink, Relation};

#[derive(Deserialize)]
pub struct UniverseTagCreator {
    pub names: LangMap,
    pub descriptions: LangMap,
    pub parents: Vec<u64>,
    pub children: Vec<u64>,
    pub related_universe_tags: Vec<(u64, Relation)>,
    pub related_links: Vec<(String, LangMap)>,
}

#[derive(Serialize)]
pub struct UniverseTagReader {
    pub id: u64,
    pub names: LangMap,
    pub descriptions: LangMap,
    pub parents: Vec<u64>,
    pub children: Vec<u64>,
    pub related_universe_tags: Vec<(u64, Relation)>,
    pub related_links: Vec<RelatedLink>,
    pub sources: Vec<u64>,
}
