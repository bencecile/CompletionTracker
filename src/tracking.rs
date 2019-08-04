use std::collections::{BTreeMap};
use std::fs::{DirBuilder};
use std::path::{Path, PathBuf};

use serde_derive::{Deserialize, Serialize};

use crate::types::{Date};
use crate::utils;

/// The tracking folder for the tracking JSON files
const TRACKING_FOLDER: &'static str = "tracking";
/// The default tracking JSON file in the tracking folder
const DEFAULT_TRACKING_FILE: &'static str = "tracking.json";

/// Keeps track of information that needs to be tracked
/// Each one is completely separate from any other
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
#[derive(Deserialize, Serialize)]
pub struct TrackerData {
    /// This is the tracking information for the sources using their ID as the key
    sources: BTreeMap<u64, Vec<TrackerEntry>>,
    /// This is tracking for other things that don't have a source yet.
    other: BTreeMap<String, Vec<TrackerEntry>>,
}
impl TrackerData {
    /// Tries to create new tracker data from the file
    /// If there is no file given, it will start off with empty data
    fn new(file: Option< impl AsRef<Path> >) -> Result<TrackerData, String> {
        if let Some(file) = file {
            // Read the JSON since we actually have a file here
            utils::read_json_file(file)
        } else {
            // Create new, empty data
            Ok(TrackerData {
                sources: BTreeMap::new(),
                other: BTreeMap::new(),
            })
        }
    }
}
#[derive(Clone, Deserialize, Serialize)]
pub struct TrackerEntry {
    /// All of the status updates for the entry.
    /// A status may only have an update once at most. Although the time status for it may change.
    status_updates: BTreeMap<CompletionStatus, Option<TrackerTimeStatus>>,
    /// A description of the completion.
    /// A common usage would be to fill it in with in the platform for a game.
    description: String,
    /// The current time status for the entry.
    /// It might be useful to keep track of the time status of the entry without needing a
    /// new status update.
    current_time: Option<TrackerTimeStatus>,
    /// The date when this entry was last updated
    last_updated: Date,
}

/// A Date and time in seconds.
/// The Date is required, but the time might not always be known so it's optional.
#[derive(Copy, Clone, Deserialize, Serialize)]
pub struct TrackerTimeStatus {
    /// The date in year, months and day
    date: Date,
    /// The completion time of the entry in seconds.
    /// This should only be set once the status is set to a variation of "complete".
    /// This generally only makes sense for games, but you could use it for anything.
    /// The rule for this would be to use "play time" instead of "real time".
    time: Option<u64>,
}

/// This is the status of the source we're tracking.
/// We don't need a "not started" status since that's just all the other sources
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Deserialize, Serialize)]
pub enum CompletionStatus {
    /// The source has been started reading, playing, watching, etc.
    InProgress,
    /// The source has been done to the end.
    /// Read to the end of the book, watched until the credits, etc.
    Complete,
    /// Games only. Not only is the game finished, it has been completed as much as possible.
    /// If the game has achievements then this is when all of them have been unlocked.
    /// If there are no achievements then it would be up to whatever the player deems
    /// "most complete".
    GameComplete,
}
