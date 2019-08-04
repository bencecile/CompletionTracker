use std::collections::{BTreeSet};

use serde_derive::{Deserialize, Serialize};

use super::{
    RecordInfo, SourceItem,
    Relation, ReleaseDate,
    Role, CharacterRole, CompanyRole,
};
// use super::err::{SourceError};

/// This is the record info for a source
#[derive(Clone, Deserialize, Serialize)]
pub struct Source {
    /// The type of media that holds the source (the name)
    pub media_type: MediaType,
    /// Whether or not the source should be restricted to those 18 and older
    pub r18: bool,
    /// The IDs for the Universe(s) where this source lives
    pub universe_tags: Vec<u64>,
    /// A list of release dates for this source
    pub release_dates: Vec<ReleaseDate>,
    /// A list of related sources
    pub related_sources: BTreeSet<Relation>,
    /// A list of characters with their role that appear in this source
    pub characters: BTreeSet<(u64, CharacterRole)>,
    /// A list of people and their role that helped create this source
    pub people: BTreeSet<(u64, Role)>,
    /// A list of companies and their role that helped create this source
    pub companies: BTreeSet<(u64, CompanyRole)>,
}
impl RecordInfo for Source {
    fn source_item() -> SourceItem { SourceItem::Source }
}
impl Source {
    /// Creates new source record info with just a media type.
    /// Anything else can be filled out after.
    pub fn new(media_type: MediaType) -> Source {
        Source {
            media_type,
            r18: false,
            universe_tags: Vec::new(),
            release_dates: Vec::new(),
            related_sources: BTreeSet::new(),
            characters: BTreeSet::new(),
            people: BTreeSet::new(),
            companies: BTreeSet::new(),
        }
    }
}
/// These methods will modify the Source
impl Source {
    // /// Sets the R18 boolean flag for the source
    // pub fn set_r18(&mut self, new_r18: bool) { self.r18 = new_r18; }
    // /// Completely replaces the related sources with new ones
    // pub fn replace_related_sources(&mut self, related_sources: impl Iterator<Item=Relation>) {
    //     // Try to retain the alloc'd size so we don't have to reallocate all the memory
    //     self.related_sources.clear();
    //     self.related_sources.extend(related_sources);
    // }
}
/// Validation methods specific to a Source
impl Source {
}

/// The type of thing that a Source lives in
#[derive(Copy, Clone, Deserialize, Serialize)]
// It makes sense to me to deliminate categories with an underscore
#[allow(non_camel_case_types)]
pub enum MediaType {
    Book_WebNovel,
}
