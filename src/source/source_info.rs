use crate::lang::{Lang, LangSelect, LangString};
use crate::source::{Arc, MediaType, Series, Source, SourceFile, Universe};
use crate::source::ui::{self, UiUniverse};
use crate::utils;

/// All of the information that is needed for the sources
#[derive(Clone)]
pub struct SourceInfo {
    /// The different possible universes where a source could be set
    universes: Vec<Universe>,
    /// The sources
    sources: Vec<Source>,
}
impl SourceInfo {
    /// Attempts to first read the source info from disk. If anything is missing, new empty
    /// ones are created.
    pub fn new() -> SourceInfo {
        let universes = match utils::read_json_list(Universe::file_path()) {
            Ok(universes) => universes,
            Err(e) => {
                println!("Creating new universes: {}", e);
                Vec::new()
            },
        };
        let sources = match utils::read_json_list(Source::file_path()) {
            Ok(sources) => sources,
            Err(e) => {
                println!("Creating new sources: {}", e);
                Vec::new()
            },
        };

        SourceInfo {
            universes,
            sources,
        }
    }
    pub fn universes<'a>(&'a self) -> &'a Vec<Universe> { &self.universes }
    pub fn sources<'a>(&'a self) -> &'a Vec<Source> { &self.sources }
    pub fn universes_ui<'u>(&'u self, lang: LangSelect) -> Vec< UiUniverse<'u> > {
        ui::create_universes_ui(self.universes(), self.sources(), lang)
    }

    //---- Some ID functions so we know what to use next ----
    fn next_universe_id(&self) -> u64 { self.universes.len() as u64 }
    fn next_series_id(&self, u_id: u64) -> u64 { self.universes[u_id as usize].series.len() as u64 }
    fn next_arc_id(&self, u_id: u64, s_id: u64) -> u64 {
        self.universes[u_id as usize].series[s_id as usize].arcs.len() as u64
    }
    fn next_source_id(&self) -> u64 { self.sources.len() as u64 }

    /// Saves all of the source information to disk
    pub fn save(&self) -> Result<(), String> {
        utils::write_json_list(&self.universes, Universe::file_path())?;
        utils::write_json_list(&self.sources, Source::file_path())
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
    pub fn remove_source(&mut self, id: u64, lang: Lang) -> Result<(), String> {
        let i = id as usize;
        if i < self.sources.len() {
            self.sources.remove(i);
            // Shift down the IDs of everything after
            self.sources.iter_mut()
                .enumerate()
                .for_each(|(j, source)| {
                    // Fix anything else that references the old IDs, we know how they will change
                    // Modify the relations
                    let relations = source.related_sources().clone();
                    for (related_index, relation) in relations.into_iter().enumerate() {
                        // See if the related ID is in the range of changes
                        let related_id = relation.related_id();
                        if related_id >= i as u64 {
                            source.modify_related_source_id(related_index, related_id - 1);
                        }
                    }

                    // Only fix the source ID once we get the index that got removed
                    if j >= i {
                        source.id -= 1;
                    }
                });
            Ok(())
        } else {
            Err(lang.source_err_not_found(id))
        }
    }

    /// Gives access to a source to modify using an ID. Can fail if that ID isn't found
    pub fn modify_source<F>(&mut self, id: u64, lang: Lang, f: F) -> Result<(), String>
    where F: FnOnce(Source) -> Source {
        let i = id as usize;
        if i < self.sources.len() {
            let new_source = f(self.sources[i].clone());
            // Replace the source then check the integrity
            let old_source = self.sources[i].clone();
            self.sources[i] = new_source;
            
            if let Err(e) = self.check_source_integrity(id, lang) {
                // Rollback with the old source
                self.sources[i] = old_source;
                Err(e)
            } else {
                // We can keep the current configuration if the integrity check passes
                Ok(())
            }
        } else {
            Err(lang.source_err_not_found(id))
        }
    }

    /// Adds a new universe to the collection.
    /// Returns the index of the newly added universe.
    pub fn add_universe(&mut self, name: LangString) -> u64 {
        let id = self.next_universe_id();
        let universe = Universe::new(name, id);
        self.universes.push(universe);
        id
    }
    /// Adds a new series underneath the given universe ID.
    /// Can fail and returns the ID of the newly added series on success.
    pub fn add_series(&mut self, name: LangString, u_id: u64, lang: Lang) -> Result<u64, String> {
        if u_id >= self.next_universe_id() {
            return Err(lang.universe_err_not_found(u_id));
        }

        let id = self.next_series_id(u_id);
        let series = Series::new(name, id);
        self.universes[u_id as usize].series.push(series);
        Ok(id)
    }
    pub fn add_arc(&mut self, name: LangString, u_id: u64, s_id: u64, lang: Lang)
    -> Result<u64, String> {
        if u_id >= self.next_universe_id() {
            return Err(lang.universe_err_not_found(u_id));
        }
        if s_id >= self.next_series_id(u_id) {
            return Err(lang.universe_err_not_found_series(u_id, s_id));
        }

        let id = self.next_arc_id(u_id, s_id);
        let arc = Arc::new(name, id);
        self.universes[u_id as usize].series[s_id as usize].arcs.push(arc);
        Ok(id)
    }

    /// Checks the source to find any inconsistencies
    pub fn check_source_integrity(&self, id: u64, lang: Lang) -> Result<(), String> {
        // Make sure the ID is in range
        if id >= self.next_source_id() {
            return Err(lang.source_err_not_found(id));
        }

        let source = &self.sources[id as usize];
        // The source's ID must match it's position in the vector
        if id != source.id() {
            return Err(lang.source_err_id_not_matching(id));
        }
        let universe = source.universe();
        let series = source.series();
        let arc = source.arc();
        // There can't be a series if there's no universe
        if universe.is_none() && series.is_some() {
            return Err(lang.source_err_missing_universe());
        }
        // There can't be an arc without a series
        if series.is_none() && arc.is_some() {
            return Err(lang.source_err_missing_series());
        }
        // The universe, series, and arc must point to valid ones
        if let Some(universe) = universe {
            let universe = *universe;
            if universe >= self.next_universe_id() {
                return Err(lang.universe_err_not_found(universe));
            }
            if let Some(series) = series {
                let series = *series;
                if series >= self.next_series_id(universe) {
                    return Err(lang.universe_err_not_found_series(universe, series));
                }
                if let Some(arc) = arc {
                    let arc = *arc;
                    if arc >= self.next_arc_id(universe, series) {
                        return Err(lang.universe_err_not_found_arc(universe, series, arc));
                    }
                }
            }
        }
        // Make sure that the relations point to valid sources
        for relation in source.related_sources.iter() {
            if relation.related_id() >= self.next_source_id() {
                return Err(lang.source_err_bad_id_relation(relation.related_id()));
            }
        }
        Ok(())
    }
    /// Checks the integrity of a universe (笑)
    pub fn check_universe_integrity(&self, u_id: u64, lang: Lang) -> Result<(), String> {
        // The ID must fit in the vector
        if u_id >= self.next_universe_id() {
            return Err(lang.universe_err_not_found(u_id));
        }

        let universe = &self.universes[u_id as usize];
        // The universe ID must match the position
        if u_id != universe.id() {
            return Err(lang.universe_err_non_matching_id(u_id));
        }
        // Check each series
        for s_id in 0..universe.series().len() {
            let series = &universe.series()[s_id];
            if s_id as u64 != series.id() {
                return Err(lang.universe_err_non_matching_id_series(u_id, s_id as u64));
            }
            for a_id in 0..series.arcs().len() {
                if a_id as u64 != series.arcs()[a_id].id() {
                    return Err(lang.universe_err_non_matching_id_arc(u_id, s_id as u64,
                        a_id as u64));
                }
            }
        }
        Ok(())
    }

    /// Makes sure that everything checks out logistically.
    /// This means that there's no duplicates of things we need to be unique, etc.
    ///
    /// Since this could return an error that a user could see, we need translations.
    pub fn check_integrity(&self, lang: Lang) -> Result<(), String> {
        // Check for unique universes, series in each universe, arcs in each series
        for i in 0..self.universes.len() {
            self.check_universe_integrity(i as u64, lang)?;
        }
        // Check for unique source IDs
        for i in 0..self.sources.len() {
            self.check_source_integrity(i as u64, lang)?;
        }

        Ok(())
    }
}
