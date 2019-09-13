mod create;
pub use self::create::{create};

use serde_derive::{Deserialize};

use crate::sources::source_types::{LangMap, LangMapList};

#[derive(Deserialize)]
pub struct CharacterCreator {
    pub names: LangMap,
    pub descriptions: LangMap,
    pub aliases: LangMapList,
    pub related_characters: Vec<(u64, LangMap)>,
}
