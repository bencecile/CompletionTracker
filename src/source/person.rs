use serde_derive::{Deserialize, Serialize};

use crate::source::{RecordInfo, SourceItem};
use crate::types::{Date};

/// This is a person. They can contribute to the creation of Sources.
#[derive(Clone, Deserialize, Serialize)]
pub struct Person {
    /// The companies where this person has been employed
    employment: Vec<EmploymentStatus>,
}
impl RecordInfo for Person {
    fn source_item(&self) -> SourceItem { SourceItem::Person }
}

/// The employment status of a person.
/// This will correspond to an employment within the company.
#[derive(Clone, Deserialize, Serialize)]
pub struct EmploymentStatus {
    /// The ID of the company
    company_id: u64,
    /// Their positions within the employment period
    role: Vec<EmploymentPosition>,
    /// The start and end dates of this employment.
    /// If there is a date, there must be a start date, but not always an end date.
    date: Option<(Date, Option<Date>)>,
}

/// These are possible roles for a person
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Deserialize, Serialize)]
pub enum Role {
    /// The writer (ex. for a novel)
    Writer,
    /// A voice actor for a Character (using its ID for reference)
    VoiceActor(u64),
}

/// A specific position that can be held within a company
#[derive(Copy, Clone, Deserialize, Serialize)]
pub enum EmploymentPosition {
}
