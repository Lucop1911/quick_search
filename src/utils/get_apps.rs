use crate::utils::utils::SearchResult;

pub fn get_applications(query: &str) -> Vec<SearchResult> {
    let mut results = Vec::new();
    
    #[cfg(target_os = "linux")]
    
    {
        use walkdir::WalkDir;
        // Search in common Linux application directories
        let app_dirs = vec![
            "/usr/share/applications",
            "/usr/local/share/applications",
        ];
        
        if let Some(home) = dirs::home_dir() {
            let local_apps = home.join(".local/share/applications");
            if local_apps.exists() {
                for entry in WalkDir::new(local_apps).max_depth(1) {
                    if let Ok(entry) = entry {
                        use crate::utils::linux::parse_desktop_file;

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
                        use crate::utils::linux::parse_desktop_file;

                        if let Some(result) = parse_desktop_file(&entry.path(), query) {
                            results.push(result);
                        }
                    }
                }
            }
        }
    }
    
    #[cfg(target_os = "windows")]
    {
        use crate::utils::windows::search_windows_shortcuts;
        // Search Windows Start Menu
        let start_menu_paths = vec![
            r"C:\ProgramData\Microsoft\Windows\Start Menu\Programs",
        ];
        
        if let Some(home) = dirs::home_dir() {
            let user_start = home.join(r"AppData\Roaming\Microsoft\Windows\Start Menu\Programs");
            if user_start.exists() {
                search_windows_shortcuts(&user_start, query, &mut results);
            }
        }
        
        for path in start_menu_paths {
            use std::path::Path;

            if Path::new(path).exists() {
                search_windows_shortcuts(Path::new(path), query, &mut results);
            }
        }
    }
    
    results.truncate(5);
    results
}