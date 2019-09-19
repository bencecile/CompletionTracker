use serde_derive::{Deserialize, Serialize};
use rusqlite::{Connection};

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

// TODO May be able to filter out the bottom 80% of search results if we get A LOT of results
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
    let mut statement = db.prepare(&format!("
        SELECT {0}.id, Strings.english, Strings.japanese FROM Strings
            INNER JOIN {0}
            ON Strings.id={0}.names OR Strings.id={0}.descriptions
    ", item_type.table_name())).map_err(|e| e.to_string())?;
    let mapped_results = statement.query_map(rusqlite::NO_PARAMS, |row| {
        let id: i64 = row.get(0)?;
        let english: Option<String> = row.get(1)?;
        let japanese: Option<String> = row.get(2)?;

        let scores = &[
            english.map_or(0_f64, |english| make_search_score(&english, query)),
            japanese.map_or(0_f64, |japanese| make_search_score(&japanese, query)),
        ];

        let mut max_score = scores[0];
        for &score in &scores[1..] {
            if score > max_score {
                max_score = score;
            }
        }

        Ok(SearchResult {
            id: id as u64,
            item_type,
            search_score: max_score,
        })
    }).map_err(|e| e.to_string())?;

    let mut search_results = Vec::new();
    for search_result in mapped_results {
        let search_result = search_result.map_err(|e| e.to_string())?;

        // Don't include the ones that didn't get matched at all
        if search_result.search_score.abs() > 1e-10 {
            search_results.push(search_result);
        }
    }
    Ok(search_results)
}

/// Good searching criteria:
/// - Any haystack and query that match at the beginning should show up first
/// - We should be able to match fragments of the query in the haystack
/// - Matching a lot of characters should be able to out-score a small match
/// - Matching the haystack entirely should come before the same full match but with a
///   bigger haystack
///
/// Returns the search score. Will be 0 if there was no match
fn make_search_score(haystack: &str, query: &str) -> f64 {
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

    // Figures out the chain score for a matched chain of characters
    let chain_score = |matched_char_count, haystack_index| {
        let haystack_start_index = haystack_index - matched_char_count;

        // The basic score that we reduce based on penalties
        let base_score = matched_char_count as f64;
        // Penalize matches that happen later in the haystack
        // Will be 1 if the match starts right at the beginning
        // Limit down to 0 at the end of the haystack
        let late_match_penalty = 1_f64 - (haystack_start_index as f64) / (haystack_len as f64);
        // Penalizes matches that don't get the entire haystack
        let haystack_fragment_penalty = (matched_char_count as f64) / (haystack_len as f64);
        // Penalizes matches that don't use up the entire query
        let query_fragment_penalty = (matched_char_count as f64) / (query_len as f64);

        base_score * late_match_penalty * haystack_fragment_penalty * query_fragment_penalty
    };

    let mut search_score = 0_f64;
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
                search_score += chain_score(matched_char_count, haystack_index);
            }
        }
    }

    if let Some(matched_char_count) = matched_chars.take() {
        // Using the length will ensure that the start_index can be 0 if the match starts at 0
        search_score += chain_score(matched_char_count, haystack_len);
    }

    search_score
}
