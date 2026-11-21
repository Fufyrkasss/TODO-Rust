pub mod init;
pub mod role;
pub mod user;
pub mod task;
pub mod status;

use rusqlite::{Connection, Result};

pub fn connect() -> Result<Connection> {
    Connection::open("todo.db")
}