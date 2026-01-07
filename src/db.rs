use rusqlite::{Connection, Result};

pub fn init() -> Result<Connection> {
    let db_path = "zel_history.db";
    Connection::open(db_path)
}
