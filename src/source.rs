mod character;
pub use self::character::{Character, CharacterRole};
mod company;
pub use self::company::{Company, CompanyRole};
pub mod err;
use self::err::{SourceError};
mod misc_types;
pub use self::misc_types::{
    Relation,
    ReleaseDate,
    RelatedLink, LinkType,
};
mod person;
pub use self::person::{Person, Role, EmploymentStatus, EmploymentPosition};
mod source;
pub use self::source::{Source, MediaType};
mod source_db;
pub use self::source_db::{SourceDB};
mod source_image;
pub use self::source_image::{ImageData, ImageType};
mod universe;
pub use self::universe::{Universe, Series, Arc};

use std::fs;
use std::ops::{Deref, DerefMut};
use std::path::{Path, PathBuf};

use serde_derive::{Deserialize, Serialize};

use crate::lang::{Lang, LangString, LangStrings};

/// This is the folder for all of the source information
const SOURCE_FOLDER: &'static str = "sources";
pub fn create_source_folder() -> Result<(), String> {
    let folder = Path::new(SOURCE_FOLDER);
    // Only try to create the folder if it isn't here
    if !folder.is_dir() {
        fs::create_dir(folder)
            .map_err(|e| e.to_string())
    } else {
        Ok(())
    }
}
/// Creates a path to the source file inside the source folder
pub fn source_file_path(file_name: &str) -> PathBuf {
    Path::new(SOURCE_FOLDER).join(file_name)
}

/// An item that is related to sources
#[derive(Copy, Clone)]
pub enum SourceItem {
    Source,

    Universe,
    /// A series belongs to a universe (u_id)
    Series(u64),
    /// An arc belongs to a series and universe by extension (u_id, s_id)
    Arc(u64, u64),

    Character,
    Person,
    Company,
}
impl SourceItem {
    /// Gets the path to the related list file for this item
    pub fn list_file(&self) -> PathBuf {
        match self {
            SourceItem::Source => source_file_path("sources.json"),
            SourceItem::Universe |
            SourceItem::Series(_) |
            SourceItem::Arc(_, _) => source_file_path("universes.json"),
            SourceItem::Character => source_file_path("characters.json"),
            SourceItem::Person => source_file_path("people.json"),
            SourceItem::Company => source_file_path("companies.json"),
        }
    }

    /// Gets the image directory for the item
    pub fn image_dir(&self) -> PathBuf {
        match self {
            SourceItem::Source => source_file_path("imgSources"),
            SourceItem::Universe => source_file_path("imgUniverses"),
            // The universes will have a directory that is the ID of the universe
            SourceItem::Series(u_id) => SourceItem::Universe.image_dir().join(format!("{}", u_id)),
            // Each series will have a directory with its ID
            SourceItem::Arc(u_id, s_id) => SourceItem::Series(*u_id).image_dir()
                .join(format!("{}", s_id)),
            SourceItem::Character => source_file_path("imgCharacters"),
            SourceItem::Person => source_file_path("imgPeople"),
            SourceItem::Company => source_file_path("imgCompanies"),
        }
    }
}

/// A common pairing of a source item with its ID
pub type SourceItemIDPair = (SourceItem, u64);

/// This is a common interface that all source items should have
pub trait RecordInfo {
    /// Gets the source item associated with this type, with ID
    fn source_item(&self) -> SourceItem;

    // ---- Default methods ----

}

/// This is a record for a source-related item
#[derive(Clone, Deserialize, Serialize)]
pub struct Record<T>
where T: RecordInfo {
    /// Every record will need a name
    names: LangStrings,
    /// Every record will need an identifier
    id: u64,
    /// There may be different or alternate names
    aliases: Vec<LangStrings>,
    /// A description of the record is usually important
    descriptions: LangStrings,
    /// An image for the record will help people recognize it
    image_type: Option<ImageType>,
    /// There's usually more information about something elsewhere
    related_links: Vec<RelatedLink>,
    /// The type specific data
    record_info: T,
}
impl <T: RecordInfo> Record<T> {
    /// Creates a new record from a name, id, and the record info.
    /// Anything else can get filled out after creation.
    pub fn new(name: LangString, id: u64, record_info: T) -> Record<T> {
        Record {
            names: LangStrings::new(name),
            id,
            aliases: Vec::new(),
            descriptions: LangStrings::new_empty(),
            image_type: None,
            related_links: Vec::new(),
            record_info,
        }
    }
}
/// Some simple getters
impl <T: RecordInfo> Record<T> {
    /// Gets a single name for a language
    pub fn name(&self, lang: Lang) -> &str { self.names.get_str(lang) }
    /// Gets a single description for a language
    pub fn description(&self, lang: Lang) -> &str { self.descriptions.get_str(lang) }
    /// Gets the list of aliases in a single language
    pub fn aliases(&self, lang: Lang) -> Vec<&str> {
        self.aliases.iter()
            .map(|alias| alias.get_str(lang))
            .collect()
    }
    /// Gets the pair of source item and ID for this record
    pub fn source_item_pair(&self) -> SourceItemIDPair { (self.record_info.source_item(), self.id) }
}
/// Some methods that add data to the record
impl <T: RecordInfo> Record<T> {
    /// Adds a name for the language
    // pub fn add_name(&mut self, name: LangString) { self.names.replace(name) }
    // /// Adds a description for the language
    // pub fn add_description(&mut self, description: LangString) {
    //     self.descriptions.replace(description)
    // }
    // /// Adds a completely new alias for a single language
    // pub fn add_new_alias(&mut self, alias: LangString) {
    //     self.aliases.push(LangStrings::new(alias));
    // }
    // /// Adds an alias to one that already exists
    // pub fn add_alias(&mut self, index: usize, alias: LangString) {
    //     self.aliases[index].replace(alias);
    // }
    /// Replace all of the links with new ones
    pub fn replace_links(&mut self, links: impl IntoIterator<Item=RelatedLink>) {
        // Try to preserve the allocated memory
        self.related_links.clear();
        self.related_links.extend(links);
    }
}
/// Any validation methods
impl <T: RecordInfo> Record<T> {
    /// Validate the record, as well as a function to verify the record info.
    /// An expected ID is used for validation.
    pub fn validate<F>(&self, expected_id: u64, fun: F) -> Result<(), SourceError>
    where F: FnOnce(&T) -> Result<(), SourceError> {
        // Check the expected ID
        if self.id != expected_id {
            return Err(SourceError::IDNotMatching(self.source_item_pair()));
        }
        // Check for the existence of an image if there should be one
        if let Some(image_result) = self.does_image_exist() {
            // Return an error if there was one
            image_result?;
        }

        // Use the function as the final validation
        fun(&self.record_info)
    }
}
/// Be able to simply access the record info inside
impl <T: RecordInfo> Deref for Record<T> {
    type Target = T;
    fn deref(&self) -> &T { &self.record_info }
}
/// Be able to change the record info with ease
impl <T: RecordInfo> DerefMut for Record<T> {
    fn deref_mut(&mut self) -> &mut T { &mut self.record_info }
}
