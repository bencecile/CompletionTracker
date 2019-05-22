use serde_derive::{Deserialize, Serialize};

use crate::lang::{LangString, LangStrings};
use crate::source::{ImageType};
use crate::types::{Date};

/// This is a person. They can contribute to the creation of Sources.
#[derive(Clone, Deserialize, Serialize)]
pub struct Person {
    /// The ID for this person
    id: u64,
    /// Their name and translated names
    names: LangStrings,
    /// The type of image for this person
    image: Option<ImageType>,
    /// Any aliases for this person
    aliases: Vec<LangStrings>,
    /// The sources that this person has worked on with their role
    sources: Vec<(u64, Role)>,
    /// The companies that this person has been employed
    employment: Vec<EmploymentStatus>,
}
impl Person {
    /// Returns an empty person with just their name
    pub fn new(name: LangString, id: u64) -> Person {
        Person {
            id,
            names: LangStrings::new(name),
            image: None,
            aliases: Vec::new(),
            sources: Vec::new(),
            employment: Vec::new(),
        }
    }
    pub fn id_eq(&self, id: u64) -> bool { self.id == id }
    pub fn sources_iter(&self) -> impl Iterator<Item = &(u64, Role)> { self.sources.iter() }

    /// Updates the references to the old source ID
    pub fn update_source_refs(&mut self, old_id: u64, new_id: u64) {
        self.sources.iter_mut()
            .for_each(|source| if source.0 == old_id {
                source.0 = new_id;
            });
    }
    /// Removes all references to this source ID
    pub fn remove_source_refs(&mut self, source_id: u64) {
        // Keep everything that isn't the one we want to remove
        // Only elements where iter_source_id == source_id are removed
        self.sources.retain(|(iter_source_id, _)| *iter_source_id != source_id);
    }
}

/// The employment status of a person.
/// This will correspond to an employment within the company.
#[derive(Clone, Deserialize, Serialize)]
pub struct EmploymentStatus {
    /// The ID of the company
    company_id: u64,
    /// The role they played during this portion of employment
    role: Vec<EmploymentPosition>,
    /// The start and end dates of this employment.
    /// If there is a date, there must be a start date, but not an end date
    date: Option<(Date, Option<Date>)>,
}

/// These are possible roles for a person
#[derive(Copy, Clone, Deserialize, Serialize)]
pub enum Role {
    /// The writer (ex. for a novel)
    Writer,
    /// A voice actor for a Character (using its ID for reference)
    VoiceActor(u64),
}

#[derive(Copy, Clone, Deserialize, Serialize)]
pub enum EmploymentPosition {
}
