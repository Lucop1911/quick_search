use std::path::Path;

use crate::utils::utils::SearchResult;

pub fn get_applications(query: &str) -> Vec<SearchResult> {
    let mut results = Vec::new();
    use walkdir::WalkDir;
    // Search in common Linux app dirs
    let app_dirs = vec![
        "/usr/share/applications",
        "/usr/local/share/applications",
    ];
    
    if let Some(home) = dirs::home_dir() {
        let local_apps = home.join(".local/share/applications");
        if local_apps.exists() {
            for entry in WalkDir::new(local_apps).max_depth(1) {
                if let Ok(entry) = entry {
                    if let Some(result) = parse_desktop_file(&entry.path(), query) {
                        results.push(result);
                    }
                }
            }
        }
    }
    
    for dir in app_dirs {
        use std::path::Path;

        if Path::new(dir).exists() {
            for entry in WalkDir::new(dir).max_depth(1) {
                if let Ok(entry) = entry {
                    if let Some(result) = parse_desktop_file(&entry.path(), query) {
                        results.push(result);
                    }
                }
            }
        }
    }
    results.truncate(5);
    results
}

pub fn parse_desktop_file(path: &Path, query: &str) -> Option<SearchResult> {
    use std::fs;

    if path.extension()? != "desktop" {
        return None;
    }
    
    let content = fs::read_to_string(path).ok()?;
    let mut name = None;
    let mut exec = None;
    let mut no_display = false;
    let mut hidden = false;
    
    for line in content.lines() {
        if line.starts_with("Name=") && name.is_none() {
            name = Some(line[5..].to_string());
        } else if line.starts_with("Exec=") {
            exec = Some(line[5..].to_string());
        } else if line.starts_with("NoDisplay=true") {
            no_display = true;
        } else if line.starts_with("Hidden=true") {
            hidden = true;
        }
    }
    
    // Skip hidden or no-display apps
    if no_display || hidden {
        return None;
    }
    
    if let (Some(app_name), Some(exec_cmd)) = (name, exec) {
        if app_name.to_lowercase().contains(query) {
            use std::path::PathBuf;
            use crate::utils::utils::{ActionType, SearchResult};

            let exec_parts: Vec<&str> = exec_cmd.split_whitespace().collect();
            if let Some(exec_bin) = exec_parts.first() {
                // Clean up field codes like %U, %F, etc.
                let exec_clean = exec_bin.trim_matches('"');
                
                // Check if it's an absolute path that exists
                if Path::new(exec_clean).is_absolute() {
                    use std::path::Path;

                    if !Path::new(exec_clean).exists() {
                        return None;
                    }
                } else {
                    // Check if command is in PATH
                    if !is_command_available(exec_clean) {
                        return None;
                    }
                }
            }
            
            return Some(SearchResult {
                title: app_name.clone(),
                subtitle: "Application".to_string(),
                icon: "[APP]".to_string(),
                action: ActionType::OpenApp(PathBuf::from(exec_cmd)),
            });
        }
    }
    
    None
}

pub fn is_command_available(cmd: &str) -> bool {
    std::process::Command::new("which")
        .arg(cmd)
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false)
}