#[cfg(test)]
mod tests {
    use crate::howmoji_config::config::Config;
    #[test]
    fn test_config_new_creates_valid_config() {
        let db_path = "/path/to/test.db".to_string();
        let config = Config::new(db_path.clone());

        assert_eq!(config.db_path, db_path);
        assert!(!config.app_version.is_empty());
        assert!(!config.emoji_data.is_empty());
    }

    #[test]
    fn test_config_with_relative_path() {
        let db_path = "./data/test.db".to_string();
        let config = Config::new(db_path.clone());

        assert_eq!(config.db_path, db_path);
        assert!(!config.app_version.is_empty());
    }

    #[test]
    fn test_app_version_consistency() {
        let config1 = Config::new("path1.db".to_string());
        let config2 = Config::new("path2.db".to_string());

        assert_eq!(config1.app_version, config2.app_version);
    }

    #[test]
    fn test_app_version_matches_cargo_version() {
        let config = Config::new("test.db".to_string());
        assert_eq!(config.app_version, env!("CARGO_PKG_VERSION"));
    }
}
