use rusqlite::{params};

use super::{UniverseTagCreator};
use crate::db_link::{ConnectionHolder};
use crate::sources::api::{self, DBSourceRelation};

pub fn create(db: &ConnectionHolder, creator: UniverseTagCreator) -> Result<u64, String> {
    let mut db = db.lock();

    let related_links = api::transform_related_links(creator.related_links)?;

    // 1. Put the LangMaps into the Strings table
    // 2. Use the IDs from the inserted Strings to create a new UniverseTag
    // 3. Insert parents and child IDs into a temp DB
    // 4. Fill out the missing parent and child IDs with our created UniverseTag ID
    // 5. Move the Children to the correct table
    // 6. Insert UniverseTag IDs into a temp DB with Relation
    // 6-1. Make sure that the ID is in the correct spot according to the relation
    // 7. Fill out the missing relation IDs with our created UniverseTag ID
    // 8. Move the Relations to the correct table
    // 9. For each related link
    // 9-1. Put the description LangMaps into the Strings table
    // 9-2. Insert a new RelatedLink row with the URL and the descriptions ID

    {
        let transaction = db.transaction()
            .map_err(|e| e.to_string())?;

        {
            // Create all of the temporary tables we will need to perform these creations
            transaction.execute_batch("
                CREATE TEMPORARY TABLE IF NOT EXISTS TempUniverseTagStrings (
                    names_id INTEGER,
                    descriptions_id INTEGER
                );
                CREATE TEMPORARY TABLE IF NOT EXISTS TempUniverseTag (
                    tag_id INTEGER NOT NULL
                );
                CREATE TEMPORARY TABLE IF NOT EXISTS TempUniverseTagChildren (
                    parent INTEGER,
                    child INTEGER
                );
                CREATE TEMPORARY TABLE IF NOT EXISTS TempUniverseTagRelations (
                    id1 INTEGER,
                    id2 INTEGER,
                    relation TEXT NOT NULL
                );
                DELETE FROM TempUniverseTagStrings;
                DELETE FROM TempUniverseTag;
                DELETE FROM TempUniverseTagChildren;
                DELETE FROM TempUNiverseTagRelations;
            ").map_err(|e| e.to_string())?;

            // #1
            api::insert_new_lang_map(&creator.names, &transaction)?;
            transaction.execute_batch("
                INSERT INTO TempUniverseTagStrings (
                    names_id, descriptions_id
                ) VALUES (last_insert_rowid(), NULL);
            ").map_err(|e| e.to_string())?;
            api::insert_new_lang_map(&creator.descriptions, &transaction)?;
            // #2
            transaction.execute_batch("
                UPDATE TempUniverseTagStrings SET descriptions_id=last_insert_rowid();
                INSERT INTO UniverseTags (names, descriptions)
                    SELECT names_id, descriptions_id FROM TempUniverseTagStrings;
                INSERT INTO TempUniverseTag VALUES (last_insert_rowid());
                DELETE FROM TempUniverseTagStrings;
            ").map_err(|e| e.to_string())?;

            // #3
            let mut insert_temp_child_statement = transaction.prepare("
                INSERT INTO TempUniverseTagChildren (
                    parent, child
                ) VALUES (?, ?)
            ").map_err(|e| e.to_string())?;
            for parent_id in creator.parents {
                insert_temp_child_statement.execute(params![parent_id as i64, None as Option<i64>])
                    .map_err(|e| e.to_string())?;
            }
            for child_id in creator.children {
                insert_temp_child_statement.execute(params![None as Option<i64>, child_id as i64])
                    .map_err(|e| e.to_string())?;
            }
            // #4 and #5
            transaction.execute_batch("
                UPDATE TempUniverseTagChildren
                    SET parent=(SELECT * FROM TempUniverseTag) WHERE parent=NULL;
                UPDATE TempUniverseTagChildren
                    SET child=(SELECT * FROM TempUniverseTag) WHERE child=NULL;
                INSERT INTO UniverseTagChildren (parent, child)
                    SELECT parent, child FROM TempUniverseTagChildren;
                DELETE FROM TempUniverseTagChildren;
            ").map_err(|e| e.to_string())?;

            // #6
            let mut insert_relations_statement = transaction.prepare("
                INSERT INTO TempUniverseTagRelations (id1, id2, relation) VALUES (?, ?, ?)
            ").map_err(|e| e.to_string())?;
            for (related_tag_id, relation) in creator.related_universe_tags {
                let (first_spot, db_relation) = DBSourceRelation::normalize(relation);
                // #6-1
                let (id1, id2) = if first_spot {
                    (Some(related_tag_id as i64), None)
                } else {
                    (None, Some(related_tag_id as i64))
                };
                insert_relations_statement.execute(params![id1, id2, db_relation])
                    .map_err(|e| e.to_string())?;
            }
            // #7 and #8
            transaction.execute_batch("
                UPDATE TempUniverseTagRelations
                    SET id1=(SELECT * FROM TempUniverseTag) WHERE id1=NULL;
                UPDATE TempUniverseTagRelations
                    SET id2=(SELECT * FROM TempUniverseTag) WHERE id2=NULL;
                INSERT INTO UniverseTagRelations (universe_tag_id1, universe_tag_id2, relation)
                    SELECT id1, id2, relation FROM TempUniverseTagRelations;
                DELETE FROM TempUniverseTagRelations;
            ").map_err(|e| e.to_string())?;

            // #9
            let mut insert_link_temp_descriptions_statement = transaction.prepare("
                INSERT INTO TempUniverseTagStrings (
                    names_id, descriptions_id
                ) VALUES (NULL, last_insert_rowid());
            ").map_err(|e| e.to_string())?;
            let mut insert_link_statement = transaction.prepare("
                INSERT INTO UniverseTagRelatedLinks (
                    universe_tag_id, url, descriptions
                ) VALUES (
                    (SELECT * FROM TempUniverseTag),
                    ?,
                    (SELECT descriptions_id FROM TempUniverseTagStrings)
                )
            ").map_err(|e| e.to_string())?;
            for related_link in related_links {
                // #9-1
                api::insert_new_lang_map(related_link.descriptions(), &transaction)?;
                insert_link_temp_descriptions_statement.execute(rusqlite::NO_PARAMS)
                    .map_err(|e| e.to_string())?;
                // #9-2
                insert_link_statement.execute(&[related_link.url().as_str()])
                    .map_err(|e| e.to_string())?;
                transaction.execute_batch("DELETE FROM TempUniverseTagStrings;")
                    .map_err(|e| e.to_string())?;
            }
        }

        transaction.commit()
            .map_err(|e| e.to_string())?;
    }

    let inserted_id = db.query_row("SELECT * FROM TempUniverseTag", rusqlite::NO_PARAMS, |row| {
        let id: i64 = row.get(0)?;
        Ok(id as u64)
    }).map_err(|e| e.to_string())?;

    Ok(inserted_id)
}
