pub mod data;
mod html_interface;
mod static_serve;
pub use self::static_serve::{StaticFile};
use self::static_serve::{StaticServer};

use std::sync::{Arc, Mutex, MutexGuard};

use rouille::{Request, Response};

use serde_derive::{Serialize};

use crate::html_gen::{HtmlGenerator};
use crate::lang::{Lang, Translatable};
use crate::run_info::{RunInfo};
use crate::settings::{Settings};
use crate::source::{self, SourceDB};
use crate::tracking::{Tracker};

/// This is the general API to the completion tracker
pub struct Api {
    /// The trackers for the user completion data
    trackers: Vec<Tracker>,
    /// The HTML generator to create the webpages
    html_generator: HtmlGenerator,
    /// The static server for the files that are on disk
    static_server: StaticServer,
    /// The source database
    source_db: Arc< Mutex<SourceDB> >,
}
impl Api {
    pub fn new(run_info: &RunInfo) -> Result<Api, String> {
        // Get all of the trackers we will need
        let trackers = run_info.trackers()?;

        // Create the HTML generator that we will use on Requests
        let html_generator = HtmlGenerator::new()?;
        // Create the thing we will use to serve our static files
        let static_server = StaticServer::new();

        // Create the source folder if it doesn't exist
        source::create_source_folder()?;

        // Read in all of the source material
        let source_db = SourceDB::new();
        // Check the sources for any errors on startup
        // This will either indicate a bug, or user tampering
        source_db.check_integrity().map_err(|e| e.to_lang_string(Lang::EN))?;

        Ok(Api {
            trackers,
            html_generator,
            static_server,
            source_db: Arc::new(Mutex::new(source_db)),
        })
    }
    /// Locks the source DB so that we can access it
    fn lock_db(&self) -> MutexGuard<SourceDB> { self.source_db.lock().unwrap() }

    // (GET) (/sources/search) => { TODO
    //     // Give a list of results based of the query parameter
    //     // Give a shortened list of results for each category
    //     // Have a link to show more which will expand the category to fill the results
    // },

    /// Creates a new Universe from the request
    pub fn create_new_universe_from_request(&self, req: &Request, settings: &Settings) -> Response {
        match data::from_request(req) {
            Ok(data) => ResponseJson::from_result(
                self.create_new_universe(data), settings.lang()
            ).create_response(),
            Err(_) => ResponseJson::new(
                false, settings.lang_ui().need_json_request.to_string()
            ).create_response(),
        }
    }
}

/// The JSON that is always the body of a Response
#[derive(Serialize)]
struct ResponseJson {
    success: bool,
    message: String,
}
impl ResponseJson {
    fn new(success: bool, message: String) -> ResponseJson {
        ResponseJson { success, message, }
    }
    /// Creates the response json from the result, given that both parts are translatable
    fn from_result<S, E>(result: Result<S, E>, lang: Lang) -> ResponseJson
    where S: Translatable + Sized, E: Translatable + Sized {
        ResponseJson::new(result.is_ok(), match result {
            Ok(success) => success.to_lang_string(lang),
            Err(error) => error.to_lang_string(lang),
        })
    }
    /// A shortcut function that will create a response from this ResponseJson
    fn create_response(&self) -> Response {
        Response::json(&self)
    }
}
