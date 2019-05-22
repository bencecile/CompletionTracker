use std::fs;
use std::path::{Path, PathBuf};

use serde::{Serialize};
use serde::de::{DeserializeOwned};
use serde_json;

/// Reads the json type from the given file
pub fn read_json_file<T>(file: impl AsRef<Path>) -> Result<T, String>
where T: DeserializeOwned {
    serde_json::from_str(
        &fs::read_to_string(file).map_err(|e| e.to_string())?
    ).map_err(|e| e.to_string())
}
/// Writes the json type into the file
pub fn write_json_file<T: Serialize>(data: &T, file: impl AsRef<Path>) -> Result<(), String> {
    fs::write(file,
        serde_json::to_string_pretty(data)
            .map_err(|e| e.to_string())?
    ).map_err(|e| e.to_string())
}

/// Reads a JSON list into a vector from the file
pub fn read_json_list<T>(file: impl AsRef<Path>) -> Result<Vec<T>, String>
where T: DeserializeOwned {
    read_json_file(file)
}
/// Writes a JSON list from a vector into the file
pub fn write_json_list<T: Serialize>(data: &Vec<T>, file: impl AsRef<Path>) -> Result<(), String> {
    write_json_file(data, file)
}

/// Gets all of the *files* from the directory.
/// If recursive, it will read everything from all the sub-directories
/// Panics if it's not a directory.
pub fn read_dir(dir: impl AsRef<Path>, recursive: bool) -> impl Iterator<Item = PathBuf> {
    fs::read_dir(dir).expect("Tried reading something that's not a directory")
        .filter_map(move |dir_entry| {
            // Get rid of everything that isn't a file (or dir if recursive)
            let dir_entry = dir_entry.expect("Failed to read a dir entry");
            let file_type = dir_entry.file_type().expect("Failed to read the file type");
            if file_type.is_file() || (recursive && file_type.is_dir()) {
                Some((dir_entry.path(), file_type))
            } else {
                None
            }
        }).flat_map(move |(file_path, file_type)| {
            // TODO This may be able to be cleaned up
            if file_type.is_file() {
                // Just use this single file
                vec![file_path]
            } else {
                // Read this directory now too
                // If it's a directory, we know that we are recursive since that's the
                //  only way we can get a directory here
                read_dir(file_path, recursive).collect::<Vec<PathBuf>>()
            }
        })
}
