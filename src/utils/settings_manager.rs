use std::fs;
use std::path::PathBuf;
use serde::{Deserialize, Serialize};

#[cfg(target_os = "windows")]
use egui::{Key, KeyboardShortcut, Modifiers};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Settings {
    pub enable_history: bool,
    pub enable_web_search: bool,
    pub enable_math_eval: bool,
    pub enable_file_search: bool,
    pub enable_app_search: bool,
    
    #[cfg(target_os = "windows")]
    #[serde(default = "default_shortcut")]
    pub open_shortcut: SerializableShortcut,
}

#[cfg(target_os = "windows")]
fn default_shortcut() -> SerializableShortcut {
    SerializableShortcut {
        modifiers: vec!["Alt".to_string()],
        key: "S".to_string(),
    }
}

#[cfg(target_os = "windows")]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SerializableShortcut {
    pub modifiers: Vec<String>,
    pub key: String,
}

#[cfg(target_os = "windows")]
impl SerializableShortcut {
    pub fn to_keyboard_shortcut(&self) -> KeyboardShortcut {
        let mut mods = Modifiers::NONE;
        
        for m in &self.modifiers {
            match m.as_str() {
                "Ctrl" | "Control" => mods |= Modifiers::CTRL,
                "Alt" => mods |= Modifiers::ALT,
                "Shift" => mods |= Modifiers::SHIFT,
                "Command" | "Cmd" => mods |= Modifiers::COMMAND,
                _ => {}
            }
        }
        
        let key = match self.key.to_uppercase().as_str() {
            "A" => Key::A, "B" => Key::B, "C" => Key::C, "D" => Key::D,
            "E" => Key::E, "F" => Key::F, "G" => Key::G, "H" => Key::H,
            "I" => Key::I, "J" => Key::J, "K" => Key::K, "L" => Key::L,
            "M" => Key::M, "N" => Key::N, "O" => Key::O, "P" => Key::P,
            "Q" => Key::Q, "R" => Key::R, "S" => Key::S, "T" => Key::T,
            "U" => Key::U, "V" => Key::V, "W" => Key::W, "X" => Key::X,
            "Y" => Key::Y, "Z" => Key::Z,
            "SPACE" => Key::Space,
            "ENTER" => Key::Enter,
            _ => Key::S, // fallback
        };
        
        KeyboardShortcut::new(mods, key)
    }
    
    pub fn from_keyboard_shortcut(shortcut: &KeyboardShortcut) -> Self {
        let mut modifiers = Vec::new();
        
        if shortcut.modifiers.ctrl {
            modifiers.push("Ctrl".to_string());
        }
        if shortcut.modifiers.alt {
            modifiers.push("Alt".to_string());
        }
        if shortcut.modifiers.shift {
            modifiers.push("Shift".to_string());
        }
        if shortcut.modifiers.command {
            modifiers.push("Command".to_string());
        }
        
        let key = format!("{:?}", shortcut.logical_key);
        
        Self { modifiers, key }
    }
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            enable_history: true,
            enable_web_search: true,
            enable_math_eval: true,
            enable_file_search: true,
            enable_app_search: true,
            
            #[cfg(target_os = "windows")]
            open_shortcut: default_shortcut(),
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
        #[cfg(target_os = "linux")]
        {
            if let Some(config_dir) = dirs::config_dir() {
                let app_dir = config_dir.join("quick_search");
                let _ = fs::create_dir_all(&app_dir);
                return app_dir.join("settings.json");
            }
        }
        
        #[cfg(target_os = "windows")]
        {
            if let Some(config_dir) = dirs::config_dir() {
                let app_dir = config_dir.join("QuickSearch");
                let _ = fs::create_dir_all(&app_dir);
                return app_dir.join("settings.json");
            }
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