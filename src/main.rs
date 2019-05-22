mod api;
mod html_gen;
mod lang;
mod run_info;
mod settings;
mod source;
mod tracking;
mod types;
mod utils;

use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::time::{Duration};

use rouille::{Response, Server, router};

use crate::api::{Api, StaticFile};
use crate::settings::{Settings};
use crate::run_info::{RunInfo};

// This is the signal to exit the program
static SHUTDOWN_SIGNAL: AtomicBool = AtomicBool::new(false);
// The time to wait before polling the connection
const POLL_WAIT_TIME: Duration = Duration::from_millis(20);

fn main() -> Result<(), String> {
    // Read in the run info to create our things
    let run_info = RunInfo::new_default()?;
    // Create the API that will actually handle the calls
    let api = Api::new(&run_info)?;

    // Start up the server
    let server = Server::new(run_info.socket_addr(), move |req| {
        // Check the cookies to build out the settings for this request
        let settings = Settings::new(&req);

        let response = router!(req,
            (GET) (/) => { api.page_home(&settings) },
            (POST) (/shutdown) => {
                // This signals that the server should be shut down
                SHUTDOWN_SIGNAL.store(true, Ordering::Relaxed);
                Response::text(settings.lang_ui().shutdown_response)
            },

            // Use a POST for searching since there may be non-trivial data to use as a query
            (POST) (/search) => {
                Response::text("NOT IMPLEMENTED")
            },

            (GET) (/sources) => { api.page_sources(&settings) },
            (GET) (/sources/new) => { api.page_sources_new(&settings) },
            // (POST) (/sources/new) => {
            //     // Process the form data here to create a new source
            // },

            (GET) (/sources/universes) => { api.page_universes(&settings) },
            // (GET) (/sources/universe/edit) => {
            //     // The page for creating new universes, series and arcs
            //     let data = json!({
            //         "universes": source_db.universes_ui(settings.lang()),
            //     });
            //     Response::html(rouille::try_or_400!(
            //         html_generator.generate(&settings, "universe_edit", data)
            //     ))
            // },
            // (POST) (/sources/universe/edit) => {
            //     // Process the format data her to create new universes, series and/or arcs
            // },
            (GET) (/sources/universe/new) => { api.page_universes_new(&settings) },
            (POST) (/sources/universe/new) => {
                api.create_new_universe_from_request(&req, &settings)
            },

            // ---- For the static files ----
            (GET) (/css/{file: String}) => { api.serve_static(StaticFile::Css(file)) },
            // This is a special request for all of the Vue components
            (GET) (/js/components) => { api.serve_vue_components() },
            (GET) (/js/{file: String}) => { api.serve_static(StaticFile::Js(file)) },
            (GET) (/svg/{file: String}) => { api.serve_static(StaticFile::Svg(file)) },

            // Give an HTML 404 page if nothing matched
            _ => { api.page_missing(&settings) },
        );
        // Set any custom headers on the Response here
        let response = settings.set_cookies(response);
        // We shouldn't ever need the browser to cache since we are just on the local network
        let response = response.with_no_cache();

        response
    }).expect("Failed to create the server");
    // Set the fixed pool size to something small since this is only running locally
    let server = server.pool_size(1);

    // Print out where the server is running and how you can get to it
    println!("Running on {}", server.server_addr());

    while !SHUTDOWN_SIGNAL.load(Ordering::Relaxed) {
        // Check for any connections
        server.poll();
        // Wait until we check again
        thread::sleep(POLL_WAIT_TIME);
    }

    Ok(())
}
