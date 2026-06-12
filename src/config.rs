use std::fs;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    #[serde(default)]
    pub settings: Settings,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Settings {
    pub refresh_rate: u64,
    pub sort_by: String,
    pub sort_order: String,
    pub theme: String,
}

impl Default for Settings {
    fn default() -> Self {
        Settings {
            refresh_rate: 1000,
            sort_by: "size".to_string(),
            sort_order: "desc".to_string(),
            theme: "dark".to_string(),
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Config {
            settings: Settings::default(),
        }
    }
}

impl Config {
    pub fn load() -> Self {
        let path = config_path();
        if !path.exists() {
            let config = Config::default();
            let _ = config.save();
            return config;
        }
        match fs::read_to_string(&path) {
            Ok(content) => match toml::from_str(&content) {
                Ok(config) => config,
                Err(_) => Config::default(),
            },
            Err(_) => Config::default(),
        }
    }

    pub fn save(&self) -> Result<(), String> {
        let path = config_path();
        if let Some(parent) = path.parent() {
            let _ = fs::create_dir_all(parent);
        }
        let content = toml::to_string_pretty(self).map_err(|e| e.to_string())?;
        fs::write(&path, content).map_err(|e| e.to_string())?;
        Ok(())
    }
}

fn config_path() -> PathBuf {
    let mut path = dirs::config_dir().unwrap_or_else(|| PathBuf::from("."));
    path.push("hdu");
    path.push("config.toml");
    path
}
