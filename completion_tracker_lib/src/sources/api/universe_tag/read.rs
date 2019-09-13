use rusqlite::{Error as SqlError};

use super::{UniverseTagReader};
use crate::db_link::{ConnectionHolder};
use crate::sources::api::{self, DBSourceRelation};
use crate::sources::source_types::{RelatedLink};

/// A root level UniverseTag is one that isn't the child of any other UniverseTag
pub fn read_root_level_ids(db: &ConnectionHolder) -> Result<Vec<u64>, String> {
    let db = db.lock();

    let mut get_page_statement = db.prepare(
        "SELECT id FROM UniverseTags
            WHERE id NOT IN (SELECT child FROM UniverseTagChildren)"
    ).map_err(|e| e.to_string())?;

    let root_ids = api::collect_query_map(get_page_statement.query_map(rusqlite::NO_PARAMS, |row| {
        let id: i64 = row.get(0)?;
        Ok(id as u64)
    })).map_err(|e| e.to_string())?;
    Ok(root_ids)
}

// TODO Instead of just IDs, we could pass an options object. This would have the IDs and other options to specify what gets returned (like not getting sources or other things)
pub fn read_list(db: &ConnectionHolder, ids: Vec<u64>)
-> Result<Vec<UniverseTagReader>, String> {
    let db = db.lock();

    let mut get_strings_statement = api::prepare_strings_get(&db)?;
    let mut get_children_statement = db.prepare(
        "SELECT child FROM UniverseTagChildren
            WHERE parent=?"
    ).map_err(|e| e.to_string())?;
    let mut get_parents_statement = db.prepare("
        SELECT parent FROM UniverseTagChildren
            WHERE child=?
    ").map_err(|e| e.to_string())?;
    let mut get_relations_statement = db.prepare(
        "SELECT universe_tag_id1, universe_tag_id2, relation FROM UniverseTagRelations
            WHERE universe_tag_id1=?1 OR universe_tag_id2=?1"
    ).map_err(|e| e.to_string())?;
    let mut get_links_statement = db.prepare(
        "SELECT url, descriptions FROM UniverseTagRelatedLinks
            WHERE universe_tag_id=?"
    ).map_err(|e| e.to_string())?;
    let mut get_sources_statement = db.prepare("
        SELECT source_id FROM SourceUniverseTags
            WHERE universe_tag_id=?
    ").map_err(|e| e.to_string())?;

    db.execute_batch("
        BEGIN TRANSACTION;
        CREATE TEMPORARY TABLE IF NOT EXISTS TempUniverseTagRead (
            id INTEGER NOT NULL REFRENCES UnvierseTag(id)
        );
    ").map_err(|e| e.to_string())?;
    let mut insert_temp_id_statement = db.prepare("
        INSERT INTO TempUniverseTagRead VALUES (?)
    ").map_err(|e| e.to_string())?;
    for id in ids {
        insert_temp_id_statement.execute(&[id as i64])
            .map_err(|e| e.to_string())?;
    }
    // We can't be in a transaction when querying stuff
    db.execute_batch("
        CREATE INDEX TempUniverseTagIDIndex ON TempUniverseTagRead(id);
        COMMIT;
    ").map_err(|e_commit| {
        if let Err(e_rollback) = db.execute_batch("ROLLBACK;") {
            format!("Failed to read UniverseTags ({}). Also failed to Rollback ({})",
                e_commit, e_rollback)
        } else {
            format!("Failed to read UniverseTags ({})", e_commit)
        }
    })?;

    let mut get_universe_tags_statement = db.prepare("
        SELECT id, names, descriptions FROM UniverseTags
            WHERE id IN (SELECT * FROM TempUniverseTagRead)
    ").map_err(|e| e.to_string())?;
    let mapped_tags = get_universe_tags_statement.query_map(rusqlite::NO_PARAMS, |row| {
        let id: i64 = row.get(0)?;
        let names_id: i64 = row.get(1)?;
        let descriptions_id: i64 = row.get(2)?;

        let names = get_strings_statement.query_map(&[names_id], |row| {
            Ok(api::make_lang_strings_from_row(row))
        })?.next().unwrap()?;
        let descriptions = get_strings_statement.query_map(&[descriptions_id], |row| {
            Ok(api::make_lang_strings_from_row(row))
        })?.next().unwrap()?;

        let parents = api::collect_query_map(get_parents_statement.query_map(&[id], |row| {
            let parent_id: i64 = row.get(0)?;
            Ok(parent_id as u64)
        }))?;
        let children = api::collect_query_map(get_children_statement.query_map(&[id], |row| {
            let child_id: i64 = row.get(0)?;
            Ok(child_id as u64)
        }))?;

        let related_universe_tags = api::collect_query_map(
            get_relations_statement.query_map(&[id], |row| {
                let related_id1: i64 = row.get(0)?;
                let related_id2: i64 = row.get(1)?;
                let relation: DBSourceRelation = row.get(2)?;

                if related_id1 == id {
                    Ok(
                        (related_id2 as u64, relation.denormalize(true))
                    )
                } else {
                    Ok(
                        (related_id1 as u64, relation.denormalize(false))
                    )
                }
            })
        )?;

        let related_links = api::collect_query_map(get_links_statement.query_map(&[id], |row| {
            let url: String = row.get(0)?;
            let descriptions_id: i64 = row.get(1)?;

            let descriptions = get_strings_statement.query_map(&[descriptions_id], |string_row| {
                Ok(api::make_lang_strings_from_row(string_row))
            })?.next().unwrap()?;

            Ok(
                RelatedLink::new(&url, descriptions).map_err(|e| {
                    SqlError::InvalidParameterName(e)
                })?
            )
        }))?;

        let sources = api::collect_query_map(get_sources_statement.query_map(&[id], |row| {
            let source_id: i64 = row.get(0)?;
            Ok(source_id as u64)
        }))?;

        Ok(UniverseTagReader {
            id: id as u64,
            names,
            descriptions,
            parents,
            children,
            related_universe_tags,
            related_links,
            sources,
        })
    });

    let universe_tags = api::collect_query_map(mapped_tags)
        .map_err(|e| e.to_string())?;

    db.execute_batch("
        DROP INDEX TempUniverseTagIDIndex;
        DELETE FROM TempUniverseTagRead;
    ").map_err(|e| e.to_string())?;

    Ok(universe_tags)
}
