mod integrity;

use std::collections::{BTreeSet};

use crate::lang::{LangString};
use crate::source::{
    Source,
    Universe, Series, Arc,
    Character,
    Person,
    Company,

    MediaType,
    Record,
    SourceItem,
};
use crate::source::err::{SourceError};
use crate::utils;

/// All of the information that is needed for the sources
#[derive(Clone)]
pub struct SourceDB {
    /// The sources
    sources: Vec< Record<Source> >,
    /// The different possible universes where a source could be set
    universes: Vec< Record<Universe> >,
    /// All the characters
    characters: Vec< Record<Character> >,
    /// All of the people
    people: Vec< Record<Person> >,
    /// All the companies
    companies: Vec< Record<Company> >,
    /// The cache of reverse-lookups for items -> sources
    cache: SourceCache,
}
impl SourceDB {
    /// Attempts to first read the source info from disk.
    /// If anything is missing, new empty ones are created.
    pub fn new() -> SourceDB {
        let sources = utils::read_json_list(SourceItem::Source.list_file())
            .unwrap_or_else(|e| {
                println!("Creating new sources: {}", e);
                Vec::new()
            });
        let universes = utils::read_json_list(SourceItem::Universe.list_file())
            .unwrap_or_else(|e| {
                println!("Creating new universes: {}", e);
                Vec::new()
            });
        let characters = utils::read_json_list(SourceItem::Character.list_file())
            .unwrap_or_else(|e| {
                println!("Creating new characters: {}", e);
                Vec::new()
            });
        let people = utils::read_json_list(SourceItem::Person.list_file())
            .unwrap_or_else(|e| {
                println!("Creating new people: {}", e);
                Vec::new()
            });
        let companies = utils::read_json_list(SourceItem::Company.list_file())
            .unwrap_or_else(|e| {
                println!("Creating new companies: {}", e);
                Vec::new()
            });

        // Create and update the cache
        let mut cache = SourceCache::empty();
        cache.update_universes_size(universes.len());
        cache.update_characters_size(characters.len());
        cache.update_people_size(people.len());
        cache.update_companies_size(companies.len());
        cache.update(&sources);

        SourceDB {
            sources,
            universes,
            characters,
            people,
            companies,
            cache,
        }
    }
    // pub fn universes_ui<'u>(&'u self, lang: Lang) -> Vec< UniverseUi<'u> > {
    //     ui::create_universes_ui(&self.universes, &self.sources, lang)
    // }

    //---- Some ID functions so we know what to use next ----
    fn next_source_id(&self) -> u64 { self.sources.len() as u64 }
    fn next_universe_id(&self) -> u64 { self.universes.len() as u64 }
    fn next_series_id(&self, u_id: u64) -> u64 { self.universes[u_id as usize].max_series_id() + 1 }
    fn next_arc_id(&self, u_id: u64, s_id: u64) -> u64 {
        self.universes[u_id as usize][s_id].max_arcs_id() + 1
    }
    fn next_character_id(&self) -> u64 { self.characters.len() as u64 }
    fn next_person_id(&self) -> u64 { self.people.len() as u64 }
    fn next_company_id(&self) -> u64 { self.companies.len() as u64 }

    /// Saves all of the source information to disk
    pub fn save(&self) -> Result<(), String> {
        utils::write_json_list(&self.sources, SourceItem::Source.list_file())?;
        utils::write_json_list(&self.universes, SourceItem::Universe.list_file())?;
        utils::write_json_list(&self.characters, SourceItem::Character.list_file())?;
        utils::write_json_list(&self.people, SourceItem::Person.list_file())?;
        utils::write_json_list(&self.companies, SourceItem::Company.list_file())
    }
}
impl SourceDB {
    /// Uses just the most basic information to create a new source entry.
    /// Returns the ID of the created source if it was created successfully.
    pub fn add_source(&mut self, name: LangString, media_type: MediaType) -> u64 {
        // Create the source
        let id = self.next_source_id();
        let source = Record::new(name, id, Source::new(media_type));

        self.sources.push(source);
        id
    }
    /// Attempts to remove the source with the given id
    pub fn remove_source(&mut self, remove_id: u64) -> Result<(), SourceError> {
        if remove_id >= self.next_source_id() {
            return Err(SourceError::NotFound( (SourceItem::Source, remove_id) ));
        }

        let remove_index = remove_id as usize;
        self.sources.remove(remove_index);

        // Shift down the IDs of everything after
        for (i, source) in self.sources.iter_mut().enumerate() {
            // Fix anything else that references the old IDs
            let related_sources_iter = source.related_sources().filter_map(|mut relation| {
                // See if the related ID is in the range of changes
                let related_id = relation.related_id();
                if related_id == remove_id {
                    // Remove any relations that referenced the old ID
                    None
                } else if related_id > remove_id {
                    // If the ID will change, we know exactly how it will change
                    relation.change_related_id(related_id - 1);
                    Some(relation)
                } else {
                    // Nothing needs to change for this one
                    Some(relation)
                }
            });
            // Replace the related sources now that we've removed and modified them
            source.replace_related_sources(related_sources_iter);

            // Only fix the source ID once we get to the index that got removed
            if i >= remove_index {
                // This is essentially bringing them back in line since they got out of order
                source.id -= 1;
            }
        }

        // Update the cache since the IDs may have changed
        self.cache.update(&self.sources);

        Ok(())
    }
    /// Gives access to a source to modify using an ID.
    /// Can fail if the integrity check afterwards fails.
    pub fn modify_source<F>(&mut self, id: u64, fun: F) -> Result<(), SourceError>
    where F: FnOnce(Record<Source>) -> Result<Record<Source>, SourceError> {
        if id >= self.next_source_id() {
            return Err(SourceError::NotFound( (SourceItem::Source, id) ));
        }

        let index = id as usize;
        // Keep a copy of the original source in case anything breaks
        let original_source = self.sources[index].clone();
        let new_source = fun(original_source.clone())?;
        
        // Replace the source then check the integrity
        self.sources[index] = new_source;
        if let Err(e) = self.check_integrity_source(id) {
            // Rollback with the old source
            self.sources[index] = original_source;
            Err(e)
        } else {
            // We can keep the current configuration if the integrity check passes
            // Update the cache since more references may have been added
            self.cache.update(&self.sources);
            Ok(())
        }
    }

    /// Adds a new universe to the collection.
    /// Returns the ID of the newly added universe.
    pub fn add_universe(&mut self, name: LangString) -> u64 {
        let id = self.next_universe_id();
        let universe = Record::new(name, id, Universe::new());
        self.universes.push(universe);
        id
    }
    // /// Adds a new series underneath the given universe ID.
    // /// Can fail and returns the ID of the newly added series on success.
    // pub fn add_series(&mut self, name: LangString, u_id: u64) -> Result<u64, SourceError> {
    //     if u_id >= self.next_universe_id() {
    //         return Err(SourceError::NotFound( (SourceItem::Universe, u_id) ));
    //     }

    //     let id = self.next_series_id(u_id);
    //     let series = Series::new(name, id);
    //     self.universes[u_id as usize].series.push(series);
    //     Ok(id)
    // }
    // /// Adds a new arc inside of the given universe and series
    // pub fn add_arc(&mut self, name: LangString, u_id: u64, s_id: u64)
    // -> Result<u64, SourceError> {
    //     if u_id >= self.next_universe_id() {
    //         return Err(SourceError::NotFound( (SourceItem::Universe, u_id) ));
    //     }
    //     if s_id >= self.next_series_id(u_id) {
    //         return Err(SourceError::NotFound( (SourceItem::Series(u_id), s_id) ));
    //     }

    //     let id = self.next_arc_id(u_id, s_id);
    //     let arc = Arc::new(name, id);
    //     self.universes[u_id as usize].series[s_id as usize].arcs.push(arc);
    //     Ok(id)
    // }

    /// Modifies the universe that has the given ID
    pub fn modify_universe<F>(&mut self, u_id: u64, fun: F) -> Result<(), SourceError>
    where F: FnOnce(Record<Universe>) -> Result<Record<Universe>, SourceError> {
        if u_id >= self.next_universe_id() {
            return Err(SourceError::NotFound( (SourceItem::Universe, u_id) ));
        }

        let u_index = u_id as usize;
        // Keep a copy of the original universe
        let original_universe = self.universes[u_index].clone();
        // Get the new universe that the function modifies
        let new_universe = fun(original_universe.clone())?;

        // Check the integrity of the modified universe
        self.universes[u_index] = new_universe;
        if let Err(e) = self.check_integrity_universe(u_id) {
            // Rollback and put back the original universe
            self.universes[u_index] = original_universe;
            Err(e)
        } else {
            Ok(())
        }
    }
}

/// The first vector is to match the ID and vector in the DB.
/// The set is the list of sources where the item can be found.
/// A BTreeSet will iterate in a sorted fashion
type IDCache = Vec< BTreeSet<u64> >;

/// A cache of source IDs for each item
#[derive(Clone)]
struct SourceCache {
    universes: Vec< BTreeSet<UniverseSourceRef> >,
    characters: IDCache,
    people: IDCache,
    companies: IDCache,
}
impl SourceCache {
    /// Creates an empty cache
    fn empty() -> SourceCache {
        SourceCache {
            universes: Vec::new(),
            characters: Vec::new(),
            people: Vec::new(),
            companies: Vec::new(),
        }
    }

    /// Updates the number of universes that we have
    fn update_universes_size(&mut self, len: usize) {
        self.universes.resize(len, BTreeSet::new());
    }
    /// Updates the number of characters that we have
    fn update_characters_size(&mut self, len: usize) {
        self.characters.resize(len, BTreeSet::new());
    }
    /// Updates the number of people that we have
    fn update_people_size(&mut self, len: usize) {
        self.people.resize(len, BTreeSet::new());
    }
    /// Updates the number of companies that we have
    fn update_companies_size(&mut self, len: usize) {
        self.companies.resize(len, BTreeSet::new());
    }

    /// Updates the cache with the latest sources
    fn update(&mut self, sources: &Vec< Record<Source> >) {
        for source in sources.iter() {
            // Keep track of the source ID
            let (_, source_id) = source.source_item_pair();

            // Look through the Universe for all the references
            if let Some( (u_id, series) ) = source.universe() {
                if let Some( (s_id, arc) ) = series {
                    if let Some(a_id) = arc {
                        // Add an arc here since we have them all
                        self.universes[u_id as usize].insert(
                            UniverseSourceRef::Arc(s_id, a_id, source_id)
                        );
                    } else {
                        // Add a series here since we don't have an arc
                        self.universes[u_id as usize].insert(
                            UniverseSourceRef::Series(s_id, source_id)
                        );
                    }
                } else {
                    // Add a universe here since we don't have a series
                    self.universes[u_id as usize].insert(
                        UniverseSourceRef::Universe(source_id)
                    );
                }
                // We do nothing if there isn't any Universe attached
            }

            // Look through the characters for all the references
            for character_id in source.characters() {
                self.characters[character_id as usize].insert(source_id);
            }
            // Look through the people for all the references
            for person_id in source.people() {
                self.people[person_id as usize].insert(source_id);
            }
            // Look through the companies for all the references
            for company_id in source.companies() {
                self.companies[company_id as usize].insert(source_id);
            }
        }
    }
}

/// The source references that can appear in a Universe
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
enum UniverseSourceRef {
    /// The source that is just on the Universe (source_id)
    Universe(u64),
    /// The source that is directly underneath a series (s_id, source_id)
    Series(u64, u64),
    /// The source inside an arc (s_id, a_id, source_id)
    Arc(u64, u64, u64),
}
