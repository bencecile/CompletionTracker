use rouille::{Request, Response};

use completion_tracker_lib::db_link::{ConnectionHolder};
use completion_tracker_lib::sources::api::universe_tag::{
    self,
    UniverseTagCreator, UniverseTagReader,
};

use crate::api_json::{APIResult};

pub fn create_request(sources_db: &ConnectionHolder, req: &Request) -> Response {
    let creator: UniverseTagCreator = match rouille::input::json_input(req) {
        Ok(creator) => creator,
        Err(e) => return Response::json(
            &APIResult {
                success: false,
                data: format!("Failed to make a UniverseTagCreator: {}", e),
            }
        ),
    };
    let id = match universe_tag::create(&sources_db, creator) {
        Ok(id) => id,
        Err(e) => return Response::json(
            &APIResult {
                success: false,
                data: format!("Failed to create a new Universe Tag: {}", e),
            }
        ),
    };

    Response::json(
        &APIResult {
            success: true,
            data: id,
        }
    )
}

pub fn read_root_request(sources_db: &ConnectionHolder, _req: &Request) -> Response {
    let root_ids = match universe_tag::read_root_level_ids(&sources_db) {
        Ok(root_ids) => root_ids,
        Err(e) => return Response::json(
            &APIResult {
                success: false,
                data: format!("Failed to read the root Universe Tag IDs: {}", e),
            }
        ),
    };

    Response::json(
        &APIResult {
            success: true,
            data: root_ids,
        }
    )
}

pub fn read_request(sources_db: &ConnectionHolder, req: &Request) -> Response {
    let reader: UniverseTagReader = match rouille::input::json_input(req) {
        Ok(reader) => reader,
        Err(e) => return Response::json(
            &APIResult {
                success: false,
                data: format!("Failed to make a UniverseTagReader: {}", e),
            }
        ),
    };
    let read_result = match universe_tag::read_list(&sources_db, reader) {
        Ok(read_result) => read_result,
        Err(e) => return Response::json(
            &APIResult {
                success: false,
                data: format!("Failed to read a Universe Tag: {}", e),
            }
        ),
    };

    Response::json(
        &APIResult {
            success: true,
            data: read_result,
        }
    )
}
