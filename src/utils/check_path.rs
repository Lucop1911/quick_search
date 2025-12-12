use std::path::Path;
use crate::utils::utils::{ActionType, SearchResult};

pub fn check_path(text: &str) -> Option<SearchResult> {
    // Handle tilde expansion first
    if text.starts_with('~') {
        return check_tilde_path(text);
    }
    
    let path = Path::new(text);
    
    if path.exists() {
        let is_dir = path.is_dir();
        return Some(SearchResult {
            title: text.to_string(),
            subtitle: if is_dir { "Open folder" } else { "Open file" }.to_string(),
            icon: if is_dir { "[DIR]" } else { "[FILE]" }.to_string(),
            action: ActionType::OpenPath(path.to_path_buf()),
        });
    }
    
    None
}

fn check_tilde_path(text: &str) -> Option<SearchResult> {
    let home = dirs::home_dir()?;
    
    let expanded = if text == "~" {
        home
    } else if text.starts_with("~/") {
        if text.len() > 2 {
            home.join(&text[2..])
        } else {
            home
        }
    } else {
        return None;
    };
    
    if expanded.exists() {
        let is_dir = expanded.is_dir();
        return Some(SearchResult {
            title: text.to_string(),
            subtitle: if is_dir { "Open folder" } else { "Open file" }.to_string(),
            icon: if is_dir { "[DIR]" } else { "[FILE]" }.to_string(),
            action: ActionType::OpenPath(expanded),
        });
    }
    
    None
}