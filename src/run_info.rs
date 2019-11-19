use std::{
    net::{Ipv4Addr, SocketAddr, SocketAddrV4},
    path::{Path, PathBuf},
};
use serde::{Deserialize};

use completion_tracker_lib::{
    tracking::{Tracker, TrackingInfo},
    utils,
};

const RUN_INFO_FILE: &'static str = "runInfo.json";

#[derive(Deserialize)]
pub struct RunInfo {
    port: u16,
    local_tracking_folder: PathBuf,
    trackers: Vec<TrackingInfo>,
}
impl RunInfo {
    pub fn new_default() -> Result<RunInfo, String> { Self::new(RUN_INFO_FILE) }
    pub fn new(info_path: impl AsRef<Path>) -> Result<RunInfo, String> {
        utils::read_json_file(info_path)
    }

    pub fn socket_addr(&self) -> SocketAddr {
        // Set up shop on every available address (for non-local connections)
        SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::UNSPECIFIED, self.port))
    }

    pub fn trackers(&self) -> Result<Vec<Tracker>, String> {
        Tracker::new_from_info(&self.local_tracking_folder, &self.trackers)
    }
}
