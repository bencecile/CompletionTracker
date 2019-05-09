use std::fs;
use std::path::{Path};

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

/// Escapes the spaces from a string, giving one usable for URLs
pub fn escape_spaces(string: &str) -> String {
    string.replace(" ", "_")
}
