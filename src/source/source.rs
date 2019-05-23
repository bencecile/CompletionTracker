use std::collections::{BTreeSet};

use serde_derive::{Deserialize, Serialize};

use crate::source::{
    RecordInfo, SourceItem,
    Relation, ReleaseDate,
    Role, CharacterRole, CompanyRole,
};
use crate::source::err::{SourceError};

/// This is the record info for a source
#[derive(Clone, Deserialize, Serialize)]
pub struct Source {
    /// The type of media that holds the source (the name)
    media_type: MediaType,
    /// The ID for the Universe where this source lives
    universe: Option<u64>,
    /// The ID of the series inside the Universe
    series: Option<u64>,
    /// The ID of the arc within the series
    arc: Option<u64>,
    /// A list of release dates for this source
    release_dates: Vec<ReleaseDate>,
    /// A list of related sources
    related_sources: BTreeSet<Relation>,
    /// A list of characters with their role that appear in this source
    characters: BTreeSet<(u64, CharacterRole)>,
    /// A list of people and their role that helped create this source
    people: BTreeSet<(u64, Role)>,
    /// A list of companies and their role that helped create this source
    companies: BTreeSet<(u64, CompanyRole)>,
}
impl RecordInfo for Source {
    fn source_item(&self) -> SourceItem { SourceItem::Source }
}
impl Source {
    /// Creates new source record info with just a media type.
    /// Anything else can be filled out after.
    pub fn new(media_type: MediaType) -> Source {
        Source {
            media_type,
            universe: None,
            series: None,
            arc: None,
            release_dates: Vec::new(),
            related_sources: BTreeSet::new(),
            characters: BTreeSet::new(),
            people: BTreeSet::new(),
            companies: BTreeSet::new(),
        }
    }

    /// Returns the universe, series and arc IDs
    pub fn universe(&self) -> Option<(u64, Option<(u64, Option<u64>)>)> {
        // Go through each level to check if each one exists
        if let Some(u_id) = self.universe {
            if let Some(s_id) = self.series {
                if let Some(a_id) = self.arc {
                    // We found them all
                    Some( (u_id, Some( (s_id, Some(a_id)) )) )
                } else {
                    // We only found a Universe and a Series
                    Some( (u_id, Some( (s_id, None) )) )
                }
            } else {
                // We only found a Universe
                Some( (u_id, None) )
            }
        } else {
            None
        }
    }

    /// Returns an iterator over the related sources
    pub fn related_sources(&self) -> impl Iterator<Item=Relation> {
        self.related_sources.clone().into_iter()
    }
    /// Returns an iterator of just the character IDs
    pub fn characters(&self) -> impl Iterator<Item=u64> {
        self.characters.clone().into_iter()
            .map(|(id, _)| id)
    }
    /// Returns an iterator of just the person IDs
    pub fn people(&self) -> impl Iterator<Item=u64> {
        self.people.clone().into_iter()
            .map(|(id, _)| id)
    }
    /// Returns an iterator of just the company IDs
    pub fn companies(&self) -> impl Iterator<Item=u64> + '_ {
        self.companies.clone().into_iter()
            .map(|(id, _)| id)
    }
}
/// These methods will modify the Source
impl Source {
    /// Completely replaces the related sources with new ones
    pub fn replace_related_sources(&mut self, related_sources: impl Iterator<Item=Relation>) {
        // Try to retain the alloc'd size so we don't have to reallocate all the memory
        self.related_sources.clear();
        self.related_sources.extend(related_sources);
    }
}
/// Validation methods specific to a Source
impl Source {
    /// Validate the universes for this source
    pub fn validate_universe(&self) -> Result<(), SourceError> {
        // There can't be a series if there's no universe
        if self.universe.is_none() && self.series.is_some() {
            Err(SourceError::MissingUniverse)
        } else if self.series.is_none() && self.arc.is_some() {
            // There can't be an arc without a series
            Err(SourceError::MissingSeries)
        } else {
            Ok(())
        }
    }
}

/// The type of thing that a Source lives in
#[derive(Copy, Clone, Deserialize, Serialize)]
// It makes sense to me to deliminate categories with an underscore
#[allow(non_camel_case_types)]
pub enum MediaType {
    Book_WebNovel,
}
