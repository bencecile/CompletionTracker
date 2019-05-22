use serde_derive::{Deserialize, Serialize};

use crate::lang::{LangString, LangStrings};
use crate::source::{ImageType};

/// This is a fictional character
#[derive(Clone, Deserialize, Serialize)]
pub struct Character {
    /// The ID of the character
    id: u64,
    /// These are the translated names of the character
    names: LangStrings,
    /// The type of image for this character
    image: Option<ImageType>,
    /// Any aliases (for names) that the character may have
    aliases: Vec<LangStrings>,
    /// These all of the sources in which the character appears
    sources: Vec<(u64, CharacterRole)>,
}
impl Character {
    /// Creates a new character with just a name
    pub fn new(name: LangString, id: u64) -> Character {
        Character {
            id,
            names: LangStrings::new(name),
            image: None,
            aliases: Vec::new(),
            sources: Vec::new(),
        }
    }
    pub fn id_eq(&self, id: u64) -> bool { self.id == id }
    pub fn sources_iter(&self) -> impl Iterator<Item = &(u64, CharacterRole)> {
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
pub enum CharacterRole {
    MainCharacter,
}
