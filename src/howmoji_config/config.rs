use log::debug;
use toml;

#[derive(Debug)]
pub struct Config {
    pub json_path: String,
    pub db_path: String,
    pub app_version: String,
}

impl Config {
    pub fn new(db_path: String) -> Self {
        Config {
            json_path: "config/howmoji.json".to_string(),
            db_path,
            app_version: Self::app_version_from_toml(),
        }
    }

    fn app_version_from_toml() -> String {
        match std::fs::read_to_string("Cargo.toml") {
            Ok(message) => {
                if let Ok(parsed) = message.parse::<toml::Value>() {
                    if let Some(version) = parsed
                        .get("package")
                        .and_then(|p| p.get("version"))
                        .and_then(|v| v.as_str())
                    {
                        return version.to_string();
                    }
                }
                debug!("Failed to extract version from Cargo.toml");
            }
            Err(e) => {
                debug!("Failed to read Cargo.toml: {}", e);
            }
        }
        "0.0.0".to_string()
    }
}
