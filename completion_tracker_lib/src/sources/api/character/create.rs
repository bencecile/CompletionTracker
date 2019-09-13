use super::{CharacterCreator};
use crate::db_link::{ConnectionHolder};
use crate::sources::api;

pub fn create(db: &ConnectionHolder, creator: CharacterCreator) -> Result<u64, String> {


unimplemented!();
    let db = db.lock();

    db.execute_batch("
        BEGIN TRANSACTION;

        CREATE TEMPORARY TABLE IF NOT EXISTS TempCharacterStrings (
            names INTEGER REFERENCES Strings(id),
            descriptions INTEGER REFERENCES Strings(id)
        );
        CREATE TEMPORARY TABLE IF NOT EXISTS TempCharacter (
            character_id INTEGER NOT NULL REFERENCES Characters(id)
        );
    ").map_err(|e| e.to_string())?;

    api::insert_new_lang_map(&creator.names, &db)?;
    db.execute_batch("
        INSERT INTO TempCharacterStrings (names, descriptions) VALUES (last_insert_id(), NULL);
    ").map_err(|e| e.to_string())?;
    api::insert_new_lang_map(&creator.descriptions, &db)?;
    db.execute_batch("
        UPDATE TempCharacterStrings SET descriptions=last_insert_id();

        INSERT INTO Characters (names, description)
            SELECT names, descriptions FROM TempCharacterStrings;
        INSERT INTO TempCharacter (character_id) VALUES (last_insert_id());
        DELETE FROM TempCharacterStrings;
    ").map_err(|e| e.to_string())?;

    // TODO The aliases and related characters
}
