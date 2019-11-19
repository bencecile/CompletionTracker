mod search_score;

use serde::{Deserialize, Serialize};
use rusqlite::{Connection};

use self::{
    search_score::{SearchScore},
};
use crate::db_link::{ConnectionHolder};

#[derive(Copy, Clone, Deserialize, Serialize)]
pub enum ItemType {
    Character,
    Company,
    Person,
    UniverseTag,
    Source,
}
impl ItemType {
    fn all() -> &'static [ItemType] {
        &[
            Self::Character,
            Self::Company,
            Self::Person,
            Self::UniverseTag,
            Self::Source,
        ]
    }
    fn table_name(self) -> &'static str {
        match self {
            Self::Character => "Characters",
            Self::Company => "Companies",
            Self::Person => "People",
            Self::UniverseTag => "UniverseTags",
            Self::Source => "Sources",
        }
    }
}

#[derive(Deserialize)]
pub struct SearchQuery {
    pub query: String,
    pub item_type: Option<ItemType>,
}

pub type SearchResults = Vec<SearchResult>;
#[derive(Serialize)]
pub struct SearchResult {
    pub id: u64,
    pub item_type: ItemType,
    // A score for how well-matched this result is to the query
    pub search_score: f64,
}

pub fn search(db: &ConnectionHolder, query: SearchQuery) -> Result<SearchResults, String> {
    let db = db.lock();

    if let Some(item_type) = query.item_type {
        search_names_and_descriptions(&db, &query.query, item_type)
    } else {
        let mut all_search_results = Vec::new();
        for &item_type in ItemType::all() {
            all_search_results.append(
                &mut search_names_and_descriptions(&db, &query.query, item_type)?
            );
        }

        Ok(all_search_results)
    }
}

fn search_names_and_descriptions(db: &Connection, query: &str, item_type: ItemType)
-> Result<SearchResults, String> {
    // TODO We will want to get aliases for sources
    let mut statement = db.prepare(&format!("
        SELECT {0}.id, Strings.english, Strings.japanese FROM Strings
            INNER JOIN {0}
            ON Strings.id={0}.names OR Strings.id={0}.descriptions
    ", item_type.table_name())).map_err(|e| e.to_string())?;
    let mapped_results = statement.query_map(rusqlite::NO_PARAMS, |row| {
        let id: i64 = row.get(0)?;
        let english: Option<String> = row.get(1)?;
        let japanese: Option<String> = row.get(2)?;

        let scores = vec![
            english.map(|english| make_search_score(&english, query)),
            // TODO Convert any hiragana to katakana
            //  This will give use a nice and easy search interface
            japanese.map(|japanese| make_search_score(&japanese, query)),
        ];
        let mut top_score = None;
        for score in scores {
            if let Some(score) = score {
                if let Some(top_score) = top_score.as_mut() {
                    if &score > top_score {
                        *top_score = score;
                    }
                } else {
                    top_score = Some(score);
                }
            }
        }

        Ok( (id as u64, top_score.unwrap()) )
    }).map_err(|e| e.to_string())?;

    let mut search_results = SearchResults::new();
    for search_result in mapped_results {
        let (id, search_score) = search_result.map_err(|e| e.to_string())?;
        // Search the ones that we have to see if we can combine any search results
        let try_is_dup = search_results.iter_mut()
        .find(|result| result.id == id);
        if let Some(found_search_result) = try_is_dup {
            found_search_result.search_score += search_score.score();
        } else {
            search_results.push(SearchResult {
                id,
                item_type,
                search_score: search_score.score(),
            });
        }
    }

    // Cutoff at 50% of the highest score
    let top_score = search_results.iter()
        .fold(0_f64, |top_score, search_result| {
            if search_result.search_score > top_score {
                search_result.search_score
            } else {
                top_score
            }
        });
    let search_results = search_results.into_iter().filter(|search_result| {
        search_result.search_score > (top_score * 0.50)
    }).collect();
    Ok(search_results)
}

/// Good searching criteria:
/// 1. Any haystack and query that match at the beginning should show up first
/// 2. We should be able to match fragments of the query in the haystack
/// 3. Matching a lot of characters should be able to out-score a small match
/// 4. Matching the haystack entirely should come before the same full match but with a
///    bigger haystack
///
/// Returns the search score. Will be 0 if there was no match
fn make_search_score(haystack: &str, query: &str) -> SearchScore {
    // Chain Score X Position
    // Search incrementally from the front of the haystack and query
    let query_chars: Vec<char> = query.chars().collect();
    let haystack_chars: Vec<char> = haystack.chars().collect();

    let query_len = query_chars.len();
    let haystack_len = haystack_chars.len();

    // See if any character in the query can match the character
    // Returns the Option<index> of the matched character in the query
    let can_match_char = |c, start_index| -> Option<usize> {
        for i in start_index..query_chars.len() {
            if query_chars[i] == c {
                return Some(i);
            }
        }
        None
    };

    let mut search_score = SearchScore::new(query_len, haystack_len);
    // The number of consecutively matched chars (currently)
    let mut matched_chars: Option<usize> = None;
    // The character that we haven't found yet
    let mut query_index = 0_usize;

    for (haystack_index, c) in haystack_chars.into_iter().enumerate() {
        if let Some(i) = can_match_char(c, query_index) {
            query_index = i;
            let matched_char_count = matched_chars.get_or_insert(0);
            *matched_char_count += 1;

            // Early termination if we've exhausted the query and won't be able to match again
            if query_index + 1 == query_len {
                break;
            }
        } else {
            if let Some(matched_char_count) = matched_chars.take() {
                search_score.update(matched_char_count, haystack_index);
            }
        }
    }

    if let Some(matched_char_count) = matched_chars.take() {
        // Using the length will ensure that the start_index can be 0 if the match starts at 0
        search_score.update(matched_char_count, haystack_len);
    }

    search_score
}
