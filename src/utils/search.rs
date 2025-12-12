use crate::utils::{get_apps::get_applications, helpers::helpers::{evaluate_math, is_url}, settings_manager::SettingsManager, utils::{ActionType, SearchResult}};
use crate::utils::check_path::check_path;

pub fn perform_search(query: &str) -> Vec<SearchResult> {
    let mut results = Vec::new();
    let query_lower = query.to_lowercase();

    let settings_manager = SettingsManager::new();
    let settings = settings_manager.load_settings();
    
    // Handle special commands starting with @
    if query.starts_with('@') {
        let command = query[1..].to_lowercase();
        
        if command.contains("settings") || "settings".contains(&command) {
            results.push(SearchResult {
                title: "Settings".to_string(),
                subtitle: "Configure Quick Search".to_string(),
                icon: "⚙️".to_string(),
                action: ActionType::OpenSettings,
            });
        }
        
        if command.contains("info") || "info".contains(&command) {
            results.push(SearchResult {
                title: "Informations".to_string(),
                subtitle: "Application information".to_string(),
                icon: "ℹ️".to_string(),
                action: ActionType::OpenInfo,
            });
        }
        
        if command.contains("history") || "history".contains(&command) {
            results.push(SearchResult {
                title: "History".to_string(),
                subtitle: "View search history".to_string(),
                icon: "📜".to_string(),
                action: ActionType::OpenHistory,
            });
        }
        
        if !results.is_empty() {
            return results;
        }
    }

    // Check for math expression
    if settings.enable_math_eval == true {
        if let Some(math_result) = evaluate_math(query) {
            results.push(SearchResult {
                title: format!("= {}", math_result),
                subtitle: "Math calculation. Click or press Enter to copy".to_string(),
                icon: "🔢".to_string(),
                action: ActionType::MathResult(math_result),
            });
        }
    }

    // Check for URL
    if settings.enable_web_search == true {
        if is_url(query) {
            results.push(SearchResult {
                title: query.to_string(),
                subtitle: "Open URL".to_string(),
                icon: "🌐".to_string(),
                action: ActionType::OpenUrl(query.to_string()),
            });
        }
    }
    // Check for file path
    if settings.enable_file_search == true {
        if let Some(path_result) = check_path(query) {
            results.push(path_result);
        }
    }
    
    // Search for applications
    if settings.enable_app_search == true {
        let app_results = get_applications(&query_lower);
        results.extend(app_results);
    }
    
    // Add web search fallback if no other results
    if results.is_empty() || results.len() < 3 {
        results.push(SearchResult {
            title: format!("Search for \"{}\"", query),
            subtitle: "Search on the web".to_string(),
            icon: "🔍".to_string(),
            action: ActionType::WebSearch(query.to_string()),
        });
    }
    
    // Limit to top 8 results
    results.truncate(8);
    results
}