use std::fs;
use std::path::PathBuf;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Settings {
    pub enable_history: bool,
    pub enable_web_search: bool,
    pub enable_math_eval: bool,
    pub enable_file_search: bool,
    pub enable_app_search: bool,
    pub terminal_command: String,
    pub text_editor_command: String,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            enable_history: true,
            enable_web_search: true,
            enable_math_eval: true,
            enable_file_search: true,
            enable_app_search: true,
            terminal_command: String::new(),
            text_editor_command: String::new(),
        }
    }
}

pub struct SettingsManager {
    settings_file: PathBuf,
}

impl SettingsManager {
    pub fn new() -> Self {
        let settings_file = Self::get_settings_path();
        Self { settings_file }
    }
    
    fn get_settings_path() -> PathBuf {
        if let Some(config_dir) = dirs::config_dir() {
            let app_dir = config_dir.join("quick_search");
            let _ = fs::create_dir_all(&app_dir);
            return app_dir.join("settings.json");
        }
    
        PathBuf::from("settings.json")
    }
    
    pub fn load_settings(&self) -> Settings {
        if !self.settings_file.exists() {
            return Settings::default();
        }
        
        match fs::read_to_string(&self.settings_file) {
            Ok(content) => {
                serde_json::from_str(&content).unwrap_or_else(|_| Settings::default())
            }
            Err(_) => Settings::default(),
        }
    }
    
    pub fn save_settings(&self, settings: &Settings) -> Result<(), std::io::Error> {
        let json = serde_json::to_string_pretty(settings)?;
        fs::write(&self.settings_file, json)?;
        Ok(())
    }
}