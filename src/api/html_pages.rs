use rouille::{self, Response};

use serde_json::{Value, json};

use super::{Api};

/// Implement the methods GET methods that return HTML
impl Api {
    /// Gets the home page
    pub fn page_home(&self) -> Response {
        let data = json!({});
        self.generate_page("home", data)
    }
    /// Send the 404 page missing response
    pub fn page_missing(&self) -> Response {
        let data = json!({});
        // Return a full HTML 404 for easy navigation (for the user)
        self.generate_page("404", data)
    }

    /// Sends the page that shows all of the sources
    pub fn page_sources(&self) -> Response {
        // Give the user the front page for the sources
        let data = json!({});
        self.generate_page("sources", data)
    }
    /// Creates a resposne for the page that can create a new source
    pub fn page_source_new(&self) -> Response {
        // The page for creating a new source
        let data = json!({});
        self.generate_page("sources_new", data)
    }

    /// Creates a response for the page that shows all the universes
    pub fn page_universes(&self) -> Response {
        // Show the available universes here (high-level)
        let data = json!({});
        self.generate_page("universes", data)
    }
    pub fn page_universe_new(&self) -> Response {
        // Show the page for creating a new universe
        let data = json!({});
        self.generate_page("universe_new", data)
    }

    
    /// The generic function to generate an HTML page
    fn generate_page(&self, page_name: &str, json_value: Value) -> Response {
        Response::html(rouille::try_or_400!(
            self.html_generator.generate(page_name, json_value)
        ))
    }
}
