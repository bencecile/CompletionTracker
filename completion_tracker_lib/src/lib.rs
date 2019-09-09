pub mod db_link;
pub mod sources;
pub mod tracking;
pub mod utils;

use rusqlite::{Connection};

use crate::db_link::{ConnectionHolder};

pub fn init_source_db() -> Result<ConnectionHolder, String> {
    // TODO Open a real file for the DB
    let connection = Connection::open_in_memory()
        .map_err(|e| e.to_string())?;
    connection.execute("PRAGMA foreign_keys = ON", rusqlite::NO_PARAMS)
        .map_err(|e| e.to_string())?;

    sources::create_tables(&connection)?;

    Ok(ConnectionHolder::new(connection))
}

// pub fn init_tracker_dbs(trackers: Vec<Tracker>) -> Result {
// }
