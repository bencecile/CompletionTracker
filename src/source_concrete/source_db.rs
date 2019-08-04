mod integrity;

use std::collections::{BTreeSet};
use std::mem;

use serde::{Serialize};

use crate::source::{
    Character, Company, Person, Source, UniverseTag,
    Record, RecordInfo,
    SourceItem,
    SourceLangStrings,
};
use crate::source::err::{SourceError};
use crate::source_concrete;
use crate::utils;

const LOG_TARGET: &'static str = "SourceDB";


/// All of the information that is needed for the sources
#[derive(Clone)]
pub struct SourceDB {
    /// All the characters
    characters: Vec< Record<Character> >,
    /// All the companies
    companies: Vec< Record<Company> >,
    /// All of the people
    people: Vec< Record<Person> >,
    /// The sources
    sources: Vec< Record<Source> >,
    /// The different possible universes where a source could be set
    universe_tags: Vec< Record<UniverseTag> >,
    
    /// The cache of reverse-lookups for items -> sources
    cache: DBCache,
}
impl SourceDB {
    /// Attempts to first read the source info from disk.
    /// If anything is missing, new empty ones are created.
    pub fn new() -> SourceDB {
        let sources = utils::read_json_list(source_concrete::list_file(SourceItem::Source))
            .unwrap_or_else(|e| {
                ::log::info!(target: LOG_TARGET, "Creating new sources: {}", e);
                Vec::new()
            });
        let universe_tags = utils::read_json_list(
            source_concrete::list_file(SourceItem::UniverseTag))
            .unwrap_or_else(|e| {
                ::log::info!(target: LOG_TARGET, "Creating new universe tags: {}", e);
                Vec::new()
            });
        let characters = utils::read_json_list(source_concrete::list_file(
            SourceItem::Character))
            .unwrap_or_else(|e| {
                ::log::info!(target: LOG_TARGET, "Creating new characters: {}", e);
                Vec::new()
            });
        let people = utils::read_json_list(source_concrete::list_file(SourceItem::Person))
            .unwrap_or_else(|e| {
                ::log::info!(target: LOG_TARGET, "Creating new people: {}", e);
                Vec::new()
            });
        let companies = utils::read_json_list(source_concrete::list_file(SourceItem::Company))
            .unwrap_or_else(|e| {
                ::log::info!(target: LOG_TARGET, "Creating new companies: {}", e);
                Vec::new()
            });

        // Create and update the cache
        let cache = DBCache::empty();

        SourceDB {
            sources,
            universe_tags,
            characters,
            people,
            companies,
            cache,
        }
    }

    /// Saves a specific list to disk
    fn save_list<T: RecordInfo + Clone + Serialize>(list: &Vec< Record<T> >) -> Result<(), SourceError> {
        utils::write_json_list(list, source_concrete::list_file(T::source_item()))
            .map_err(|e| {
                ::log::error!(target: LOG_TARGET, "Failed to save: {}", e);
                SourceError::FailedSave
            })
    }

    pub fn save_universe_tags(&self) -> Result<(), SourceError> {
        SourceDB::save_list(&self.universe_tags)
    }
}

// TODO Instead of just modifying and adding these, we could have a DB transaction. This transaction would allow things to change and keeping track of the changes. Once the transaction is complete, we should validate the DB again, and roll back with any errors. On success, the DB should be saved and the cache updated.
// /// A transaction with the DB.
// /// This will keep track of the things that change in the DB during this transaction.
// pub struct DBTransaction {
//     characters: BTreeMap<u64, (Record<Character>, DBAction)>,
//     companies: BTreeMap<u64, (Record<Company>, DBAction)>,
//     people: BTreeMap<u64, (Record<Person>, DBAction)>,
//     sources: BTreeMap<u64, (Record<Source>, DBAction)>,
//     universe_tags: BTreeMap<u64, (Record<UniverseTag>, DBAction)>,

//     /// So that we can keep track of the order of the actions in the transaction
//     action_counter: u64,
// }
// /// These are the different kinds of things that can occur in a transaction
// #[derive(Copy)]
// enum DBAction {
//     Create,
//     Modify,
//     // Remove,
// }

impl SourceDB {
    /// Checks that the record with the given ID does exist
    fn check_record_id<T: RecordInfo + Clone>(list: &Vec< Record<T> >, id: u64)
    -> Result<(), SourceError> {
        if id >= (list.len() as u64) {
            Err(SourceError::NotFound( (T::source_item(), id) ))
        } else {
            Ok(())
        }
    }
    /// Adds a generic record to a list of records
    fn add_record<T: RecordInfo + Clone>(list: &mut Vec< Record<T> >,
    names: SourceLangStrings, record_info: T) -> u64 {
        // Use the list as the source of the ID
        let id = list.len() as u64;
        let record = Record::new(names, id, record_info);

        list.push(record);
        id
    }
    /// Modifies a record from a record list, using an ID to fetch it
    fn modify_record<F, T>(list: &mut Vec< Record<T> >, id: u64, mod_fn: F)
    -> Result<(), SourceError>
    where F: FnOnce(Record<T>) -> Result<Record<T>, SourceError>,
    T: RecordInfo + Clone {
        // Check that the given ID is good
        SourceDB::check_record_id(list, id)?;

        let index = id as usize;
        // Keep a copy of the original record
        let mut new_record = mod_fn(list[index].clone())?;
        // Set the new record by swapping it with the old one
        mem::swap(&mut list[index], &mut new_record);

        Ok(())
    }
}
/// Any universe related operations
impl SourceDB {
    /// Adds a new universe to the collection.
    /// Returns the ID of the newly added universe.
    pub fn add_universe_tag(&mut self, names: SourceLangStrings) -> u64 {
        SourceDB::add_record(&mut self.universe_tags, names, UniverseTag::new())
    }
    // /// Removes the universe, adjusting the sources as well.
    // /// The linked sources will be removed if remove_source is true.
    // pub fn remove_universe(&mut self, u_id: u64, remove_source: bool) -> Result<(), SourceError> {
    //     record_id_check! { u_id, self.next_universe_tag_id(), SourceItem::UniverseTag }

    //     let remove_index = u_id as usize;
    //     self.universes.remove(remove_index);

    //     // Adjust the universe IDs, jumping to the ones that need to be changed
    //     for universe in self.universes.iter_mut().skip(remove_index) {
    //         universe.id -= 1;
    //     }

    //     // Find the sources that are affected
    //     if remove_source {
    //         // Remove all the sources that are in the universe
    //         //  in other words, keep all sources that aren't related
    //         self.sources.retain(|source|
    //             source.universe()
    //                 .map_or(true, |(source_u_id, _)| source_u_id != u_id)
    //         );
    //     } else {
    //         // Just unlink the universe from these sources
    //         self.sources.iter_mut().for_each(|source| {
    //             source.set_universe(None);
    //         });
    //     }

    //     Ok(())
    // }
    /// Modifies the universe that has the given ID
    pub fn modify_universe_tag<F>(&mut self, u_id: u64, mod_fn: F) -> Result<(), SourceError>
    where F: FnOnce(Record<UniverseTag>) -> Result<Record<UniverseTag>, SourceError> {
        SourceDB::modify_record(&mut self.universe_tags, u_id, mod_fn)
    }
}

/// The first vector is to match the ID and vector in the DB.
/// The set is the list of sources where the item can be found.
/// A BTreeSet will iterate in a sorted fashion
type IDCache = Vec< BTreeSet<u64> >;

/// A cache of source IDs for each item
#[derive(Clone)]
struct DBCache {
    characters: IDCache,
    companies: IDCache,
    people: IDCache,
    universe_tags: IDCache,

    /// Keeps track of all the parents for any given universe tag
    /// This should make lookup for the parents of a tag O(1)
    universe_tag_parents: IDCache,
}
impl DBCache {
    /// Creates an empty cache
    fn empty() -> DBCache {
        DBCache {
            characters: Vec::new(),
            companies: Vec::new(),
            people: Vec::new(),
            universe_tags: Vec::new(),
            universe_tag_parents: Vec::new(),
        }
    }

    // TODO Possibly use a bitset to mark what's changed

    // /// Updates the number of universes that we have
    // fn update_universe_tags(&mut self,
    // universe_tags: &Vec< Record<UniverseTag> >, sources: &Vec< Record<Source> >) {
    //     self.universe_tags.resize(len, BTreeSet::new());
    //     self.universe_tag_parents.resize(len, BTreeSet::new());
    // }
    // /// Updates the number of characters that we have
    // fn update_characters(&mut self, len: usize) {
    //     self.characters.resize(len, BTreeSet::new());
    // }
    // /// Updates the number of people that we have
    // fn update_people(&mut self, len: usize) {
    //     self.people.resize(len, BTreeSet::new());
    // }
    // /// Updates the number of companies that we have
    // fn update_companies(&mut self, len: usize) {
    //     self.companies.resize(len, BTreeSet::new());
    // }

    // /// Updates the cache with the latest sources
    // fn update_with_sources(&mut self, sources: &Vec< Record<Source> >) {
    //     for source in sources.iter() {
    //         // Keep track of the source ID
    //         let (_, source_id) = source.source_item_pair();

    //         // Look through the Universe for all the references
    //         if let Some( (u_id, series) ) = source.universe() {
    //             if let Some( (s_id, arc) ) = series {
    //                 if let Some(a_id) = arc {
    //                     // Add an arc here since we have them all
    //                     self.universes[u_id as usize].insert(
    //                         UniverseSourceRef::Arc(s_id, a_id, source_id)
    //                     );
    //                 } else {
    //                     // Add a series here since we don't have an arc
    //                     self.universes[u_id as usize].insert(
    //                         UniverseSourceRef::Series(s_id, source_id)
    //                     );
    //                 }
    //             } else {
    //                 // Add a universe here since we don't have a series
    //                 self.universes[u_id as usize].insert(
    //                     UniverseSourceRef::Universe(source_id)
    //                 );
    //             }
    //             // We do nothing if there isn't any Universe attached
    //         }

    //         // Look through the characters for all the references
    //         for character_id in source.characters() {
    //             self.characters[character_id as usize].insert(source_id);
    //         }
    //         // Look through the people for all the references
    //         for person_id in source.people() {
    //             self.people[person_id as usize].insert(source_id);
    //         }
    //         // Look through the companies for all the references
    //         for company_id in source.companies() {
    //             self.companies[company_id as usize].insert(source_id);
    //         }
    //     }
    // }
}
