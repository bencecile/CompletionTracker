use serde_derive::{Deserialize, Serialize};

use crate::source::{RecordInfo, SourceItem};

/// This is a company
#[derive(Clone, Deserialize, Serialize)]
pub struct Company {
}
impl RecordInfo for Company {
    fn source_item(&self) -> SourceItem { SourceItem::Company }
}

/// These are the roles that a character can perform as, in a source
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Deserialize, Serialize)]
pub enum CompanyRole {
    Developer,
    Publisher,
}
