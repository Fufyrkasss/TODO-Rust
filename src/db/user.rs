use rusqlite::{
    Connection,
    Result,
};
use crate::models::user::User;
use super::role::get_role_by_id;

pub fn get_user_by_id(conn: &Connection, id: i32) -> Result<User> {
    let user = conn.query_row(
        "SELECT * FROM user WHERE id = ?1",
        [id],
        |row| {
            Ok(User{
                id: row.get(0)?,
                name: row.get(1)?,
                email: row.get(2)?,
                password: row.get(3)?,
                role: get_role_by_id(conn, row.get(4)?)
            })
        }
    );
    user
}