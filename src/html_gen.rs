use std::fs;
use std::path::{Path};

use handlebars::{Handlebars, RenderError};

use serde_json::{self, Value};

use crate::settings::{Settings};

/// The directory for the HTML templates
const TEMPLATE_DIR: &'static str = "htmlTemplates";
/// The directory for the handlebar partials
const PARTIAL_DIR: &'static str = "htmlPartials";

pub struct HtmlGenerator {
    handlebars: Handlebars,
}
impl HtmlGenerator {
    pub fn new() -> Result<HtmlGenerator, String> {
        let handlebars = init_handlebars()?;
        Ok(HtmlGenerator {
            handlebars,
        })
    }

    /// Does the generate, but creates a new Handlebars instance to fetch any changed files.
    /// This is only when debugging.
    #[cfg(debug_assertions)]
    pub fn generate(&self, settings: &Settings, template_name: &str, mut data: Value)
    -> Result<String, RenderError> {
        HtmlGenerator::modify_data(settings, &mut data);
        init_handlebars().unwrap()
            .render(template_name, &data)
    }
    /// Generate the HTML file by looking up the template.
    /// Inserts some extra keys that will pretty much always be needed.
    #[cfg(not(debug_assertions))]
    pub fn generate(&self, settings: &Settings, template_name: &str, mut data: Value)
    -> Result<String, RenderError> {
        HtmlGenerator::modify_data(settings, &mut data);
        self.handlebars.render(template_name, &data)
    }

    /// Generates HTML from the template string.
    /// Remakes a new Handlebars when debugging.
    #[cfg(not(debug_assertions))]
    pub fn generate_from_string(&self, settings: &Settings, template_str: &str, mut data: Value)
    -> Result<String, RenderError> {
        HtmlGenerator::modify_data(settings, &mut data);
        init_handlebars().unwrap()
            .render_template(template_str, &data)
    }
    /// Generates HTML from the template string
    #[cfg(not(debug_assertions))]
    pub fn generate_from_string(&self, settings: &Settings, template_str: &str, mut data: Value)
    -> Result<String, RenderError> {
        HtmlGenerator::modify_data(settings, &mut data);
        self.handlebars.render_template(template_str, &data)
    }

    /// Modifies the data before it gets used for rendering
    fn modify_data(settings: &Settings, data: &mut Value) {
        let data_map = data.as_object_mut().unwrap();
        file_paths::modify_data_map(data_map);
        // Insert the language strings
        data_map.insert("lang_strings".to_string(), serde_json::to_value(
            settings.lang_ui()
        ).unwrap());
    }
}

/// Initializes a new Handlebars instance
fn init_handlebars() -> Result<Handlebars, String> {
    // Create the handlebars object we will use for generating the HTML
    let mut handlebars = Handlebars::new();
    // Fail on any missing values
    handlebars.set_strict_mode(true);
    // Disable the source map on Production
    if !cfg!(debug_assertions) {
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

    Ok(handlebars)
}

/// The file path constants to use when rendering HTML
mod file_paths {
    use serde_json::{Map, Value, json};

    #[cfg(debug_assertions)]
    fn vue() -> &'static str { "/js/vue-2.6.10.js" }
    #[cfg(not(debug_assertions))]
    fn vue() -> &'static str { "/js/vue-2.6.10.min.js" }

    #[cfg(debug_assertions)]
    fn bootstrap_grid() -> &'static str { "/css/bootstrap-grid-4.3.1.css" }
    #[cfg(not(debug_assertions))]
    fn bootstrap_grid() -> &'static str { "/css/bootstrap-grid-4.3.1.min.css" }

    /// Modifies the JSON value map to add the file paths
    pub fn modify_data_map(data_map: &mut Map<String, Value>) {
        // Insert the paths to our resource files
        data_map.insert("file_paths".to_string(), json!({
            "bootstrap_grid": bootstrap_grid(),
            "vue": vue(),
        }));
    }
}
