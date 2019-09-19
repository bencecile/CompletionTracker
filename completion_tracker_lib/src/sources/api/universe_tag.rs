mod create;
pub use self::create::{create};
mod read;
pub use self::read::{read_root_level_ids, read_list};

use serde_derive::{Deserialize, Serialize};

use crate::sources::source_types::{LangMap, RelatedLink, Relation};

#[derive(Deserialize)]
pub struct UniverseTagCreator {
    pub names: LangMap,
    pub descriptions: LangMap,
    pub related_links: Vec<(String, LangMap)>,
    pub parents: Vec<u64>,
    pub children: Vec<u64>,
    pub related_universe_tags: Vec<(u64, Relation)>,
}

#[derive(Serialize)]
pub struct UniverseTagReadResult {
    pub id: u64,
    pub names: LangMap,
    pub descriptions: LangMap,
    pub related_links: Vec<RelatedLink>,
    pub parents: Vec<u64>,
    pub children: Vec<u64>,
    pub related_universe_tags: Vec<(u64, Relation)>,
    pub sources: Vec<u64>,
}

#[derive(Deserialize)]
pub struct UniverseTagReader {
    pub ids: Vec<u64>,
}
