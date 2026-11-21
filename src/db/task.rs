use rusqlite::{
    Connection,
    Result,
};

use crate::models::task::Task;
use super::status::get_status_by_id;
use super::user::get_user_by_id;

pub fn get_task_by_id(conn: &Connection, id: i32) -> Result<Task> {
    let task = conn.query_row(
        "SELECT * FROM task WHERE id = ?1",
        [id],
        |row| {
            Ok(Task{
                id: row.get(0)?,
                name: row.get(1)?,
                content: row.get(2)?,
                status: get_status_by_id(conn, row.get(3)?),
                creator: get_user_by_id(conn, row.get(4)?).unwrap(),
                deadline: row.get(5)?
            })
        }
    );
    task
}