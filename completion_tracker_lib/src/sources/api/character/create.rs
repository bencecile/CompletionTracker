use super::{CharacterCreator};
use crate::db_link::{ConnectionHolder};
use crate::sources::api;

pub fn create(db: &ConnectionHolder, creator: CharacterCreator) -> Result<u64, String> {


unimplemented!();
    let db = db.lock();

    let transaction = db.transaction()
        .map_err(|e| e.to_string())?;

    transaction.execute_batch("
        CREATE TEMPORARY TABLE IF NOT EXISTS TempCharacterStrings (
            names INTEGER,
            descriptions INTEGER
        );
        CREATE TEMPORARY TABLE IF NOT EXISTS TempCharacter (
            character_id INTEGER NOT NULL
        );
        DELETE FROM TempCharacterStrings;
        DELETE FROM TempCharacter;
    ").map_err(|e| e.to_string())?;

    api::insert_new_lang_map(&creator.names, &transaction)?;
    transaction.execute_batch("
        INSERT INTO TempCharacterStrings (names, descriptions) VALUES (last_insert_id(), NULL);
    ").map_err(|e| e.to_string())?;
    api::insert_new_lang_map(&creator.descriptions, &transaction)?;
    transaction.execute_batch("
        UPDATE TempCharacterStrings SET descriptions=last_insert_id();

        INSERT INTO Characters (names, description)
            SELECT names, descriptions FROM TempCharacterStrings;
        INSERT INTO TempCharacter (character_id) VALUES (last_insert_id());
        DELETE FROM TempCharacterStrings;
    ").map_err(|e| e.to_string())?;

    // TODO The aliases and related characters

    transaction.commit()
        .map_err(|e| e.to_string())?;

    // TODO Get the created ID
}
