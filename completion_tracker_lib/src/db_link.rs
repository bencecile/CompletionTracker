use std::sync::{Mutex, MutexGuard};

use rusqlite::{Connection};

pub struct ConnectionHolder {
    connection: Mutex<Connection>,
}
impl ConnectionHolder {
    pub fn new(connection: Connection) -> ConnectionHolder {
        ConnectionHolder {
            connection: Mutex::new(connection),
        }
    }

    pub fn lock(&self) -> MutexGuard<Connection> { self.connection.lock().unwrap() }
}
