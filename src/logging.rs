use std::fs::{File, OpenOptions};
use std::io::{self, Stdout};
use std::io::prelude::*;
use std::sync::{Arc, Mutex};

use log::{self, Log, Level, LevelFilter, Metadata, Record};

use time;

/// The logging file. Create a directory and make it change by day or size if they get too big.
const LOG_FILE: &'static str = "logs.log";

/// The basic logger that can be used for the entire application
pub struct MyLogger {
    log_file: Arc<Mutex<File>>,
    /// We may not have a stdout if it's in release
    stdout: Option<Stdout>,
}
impl MyLogger {
    /// Initialize the logger
    pub fn init() {
        // We will always want to use the log file
        let log_file = OpenOptions::new()
            .append(true)
            .create(true)
            .open(LOG_FILE)
            .expect("Failed to open the log file");
        let stdout = if cfg!(debug_assertions) {
            // Also go to Stdout if we're debugging
            Some(io::stdout())
        } else {
            None
        };

        let my_logger = MyLogger {
            log_file: Arc::new(Mutex::new(log_file)),
            stdout,
        };
        // Only use the log file on release
        log::set_boxed_logger(Box::new(my_logger))
            .expect("Failed to set the logger");

        // Set the log level based on release status
        if cfg!(debug_assertions) {
            log::set_max_level(LevelFilter::Trace);
        } else {
            log::set_max_level(LevelFilter::Info);
        }
    }
}
impl Log for MyLogger {
    fn enabled(&self, _metadata: &Metadata) -> bool { true }
    fn log(&self, record: &Record) {
        // Ignore some targets that we don't care about logging
        if record.target().starts_with("handlebars") {
            return;
        }

        // Get a string for the current date and time
        let now = time::now();
        let date_time = now.strftime("%Y-%m-%d %H:%M:%S")
            .expect("Failed to strftime for logging");
        // Get the milliseconds for the time
        let millis = now.tm_nsec / 1_000_000;

        let target = record.target();
        // Get the log level as a short form string
        let level = match record.level() {
            Level::Error => "E",
            Level::Warn => "W",
            Level::Info => "I",
            Level::Debug => "D",
            Level::Trace => "T",
        };
        let args = record.args();

        // Create the log line
        let write_log = |writer: &mut dyn Write| {
            writeln!(writer, "[{}.{:03}] |{}| <{}> {}", date_time, millis, level, target, args)
                .expect("Failed to write to a log file");
        };

        // Write to the log file first
        let mut log_lock = self.log_file.lock().unwrap();
        // Deref the lock so that it can access the File inside
        write_log(&mut *log_lock);

        // Only log to stdout if we can
        if let Some(ref stdout) = self.stdout {
            let mut stdout_lock = stdout.lock();
            write_log(&mut stdout_lock);
        }
    }
    fn flush(&self) {}
}
