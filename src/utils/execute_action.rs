use crate::utils::{helpers::helpers, utils::{ActionType, SearchResult}};

pub fn execute_action(result: &SearchResult, query: &str) {
    // Save to history (except for special commands)
    let should_save_history = !matches!(
        result.action,
        ActionType::OpenHistory | ActionType::OpenSettings | ActionType::OpenInfo
    );
    
    if should_save_history {
        use crate::utils::history_manager::{HistoryManager, HistoryEntry};
        let manager = HistoryManager::new();
        let entry = HistoryEntry::from_search(query, result);
        manager.add_entry(entry);
    }
    
    match &result.action {
        ActionType::OpenHistory => {
            // Launch history window as a separate process
            let exe_path = std::env::current_exe().ok();
            if let Some(exe) = exe_path {
                let _ = std::process::Command::new(exe)
                    .arg("--history")
                    .spawn();
            }
        }
        ActionType::OpenSettings => {
            // Launch settings window as a separate process
            let exe_path = std::env::current_exe().ok();
            if let Some(exe) = exe_path {
                let _ = std::process::Command::new(exe)
                    .arg("--settings")
                    .spawn();
            }
        }
        ActionType::OpenInfo => {
            // Launch info window as a separate process
            let exe_path = std::env::current_exe().ok();
            if let Some(exe) = exe_path {
                let _ = std::process::Command::new(exe)
                    .arg("--info")
                    .spawn();
            }
        }
        ActionType::OpenApp(path) => {
            #[cfg(target_os = "linux")]
            {
                let path_str = path.to_string_lossy();
                // Extract the executable from the Exec line
                let exec_parts: Vec<&str> = path_str.split_whitespace().collect();
                if let Some(exec) = exec_parts.first() {
                    let _ = std::process::Command::new(exec)
                        .spawn();
                }
            }
            
            #[cfg(target_os = "windows")]
            {
                let _ = std::process::Command::new("cmd")
                    .args(&["/C", "start", "", path.to_str().unwrap_or("")])
                    .spawn();
            }
        }
        ActionType::OpenPath(path) => {
            #[cfg(target_os = "linux")]
            {
                let _ = std::process::Command::new("xdg-open")
                    .arg(path)
                    .spawn();
            }
            
            #[cfg(target_os = "windows")]
            {
                let _ = std::process::Command::new("explorer")
                    .arg(path)
                    .spawn();
            }
        }
        ActionType::OpenUrl(url) => {
            let mut url_to_open = url.clone();
            if !url.starts_with("http://") && !url.starts_with("https://") {
                url_to_open = format!("https://{}", url);
            }
            let _ = webbrowser::open(&url_to_open);
        }
        ActionType::MathResult(result) => {
            // Clipboard implementation
            println!("Math result: {}", result);
        }
        ActionType::WebSearch(query) => {
            let search_url = format!("https://www.google.com/search?q={}", 
                helpers::encode(query));
            let _ = webbrowser::open(&search_url);
        }
    }
}