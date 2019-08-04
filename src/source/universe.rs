use serde_derive::{Deserialize, Serialize};

use super::{RecordInfo, Relation, SourceItem};

#[derive(Clone, Deserialize, Serialize)]
pub struct UniverseTag {
    /// Any tags that are more specific than this one
    child_tags: Vec<u64>,
    /// Any directly related universe tags
    related_tags: Vec<Relation>,
}
impl RecordInfo for UniverseTag {
    fn source_item() -> SourceItem { SourceItem::UniverseTag }
}
impl UniverseTag {
    pub fn new() -> UniverseTag {
        UniverseTag {
            child_tags: Vec::new(),
            related_tags: Vec::new(),
        }
    }
}
