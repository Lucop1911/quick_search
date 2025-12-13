use crate::utils::{
    helpers::helpers::{self, copy_to_clipboard}, paths::open_path_intelligently, run_commands::run_command, utils::{ActionType, SearchResult}
};

pub fn execute_action(result: &SearchResult, query: &str) {
    // Save to history (except for special commands)
    let should_save_history = !matches!(
        result.action,
        ActionType::OpenHistory | ActionType::OpenSettings | ActionType::OpenInfo
    );

    if should_save_history {
        use crate::utils::history_manager::{HistoryEntry, HistoryManager};
        let manager = HistoryManager::new();
        let entry = HistoryEntry::from_search(query, result);
        manager.add_entry(entry);
    }

    match &result.action {
        ActionType::OpenHistory => {
            let exe_path = std::env::current_exe().ok();
            if let Some(exe) = exe_path {
                let _ = std::process::Command::new(exe).arg("--history").spawn();
            }
        }
        ActionType::OpenSettings => {
            let exe_path = std::env::current_exe().ok();
            if let Some(exe) = exe_path {
                let _ = std::process::Command::new(exe).arg("--settings").spawn();
            }
        }
        ActionType::OpenInfo => {
            let exe_path = std::env::current_exe().ok();
            if let Some(exe) = exe_path {
                let _ = std::process::Command::new(exe).arg("--info").spawn();
            }
        }
        ActionType::OpenApp(path) => {
            let path_str = path.to_string_lossy();
            // Extract the executable from the Exec line
            let exec_parts: Vec<&str> = path_str.split_whitespace().collect();
            if let Some(exec) = exec_parts.first() {
                let _ = std::process::Command::new(exec).spawn();
            }
        }
        ActionType::OpenPath(path) => {
            open_path_intelligently(path);
        }
        ActionType::OpenUrl(url) => {
            let mut url_to_open = url.clone();
            if !url.starts_with("http://") && !url.starts_with("https://") {
                url_to_open = format!("https://{}", url);
            }
            let _ = webbrowser::open(&url_to_open);
        }
        ActionType::MathResult(result) => {
            copy_to_clipboard(&result);
            println!("Math result: {}", result);
        }
        ActionType::WebSearch(query) => {
            let search_url = format!("https://www.google.com/search?q={}", helpers::encode(query));
            let _ = webbrowser::open(&search_url);
        }
        ActionType::RunCommand(command) => {
            let _ = run_command(command);
        }
    }
}