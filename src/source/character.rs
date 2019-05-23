use serde_derive::{Deserialize, Serialize};

use crate::source::{RecordInfo, SourceItem};

/// This is a fictional character
#[derive(Clone, Deserialize, Serialize)]
pub struct Character {
}
impl RecordInfo for Character {
    fn source_item(&self) -> SourceItem { SourceItem::Character }
}

/// These are the roles that a character can perform as, in a source
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Deserialize, Serialize)]
pub enum CharacterRole {
    MainCharacter,
}
