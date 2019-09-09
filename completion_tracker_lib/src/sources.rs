// Define this first for the macro
mod simple_enum;

pub mod api;
pub mod source_types;

use rusqlite::{Connection};

pub fn create_tables(db_connection: &Connection) -> Result<(), String> {
    // Future Notes:
    // - All of the relations are id1 -> id2
    //   So if id1 -> Before -> id2, then id2 -> After -> id1
    // - All of the tables end with a plural form of the word
    //   If it's a sub-table, it will be [singular main table name][plural other name]
    db_connection.execute(r#"CREATE TABLE IF NOT EXISTS Strings (
        id INTEGER PRIMARY KEY,
        english TEXT,
        japanese TEXT
    );
    
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
    CREATE TABLE IF NOT EXISTS UniverseTagLinks (
        id INTEGER PRIMARY KEY,
        universe_tag_id INTEGER NOT NULL REFERENCES UniverseTags(id),
        url TEXT NOT NULL,
        descriptions INTEGER NOT NULL REFERENCES Strings(id),
        UNIQUE(universe_tag_id, url)
    );

    CREATE TABLE IF NOT EXISTS People (
        id INTEGER PRIMARY KEY,
        names INTEGER NOT NULL REFERENCES Strings(id),
        descriptions INTEGER NOT NULL REFERENCES Strings(id)
    );
    CREATE TABLE IF NOT EXISTS PersonAliases (
        id INTEGER PRIMARY KEY,
        person_id INTEGER NOT NULL REFERENCES People(id),
        lang TEXT NOT NULL,
        alias TEXT NOT NULL,
        UNIQUE(person_id, lang, alias)
    );

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
    
    CREATE TABLE IF NOT EXISTS Sources (
        id INTEGER PRIMARY KEY,
        names INTEGER NOT NULL REFERENCES Strings(id),
        descriptions INTEGER NOT NULL REFERENCES Strings(id),
        source_type TEXT NOT NULL
    );
    CREATE TABLE IF NOT EXISTS SourceLinks (
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
        person_id INTEGER NOT NULL REFERENCES People(id)
        UNIQUE(source_id, person_id)
    );
    CREATE TABLE IF NOT EXISTS SourcePersonRoles (
        id INTEGER PRIMARY KEY,
        source_person_id INTEGER NOT NULL REFERENCES SourcePeople(id),
        role TEXT NOT NULL,
        voice_actor_character_id INTEGER REFERENCES Characters(id),
        UNIQUE(source_person_id, role),
        CHECK(
            CASE role
                WHEN 'VoiceActor' THEN
                    voice_actor_character_id IS NOT NULL
                WHEN NOT 'VoiceActor' THEN
                    voice_actor_character_id IS NULL
            END
        )
    );
    CREATE TABLE IF NOT EXISTS SourceCharacters (
        id INTEGER PRIMARY KEY,
        source_id INTEGER NOT NULL REFERENCES Sources(id),
        character_id INTEGER NOT NULL REFERENCES Characters(id),
        UNIQUE(source_id, character_id)
    );"#, rusqlite::NO_PARAMS).map(|_| ()).map_err(|e| e.to_string())
}
