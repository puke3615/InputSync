use serde::{Deserialize, Serialize};
use std::path::PathBuf;

use crate::scene::Scene;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    #[serde(default)]
    pub scenes: Vec<Scene>,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            scenes: crate::scene::default_scenes(),
        }
    }
}

pub fn config_dir_path() -> PathBuf {
    let dir = config_dir();
    if !dir.exists() {
        let _ = std::fs::create_dir_all(&dir);
    }
    dir
}

fn config_dir() -> PathBuf {
    #[cfg(target_os = "windows")]
    {
        let appdata = std::env::var("APPDATA").unwrap_or_else(|_| ".".into());
        PathBuf::from(appdata).join("TalkType")
    }
    #[cfg(not(target_os = "windows"))]
    {
        let home = std::env::var("HOME").unwrap_or_else(|_| ".".into());
        PathBuf::from(home).join(".talktype")
    }
}

fn config_path() -> PathBuf {
    config_dir().join("config.json")
}

pub fn load_config() -> AppConfig {
    let path = config_path();
    if !path.exists() {
        let config = AppConfig::default();
        save_config(&config);
        return config;
    }

    match std::fs::read_to_string(&path) {
        Ok(content) => match serde_json::from_str::<AppConfig>(&content) {
            Ok(mut config) => {
                merge_builtin_scenes(&mut config);
                config
            }
            Err(e) => {
                log::error!("Failed to parse config at {}: {}", path.display(), e);
                AppConfig::default()
            }
        },
        Err(e) => {
            log::error!("Failed to read config at {}: {}", path.display(), e);
            AppConfig::default()
        }
    }
}

/// Ensure all built-in scenes exist in the config, adding any missing ones.
fn merge_builtin_scenes(config: &mut AppConfig) {
    let defaults = crate::scene::default_scenes();
    for default in defaults {
        if !config.scenes.iter().any(|s| s.id == default.id) {
            config.scenes.push(default);
        }
    }
}

pub fn save_config(config: &AppConfig) {
    let dir = config_dir();
    if let Err(e) = std::fs::create_dir_all(&dir) {
        log::error!("Failed to create config dir {}: {}", dir.display(), e);
        return;
    }

    let path = config_path();
    match serde_json::to_string_pretty(config) {
        Ok(json) => {
            if let Err(e) = std::fs::write(&path, json) {
                log::error!("Failed to write config to {}: {}", path.display(), e);
            } else {
                log::info!("Config saved to {}", path.display());
            }
        }
        Err(e) => {
            log::error!("Failed to serialize config: {}", e);
        }
    }
}

pub fn save_scenes(scenes: &[Scene]) {
    let mut config = load_config();
    config.scenes = scenes.to_vec();
    save_config(&config);
}
