mod create;
pub use self::create::{create};

use serde::{Deserialize};

use crate::sources::source_types::{LangMap, LangMapList};

// TODO Make just a Sources page (link in the header next to home). Here we can put links to the universes and other Source related things


#[derive(Deserialize)]
pub struct PersonCreator {
    pub names: LangMap,
    pub descriptions: LangMap,
    pub aliases: LangMapList,
}
