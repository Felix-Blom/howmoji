use rusqlite::{Connection, Result};

#[derive(Debug, Clone)]
pub struct Howmoji {
    pub id: i32,
    pub emoji: String,
    pub description: String,
}

impl Howmoji {
    pub fn new(id: i32, emoji: String, description: String) -> Self {
        Howmoji {
            id,
            emoji,
            description,
        }
    }

    pub fn from_row(row: &rusqlite::Row) -> Result<Self> {
        let id: i32 = row.get(0).expect("Failed to get id");
        let emoji: String = row.get(1).expect("Failed to get emoji");
        let description: String = row.get(2).expect("Failed to get description");
        Ok(Howmoji::new(id, emoji, description))
    }

    fn exist_in_db(&self, conn: &Connection) -> Result<bool> {
        let mut stmt = conn.prepare("SELECT COUNT(*) FROM howmoji WHERE id = ?1")?;
        let count: i32 = stmt.query_row(rusqlite::params![self.id], |row| row.get(0))?;
        Ok(count > 0)
    }

    pub fn save_to_db(&self, conn: &Connection) -> Result<()> {
        // Simple overwrite if the howmoji already exists
        if self.exist_in_db(conn)? {
            println!(
                "Howmoji with id {} already exists in the database, updating",
                self.id
            );
            conn.execute(
                "DELETE FROM howmoji WHERE id = ?1",
                rusqlite::params![self.id],
            )
            .expect("Failed to delete existing howmoji from database");
            return Ok(());
        }

        conn.execute(
            "INSERT INTO howmoji (id, emoji, description) VALUES (?1, ?2, ?3)",
            rusqlite::params![self.id, self.emoji, self.description],
        )
        .expect("Failed to insert howmoji into database");
        Ok(())
    }

    // Test functions for private methods
    #[cfg(test)]
    pub fn test_exist_in_db(&self, conn: &Connection) -> Result<bool> {
        self.exist_in_db(conn)
    }
}
