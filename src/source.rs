mod character;
mod company;
pub mod err;
mod misc_types;
mod person;
mod source;
mod source_lang;
mod universe;

pub use self::character::{Character, CharacterRole};
pub use self::company::{Company, CompanyRole};
use self::err::{SourceError};
pub use self::misc_types::{
    Country,
    ImageType,
    Relation,
    ReleaseDate,
    RelatedLink, LinkType,
};
pub use self::person::{Person, Role, EmploymentStatus, EmploymentPosition};
pub use self::source::{Source, MediaType};
pub use self::source_lang::{SourceLang, SourceLangString, SourceLangStrings};
pub use self::universe::{UniverseTag};

use std::collections::{BTreeMap};
use std::ops::{Deref, DerefMut};

use serde_derive::{Deserialize, Serialize};

/// An item that is related to sources
#[derive(Copy, Clone)]
pub enum SourceItem {
    Character,
    Company,
    Person,
    Source,
    UniverseTag,
}

/// A common pairing of a source item with its ID
pub type SourceItemIDPair = (SourceItem, u64);

/// This is a common interface that all source items should have
pub trait RecordInfo {
    /// The source item for this record
    fn source_item() -> SourceItem;
}

/// This is a record for a source-related item
#[derive(Clone, Deserialize, Serialize)]
pub struct Record<T>
where T: RecordInfo + Clone {
    /// Every record will need a name
    pub names: SourceLangStrings,
    /// Every record will need an identifier
    id: u64,
    /// There may be different or alternate names.
    /// It makes more sense to have the aliases specific to a single language.
    pub aliases: BTreeMap<SourceLang, Vec<String>>,
    /// A description of the record is usually important
    pub descriptions: SourceLangStrings,
    /// An image for the record will help people recognize the item
    pub image_type: Option<ImageType>,
    /// There's usually more information about something elsewhere
    pub related_links: Vec<RelatedLink>,
    /// The type specific data
    record_info: T,
}
impl <T: RecordInfo + Clone> Record<T> {
    /// Creates a new record from a name, id, and the record info.
    /// Anything else can get filled out after creation.
    pub fn new(names: SourceLangStrings, id: u64, record_info: T) -> Record<T> {
        // Push empty vectors into every language
        let mut aliases = BTreeMap::new();
        for lang in SourceLang::all().iter() {
            aliases.insert(*lang, Vec::new());
        }
        
        Record {
            names,
            id,
            aliases,
            descriptions: SourceLangStrings::new_empty(),
            image_type: None,
            related_links: Vec::new(),
            record_info,
        }
    }
}
/// Some simple getters
impl <T: RecordInfo + Clone> Record<T> {
    /// Gets the pair of source item and ID for this record
    pub fn source_item_pair(&self) -> SourceItemIDPair { (T::source_item(), self.id) }

}
/// Any validation methods
impl <T: RecordInfo + Clone> Record<T> {
    /// Validate the record, as well as a function to verify the record info.
    /// An expected ID is used for validation.
    pub fn validate<F>(&self, expected_id: u64, fun: F) -> Result<(), SourceError>
    where F: FnOnce(&Record<T>) -> Result<(), SourceError> {
        // Check the expected ID
        if self.id != expected_id {
            return Err(SourceError::IDNotMatching(self.source_item_pair()));
        }

        // Use the function as the final validation
        fun(&self)
    }
}
/// Be able to simply access the record info inside
impl <T: RecordInfo + Clone> Deref for Record<T> {
    type Target = T;
    fn deref(&self) -> &T { &self.record_info }
}
/// Be able to change the record info with ease
impl <T: RecordInfo + Clone> DerefMut for Record<T> {
    fn deref_mut(&mut self) -> &mut T { &mut self.record_info }
}
