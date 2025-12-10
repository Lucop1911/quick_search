#[cfg(target_os = "windows")]
use std::path::Path;

#[cfg(target_os = "windows")]
use crate::utils::utils::SearchResult;

#[cfg(target_os = "windows")]
pub fn search_windows_shortcuts(dir: &Path, query: &str, results: &mut Vec<SearchResult>) {
    use walkdir::WalkDir;

    for entry in WalkDir::new(dir).max_depth(3) {
        if let Ok(entry) = entry {
            let path = entry.path();
            if let Some(ext) = path.extension() {
                if ext == "lnk" {
                    if let Some(name) = path.file_stem() {
                        let name_str = name.to_string_lossy().to_lowercase();
                        if name_str.contains(query) {
                            use crate::utils::utils::{ActionType, SearchResult};

                            results.push(SearchResult {
                                title: name.to_string_lossy().to_string(),
                                subtitle: "Application".to_string(),
                                icon: "📱".to_string(),
                                action: ActionType::OpenApp(path.to_path_buf()),
                            });
                        }
                    }
                }
            }
        }
    }
}