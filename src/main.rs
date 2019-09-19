mod api_json;
mod run_info;
mod static_server;

use rouille::{Response, Server, router};

use crate::run_info::{RunInfo};
use crate::static_server::{StaticServer};

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

fn main() -> Result<(), String> {
    let run_info = RunInfo::new_default()
        .map_err(|e| format!("Error in the runInfo.json: {}", e))?;
    let trackers = run_info.trackers()?;

    let sources_db = completion_tracker_lib::init_source_db()?;

    // Start up the server
    let server = Server::new(run_info.socket_addr(), move |req| router!(req,
        (GET) (/jsbundle) => { StaticServer.serve_bundle_js() },

        (POST) (/api/search) => {
            crate::api_json::search(&sources_db, req)
        },

        (POST) (/api/universeTag/create) => {
            crate::api_json::universe_tag::create_request(&sources_db, req)
        },
        (POST) (/api/universeTags/read) => {
            crate::api_json::universe_tag::read_request(&sources_db, req)
        },
        (POST) (/api/universeTags/readRoot) => {
            crate::api_json::universe_tag::read_root_request(&sources_db, req)
        },

        _ => {
            if req.method() == "GET" {
                // Send the main page, inserting the path after the "#"
                //  This is good for saved pages (bookmarks or otherwise)
                StaticServer.serve_main_html()
            } else {
                // The only time we should ever hit this is if the frontend misses a call
                //  or somebody calls this directly
                // In both cases, a normal 404 should be enough to tell them to
                //  take their business elsewhere
                Response::empty_404()
            }
        },
        // All the other real API calls
    )).expect("Failed to create the server");
    // Set the fixed pool size to something small since this is only running locally
    let server = server.pool_size(10);

    println!("Running on {}", server.server_addr());
    server.run();

    Ok(())
}
