use std::fs;
use std::path::{Path};

use handlebars::{Handlebars, RenderError};

use serde_json::{self, Value};

use crate::lang::{UILang, UIStrings};

/// The directory for the HTML templates
const TEMPLATE_DIR: &'static str = "assets/htmlTemplates";
/// The directory for the handlebar partials
const PARTIAL_DIR: &'static str = "assets/htmlPartials";

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
    pub fn generate(&self, template_name: &str, mut data: Value)
    -> Result<String, RenderError> {
        HtmlGenerator::modify_data(&mut data);
        init_handlebars().unwrap()
            .render(template_name, &data)
    }
    /// Generate the HTML file by looking up the template.
    /// Inserts some extra keys that will pretty much always be needed.
    #[cfg(not(debug_assertions))]
    pub fn generate(&self, template_name: &str, mut data: Value)
    -> Result<String, RenderError> {
        HtmlGenerator::modify_data(&mut data);
        self.handlebars.render(template_name, &data)
    }

    /// Modifies the data before it gets used for rendering
    fn modify_data(data: &mut Value) {
        let data_map = data.as_object_mut().unwrap();
        file_paths::modify_data_map(data_map);

        // Insert the language strings
        data_map.insert("ui_strings".to_string(),
            serde_json::to_value(UIStrings::new()).unwrap());
        // Insert all of the languages that could be displayed
        data_map.insert("langs".to_string(),
            serde_json::to_value(UILang::all_short_str()).unwrap());
        // Inject the compiled version string
        data_map.insert("version".to_string(), Value::String(crate::VERSION.to_string()));
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

    // Register the helpers
    handlebars_helpers::register_helpers(&mut handlebars);

    Ok(handlebars)
}

/// The file path constants to use when rendering HTML
mod file_paths {
    use serde_json::{Map, Value, json};

    #[cfg(debug_assertions)]
    fn vue() -> &'static str { "/js/vue-2.6.10.js" }
    #[cfg(not(debug_assertions))]
    fn vue() -> &'static str { "/js/vue-2.6.10.min.js" }

    /// Modifies the JSON value map to add the file paths
    pub fn modify_data_map(data_map: &mut Map<String, Value>) {
        // Insert the paths to our resource files
        data_map.insert("file_paths".to_string(), json!({
            "vue": vue(),
        }));
    }
}

mod handlebars_helpers {
    //! A nice module to keep all of the helpers to use with Handlebars
    use handlebars::{Context, Handlebars, Helper, HelperResult, Output, RenderContext};

    /// Registers all of the helpers in the module onto the Handlebars object
    pub fn register_helpers(handlebars: &mut Handlebars) {
        handlebars.register_helper("camelCase", Box::new(camel_case));
    }

    /// Turns a snake_case string into a camelCase string
    fn camel_case(h: &Helper, _: &Handlebars, _: &Context, _rc: &mut RenderContext,
    out: &mut Output) -> HelperResult {
        // Get the parameter we want to camel case
        let param = h.param(0).and_then(|v| v.value().as_str()).unwrap_or("");
        // Split the thing at all of the underscores
        let mut split = param.split("_");
        // Push the entire first part in normally, but uppercase the first letter of all the next
        out.write(split.next().unwrap())?;

        split.for_each(|split_part| {
            if split_part.len() > 0 {
                // Uppercase the character right after
                // This assumes the first character will always just be a single byte
                out.write(&split_part[0..1].to_uppercase());
                // Give it the rest of the string
                out.write(&split_part[1..]);
            }
        });

        Ok(())
    }
}
