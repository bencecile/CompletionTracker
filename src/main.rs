mod html_gen;
mod lang;
mod run_info;
mod settings;
mod source;
mod static_serve;
mod tracking;
mod utils;

use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::time::{Duration};

use rouille::{Response, Server, router};

use serde_json::{json};

use crate::html_gen::{HtmlGenerator};
use crate::lang::{Lang, LangSelect};
use crate::run_info::{RunInfo};
use crate::settings::{Settings};
use crate::source::{SourceInfo};
use crate::static_serve::{StaticFile, static_serve};

const RUN_INFO_FILE: &'static str = "runInfo.json";

// This is the signal to exit the program
static SHUTDOWN_SIGNAL: AtomicBool = AtomicBool::new(false);
// The time to wait before polling the connection
const POLL_WAIT_TIME: Duration = Duration::from_millis(20);

fn main() -> Result<(), String> {
    // Read in the runInfo.json file that will give us some constants
    let run_info = RunInfo::new(RUN_INFO_FILE)?;
    // Get all of the trackers we will need
    let trackers = run_info.trackers()?;
    // Create the HTML generator we will share
    let html_generator = HtmlGenerator::new()?;

    // Create the source folder if it doesn't exist
    source::create_source_folder()?;

    // Read in all of the source material
    let source_info = SourceInfo::new();
    // Check the sources for any errors on startup
    // This will either indicate a bug, or user tampering
    source_info.check_integrity(Lang::new(LangSelect::EN))?;

    // Start up the server
    let server = Server::new(run_info.socket_addr(), move |req| {
        // Check the cookies to build out the settings for this request
        let settings = Settings::new(&req);

        let response = router!(req,
            (GET) (/) => {
                let data = json!({
                    "lang": settings.lang().ui(),
                });
                Response::html(rouille::try_or_400!(
                    html_generator.generate("home", &data)
                ))
            },
            (POST) (/shutdown) => {
                // This signals that the server should be shut down
                SHUTDOWN_SIGNAL.store(true, Ordering::Relaxed);
                Response::text("Shutting down")
            },

            (GET) (/sources) => {
                // Give the user the front page for the sources
                let data = json!({
                    "lang": settings.lang().ui(),
                });
                Response::html(rouille::try_or_400!(
                    html_generator.generate("sources", &data)
                ))
            },
            (GET) (/sources/new) => {
                // The page for creating a new source
                let data = json!({
                    "lang": settings.lang().ui(),
                    "langs": LangSelect::all_short_str(),
                    "universes": source_info.universes_ui(settings.lang().lang()),
                });
                Response::html(rouille::try_or_400!(
                    html_generator.generate("sources_new", &data)
                ))
            },
            // (POST) (/sources/new) => {
            //     // Process the form data here to create a new source
            // },
            // (GET) (/sources/search) => { TODO
            //     // Give a list of results based of the query parameter
            //     // Give a shortened list of results for each category
            //     // Have a link to show more which will expand the category to fill the results
            // },
            (GET) (/sources/universes) => {
                // Show the avaiable universes here (high-level)
                let data = json!({
                    "lang": settings.lang().ui(),
                    "universes": source_info.universes_ui(settings.lang().lang()),
                });
                Response::html(rouille::try_or_400!(
                    html_generator.generate("universes", &data)
                ))
            },
            (GET) (/sources/universe/edit) => {
                // The page for creating new universes, series and arcs
                let data = json!({
                    "lang": settings.lang().ui(),
                    "universes": source_info.universes_ui(settings.lang().lang()),
                });
                Response::html(rouille::try_or_400!(
                    html_generator.generate("universe_edit", &data)
                ))
            },
            // (POST) (/sources/universe/edit) => {
            //     // Process the format data her to create new universes, series and/or arcs
            // },
            (GET) (/sources/universe/new) => {
                // Show the page for creating a new universe
                let data = json!({
                    "lang": settings.lang().ui(),
                    "langs": LangSelect::all_short_str(),
                });
                Response::html(rouille::try_or_400!(
                    html_generator.generate("universe_new", &data)
                ))
            },
            // (POST) (/sources/universe/new) => {
            //     // Attempt to create a new universe
            // },

            // ---- For the static files ----
            (GET) (/css/{file: String}) => {
                static_serve(StaticFile::Css(file))
            },
            (GET) (/js/{file: String}) => {
                static_serve(StaticFile::Js(file))
            },
            _ => {
                // Return a full HTML 404 for easy navigation
                Response::html(rouille::try_or_400!(
                    html_generator.generate("404", &())
                ))
            },
        );
        // Set any custom headers on the Response here
        let response = settings.set_cookies(response);

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
