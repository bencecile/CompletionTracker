use std::fs::{File};
use std::path::{PathBuf};

use rouille::{Response};

/// The root folder to start serving
const STATIC_ROOT: &'static str = "staticServe";

pub fn static_serve(static_file: StaticFile) -> Response {
    // Get the file that we need to serve
    let file_to_serve = static_file.full_path();
    // Check for the existence of the file first
    if !file_to_serve.exists() {
        Response::empty_404()
    } else {
        // Make the response
        Response::from_file(
            rouille::extension_to_mime(file_to_serve.extension().unwrap().to_str().unwrap()),
            rouille::try_or_400!(File::open(file_to_serve))
        )
    }
}

/// Describes the different static files that we can serve
pub enum StaticFile {
    Css(String),
    Js(String),
}
impl StaticFile {
    // Gets the path to the file described
    fn full_path(&self) -> PathBuf {
        let prefix = match self {
            StaticFile::Css(_) => "css",
            StaticFile::Js(_) => "js",
        };
        let file_name = match self {
            StaticFile::Css(file_name) |
            StaticFile::Js(file_name) => file_name,
        };
        [STATIC_ROOT, prefix, file_name].iter().collect()
    }
}
