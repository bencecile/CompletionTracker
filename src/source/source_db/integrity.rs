use super::{SourceDB};
use crate::source::{SourceItem};
use crate::source::err::{SourceError};

/// Implement the integrity functions in this file since they are long and
/// not really related to the main logic
impl SourceDB {
    /// Checks the source to find any inconsistencies
    pub fn check_integrity_source(&self, id: u64) -> Result<(), SourceError> {
        // Make sure the ID is in range
        if id >= self.next_source_id() {
            return Err(SourceError::NotFound( (SourceItem::Source, id) ));
        }

        self.sources[id as usize].validate(id, |source| {
            source.validate_universe()?;

            // The universe, series, and arc must point to valid ones
            if let Some( (u_id, series) ) = source.universe() {
                if u_id >= self.next_universe_id() {
                    return Err(SourceError::NotFound( (SourceItem::Universe, u_id) ));
                }
                if let Some( (s_id, arc) ) = series {
                    if s_id >= self.next_series_id(u_id) {
                        return Err(SourceError::NotFound( (SourceItem::Series(u_id), s_id) ));
                    }
                    if let Some(a_id) = arc {
                        if a_id >= self.next_arc_id(u_id, s_id) {
                            return Err(SourceError::NotFound(
                                (SourceItem::Arc(u_id, s_id), a_id)
                            ));
                        }
                    }
                }
            }
            // Make sure that the relations point to valid sources
            for relation in source.related_sources() {
                if relation.related_id() >= self.next_source_id() {
                    return Err(SourceError::NotFoundRelation(
                        (SourceItem::Source, id), relation.related_id()
                    ));
                }
            }
            // Make sure all the characters exist
            for c_id in source.characters() {
                if c_id >= self.next_character_id() {
                    return Err(SourceError::NotFound( (SourceItem::Character, c_id) ));
                }
            }
            // Make sure the people exist
            for p_id in source.people() {
                if p_id >= self.next_person_id() {
                    return Err(SourceError::NotFound( (SourceItem::Person, p_id) ));
                }
            }
            // Make sure all the companies exist
            for c_id in source.companies() {
                if c_id >= self.next_character_id() {
                    return Err(SourceError::NotFound( (SourceItem::Company, c_id) ));
                }
            }

            Ok(())
        })
    }
    /// Checks the integrity of a universe (笑)
    pub fn check_integrity_universe(&self, u_id: u64) -> Result<(), SourceError> {
        // The ID must fit in the vector
        if u_id >= self.next_universe_id() {
            return Err(SourceError::NotFound( (SourceItem::Universe, u_id) ));
        }

        self.universes[u_id as usize].validate(u_id, |universe| {
            for s_id in 0..universe.max_series_id() {
                // Validate each series
                universe[s_id].validate(s_id, |series| {
                    for a_id in 0..series.max_arcs_id() {
                        // Validate each arc
                        series[a_id].validate(a_id, |_arc| {
                            Ok(())
                        })?;
                    }
                    Ok(())
                })?;
            }
            Ok(())
        })
    }
    /// Check the integrity of a single character
    pub fn check_integrity_character(&self, c_id: u64) -> Result<(), SourceError> {
        // The id must exist first
        if c_id >= self.next_character_id() {
            return Err(SourceError::NotFound( (SourceItem::Character, c_id) ));
        }

        self.characters[c_id as usize].validate(c_id, |_character| {
            Ok(())
        })
    }
    /// Check the integrity of a single person
    pub fn check_integrity_person(&self, p_id: u64) -> Result<(), SourceError> {
        // The id must exist first
        if p_id >= self.next_person_id() {
            return Err(SourceError::NotFound( (SourceItem::Person, p_id) ));
        }

        self.people[p_id as usize].validate(p_id, |_person| {
            Ok(())
        })
    }
    /// Checks the integrity of the company
    pub fn check_integrity_company(&self, c_id: u64) -> Result<(), SourceError> {
        if c_id >= self.next_company_id() {
            return Err(SourceError::NotFound( (SourceItem::Company, c_id) ));
        }

        self.companies[c_id as usize].validate(c_id, |_company| {
            Ok(())
        })
    }

    /// Makes sure that everything checks out logistically.
    /// This means that there's no duplicates of things we need to be unique, etc.
    ///
    /// Since this could return an error that a user could see, we need translations.
    pub fn check_integrity(&self) -> Result<(), SourceError> {
        for i in 0..self.next_universe_id() {
            self.check_integrity_universe(i)?;
        }
        for i in 0..self.next_source_id() {
            self.check_integrity_source(i)?;
        }
        for i in 0..self.next_person_id() {
            self.check_integrity_person(i)?;
        }
        for i in 0..self.next_character_id() {
            self.check_integrity_character(i)?;
        }
        for i in 0..self.next_company_id() {
            self.check_integrity_company(i)?;
        }

        Ok(())
    }
}
