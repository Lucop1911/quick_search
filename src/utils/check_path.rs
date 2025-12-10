use std::path::Path;
use crate::utils::utils::{ActionType, SearchResult};

pub fn check_path(text: &str) -> Option<SearchResult> {
    let path = Path::new(text);
    
    if path.exists() {
        let is_dir = path.is_dir();
        return Some(SearchResult {
            title: text.to_string(),
            subtitle: if is_dir { "Open folder" } else { "Open file" }.to_string(),
            icon: if is_dir { "📁" } else { "📄" }.to_string(),
            action: ActionType::OpenPath(path.to_path_buf()),
        });
    }
    
    // Try expanding ~ for home directory
    if text.starts_with('~') {
        if let Some(home) = dirs::home_dir() {
            let expanded = home.join(&text[2..]);
            if expanded.exists() {
                let is_dir = expanded.is_dir();
                return Some(SearchResult {
                    title: text.to_string(),
                    subtitle: if is_dir { "Open folder" } else { "Open file" }.to_string(),
                    icon: if is_dir { "📁" } else { "📄" }.to_string(),
                    action: ActionType::OpenPath(expanded),
                });
            }
        }
    }
    None
}