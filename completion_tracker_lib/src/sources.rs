pub mod api;
pub mod source_types;

use rusqlite::{Connection};

pub fn create_tables(db: &Connection) -> Result<(), String> {
    // Future Notes:
    // - All of the relations are id1 -> id2
    //   So if id1 -> Before -> id2, then id2 -> After -> id1
    // - All of the tables end with a plural form of the word
    //   If it's a sub-table, it will be [singular main table name][plural other name]
    db.execute_batch("
        CREATE TABLE IF NOT EXISTS Strings (
            id INTEGER PRIMARY KEY,
            english TEXT,
            japanese TEXT,
            CHECK(NOT is_empty(english) OR NOT is_empty(japanese))
        );
    ").map_err(|e| format!("Error creating the Strings table: {}", e))?;
    db.execute_batch("
        CREATE TABLE IF NOT EXISTS UniverseTags (
            id INTEGER PRIMARY KEY,
            names INTEGER NOT NULL REFERENCES Strings(id),
            descriptions INTEGER NOT NULL REFERENCES Strings(id)
        );
        CREATE TABLE IF NOT EXISTS UniverseTagChildren (
            id INTEGER PRIMARY KEY,
            parent INTEGER NOT NULL REFERENCES UniverseTags(id),
            child INTEGER NOT NULL REFERENCES UniverseTags(id),
            UNIQUE(parent, child),
            CHECK(parent != child)
        );
        CREATE TABLE IF NOT EXISTS UniverseTagRelations (
            id INTEGER PRIMARY KEY,
            universe_tag_id1 INTEGER NOT NULL REFERENCES UniverseTags(id),
            universe_tag_id2 INTEGER NOT NULL REFERENCES UniverseTags(id),
            relation TEXT NOT NULL,
            UNIQUE(universe_tag_id1, universe_tag_id2)
        );
        CREATE TABLE IF NOT EXISTS UniverseTagRelatedLinks (
            id INTEGER PRIMARY KEY,
            universe_tag_id INTEGER NOT NULL REFERENCES UniverseTags(id),
            url TEXT NOT NULL,
            descriptions INTEGER NOT NULL REFERENCES Strings(id),
            UNIQUE(universe_tag_id, url)
        );
    ").map_err(|e| format!("Error creating the Universe Tag tables: {}", e))?;
    db.execute_batch("
        CREATE TABLE IF NOT EXISTS People (
            id INTEGER PRIMARY KEY,
            names INTEGER NOT NULL REFERENCES Strings(id),
            descriptions INTEGER NOT NULL REFERENCES Strings(id),
            birth_country TEXT,
            birth_date TEXT,
            death_date TEXT
        );
        CREATE TABLE IF NOT EXISTS PersonAliases (
            id INTEGER PRIMARY KEY,
            person_id INTEGER NOT NULL REFERENCES People(id),
            lang TEXT NOT NULL,
            alias TEXT NOT NULL,
            UNIQUE(person_id, lang, alias)
        );
    ").map_err(|e| format!("Error creating the People tables: {}", e))?;
    db.execute_batch("
        CREATE TABLE IF NOT EXISTS Companies (
            id INTEGER PRIMARY KEY,
            names INTEGER NOT NULL REFERENCES Strings(id),
            descriptions INTEGER NOT NULL REFERENCES Strings(id),
            country TEXT
        );
    ").map_err(|e| format!("Error creating the Company tables: {}", e))?;
    db.execute_batch("
        CREATE TABLE IF NOT EXISTS Characters (
            id INTEGER PRIMARY KEY,
            names INTEGER NOT NULL REFERENCES Strings(id),
            descriptions INTEGER NOT NULL REFERENCES Strings(id)
        );
        CREATE TABLE IF NOT EXISTS CharacterAliases (
            id INTEGER PRIMARY KEY,
            character_id INTEGER NOT NULL REFERENCES Characters(id),
            lang TEXT NOT NULL,
            alias TEXT NOT NULL,
            UNIQUE(character_id, lang, alias)
        );
        CREATE TABLE IF NOT EXISTS CharacterRelations (
            id INTEGER PRIMARY KEY,
            character_id1 INTEGER NOT NULL REFERENCES Characters(id),
            character_id2 INTEGER NOT NULL REFERENCES Characters(id),
            descriptions INTEGER NOT NULL REFERENCES Strings(id),
            UNIQUE(character_id1, character_id2),
            CHECK(character_id1 != character_id2)
        );
    ").map_err(|e| format!("Error creating the Characters tables: {}", e))?;
    db.execute_batch("
        CREATE TABLE IF NOT EXISTS Sources (
            id INTEGER PRIMARY KEY,
            names INTEGER NOT NULL REFERENCES Strings(id),
            descriptions INTEGER NOT NULL REFERENCES Strings(id),
            source_type TEXT NOT NULL
        );
        CREATE TABLE IF NOT EXISTS SourceRelatedLinks (
            id INTEGER PRIMARY KEY,
            source_id INTEGER NOT NULL REFERENCES Sources(id),
            url TEXT NOT NULL,
            descriptions INTEGER NOT NULL REFERENCES Strings(id),
            UNIQUE(source_id, url)
        );
        CREATE TABLE IF NOT EXISTS SourceDates (
            id INTEGER PRIMARY KEY,
            source_id INTEGER NOT NULL REFERENCES Sources(id),
            date TEXT NOT NULL,
            descriptions INTEGER NOT NULL REFERENCES Strings(id),
            UNIQUE(source_id, date)
        );
        CREATE TABLE IF NOT EXISTS SourceUniverseTags (
            id INTEGER PRIMARY KEY,
            source_id INTEGER NOT NULL REFERENCES Sources(id),
            universe_tag_id INTEGER NOT NULL REFERENCES UniverseTags(id),
            UNIQUE(source_id, universe_tag_id)
        );
        CREATE TABLE IF NOT EXISTS SourceRelations (
            id INTEGER PRIMARY KEY,
            source_id1 INTEGER NOT NULL REFERENCES Sources(id),
            source_id2 INTEGER NOT NULL REFERENCES Sources(id),
            relation TEXT NOT NULL,
            UNIQUE(source_id1, source_id2),
            CHECK(source_id1 != source_id2)
        );
        CREATE TABLE IF NOT EXISTS SourcePeople (
            id INTEGER PRIMARY KEY,
            source_id INTEGER NOT NULL REFERENCES Sources(id),
            person_id INTEGER NOT NULL REFERENCES People(id),
            role TEXT NOT NULL
        );
        CREATE TABLE IF NOT EXISTS SourcePersonVoiceActors (
            id INTEGER PRIMARY KEY,
            source_person_id INTEGER NOT NULL REFERENCES SourcePeople(id),
            character_id INTEGER NOT NULL REFERENCES Characters(id),
            lang TEXT NOT NULL,
            UNIQUE(source_person_id, character_id, lang)
        );
        CREATE TABLE IF NOT EXISTS SourceCharacters (
            id INTEGER PRIMARY KEY,
            source_id INTEGER NOT NULL REFERENCES Sources(id),
            character_id INTEGER NOT NULL REFERENCES Characters(id),
            UNIQUE(source_id, character_id)
        );
        CREATE TABLE IF NOT EXISTS SourceCompanies (
            id INTEGER PRIMARY KEY,
            source_id INTEGER NOT NULL REFERENCES Sources(id),
            company_id INTEGER NOT NULL REFERENCES Companies(id),
            company_role TEXT,
            UNIQUE(source_id, company_id, company_role)
        );
    ").map_err(|e| format!("Error creating the Sources tables: {}", e))?;
    Ok(())
}
