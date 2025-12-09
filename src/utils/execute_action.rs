use crate::utils::{helpers::helpers, utils::{ActionType, SearchResult}};

pub fn execute_action(result: &SearchResult) {
    match &result.action {
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
            // Copy to clipboard (would need a clipboard crate in real implementation)
            println!("Math result: {}", result);
        }
        ActionType::WebSearch(query) => {
            let search_url = format!("https://www.google.com/search?q={}", 
                helpers::encode(query));
            let _ = webbrowser::open(&search_url);
        }
    }
}