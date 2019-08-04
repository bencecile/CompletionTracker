mod api;
mod automate;
mod lang;
mod logging;
mod run_info;
mod source;
mod source_concrete;
mod tracking;
mod types;
mod utils;

use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::time::{Duration};

use rouille::{Server};

use crate::api::{Api};
use crate::logging::{MyLogger};
use crate::run_info::{RunInfo};

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

// This is the signal to exit the program
static SHUTDOWN_SIGNAL: AtomicBool = AtomicBool::new(false);
// The time to wait before polling the connection
const POLL_WAIT_TIME: Duration = Duration::from_millis(20);

fn main() -> Result<(), String> {
    // Init the logging very first
    MyLogger::init();

    // Read in the run info to create our things
    let run_info = RunInfo::new_default()?;

    log::info!("Creating the API struct");
    // Create the API that will actually handle the calls
    let api = Api::new(run_info.trackers()?)?;

    // Start up the server
    let server = Server::new(run_info.socket_addr(), move |req| api.handle_request(req))
        .expect("Failed to create the server");
    // Set the fixed pool size to something small since this is only running locally
    let server = server.pool_size(10);

    // Print out where the server is running and how you can get to it
    log::info!("Running on {}", server.server_addr());

    while !SHUTDOWN_SIGNAL.load(Ordering::Relaxed) {
        // Check for any connections
        server.poll();
        // Wait until we check again
        thread::sleep(POLL_WAIT_TIME);
    }

    Ok(())
}
