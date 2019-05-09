use std::fs;
use std::path::{Path};

use handlebars::{Handlebars, RenderError};

use serde::{Serialize};

/// The directory for the HTML templates
const TEMPLATE_DIR: &'static str = "htmlTemplates";
/// The directory for the handlebar partials
const PARTIAL_DIR: &'static str = "htmlPartials";

pub struct HtmlGenerator {
    handlebars: Handlebars,
}
impl HtmlGenerator {
    pub fn new() -> Result<HtmlGenerator, String> {
        // Create the handlebars object we will use for generating the HTML
        let mut handlebars = Handlebars::new();
        // Only enable source mapping if we are running on Debug
        if cfg!(debug_assertions) {
            handlebars.source_map_enabled(false);
        }

        // Register all of the parials
        for entry in Path::new(PARTIAL_DIR).read_dir().map_err(|e| e.to_string())? {
            let entry = entry.map_err(|e| e.to_string())?;
            let path = entry.path();
            // Use the base name of the files as the name of the partial
            let name = path.file_stem().unwrap().to_str().unwrap();
            let partial = fs::read_to_string(&path)
                .map_err(|e| e.to_string())?;
            handlebars.register_partial(name, partial)
                .map_err(|e| e.to_string())?;
        }

        // Register the templates
        handlebars.register_templates_directory(".hbs", TEMPLATE_DIR)
            .map_err(|e| e.to_string())?;

        Ok(HtmlGenerator {
            handlebars,
        })
    }

    pub fn generate<T>(&self, template: &str, data: &T) -> Result<String, RenderError>
    where T: Serialize {
        self.handlebars.render(template, data)
    }
}
