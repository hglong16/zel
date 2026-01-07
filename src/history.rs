use rusqlite::{Connection, Result};

pub struct History {
    pub db_conn: Connection,
}

impl History {
    pub fn new() -> Result<Self> {
        let conn = crate::db::init()?;
        conn.execute(
            "CREATE TABLE IF NOT EXISTS history (
                id INTEGER PRIMARY KEY,
                command TEXT NOT NULL,
                timestamp INTEGER,
                directory TEXT
            )"
            , []);
        Ok(History { db_conn: conn })
    }

    pub fn save(&self, cmd: &str) -> Result<()> {

        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        self.db_conn.execute("INSERT INTO  history (command, timestamp) VALUES (?1, ?2)", (cmd, timestamp as i64))?;

        Ok(())
    }

    pub fn list(&self, limit: usize) -> Result <Vec<String>> {
        let mut stmt = self.db_conn.prepare(
            "SELECT command FROM history ORDER BY timestamp DESC LIMIT ?1", 
        )?;

        let rows = stmt.query_map([limit as i64], |row| {
            row.get(0)
        })?;

        let mut commands = Vec::new();
        for command in rows {
            commands.push(command?);
        }

        Ok(commands)
    }
}
