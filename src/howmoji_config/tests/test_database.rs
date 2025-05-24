#[cfg(test)]
mod tests {
    use crate::howmoji_config::howmoji::Howmoji;
    use rusqlite::Connection;

    fn setup_test_db() -> Connection {
        let conn = Connection::open_in_memory().expect("Failed to create in-memory database");
        conn.execute(
            "CREATE TABLE howmoji (
                id INTEGER PRIMARY KEY,
                emoji TEXT NOT NULL,
                description TEXT NOT NULL
            )",
            [],
        )
        .expect("Failed to create table");
        conn
    }

    #[test]
    fn test_new() {
        let howmoji = Howmoji::new(1, "😀".to_string(), "Happy face".to_string());
        assert_eq!(howmoji.id, 1);
        assert_eq!(howmoji.emoji, "😀");
        assert_eq!(howmoji.description, "Happy face");
    }

    #[test]
    fn test_from_row() {
        let conn = setup_test_db();
        conn.execute(
            "INSERT INTO howmoji (id, emoji, description) VALUES (1, '😀', 'Happy face')",
            [],
        )
        .expect("Failed to insert test data");

        let mut stmt = conn
            .prepare("SELECT id, emoji, description FROM howmoji WHERE id = 1")
            .expect("Failed to prepare statement");

        let howmoji = stmt
            .query_row([], Howmoji::from_row)
            .expect("Failed to query row");

        assert_eq!(howmoji.id, 1);
        assert_eq!(howmoji.emoji, "😀");
        assert_eq!(howmoji.description, "Happy face");
    }

    #[test]
    fn test_exist_in_db_true() {
        let conn = setup_test_db();
        let howmoji = Howmoji::new(1, "😀".to_string(), "Happy face".to_string());

        conn.execute(
            "INSERT INTO howmoji (id, emoji, description) VALUES (1, '😀', 'Happy face')",
            [],
        )
        .expect("Failed to insert test data");

        assert!(howmoji
            .test_exist_in_db(&conn)
            .expect("Failed to check existence"));
    }

    #[test]
    fn test_exist_in_db_false() {
        let conn = setup_test_db();
        let howmoji = Howmoji::new(999, "😀".to_string(), "Happy face".to_string());

        assert!(!howmoji
            .test_exist_in_db(&conn)
            .expect("Failed to check existence"));
    }

    #[test]
    fn test_save_to_db_new_record() {
        let conn = setup_test_db();
        let howmoji = Howmoji::new(1, "😀".to_string(), "Happy face".to_string());

        howmoji.save_to_db(&conn).expect("Failed to save howmoji");

        let mut stmt = conn
            .prepare("SELECT COUNT(*) FROM howmoji WHERE id = 1")
            .expect("Failed to prepare statement");
        let count: i32 = stmt
            .query_row([], |row| row.get(0))
            .expect("Failed to get count");

        assert_eq!(count, 1);
    }

    #[test]
    fn test_save_to_db_existing_record() {
        let conn = setup_test_db();

        conn.execute(
            "INSERT INTO howmoji (id, emoji, description) VALUES (1, '😀', 'Happy face')",
            [],
        )
        .expect("Failed to insert initial data");

        let howmoji = Howmoji::new(1, "😃".to_string(), "Updated happy face".to_string());
        howmoji.save_to_db(&conn).expect("Failed to save howmoji");

        let mut stmt = conn
            .prepare("SELECT COUNT(*) FROM howmoji WHERE id = 1")
            .expect("Failed to prepare statement");
        let count: i32 = stmt
            .query_row([], |row| row.get(0))
            .expect("Failed to get count");

        assert_eq!(count, 0);
    }

    #[test]
    fn test_save_to_db_multiple_records() {
        let conn = setup_test_db();

        let howmoji1 = Howmoji::new(1, "😀".to_string(), "Happy".to_string());
        let howmoji2 = Howmoji::new(2, "😢".to_string(), "Sad".to_string());

        howmoji1
            .save_to_db(&conn)
            .expect("Failed to save first howmoji");
        howmoji2
            .save_to_db(&conn)
            .expect("Failed to save second howmoji");

        let mut stmt = conn
            .prepare("SELECT COUNT(*) FROM howmoji")
            .expect("Failed to prepare statement");
        let count: i32 = stmt
            .query_row([], |row| row.get(0))
            .expect("Failed to get count");

        assert_eq!(count, 2);
    }

    #[test]
    fn test_clone() {
        let original = Howmoji::new(1, "😀".to_string(), "Happy".to_string());
        let cloned = original.clone();

        assert_eq!(original.id, cloned.id);
        assert_eq!(original.emoji, cloned.emoji);
        assert_eq!(original.description, cloned.description);
    }
}
