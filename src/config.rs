use dirs::home_dir;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub default_url: Option<String>,
    pub default_email: Option<String>,
    pub default_policy: Option<String>,
    pub default_upload_path: Option<String>,
    pub default_download_dir: Option<String>,
    pub log_level: Option<String>,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            default_url: None,
            default_email: None,
            default_policy: None,
            default_upload_path: Some("/".to_string()),
            default_download_dir: Some(".".to_string()),
            log_level: Some("info".to_string()),
        }
    }
}

impl Config {
    pub fn load() -> Result<Self, Box<dyn std::error::Error>> {
        let config_path = Self::config_file_path()?;

        if !config_path.exists() {
            return Ok(Self::default());
        }

        let content = fs::read_to_string(&config_path)?;
        let config: Config = toml::from_str(&content)?;
        Ok(config)
    }

    pub fn _save(&self) -> Result<(), Box<dyn std::error::Error>> {
        let config_path = Self::config_file_path()?;

        // Ensure config directory exists
        if let Some(parent) = config_path.parent() {
            fs::create_dir_all(parent)?;
        }

        let content = toml::to_string_pretty(self)?;
        fs::write(&config_path, content)?;
        Ok(())
    }

    fn config_file_path() -> Result<PathBuf, Box<dyn std::error::Error>> {
        let config_dir = home_dir()
            .ok_or("Cannot determine home directory")?
            .join(".config")
            .join("cloudreve-cli");

        Ok(config_dir.join("config.toml"))
    }
}
