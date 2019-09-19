pub mod character;
pub mod people;
pub mod search;
pub mod universe_tag;

use rusqlite::{Connection, Result as SqlResult, Row, Statement};

use crate::{impl_sql_simple_enum};
use crate::simple_enum::{SimpleEnum};
use crate::sources::source_types::{
    Lang, LangMap,
    RelatedLink, LinkType,
    Relation,
};

/// These are the Relations between items in the database.
/// This is to normalize the database so that there is only a single way to express a relationship
/// between two things.
#[derive(Copy, Clone)]
enum DBSourceRelation {
    Alternate,
    Before,
}
impl DBSourceRelation {
    /// This is with regards to the target
    /// Returns (placement, relation to put in DB).
    /// placement: true if the source id should be in the 1st spot, false if 2nd spot.
    fn normalize(relation: Relation) -> (bool, DBSourceRelation) {
        match relation {
            // The source must become the target, so switch spots in the DB
            Relation::After => (false, Self::Before),
            Relation::Before => (true, Self::Before),
            Relation::Alternate => (true, Self::Alternate),
        }
    }
    /// When a source is selected and pulled out of the DB, we will want to show a relationship
    /// that makes sense for it. Since we will show the relation with a link to the other source.
    fn denormalize(self, first_spot: bool) -> Relation {
        match self {
            Self::Alternate => Relation::Alternate,
            Self::Before => if first_spot {
                Relation::Before
            } else {
                Relation::After
            },
        }
    }
}
impl SimpleEnum for DBSourceRelation {
    fn all() -> &'static [DBSourceRelation] {
        &[
            Self::Alternate,
            Self::Before,
        ]
    }
    fn as_str(&self) -> &'static str {
        match self {
            Self::Alternate => "Alternate",
            Self::Before => "Before",
        }
    }
}
impl_sql_simple_enum!(DBSourceRelation);


/// Inserts the LangMap into the database through the connection.
fn insert_new_lang_map(map: &LangMap, db: &Connection) -> Result<(), String> {
    db.execute("INSERT INTO Strings (english, japanese) VALUES (?, ?)",
        &[
            map.get(&Lang::English),
            map.get(&Lang::Japanese)
        ]
    ).map(|_| ()).map_err(|e| e.to_string())
}
fn prepare_strings_get(db: &Connection) -> Result<Statement, String> {
    db.prepare("SELECT english, japanese FROM Strings WHERE id=?")
        .map_err(|e| e.to_string())
}
/// This assumes that the row was SELECTed from the prepare_strings_get()
fn make_lang_strings_from_row(row: &Row) -> LangMap {
    let mut lang_map = LangMap::new();
    let english: Option<String> = row.get_unwrap(0);
    let japanese: Option<String> = row.get_unwrap(1);

    if let Some(english) = english { lang_map.insert(Lang::English, english); }
    if let Some(japanese) = japanese { lang_map.insert(Lang::Japanese, japanese); }

    lang_map
}

/// Make sure that we have good links that we can support (or want).
/// Do this by verifying that they can
fn transform_related_links(raw_links: Vec<(String, LangMap)>) -> Result<Vec<RelatedLink>, String> {
    let mut links = Vec::with_capacity(raw_links.capacity());

    for (string_url, descriptions) in raw_links {
        links.push(
            RelatedLink::new(&string_url, descriptions)?
        );
    }
    Ok(links)
}

fn collect_query_map<T, I>(iterator: SqlResult<I>) -> SqlResult< Vec<T> >
where I: Iterator<Item = SqlResult<T>> {
    let iterator = iterator?;
    let mut new_vec = Vec::new();

    for item in iterator {
        new_vec.push(item?);
    }
    Ok(new_vec)
}
