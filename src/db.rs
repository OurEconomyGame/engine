use rusqlite::{Connection, Result};

pub fn init_db() -> Result<Connection> {
    let conn = Connection::open("main.db")?;

    // Enable foreign key support (very important in SQLite)
    conn.execute("PRAGMA foreign_keys = ON;", [])?;
    let _ = conn.query_row("PRAGMA journal_mode = WAL;", [], |_row| Ok(()));


    // Create `user` table
    conn.execute(
        "CREATE TABLE IF NOT EXISTS user (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            username TEXT UNIQUE NOT NULL,
            password_hash TEXT NOT NULL,
            usd INTEGER NOT NULL DEFAULT 0,
            data TEXT
        );",
        [],
    )?;

    // Create `company` table
    conn.execute(
        "CREATE TABLE IF NOT EXISTS company (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL,
            owner TEXT NOT NULL,
            type TEXT NOT NULL,
            data TEXT
        );",
        [],
    )?;

    // Create `extchange` table
    conn.execute(
        "CREATE TABLE IF NOT EXISTS extchange (
            id INTEGER PRIMARY KEY,
            item TEXT NOT NULL,
            type BOOLEAN NOT NULL,
            amount INTEGER NOT NULL,
            unit_price INTEGER NOT NULL,
            unit_type TEXT NOT NULL,
            entity INTEGER NOT NULL,
            entity_type INTEGER NOT NULL
        );",
        [],
    )?;

    // Create `job_offers` table
    conn.execute(
        "CREATE TABLE IF NOT EXISTS job_offers (
            id INTEGER PRIMARY KEY,
            entity_id INTEGER NOT NULL,
            FOREIGN KEY (entity_id) REFERENCES company(id)
        );",
        [],
    )?;

    Ok(conn)
}
