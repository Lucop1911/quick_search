use crate::utils::{get_apps::get_applications, helpers::helpers::{evaluate_math, is_url}, utils::{ActionType, SearchResult}};
use crate::utils::check_path::check_path;

pub fn perform_search(query: &str) -> Vec<SearchResult> {
    let mut results = Vec::new();
    let query_lower = query.to_lowercase();
    
    // Check for math expression
    if let Some(math_result) = evaluate_math(query) {
        results.push(SearchResult {
            title: format!("= {}", math_result),
            subtitle: "Math calculation".to_string(),
            icon: "🔢".to_string(),
            action: ActionType::MathResult(math_result),
        });
    }
    
    // Check for URL
    if is_url(query) {
        results.push(SearchResult {
            title: query.to_string(),
            subtitle: "Open URL".to_string(),
            icon: "🌐".to_string(),
            action: ActionType::OpenUrl(query.to_string()),
        });
    }
    
    // Check for file path
    if let Some(path_result) = check_path(query) {
        results.push(path_result);
    }
    
    // Search for applications
    let app_results = get_applications(&query_lower);
    results.extend(app_results);
    
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