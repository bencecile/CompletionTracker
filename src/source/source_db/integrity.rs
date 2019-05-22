use super::{SourceDB};
use crate::source::err::{SourceError, SourceErrorType};

/// Implement the integrity functions in this file since they are long and
/// not really related to the main logic
impl SourceDB {
    /// Checks the source to find any inconsistencies
    pub fn check_integrity_source(&self, id: u64) -> Result<(), SourceError> {
        TODO Check for the existence of the images
        // Make sure the ID is in range
        if id >= self.next_source_id() {
            return Err(SourceError::NotFound(SourceErrorType::Source(id)));
        }

        let source = &self.sources[id as usize];
        // The source's ID must match it's position in the vector
        if !source.id_eq(id) {
            return Err(SourceError::IDNotMatching(SourceErrorType::Source(id)));
        }
        let universe = &source.universe;
        let series = &source.series;
        let arc = &source.arc;
        // There can't be a series if there's no universe
        if universe.is_none() && series.is_some() {
            return Err(SourceError::MissingUniverse);
        }
        // There can't be an arc without a series
        if series.is_none() && arc.is_some() {
            return Err(SourceError::MissingSeries);
        }
        // The universe, series, and arc must point to valid ones
        if let Some(u_id) = universe {
            let u_id = *u_id;
            if u_id >= self.next_universe_id() {
                return Err(SourceError::NotFound(SourceErrorType::Universe(u_id)));
            }
            if let Some(s_id) = series {
                let s_id = *s_id;
                if s_id >= self.next_series_id(u_id) {
                    return Err(SourceError::NotFound(SourceErrorType::Series(u_id, s_id)));
                }
                if let Some(a_id) = arc {
                    let a_id = *a_id;
                    if a_id >= self.next_arc_id(u_id, s_id) {
                        return Err(SourceError::NotFound(SourceErrorType::Arc(u_id, s_id, a_id)));
                    }
                }
            }
        }
        // Make sure that the relations point to valid sources
        for relation in source.related_sources.iter() {
            if relation.related_id() >= self.next_source_id() {
                return Err(SourceError::NotFoundSourceRelation(SourceErrorType::Source(id),
                    relation.related_id()));
            }
        }
        // Make sure the people exist
        for p_id in source.people.iter() {
            if *p_id >= self.next_person_id() {
                return Err(SourceError::NotFound(SourceErrorType::Person(*p_id)));
            }
        }
        // Make sure all the characters exist
        for c_id in source.characters.iter() {
            if *c_id >= self.next_character_id() {
                return Err(SourceError::NotFound(SourceErrorType::Character(*c_id)));
            }
        }
        // Make sure all the companies exist
        for c_id in source.companies.iter() {
            if *c_id >= self.next_character_id() {
                return Err(SourceError::NotFound(SourceErrorType::Company(*c_id)));
            }
        }
        Ok(())
    }
    /// Checks the integrity of a universe (笑)
    pub fn check_integrity_universe(&self, u_id: u64) -> Result<(), SourceError> {
        // The ID must fit in the vector
        if u_id >= self.next_universe_id() {
            return Err(SourceError::NotFound(SourceErrorType::Universe(u_id)));
        }

        let universe = &self.universes[u_id as usize];
        // The universe ID must match the position
        if !universe.id_eq(u_id) {
            return Err(SourceError::IDNotMatching(SourceErrorType::Universe(u_id)));
        }
        // Check each series
        for s_id in 0..universe.series.len() {
            let s_id = s_id as u64;
            let series = &universe.series[s_id as usize];
            if !series.id_eq(s_id) {
                return Err(SourceError::IDNotMatching(SourceErrorType::Series(u_id, s_id)));
            }
            for a_id in 0..series.arcs.len() {
                let a_id = a_id as u64;
                if !series.arcs[a_id as usize].id_eq(a_id) {
                    return Err(SourceError::IDNotMatching(SourceErrorType::Arc(u_id, s_id, a_id)));
                }
            }
        }
        Ok(())
    }
    /// Check the integrity of a single person
    pub fn check_integrity_person(&self, p_id: u64) -> Result<(), SourceError> {
        // The id must exist first
        if p_id >= self.next_person_id() {
            return Err(SourceError::NotFound(SourceErrorType::Person(p_id)));
        }

        let person = &self.people[p_id as usize];
        // The ID on the person must match
        if !person.id_eq(p_id) {
            return Err(SourceError::IDNotMatching(SourceErrorType::Person(p_id)));
        }
        // Make sure all of the related sources exist
        for (source_id, _) in person.sources_iter() {
            if *source_id >= self.next_source_id() {
                return Err(SourceError::NotFoundSourceRelation(SourceErrorType::Person(p_id),
                    *source_id));
            }
        }
        Ok(())
    }
    /// Check the integrity of a single character
    pub fn check_integrity_character(&self, c_id: u64) -> Result<(), SourceError> {
        // The id must exist first
        if c_id >= self.next_character_id() {
            return Err(SourceError::NotFound(SourceErrorType::Character(c_id)));
        }

        let character = &self.characters[c_id as usize];
        // The ID on the person must match
        if !character.id_eq(c_id) {
            return Err(SourceError::IDNotMatching(SourceErrorType::Character(c_id)));
        }
        // Make sure all of the related sources exist
        for (source_id, _) in character.sources_iter() {
            if *source_id >= self.next_source_id() {
                return Err(SourceError::NotFoundSourceRelation(SourceErrorType::Character(c_id),
                    *source_id));
            }
        }
        Ok(())
    }
    /// Checks the integrity of the company
    pub fn check_integrity_company(&self, c_id: u64) -> Result<(), SourceError> {
        if c_id >= self.next_company_id() {
            return Err(SourceError::NotFound(SourceErrorType::Company(c_id)));
        }

        let company = &self.companies[c_id as usize];
        // The ID of the company must match its position
        if !company.id_eq(c_id) {
            return Err(SourceError::IDNotMatching(SourceErrorType::Company(c_id)));
        }
        // Make sure all of the related sources exist
        for (source_id, _) in company.sources_iter() {
            if *source_id >= self.next_source_id() {
                return Err(SourceError::NotFoundSourceRelation(SourceErrorType::Company(c_id),
                    *source_id));
            }
        }
        Ok(())
    }

    /// Makes sure that everything checks out logistically.
    /// This means that there's no duplicates of things we need to be unique, etc.
    ///
    /// Since this could return an error that a user could see, we need translations.
    pub fn check_integrity(&self) -> Result<(), SourceError> {
        // Check for unique universes, series in each universe, arcs in each series
        for i in 0..self.next_universe_id() {
            self.check_integrity_universe(i)?;
        }
        // Check for unique source IDs
        for i in 0..self.next_source_id() {
            self.check_integrity_source(i)?;
        }
        // Check the integrity of the people
        for i in 0..self.next_person_id() {
            self.check_integrity_person(i)?;
        }
        // Check the integrity of the characters
        for i in 0..self.next_character_id() {
            self.check_integrity_character(i)?;
        }
        // Check the integrity of the companies
        for i in 0..self.next_company_id() {
            self.check_integrity_company(i)?;
        }

        Ok(())
    }
}
