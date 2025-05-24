#[derive(Debug)]
pub struct Config {
    pub db_path: String,
    pub app_version: String,
    pub emoji_data: &'static str,
}

impl Config {
    pub fn new(db_path: String) -> Self {
        Config {
            db_path,
            app_version: Self::app_version_from_toml(),
            emoji_data: include_str!("../../config/howmoji.json"),
        }
    }

    fn app_version_from_toml() -> String {
        env!("CARGO_PKG_VERSION").to_string()
    }
}
