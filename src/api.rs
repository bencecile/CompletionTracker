mod endpoints;
mod html_gen;
mod lang_impl;
mod html_pages;
mod static_serve;
mod ui;

use self::endpoints::{
    ApiEndpoint,
    UniverseEndpoint,
};
use self::html_gen::{HtmlGenerator};
use self::static_serve::{StaticFile};
use self::static_serve::{StaticServer};

use std::sync::{Arc, Mutex, MutexGuard};
use std::sync::atomic::{Ordering};

use rouille::{Request, Response, router};

use crate::lang::{Translatable};
use crate::source_concrete::{self, SourceDB};
use crate::tracking::{Tracker};

const LOG_TARGET: &'static str = "Api";

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
    pub fn new(trackers: Vec<Tracker>) -> Result<Api, String> {
        // Create the HTML generator that we will use on Requests
        let html_generator = HtmlGenerator::new()?;
        // Create the thing we will use to serve our static files
        let static_server = StaticServer::new();

        // Create the source folder if it doesn't exist
        source_concrete::create_source_folder()?;

        // Read in all of the source material
        let source_db = SourceDB::new();
        // Check the sources for any errors on startup
        // This will either indicate a bug, or user tampering
        source_db.check_integrity().map_err(|e| e.to_lang_strings().en)?;

        Ok(Api {
            trackers,
            html_generator,
            static_server,
            source_db: Arc::new(Mutex::new(source_db)),
        })
    }
    /// Locks the source DB so that we can access it
    fn lock_db(&self) -> MutexGuard<SourceDB> { self.source_db.lock().unwrap() }
}
/// Implement the static serve methods
impl Api {
    pub fn serve_static(&self, static_file: StaticFile) -> Response {
        self.static_server.serve_static(static_file)
    }
    pub fn serve_vue_components(&self) -> Response {
        self.static_server.serve_vue_components()
    }
}
/// Create directly related server requests
impl Api {
    /// This is the base function to actually handle a request
    pub fn handle_request(&self, req: &Request) -> Response {
        RequestPipeline::handle_request(&self, req)
    }
}

/// Whenever the server gets a request, this is created to save some calls
struct RequestPipeline<'a> {
    /// The API that created this pipeline
    api: &'a Api,
    /// The actual request object
    req: &'a Request,
}
impl <'a> RequestPipeline<'a> {
    fn handle_request(api: &'a Api, req: &'a Request) -> Response {
        let pipeline = RequestPipeline {
            api,
            req,
        };
        pipeline.generate_response()
    }

    /// Creates the response with the request in the pipeline
    fn generate_response(&self) -> Response {
        let response = self.check_specific_file_names()
            .or_else(|| self.check_web_pages())
            .or_else(|| self.check_api_endpoints())
            .unwrap_or_else(||
                // Set the 404 response as a last resort
                self.api.page_missing()
            );

        // The browser shouldn't ever cache since we are just on the local network
        let response = response.with_no_cache();

        response
    }

    /// This function will check the request for any really specific file names
    ///  that we need to handle differently
    fn check_specific_file_names(&self) -> Option<Response> {
        router!(self.req,
            // This is a special request for all of the Vue components
            (GET) ["/js/components.js"] => { Some(self.api.static_server.serve_vue_components()) },
            // TODO Make a favicon to use
            (GET) ["/favicon.ico"] => { None },
            _ => { None }
        )
    }
    /// Check if the request was for any web pages
    fn check_web_pages(&self) -> Option<Response> {
        router!(self.req,
            // Then do the HTML and other web resources
            (GET) (/) => { Some(self.api.page_home()) },
            (POST) (/shutdown) => {
                // This signals that the server should be shut down
                crate::SHUTDOWN_SIGNAL.store(true, Ordering::Relaxed);
                Some(endpoints::shutdown_response())
            },
            // Use a POST for searching since there may be non-trivial data to use as a query
            (POST) (/search) => {
                Some(Response::text("NOT IMPLEMENTED").with_status_code(404))
            },

            (GET) (/sources) => { Some(self.api.page_sources()) },
            (GET) (/source/new) => { Some(self.api.page_source_new()) },

            (GET) (/universes) => { Some(self.api.page_universes()) },
            (GET) (/universe/new) => { Some(self.api.page_universe_new()) },

            // ---- For the static files ----
            (GET) (/css/{file: String}) => { Some(self.api.serve_static(StaticFile::Css(file))) },
            (GET) (/img/{file: String}) => { Some(self.api.serve_static(StaticFile::Img(file))) },
            (GET) (/js/{file: String}) => { Some(self.api.serve_static(StaticFile::Js(file))) },
            _ => { None },
        )
    }
    /// Checks if the request is asking for an API
    fn check_api_endpoints(&self) -> Option<Response> {
        // Chain all of the API matches
        UniverseEndpoint::match_request(&self)
    }
}
