use std::{
    fs::{DirBuilder},
    path::{Path, PathBuf},
};

use serde::{Deserialize};

const DEFAULT_TRACKER_NAME: &'static str = "Default Tracker";

#[derive(Deserialize)]
pub struct TrackingInfo {
    tracker_name: Option<String>,
    game_id: GameID,
}

#[derive(Clone, Deserialize)]
pub struct GameID {
    /// The Playstation Network ID to use
    psn_id: Option<String>,
    /// The Xbox ID (gamertag)
    xbox_id: Option<String>,
    /// The Steam user ID
    steam_id: Option<String>,
}
impl GameID {
    fn empty() -> GameID {
        GameID {
            psn_id: None,
            xbox_id: None,
            steam_id: None,
        }
    }
}

/// Each one is completely separate from any other
pub struct Tracker {
    name: String,
    file: PathBuf,
    game_id: GameID,
}
impl Tracker {
    pub fn new_from_info(tracking_folder: impl AsRef<Path>, tracking_infos: &[TrackingInfo])
    -> Result<Vec<Tracker>, String> {
        let tracking_folder = tracking_folder.as_ref();
        if !tracking_folder.is_dir() {
            DirBuilder::new().create(&tracking_folder)
                .map_err(|e| e.to_string())?;
        }

        let mut trackers = Vec::with_capacity(tracking_infos.len());
        if tracking_infos.is_empty() {
            trackers.push(Tracker::new(
                tracking_folder,
                DEFAULT_TRACKER_NAME.to_string(),
                GameID::empty(),
            )?);
        } else {
            for tracking_info in tracking_infos.iter() {
                let name = if let Some(ref tracker_name) = tracking_info.tracker_name {
                    tracker_name.clone()
                } else {
                    DEFAULT_TRACKER_NAME.to_string()
                };

                // Don't let them use the any tracking name more than once
                for other_tracker in trackers.iter() {
                    if other_tracker.name() == name {
                        return Err(
                            format!("Cannot have duplicate tracker names: {}", name)
                        );
                    }
                }

                trackers.push(Tracker::new(
                    tracking_folder.clone(),
                    name,
                    tracking_info.game_id.clone(),
                )?);
            }
        }

        Ok(trackers)
    }

    fn new(tracking_folder: impl AsRef<Path>, name: String, game_id: GameID)
    -> Result<Tracker, String> {
        if name.is_empty() {
            return Err("The tracker name must not be empty".to_string());
        }
        if name.chars().any(|c|
            c.is_ascii_punctuation() ||
            c.is_ascii_control()
        ) {
            return Err(
                "There cannot be any punctuation or control characters in the tracker name".to_string()
            );
        }

        let mut file = tracking_folder.as_ref().join(&name);
        file.set_extension("db");

        Ok(Tracker {
            name,
            file,
            game_id,
        })
    }

    pub fn name(&self) -> &str { self.name.as_str() }
}

// #[derive(Clone)]
// pub struct TrackerEntry {
//     /// All of the status updates for the entry.
//     /// A status may only have an update once at most. Although the time status for it may change.
//     status_updates: BTreeMap<CompletionStatus, Option<TrackerTimeStatus>>,
//     /// A description of the completion.
//     /// A common usage would be to fill it in with in the platform for a game.
//     description: String,
//     /// The current time status for the entry.
//     /// It might be useful to keep track of the time status of the entry without needing a
//     /// new status update.
//     current_time: Option<TrackerTimeStatus>,
//     /// The date when this entry was last updated
//     last_updated: Date,
// }

// /// A Date and time in seconds.
// /// The Date is required, but the time might not always be known so it's optional.
// #[derive(Copy, Clone)]
// pub struct TrackerTimeStatus {
//     /// The date in year, months and day
//     date: Date,
//     /// The completion time of the entry in seconds.
//     /// This should only be set once the status is set to a variation of "complete".
//     /// This generally only makes sense for games, but you could use it for anything.
//     /// The rule for this would be to use "play time" instead of "real time".
//     time: Option<u64>,
// }

// /// This is the status of the source we're tracking.
// /// We don't need a "not started" status since that's just all the other sources
// #[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
// pub enum CompletionStatus {
//     /// The source has been started reading, playing, watching, etc.
//     InProgress,
//     /// The source has been done to the end.
//     /// Read to the end of the book, watched until the credits, etc.
//     Complete,
//     /// Games only. Not only is the game finished, it has been completed as much as possible.
//     /// If the game has achievements then this is when all of them have been unlocked.
//     /// If there are no achievements then it would be up to whatever the player deems
//     /// "most complete".
//     GameComplete,
// }
