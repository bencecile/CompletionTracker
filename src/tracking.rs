use std::fs::{DirBuilder};
use std::path::{Path, PathBuf};

use serde_derive::{Deserialize, Serialize};

use crate::utils;

/// The tracking folder for the tracking JSON files
const TRACKING_FOLDER: &'static str = "tracking";
/// The default tracking JSON file in the tracking folder
const DEFAULT_TRACKING_FILE: &'static str = "tracking.json";

/// Keeps track of information that needs to be tracked
/// Each one is completely separate from any other
#[derive(Debug)]
pub struct Tracker {
    tracking_file: PathBuf,
    tracker_data: TrackerData,
}
impl Tracker {
    /// Creates a new tracker that is attached to the given file
    /// Using the given "file" we will search for it ourselves inside the tracking folder
    /// If we get a None, the default tracking file will be used
    pub fn new(file: Option<&str>) -> Result<Tracker, String> {
        // Check for the existence of the tracking folder
        let tracking_folder = PathBuf::from(TRACKING_FOLDER);
        // Create it if necessary
        if !tracking_folder.is_dir() {
            DirBuilder::new().create(&tracking_folder)
                .map_err(|e| e.to_string())?;
        }

        // Create the actual tracking file
        let tracking_file = tracking_folder.join(
            if let Some(file) = file {
                file
            } else {
                DEFAULT_TRACKING_FILE
            }
        );
        // Check for tracking file correctness
        if tracking_file.parent().unwrap() != tracking_folder {
            // Make sure that we only specified a single file (one level inside the parent)
            return Err(format!("'{:?}' must be a single file", tracking_file.file_name()));
        }

        // Create the tracker data based on the existence of the file
        let tracker_data = TrackerData::new(
            if tracking_file.exists() {
                Some(&tracking_file)
            } else {
                None
            }
        )?;

        Ok(Tracker {
            tracking_file,
            tracker_data,
        })
    }

    /// Updates the data within the tracker
    /// Saves the newly updated data to file
    pub fn update_data<F>(&mut self, f: F) -> Result<(), String>
    where F: FnOnce(&mut TrackerData) {
        // Update the data
        f(&mut self.tracker_data);
        // Write the new data to our tracking file
        utils::write_json_file(&self.tracker_data, &self.tracking_file)
    }
}

/// All of the data that a tracker will carry around
/// This is what will be written into the tracking file
#[derive(Debug, Deserialize, Serialize)]
pub struct TrackerData {
}
impl TrackerData {
    /// Tries to create new tracker data from the file
    /// If there is no file given, it will start off with empty data
    fn new(file: Option< impl AsRef<Path> >) -> Result<TrackerData, String> {
        if let Some(file) = file {
            // Read the JSON since we actually have a file here
            utils::read_json_file(file)
        } else {
            // Create some new data
            Ok(TrackerData {
            })
        }
    }
}
