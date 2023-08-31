use rusqlite::{Connection, Result};

pub fn init_db() -> Result<()> {
    let conn = Connection::open("merchant.db")?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS items (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT UNIQUE NOT NULL
        )",
        [],
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS trades (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            item_id INTEGER NOT NULL,
            quantity INTEGER NOT NULL,
            total_price INTEGER NOT NULL,
            is_purchase BOOLEAN NOT NULL,
            timestamp TEXT NOT NULL,
            FOREIGN KEY (item_id) REFERENCES items (id)
        )",
        [],
    )?;

    Ok(())
}
