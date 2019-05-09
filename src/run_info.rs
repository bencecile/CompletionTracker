use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4};
use std::path::{Path};

use serde_derive::{Deserialize};

use crate::tracking::{Tracker};
use crate::utils;

/// Holds any info needed when running the server
#[derive(Debug, Deserialize)]
pub struct RunInfo {
    /// The port to run the server on
    port: u16,
    /// The trackers to use
    trackers: Vec<TrackingInfo>,
}
impl RunInfo {
    /// Reads in the file to create the run info
    pub fn new(info_path: impl AsRef<Path>) -> Result<RunInfo, String> {
        utils::read_json_file(info_path)
    }

    // Gets the socket address to run on
    pub fn socket_addr(&self) -> SocketAddr {
        // Set up shop on every available address (for non-local connections)
        SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::UNSPECIFIED, self.port))
    }

    /// The list of trackers specified in the RunInfo
    /// Returns 1 item for each String in the "tracking_files" list
    pub fn trackers(&self) -> Result<Vec<Tracker>, String> {
        // We will need to create the default tracker if none are given
        Ok(if self.trackers.is_empty() {
            vec![Tracker::new(None)?]
        } else {
            // Create the new trackers
            let mut trackers = Vec::with_capacity(self.trackers.capacity());
            // Don't let them use the default tracking file more than once
            let mut used_default = false;
            for tracker in self.trackers.iter() {
                // Check for an empty tracking file and use the default tracking file
                let tracking_file = match tracker.tracking_file {
                    Some(ref tracking_file) => {
                        if tracking_file.is_empty() {
                            if used_default {
                                // Bail out here
                                return Err(
                                    "The default tracking file cannot be used twice".to_string()
                                );
                            }

                            used_default = true;
                            None
                        } else {
                            Some(tracking_file.as_str())
                        }
                    },
                    None => {
                        if used_default {
                            // Bail out here
                            return Err(
                                "The default tracking file cannot be used twice".to_string()
                            );
                        }

                        used_default = true;
                        None
                    }
                };
                // Since we need to possibly return with an error, we can't do this
                //  with an Iter.map()
                trackers.push(Tracker::new(tracking_file)?);
            }
            
            trackers
        })
    }
}

/// Info for each tracker
#[derive(Debug, Deserialize)]
struct TrackingInfo {
    /// The tracking file to write to
    tracking_file: Option<String>,
    /// The Playstation Network ID to use
    psn_id: Option<String>,
    /// The Xbox ID (gamertag)
    xbox_id: Option<String>,
    /// The Steam user ID
    steam_id: Option<String>,
}
