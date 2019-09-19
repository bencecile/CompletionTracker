use rusqlite::{params};

use super::{PersonCreator};
use crate::db_link::{ConnectionHolder};
use crate::simple_enum::{SimpleEnum};
use crate::sources::api;
use crate::sources::source_types::{Lang};

pub fn create(db: &ConnectionHolder, mut creator: PersonCreator) -> Result<u64, String> {
    let mut db = db.lock();

    {
        let transaction = db.transaction()
            .map_err(|e| e.to_string())?;

        {
            transaction.execute_batch("
                CREATE TEMPORARY TABLE IF NOT EXISTS TempPersonStrings (
                    names INTEGER,
                    descriptions INTEGER
                );
                CREATE TEMPORARY TABLE IF NOT EXISTS TempPerson (
                    person_id INTEGER NOT NULL
                );
                DELETE FROM TempPersonStrings;
                DELETE FROM TempPerson;
            ").map_err(|e| e.to_string())?;

            api::insert_new_lang_map(&creator.names, &transaction)?;
            transaction.execute_batch("
                INSERT INTO TempPersonStrings (names, descriptions) VALUES (last_insert_id(), NULL);
            ").map_err(|e| e.to_string())?;
            api::insert_new_lang_map(&creator.descriptions, &transaction)?;
            transaction.execute_batch("
                UPDATE TempPersonStrings SET descriptions=last_insert_id();

                INSERT INTO People (names, descriptions)
                    SELECT names, descriptions FROM TempPersonStrings;
                INSERT INTO TempPerson (person_id) VALUES (last_insert_id());

                DELETE FROM TempPersonStrings;
            ").map_err(|e| e.to_string())?;

            let mut insert_alias_statement = transaction.prepare("
                INSERT INTO PersonAliases (person_id, lang, alias)
                    VALUES (SELECT * FROM TempPerson, ?, ?)
            ").map_err(|e| e.to_string())?;
            for lang in Lang::all().iter() {
                if let Some(aliases) = creator.aliases.remove(lang) {
                    for alias in aliases {
                        insert_alias_statement.execute(params![lang, alias])
                            .map_err(|e| e.to_string())?;
                    }
                }
            }
        }

        transaction.commit()
            .map_err(|e| e.to_string())?;
    }

    let inserted_id = db.query_row("SELECT * FROM TempPerson", rusqlite::NO_PARAMS, |row| {
        let id: i64 = row.get(0)?;
        Ok(id as u64)
    }).map_err(|e| e.to_string())?;

    Ok(inserted_id)
}
