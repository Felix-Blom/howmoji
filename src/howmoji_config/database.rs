use crate::howmoji_config::config;
use crate::howmoji_config::howmoji::Howmoji;
use config::Config;
use log::debug;
use rusqlite::{Connection, Result};
use std::path::Path;

pub struct Database<'config> {
    conn: Connection,
    config: &'config Config,
}

impl<'config> Database<'config> {
    pub fn new(config: &'config Config) -> Self {
        let conn = Connection::open(&config.db_path).expect("Failed to open database");
        Database { conn, config }
    }

    pub fn initialize(&self) -> Result<()> {
        debug!("Initializing the database.");
        if self.requires_update() {
            debug!("Database requires update, dropping and creating tables.");
            self.drop_tables()?;
            self.create_tables()?;
            self.insert_emojis_from_json()?;
            self.insert_version()?;
        } else {
            debug!("Database is up to date, no action required.");
        }
        Ok(())
    }

    pub fn requires_update(&self) -> bool {
        if !Path::new(&self.config.db_path).exists() {
            debug!("Database does not exist, requires update.");
            return true;
        }
        debug!("Database exists, checking version.");
        if let Some(database_version) = self.get_db_version() {
            if database_version != self.config.app_version {
                debug!("Database version mismatch, requires update.");
                return true;
            }
            debug!("Database version matches, no update required.");
            false
        } else {
            debug!("Failed to get database version, re-initializing.");
            true
        }
    }

    fn get_db_version(&self) -> Option<String> {
        let mut stmt = self
            .conn
            .prepare("SELECT version FROM version ORDER BY id DESC LIMIT 1")
            .ok()?;
        let version: String = stmt.query_row([], |row| row.get(0)).ok()?;
        Some(version)
    }

    fn drop_tables(&self) -> Result<()> {
        debug!("Dropping tables in the database.");
        self.conn.execute("DROP TABLE IF EXISTS howmoji", [])?;
        self.conn.execute("DROP TABLE IF EXISTS version", [])?;
        Ok(())
    }

    fn create_tables(&self) -> Result<()> {
        debug!("Creating tables in the database.");
        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS version (
                id INTEGER PRIMARY KEY,
                version TEXT NOT NULL
            )",
            [],
        )?;
        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS howmoji (
                id INTEGER PRIMARY KEY,
                emoji TEXT NOT NULL,
                description TEXT NOT NULL
            )",
            [],
        )?;
        Ok(())
    }

    fn insert_emojis_from_json(&self) -> Result<()> {
        debug!("Inserting emojis from embedded JSON into the database.");

        // Parse the embedded JSON data instead of reading from file
        let json: serde_json::Value =
            serde_json::from_str(self.config.emoji_data).map_err(|e| {
                rusqlite::Error::InvalidParameterName(format!(
                    "Failed to parse embedded JSON: {}",
                    e
                ))
            })?;

        let emoji_list = json["gitmojis"].as_array().ok_or_else(|| {
            rusqlite::Error::InvalidParameterName("Expected an array of gitmojis".to_string())
        })?;

        for (i, emoji_data) in emoji_list.iter().enumerate() {
            let emoji = emoji_data["emoji"].as_str().unwrap_or("");
            let description = emoji_data["description"].as_str().unwrap_or("");

            let howmoji = Howmoji::new(i as i32, emoji.to_string(), description.to_string());
            howmoji
                .save_to_db(&self.conn)
                .map_err(|_| rusqlite::Error::ExecuteReturnedResults)?;
        }
        Ok(())
    }
    fn insert_version(&self) -> Result<()> {
        debug!("Inserting version into the database.");
        self.conn.execute(
            "INSERT INTO version (version) VALUES (?1)",
            [&self.config.app_version],
        )?;
        Ok(())
    }

    pub fn get_howmojis(&self) -> Result<Vec<Howmoji>> {
        debug!("Fetching howmojis from the database.");
        let mut stmt = self
            .conn
            .prepare("SELECT id, emoji, description FROM howmoji")?;

        let howmojis = stmt
            .query_map([], Howmoji::from_row)?
            .collect::<Result<Vec<_>, _>>()?;

        Ok(howmojis)
    }

    pub fn get_howmoji_by_id(&self, id: i32) -> Result<Option<Howmoji>> {
        debug!("Fetching howmoji with ID {} from the database.", id);
        let mut stmt = self
            .conn
            .prepare("SELECT id, emoji, description FROM howmoji WHERE id = ?1")?;

        let result = stmt.query_row([id], Howmoji::from_row);

        match result {
            Ok(howmoji) => Ok(Some(howmoji)),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(e),
        }
    }
}
