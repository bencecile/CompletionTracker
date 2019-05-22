use rouille::{self, Response};

use serde_json::{json};

use super::{Api};

use crate::settings::{Settings};
use crate::lang::{Lang};

/// Implement the methods GET methods that return HTML
impl Api {
    /// Gets the home page
    pub fn page_home(&self, settings: &Settings) -> Response {
        let data = json!({});
        Response::html(rouille::try_or_400!(
            self.html_generator.generate(&settings, "home", data)
        ))
    }
    /// Send the 404 page missing response
    pub fn page_missing(&self, settings: &Settings) -> Response {
        let data = json!({});
        // Return a full HTML 404 for easy navigation
        Response::html(rouille::try_or_400!(
            self.html_generator.generate(&settings, "404", data)
        )).with_status_code(404)
    }

    /// Sends the page that shows all of the sources
    pub fn page_sources(&self, settings: &Settings) -> Response {
        // Give the user the front page for the sources
        let data = json!({});
        Response::html(rouille::try_or_400!(
            self.html_generator.generate(&settings, "sources", data)
        ))
    }
    /// Creates a resposne for the page that can create a new source
    pub fn page_sources_new(&self, settings: &Settings) -> Response {
        // The page for creating a new source
        let data = json!({
            "langs": Lang::all_short_str(),
            // "universes": source_db.universes_ui(settings.lang()),
        });
        Response::html(rouille::try_or_400!(
            self.html_generator.generate(&settings, "sources_new", data)
        ))
    }

    /// Creates a response for the page that shows all the universes
    pub fn page_universes(&self, settings: &Settings) -> Response {
        // Show the available universes here (high-level)
        let data = json!({
            // "universes": source_db.universes_ui(settings.lang()),
        });
        Response::html(rouille::try_or_400!(
            self.html_generator.generate(&settings, "universes", data)
        ))
    }
    pub fn page_universes_new(&self, settings: &Settings) -> Response {
        // Show the page for creating a new universe
        let data = json!({
            "langs": Lang::all_short_str(),
        });
        Response::html(rouille::try_or_400!(
            self.html_generator.generate(&settings, "universe_new", data)
        ))
    }
}
