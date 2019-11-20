use rouille::{Request, Response};

use completion_tracker_lib::{
    db_link::{ConnectionHolder},
    sources::api::search::{
        SearchQuery, SearchResults,
        search as api_search
    },
};

use crate::api_json::{APIResult};

pub fn search(sources_db: &ConnectionHolder, req: &Request) -> Response {
    let search_query: SearchQuery = match rouille::input::json_input(req) {
        Ok(search_query) => search_query,
        Err(e) => return Response::json(
            &APIResult {
                success: false,
                data: format!("Failed to make a SearchQuery: {}", e),
            }
        ),
    };
    let search_results: SearchResults = match api_search(&sources_db, search_query) {
        Ok(search_results) => search_results,
        Err(e) => return Response::json(
            &APIResult {
                success: false,
                data: format!("Failed to get any search results: {}", e),
            }
        ),
    };

    Response::json(
        &APIResult {
            success: true,
            data: search_results,
        }
    )
}
