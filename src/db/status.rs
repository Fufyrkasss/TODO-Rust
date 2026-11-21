use rusqlite::{
    Connection,
};
use crate::models::task::Status;

pub fn get_status_by_id(conn: &Connection, id: i32) -> Status {
    let status_name: String = conn.query_row(
            "SELECT name FROM status WHERE id = ?1",
            [id],
            |row| row.get(0),
        ).unwrap();
    let status = match status_name.as_str() {
        "done" => Status::Done,
        "undone" => Status::UnDone,
        "in progress" => Status::InProgress,
        _ => Status::None,
    };
    status
}