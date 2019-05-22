use std::collections::{BTreeMap};
use std::fs::{self, File};
use std::path::{Path, PathBuf};

use rouille::{Response};

use crate::utils;
use super::{Api};

/// The root folder to start serving
const STATIC_ROOT: &'static str = "htmlStatic";

/// Implement the static serve methods
impl Api {
    pub fn serve_static(&self, static_file: StaticFile) -> Response {
        self.static_server.serve_static(static_file)
    }
    pub fn serve_vue_components(&self) -> Response {
        self.static_server.serve_vue_components()
    }
}

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
        let mut vue_components = Vec::new();
        if !cfg!(debug_assertions) {
            // Pre-cache everything when in production
            [StaticFile::default_css(), StaticFile::default_js()].iter()
                .flat_map(|static_file| utils::read_dir(static_file.dir(), false))
                .for_each(|file_path| {
                    let content = fs::read(&file_path)
                        .expect("Failed to read the file when pre-caching");
                    cache.insert(file_path, content);
                });

            // Read in the Vue components
            vue_components = vue::bundle_components();
        }

        StaticServer {
            cache,
            vue_components,
        }
    }

    /// Serve directly from disk when debugging
    #[cfg(debug_assertions)]
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
    /// Check the cache when on Production
    #[cfg(not(debug_assertions))]
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

    /// Serves all of the Vue components, bundled as a single file.
    /// This will happen every call when debugging.
    #[cfg(debug_assertions)]
    pub fn serve_vue_components(&self)
    -> Response {
        Response::from_data(
            StaticFile::default_js().mime_type(),
            vue::bundle_components()
        )
    }
    /// Serves all of the Vue components as a single file.
    /// This will come from the cache since this is Production.
    #[cfg(not(debug_assertions))]
    pub fn serve_vue_components(&self)
    -> Response {
        Response::from_data(
            StaticFile::Js("").mime_type(),
            self.vue_components.clone()
        )
    }
}

/// Describes the different static files that we can serve
pub enum StaticFile {
    Css(String),
    Js(String),
    Svg(String),
}
impl StaticFile {
    // Define some easy access functions for the basic variants
    fn default_css() -> StaticFile { StaticFile::Css(String::new()) }
    fn default_js() -> StaticFile { StaticFile::Js(String::new()) }

    /// Gets the directory for this type of file
    fn dir(&self) -> PathBuf {
        let type_dir = match self {
            StaticFile::Css(_) => "css",
            StaticFile::Js(_) => "js",
            StaticFile::Svg(_) => "svg",
        };
        Path::new(STATIC_ROOT).join(type_dir)
    }
    /// Gets the path to the file described
    fn full_path(&self) -> PathBuf {
        let file_name = match self {
            StaticFile::Css(file_name) |
            StaticFile::Js(file_name) |
            StaticFile::Svg(file_name) => file_name,
        };
        self.dir().join(file_name)
    }
    /// Gets the MIME type for this static file
    fn mime_type(&self) -> &'static str {
        match self {
            StaticFile::Css(_) => "text/css",
            StaticFile::Js(_) => "text/javascript",
            StaticFile::Svg(_) => "image/svg+xml",
        }
    }
}

/// This module will handle the building of Vue files and making them into something useable
mod vue {
    use std::fs;
    use std::io::{Write};

    use lazy_static::{lazy_static};

    use regex::{Regex, RegexBuilder};

    use crate::utils;

    const COMPONENT_DIR: &'static str = "htmlComponents";

    // Only compile our expressions once
    lazy_static! {
        static ref TEMPLATE: Regex = build_regex(r"<template>(.*?)</template>");
        static ref SCRIPT: Regex = build_regex(r"<script>.*?export default ?\{(.*?)\}[^\}]*?</script>");
        static ref STYLE: Regex = build_regex(r"<style>(.*?)</style>");
    }
    /// Bundles all of the Vue components together into a single file
    pub fn bundle_components() -> Vec<u8> {
        let mut bundled_components = Vec::new();

        // Go through each file, appending them to the data
        for file_path in utils::read_dir(COMPONENT_DIR, true) {
            let component_data = fs::read_to_string(&file_path)
                .expect("Failed to read a component");
            let template_data = TEMPLATE.captures(&component_data)
                .expect("Failed to get the template from a component")
                .get(1).unwrap();
            let script_data = SCRIPT.captures(&component_data)
                .expect("Failed to get the script data from a component")
                .get(1).unwrap();
            let style_data = STYLE.captures(&component_data)
                .expect("Failed to get the style data from a component")
                .get(1).unwrap();

            // Write a slightly elaborate JS string into our component data
            write!(bundled_components, r#"
Vue.component("{file_name}", {{
    {script_data}
    template: `{template_data}`,
}});
(function() {{
    const style = document.createElement("style");
    style.innerHTML = `{style_data}`;
    document.head.appendChild(style);
}})();"#, file_name=file_path.file_stem().unwrap().to_str().unwrap(),
            template_data=template_data.as_str().trim(),
            script_data=script_data.as_str().trim(),
            style_data=style_data.as_str().trim())
                .unwrap();
        }

        bundled_components
    }

    /// Builds a Regex from the given pattern
    fn build_regex(pattern: &str) -> Regex {
        RegexBuilder::new(pattern)
            .dot_matches_new_line(true)
            .build()
            .expect("Failed to compile the template pattern")
    }
}
