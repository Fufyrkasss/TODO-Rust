use rusqlite::{
    Connection,
};
use crate::models::user::Role;

pub fn get_role_by_id(conn: &Connection, id: i32) -> Role {
    let role_name: String = conn.query_row(
            "SELECT name FROM role WHERE id = ?1",
            [id],
            |row| row.get(0),
        ).unwrap();
    let role = match role_name.as_str() {
        "admin" => Role::Admin,
        "user" => Role::User,
        _ => Role::None,
    };
    role
}