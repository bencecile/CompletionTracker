use serde_derive::{Deserialize, Serialize};

use super::{Country, RecordInfo, SourceItem};

/// This is a company
#[derive(Clone, Deserialize, Serialize)]
pub struct Company {
    /// The country where this record was founded (originated)
    country_of_origin: Option<Country>,
}
impl Company {
    pub fn new() -> Company {
        Company {
            country_of_origin: None,
        }
    }
}
impl RecordInfo for Company {
    fn source_item() -> SourceItem { SourceItem::Company }
}

/// These are the roles that a character can perform as, in a source
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Deserialize, Serialize)]
pub enum CompanyRole {
    Developer,
    Publisher,
}
