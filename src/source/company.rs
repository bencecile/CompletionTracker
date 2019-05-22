use serde_derive::{Deserialize, Serialize};

use crate::lang::{LangString, LangStrings};
use crate::source::{ImageType};

/// This is a company
#[derive(Clone, Deserialize, Serialize)]
pub struct Company {
    /// The ID of the company
    id: u64,
    /// These are the translated names of the company
    names: LangStrings,
    /// The type of image for this company
    image: Option<ImageType>,
    /// Any aliases (for names) that the company may have
    aliases: Vec<LangStrings>,
    /// These all of the sources in which the company has a role in
    sources: Vec<(u64, CompanyRole)>,
    /// The people who are or have at one point, been affiliated with this company
    people: Vec<u64>,
}
impl Company {
    /// Creates a new character with just a name
    pub fn new(name: LangString, id: u64) -> Company {
        Company {
            id,
            names: LangStrings::new(name),
            image: None,
            aliases: Vec::new(),
            sources: Vec::new(),
            people: Vec::new(),
        }
    }
    pub fn id_eq(&self, id: u64) -> bool { self.id == id }
    pub fn sources_iter(&self) -> impl Iterator<Item = &(u64, CompanyRole)> {
        self.sources.iter()
    }

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

/// These are the roles that a character can perform as, in a source
#[derive(Clone, Deserialize, Serialize)]
pub enum CompanyRole {
    Developer,
    Publisher,
}
