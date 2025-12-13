use std::fs;
use std::path::PathBuf;
use serde::{Deserialize, Serialize};
use crate::{gui::history::HistoryApp, utils::{execute_action::execute_action, settings_manager::SettingsManager, utils::SearchResult}};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct HistoryEntry {
    pub query: String,
    pub result_title: String,
    pub result_subtitle: String,
    pub result_icon: String,
    pub action_type: String,
    pub action_data: String,
    pub timestamp: String,
}

impl HistoryEntry {
    pub fn from_search(query: &str, result: &SearchResult) -> Self {
        use crate::utils::utils::ActionType;
        
        let (action_type, action_data) = match &result.action {
            ActionType::OpenSettings => ("OpenSettings".to_string(), String::new()),
            ActionType::OpenHistory => ("OpenHistory".to_string(), String::new()),
            ActionType::OpenInfo => ("OpenInfo".to_string(), String::new()),
            ActionType::OpenApp(path) => ("OpenApp".to_string(), path.to_string_lossy().to_string()),
            ActionType::OpenPath(path) => ("OpenPath".to_string(), path.to_string_lossy().to_string()),
            ActionType::OpenUrl(url) => ("OpenUrl".to_string(), url.clone()),
            ActionType::MathResult(res) => ("MathResult".to_string(), res.clone()),
            ActionType::WebSearch(q) => ("WebSearch".to_string(), q.clone()),
            ActionType::RunCommand(command) => ("Command".to_string(), command.clone()),
        };
        
        let timestamp = chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
        
        Self {
            query: query.to_string(),
            result_title: result.title.clone(),
            result_subtitle: result.subtitle.clone(),
            result_icon: result.icon.clone(),
            action_type,
            action_data,
            timestamp,
        }
    }
    
    pub fn to_search_result(&self) -> SearchResult {
        use std::path::PathBuf;
        use crate::utils::utils::ActionType;
        
        let action = match self.action_type.as_str() {
            "OpenSettings" => ActionType::OpenSettings,
            "OpenHistory" => ActionType::OpenHistory,
            "OpenInfo" => ActionType::OpenInfo,
            "OpenApp" => ActionType::OpenApp(PathBuf::from(&self.action_data)),
            "OpenPath" => ActionType::OpenPath(PathBuf::from(&self.action_data)),
            "OpenUrl" => ActionType::OpenUrl(self.action_data.clone()),
            "MathResult" => ActionType::MathResult(self.action_data.clone()),
            "WebSearch" => ActionType::WebSearch(self.action_data.clone()),
            "Command" => ActionType::RunCommand(self.action_data.clone()),
            _ => ActionType::WebSearch(self.query.clone()),
        };
        
        SearchResult {
            title: self.result_title.clone(),
            subtitle: self.result_subtitle.clone(),
            icon: self.result_icon.clone(),
            action,
        }
    }
}

pub struct HistoryManager {
    history_file: PathBuf,
}

impl HistoryManager {
    pub fn new() -> Self {
        let history_file = Self::get_history_path();
        Self { history_file }
    }
    
    fn get_history_path() -> PathBuf {
        if let Some(config_dir) = dirs::config_dir() {
            let app_dir = config_dir.join("quick_search");
            let _ = fs::create_dir_all(&app_dir);
            return app_dir.join("history.json");
        }
        
        // Fallback
        PathBuf::from("history.json")
    }

    pub fn load_history(&self) -> Vec<HistoryEntry> {
        if !self.history_file.exists() {
            return Vec::new();
        }
        
        match fs::read_to_string(&self.history_file) {
            Ok(content) => {
                serde_json::from_str(&content).unwrap_or_else(|_| Vec::new())
            }
            Err(_) => Vec::new(),
        }
    }
    
    pub fn save_history(&self, history: &[HistoryEntry]) -> Result<(), std::io::Error> {
        let settings_manager = SettingsManager::new();
        let settings = settings_manager.load_settings();

        if settings.enable_history == true {
            let json = serde_json::to_string_pretty(history)?;
            fs::write(&self.history_file, json)?;
        }
        Ok(())
    }
    
    pub fn add_entry(&self, entry: HistoryEntry) {
        let mut history = self.load_history();
        
        // Most recent first
        history.insert(0, entry);
        
        // Only last 100 entries
        history.truncate(100);
        
        let _ = self.save_history(&history);
    }
    
    pub fn clear_history(&self) {
        let _ = self.save_history(&[]);
    }
}

impl HistoryApp {
    pub fn clear_history(&mut self) {
        self.history_manager.clear_history();
        self.selected_index = None;
    }

    pub fn get_filtered_history(&self) -> Vec<HistoryEntry> {
        let history = self.history_manager.load_history();
        
        if self.search_filter.is_empty() {
            history
        } else {
            history.iter()
                .filter(|e| {
                    e.query.to_lowercase().contains(&self.search_filter.to_lowercase()) ||
                    e.result_title.to_lowercase().contains(&self.search_filter.to_lowercase())
                })
                .cloned()
                .collect()
        }
    }

    pub fn execute_history_item(&mut self, entry: &HistoryEntry, ctx: &egui::Context) {
        let result = entry.to_search_result();
        execute_action(&result, &entry.query);
        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
    }
    
    pub fn delete_entry(&mut self, entry: &HistoryEntry) {
        let mut all = self.history_manager.load_history();
    
        let target = entry;
    
        // Find the matching entry in the list
        if let Some(real_index) = all.iter().position(|e|
            e.query == target.query &&
            e.result_title == target.result_title &&
            e.result_subtitle == target.result_subtitle &&
            e.timestamp == target.timestamp
        ) {
            all.remove(real_index);
            let _ = self.history_manager.save_history(&all);
    
            self.selected_index = None;
        }
    }
}