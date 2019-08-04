use super::{SourceDB};
use crate::source::{SourceItem, Record, RecordInfo};
use crate::source::err::{SourceError};
use crate::source_concrete::source_image;

/// Implement the integrity functions in this file since they are long and
/// not really related to the main logic
impl SourceDB {
    /// Makes sure that everything checks out logistically.
    /// This means that there's no duplicates of things we need to be unique, etc.
    pub fn check_integrity(&self) -> Result<(), SourceError> {
        for i in 0..(self.sources.len() as u64) {
            self.check_integrity_source(i)?;
        }
        for i in 0..(self.universe_tags.len() as u64) {
            self.check_integrity_universe_tags(i)?;
        }
        for i in 0..(self.people.len() as u64) {
            self.check_integrity_person(i)?;
        }
        for i in 0..(self.characters.len() as u64) {
            self.check_integrity_character(i)?;
        }
        for i in 0..(self.companies.len() as u64) {
            self.check_integrity_company(i)?;
        }

        Ok(())
    }

    /// Checks the source to find any inconsistencies
    pub fn check_integrity_source(&self, id: u64) -> Result<(), SourceError> {
        SourceDB::check_record_id(&self.sources, id)?;

        self.sources[id as usize].validate(id, |source| {
            // Make sure that the relations point to valid sources
            for relation in source.related_sources.iter() {
                if SourceDB::check_record_id(&self.sources, relation.related_id()).is_err() {
                    return Err(SourceError::NotFoundRelation(
                        (SourceItem::Source, id), relation.related_id()
                    ));
                }
            }
            // The universe tags must point to valid ones
            for u_id in source.universe_tags.iter() {
                SourceDB::check_record_id(&self.universe_tags, *u_id)?;
            }
            // Make sure all the characters exist
            for (c_id, _) in source.characters.iter() {
                SourceDB::check_record_id(&self.characters, *c_id)?;
            }
            // Make sure the people exist
            for (p_id, _) in source.people.iter() {
                SourceDB::check_record_id(&self.people, *p_id)?;
            }
            // Make sure all the companies exist
            for (c_id, _) in source.companies.iter() {
                SourceDB::check_record_id(&self.companies, *c_id)?;
            }

            check_image(source)
        })
    }
    /// Checks the integrity of a universe (笑)
    pub fn check_integrity_universe_tags(&self, u_id: u64) -> Result<(), SourceError> {
        SourceDB::check_record_id(&self.universe_tags, u_id)?;

        self.universe_tags[u_id as usize].validate(u_id, |universe_tag| {
            check_image(universe_tag)
        })
    }
    /// Check the integrity of a single character
    pub fn check_integrity_character(&self, c_id: u64) -> Result<(), SourceError> {
        SourceDB::check_record_id(&self.characters, c_id)?;

        self.characters[c_id as usize].validate(c_id, |character| {
            check_image(character)
        })
    }
    /// Check the integrity of a single person
    pub fn check_integrity_person(&self, p_id: u64) -> Result<(), SourceError> {
        SourceDB::check_record_id(&self.people, p_id)?;

        self.people[p_id as usize].validate(p_id, |person| {
            check_image(person)
        })
    }
    /// Checks the integrity of the company
    pub fn check_integrity_company(&self, c_id: u64) -> Result<(), SourceError> {
        SourceDB::check_record_id(&self.companies, c_id)?;

        self.companies[c_id as usize].validate(c_id, |company| {
            check_image(company)
        })
    }

}

/// Checks the image on the record to make sure it exists
fn check_image<T: RecordInfo + Clone>(record: &Record<T>) -> Result<(), SourceError> {
    if let Some(image_result) = source_image::image_path(record) {
        image_result?;
    }
    Ok(())
}
