//! These are the various concrete things to do with sources

mod source_db;
pub mod source_image;

pub use self::source_db::{SourceDB};

use std::fs;
use std::path::{Path, PathBuf};

use crate::source::{SourceItem};

/// This is the folder for all of the source information
const SOURCE_FOLDER: &'static str = "sources";
pub fn create_source_folder() -> Result<(), String> {
    let folder = Path::new(SOURCE_FOLDER);
    // Only try to create the folder if it isn't here
    if !folder.is_dir() {
        fs::create_dir(folder).map_err(|e| e.to_string())
    } else {
        Ok(())
    }
}
/// Creates a path to the source file inside the source folder
pub fn source_file_path(file_name: &str) -> PathBuf {
    Path::new(SOURCE_FOLDER).join(file_name)
}


/// Gets the path to the related list file for this item
pub fn list_file(source_item: SourceItem) -> PathBuf {
    match source_item {
        SourceItem::Character => source_file_path("characters.json"),
        SourceItem::Company => source_file_path("companies.json"),
        SourceItem::Person => source_file_path("people.json"),
        SourceItem::Source => source_file_path("sources.json"),
        SourceItem::UniverseTag => source_file_path("universe_tags.json"),
    }
}
