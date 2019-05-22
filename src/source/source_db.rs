mod integrity;

use crate::lang::{Lang, LangString};
use crate::source::{Arc, MediaType, Series, Source, Universe};
use crate::source::character::{Character};
use crate::source::company::{Company};
use crate::source::err::{SourceError, SourceErrorType};
use crate::source::person::{Person};
use crate::source::ui::{self, UniverseUi};
use crate::utils;

/// All of the information that is needed for the sources
#[derive(Clone)]
pub struct SourceDB {
    /// The different possible universes where a source could be set
    universes: Vec<Universe>,
    /// The sources
    sources: Vec<Source>,
    /// All the characters
    characters: Vec<Character>,
    /// All of the people
    people: Vec<Person>,
    /// All the companies
    companies: Vec<Company>,
}
impl SourceDB {
    /// Attempts to first read the source info from disk.
    /// If anything is missing, new empty ones are created.
    pub fn new() -> SourceDB {
        let universes = utils::read_json_list(SourceItem::Universes.list_file())
            .unwrap_or_else(|e| {
                println!("Creating new universes: {}", e);
                Vec::new()
            });
        let sources = utils::read_json_list(SourceItem::Source.list_file())
            .unwrap_or_else(|e| {
                println!("Creating new sources: {}", e);
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

        SourceDB {
            universes,
            sources,
            characters,
            people,
            companies,
        }
    }
    pub fn universes_ui<'u>(&'u self, lang: Lang) -> Vec< UniverseUi<'u> > {
        ui::create_universes_ui(&self.universes, &self.sources, lang)
    }

    //---- Some ID functions so we know what to use next ----
    fn next_universe_id(&self) -> u64 { self.universes.len() as u64 }
    fn next_series_id(&self, u_id: u64) -> u64 { self.universes[u_id as usize].series.len() as u64 }
    fn next_arc_id(&self, u_id: u64, s_id: u64) -> u64 {
        self.universes[u_id as usize].series[s_id as usize].arcs.len() as u64
    }
    fn next_source_id(&self) -> u64 { self.sources.len() as u64 }
    fn next_person_id(&self) -> u64 { self.people.len() as u64 }
    fn next_character_id(&self) -> u64 { self.characters.len() as u64 }
    fn next_company_id(&self) -> u64 { self.companies.len() as u64 }

    /// Saves all of the source information to disk
    pub fn save(&self) -> Result<(), String> {
        utils::write_json_list(&self.universes, SourceItem::Universes.list_file())?;
        utils::write_json_list(&self.sources, SourceItem::Source.list_file())?;
        utils::write_json_list(&self.characters, SourceItem::Characters.list_file())?;
        utils::write_json_list(&self.people, SourceItem::Person.list_file())?;
        utils::write_json_list(&self.companies, SourceItem::Company.list_file())
    }

    /// Uses just the most basic information to create a new source entry.
    /// Returns the ID of the created source if it was created successfully.
    pub fn add_source(&mut self, name: LangString, media_type: MediaType) -> u64 {
        // Create the source
        let id = self.next_source_id();
        let source = Source::new(name, media_type, id);

        self.sources.push(source);
        id
    }
    /// Attempts to remove the source with the given id
    pub fn remove_source(&mut self, id: u64) -> Result<(), SourceError> {
        let i = id as usize;
        if i >= self.sources.len() {
            return Err(SourceError::NotFound(SourceErrorType::Source(id)));
        }

        self.sources.remove(i);

        // Remove any references to the removed source in the people
        self.people.iter_mut()
            .for_each(|person| person.remove_source_refs(id));
        self.characters.iter_mut()
            .for_each(|character| character.remove_source_refs(id));
        self.companies.iter_mut()
            .for_each(|company| company.remove_source_refs(id));

        // Shift down the IDs of everything after
        for (j, source) in self.sources.iter_mut().enumerate() {
            // Fix anything else that references the old IDs, we know how they will change
            // Modify the relations
            let relations = source.related_sources.clone();
            // Iterate in reverse for removal stability
            for (related_index, relation) in relations.into_iter().enumerate().rev() {
                // See if the related ID is in the range of changes
                let related_id = relation.related_id();
                if related_id == id {
                    // Remove any relations that referenced the old ID
                    source.related_sources.remove(related_index);
                } else if related_id > id {
                    source.modify_related_source_id(related_index, related_id - 1);
                }
            }

            // Only fix the source ID once we get the index that got removed
            if j >= i {
                let old_id = source.id;
                source.id -= 1;
                // Update all of the people and characters
                self.people.iter_mut()
                    .for_each(|person| person.update_source_refs(old_id, source.id));
                self.characters.iter_mut()
                    .for_each(|character| character.update_source_refs(old_id, source.id));
                self.companies.iter_mut()
                    .for_each(|company| company.update_source_refs(old_id, source.id));
            }
        }
        Ok(())
    }
    /// Gives access to a source to modify using an ID.
    /// Can fail if the integrity check afterwards fails.
    pub fn modify_source<F>(&mut self, id: u64, fun: F) -> Result<(), SourceError>
    where F: FnOnce(Source) -> Result<Source, SourceError> {
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
            Ok(())
        }
    }

    /// Adds a new universe to the collection.
    /// Returns the ID of the newly added universe.
    pub fn add_universe(&mut self, name: LangString) -> u64 {
        let id = self.next_universe_id();
        let universe = Universe::new(name, id);
        self.universes.push(universe);
        id
    }
    /// Adds a new series underneath the given universe ID.
    /// Can fail and returns the ID of the newly added series on success.
    pub fn add_series(&mut self, name: LangString, u_id: u64) -> Result<u64, SourceError> {
        if u_id >= self.next_universe_id() {
            return Err(SourceError::NotFound(SourceErrorType::Universe(u_id)));
        }

        let id = self.next_series_id(u_id);
        let series = Series::new(name, id);
        self.universes[u_id as usize].series.push(series);
        Ok(id)
    }
    /// Adds a new arc inside of the given universe and series
    pub fn add_arc(&mut self, name: LangString, u_id: u64, s_id: u64)
    -> Result<u64, SourceError> {
        if u_id >= self.next_universe_id() {
            return Err(SourceError::NotFound(SourceErrorType::Universe(u_id)));
        }
        if s_id >= self.next_series_id(u_id) {
            return Err(SourceError::NotFound(SourceErrorType::Series(u_id, s_id)));
        }

        let id = self.next_arc_id(u_id, s_id);
        let arc = Arc::new(name, id);
        self.universes[u_id as usize].series[s_id as usize].arcs.push(arc);
        Ok(id)
    }

    /// Modifies the universe that has the given ID
    pub fn modify_universe<F>(&mut self, u_id: u64, fun: F) -> Result<(), SourceError>
    where F: FnOnce(Universe) -> Result<Universe, SourceError> {
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
