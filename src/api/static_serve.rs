use std::path::{Path, PathBuf};

/// The root folder to start serving
const STATIC_ROOT: &'static str = "assets";

pub use self::static_server::{StaticServer};

#[cfg(debug_assertions)]
mod static_server {
    use std::fs::{File};

    use rouille::{Response};

    use super::{StaticFile};
    use super::vue;

    /// Give a special debug implementation of the static server to make it cache-less
    pub struct StaticServer;
    impl StaticServer {
        /// This is just to match the release implementation
        pub fn new() -> StaticServer { StaticServer }

        pub fn serve_static(&self, static_file: StaticFile) -> Response {
            // Get the file that we need to serve
            let file_to_serve = static_file.full_path();
            // Check for the existence of the file first
            if !file_to_serve.exists() {
                Response::empty_404()
            } else {
                // Make the response
                Response::from_file(
                    static_file.mime_type(),
                    rouille::try_or_400!(File::open(file_to_serve))
                )
            }
        }
        
        pub fn serve_vue_components(&self)
        -> Response {
            Response::from_data(
                StaticFile::default_js().mime_type(),
                vue::bundle_components()
            )
        }
    }
}

#[cfg(not(debug_assertions))]
mod static_server {
    use std::collections::{BTreeMap};
    use std::path::{PathBuf};

    use rouille::{Response};

    use super::{StaticFile};
    use super::vue;
    use crate::utils;

    /// Caches the files that get served, to make it even faster (in production).
    /// In development, it reads the disk and directories every single request.
    pub struct StaticServer {
        /// The cache we will use when in production
        cache: BTreeMap<PathBuf, Vec<u8>>,
        /// Just the blob of data for the Vue components
        vue_components: Vec<u8>,
    }
    impl StaticServer {
        pub fn new() -> StaticServer {
            let mut cache = BTreeMap::new();
            // Pre-cache everything when in production
            [
                StaticFile::default_css(),
                StaticFile::default_img(),
                StaticFile::default_js(),
            ].iter()
                .flat_map(|static_file| utils::read_dir(static_file.dir(), false))
                .for_each(|file_path| {
                    let content = fs::read(&file_path)
                        .expect("Failed to read the file when pre-caching");
                    cache.insert(file_path, content);
                });

            // Read in the Vue components
            let vue_components = vue::bundle_components();

            StaticServer {
                cache,
                vue_components,
            }
        }

        /// Check the cache to serve file
        pub fn serve_static(&self, static_file: StaticFile) -> Response {
            if let Some(file_content) = self.cache.get(&static_file.full_path()) {
                // Send the found file content
                Response::from_data(
                    static_file.mime_type(),
                    file_content.clone()
                )
            } else {
                // Send a 404 since it doesn't exist in the map
                Response::empty_404()
            }
        }
        
        /// Serves all of the Vue components as a single file.
        /// This will come from the cache since this is Production.
        pub fn serve_vue_components(&self)
        -> Response {
            Response::from_data(
                StaticFile::default_js().mime_type(),
                self.vue_components.clone()
            )
        }
    }
}


/// Describes the different static files that we can serve
pub enum StaticFile {
    Css(String),
    Img(String),
    Js(String),
}
impl StaticFile {
    // Define some easy access functions for the basic variants
    fn default_css() -> StaticFile { StaticFile::Css(String::new()) }
    fn default_img() -> StaticFile { StaticFile::Img(String::new()) }
    fn default_js() -> StaticFile { StaticFile::Js(String::new()) }

    /// Gets the directory for this type of file
    fn dir(&self) -> PathBuf {
        let type_dir = match self {
            StaticFile::Css(_) => "css",
            StaticFile::Img(_) => "img",
            StaticFile::Js(_) => "js",
        };
        Path::new(STATIC_ROOT).join(type_dir)
    }
    /// Gets the path to the file described
    fn full_path(&self) -> PathBuf {
        let file_name = match self {
            StaticFile::Css(file_name) |
            StaticFile::Img(file_name) |
            StaticFile::Js(file_name) => file_name,
        };
        self.dir().join(file_name)
    }
    /// Gets the MIME type for this static file
    fn mime_type(&self) -> &'static str {
        match self {
            StaticFile::Css(_) => "text/css",
            StaticFile::Img(file_name) => {
                // Do a simple match to try to find the type
                let extension = file_name.rsplitn(2, ".").next()
                    .expect(&format!("An image '{}' doesn't have an extension", &file_name));
                match extension {
                    "jpeg" | "jpg" => "image/jpeg",
                    "png" => "image/png",
                    _ => panic!("Failed to recognize an image extension '{}', '{}'",
                        &file_name, extension),
                }
            },
            StaticFile::Js(_) => "text/javascript",
        }
    }
}

/// This module will handle the building of Vue files and making them into something useable
mod vue {
    use std::fs;
    use std::io::{Write};

    use crate::utils;

    const COMPONENT_DIR: &'static str = "assets/htmlComponents";

    // Some constants for the tags that we will be finding
    const TEMPLATE_TAG: &'static str = "<template>";
    const TEMPLATE_TAG_END: &'static str = "</template>";
    const SCRIPT_TAG: &'static str = "<script>";
    const SCRIPT_TAG_END: &'static str = "</script>";
    const EXPORT_START: &'static str = "export default {";
    const EXPORT_END: &'static str = "}";
    const STYLE_TAG: &'static str = "<style>";
    const STYLE_TAG_END: &'static str = "</style>";


    /// Finds the content within the tag in the search data
    fn find_tag_content<'s>(search_data: &'s str, tag: &str, tag_end: &str) -> &'s str {
        let start_index = search_data.find(tag)
            .expect(&format!("Failed to find the '{}' tag", tag))
            // Go to the index just after the end of the tag
            + tag.len();
        // Look from the end to find the matching end tag
        let end_index = search_data.rfind(tag_end)
            .expect(&format!("Failed to find the '{}' end tag", tag_end));
        // Return the slice of the data that will go right in-between the tags
        // Also trim it so we lose any unneeded whitespace
        search_data[start_index..end_index].trim()
    }

    /// Bundles all of the Vue components together into a single file
    pub fn bundle_components() -> Vec<u8> {
        let mut bundled_components = Vec::new();

        // Go through each file, appending them to the data
        for file_path in utils::read_dir(COMPONENT_DIR, true) {
            let component_data = fs::read_to_string(&file_path)
                .expect("Failed to read a component");

            let template_data = find_tag_content(&component_data, TEMPLATE_TAG, TEMPLATE_TAG_END);
            let script_data = find_tag_content(&component_data, SCRIPT_TAG, SCRIPT_TAG_END);
            // Modify the script data to not include some stuff
            let script_data = find_tag_content(&script_data, EXPORT_START, EXPORT_END);
            let style_data = find_tag_content(&component_data, STYLE_TAG, STYLE_TAG_END);

            // Only include the template entry if we get something
            let template_entry = if template_data.is_empty() {
                String::new()
            } else {
                format!("template: `{}`,", template_data)
            };
            // Only add the function for the style if there is some style to use
            let style_function = if style_data.is_empty() {
                String::new()
            } else {
                format!(r#"
(function() {{
    const style = document.createElement("style");
    style.innerHTML = `{}`;
    document.head.appendChild(style);
}})();"#, style_data)
            };

            // Write a slightly elaborate JS string into our component data
            write!(bundled_components, r#"
Vue.component("{file_name}", {{
    {template_entry}
    {script_data}
}});{style_function}"#, file_name=file_path.file_stem().unwrap().to_str().unwrap(),
            template_entry=template_entry,
            script_data=script_data,
            style_function=style_function)
                .expect("Failed to write the component data");
        }

        bundled_components
    }
}
