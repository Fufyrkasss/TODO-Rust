use rusqlite::{
    Connection,
    Result,
};

pub fn init_db(conn: &Connection) -> Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS user (
            id       INTEGER PRIMARY KEY,
            name     TEXT NOT NULL,
            email    TEXT NOT NULL,
            password TEXT NOT NULL,
            role     INTEGER
        )",
        (),
    )?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS role (
            id    INTEGER PRIMARY KEY,
            name  TEXT NOT NULL
        )",
        (),
    )?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS task (
            id       INTEGER PRIMARY KEY,
            name     TEXT NOT NULL,
            content  TEXT NOT NULL,
            status   INTEGER,
            creator  INTEGER,
            deadline TEXT NOT NULL
        )",
        (),
    )?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS status (
            id       INTEGER PRIMARY KEY,
            name     TEXT NOT NULL
        )",
        (),
    )?;
    conn.execute(
        "INSERT OR IGNORE INTO status(name)
        VALUES ('done')",
        (),
    )?;
    conn.execute(
        "INSERT OR IGNORE INTO status(name)
        VALUES ('undone')",
        (),
    )?;
    conn.execute(
        "INSERT OR IGNORE INTO status(name)
        VALUES ('in progress')",
        (),
    )?;
    conn.execute(
        "INSERT OR IGNORE INTO role(name)
        VALUES ('user')",
        (),
    )?;
    conn.execute(
        "INSERT OR IGNORE INTO role(name)
        VALUES ('admin')",
        (),
    )?;
    conn.execute(
        "INSERT OR IGNORE INTO user (name, email, password, role)
        VALUES ('admin', 'Admin@gg.com', 'admin', 1)",
        (),
    )?;
    Ok(())
}