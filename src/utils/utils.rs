use std::path::PathBuf;

#[derive(Clone, Debug)]
pub struct SearchResult {
    pub title: String,
    pub subtitle: String,
    pub icon: String,
    pub action: ActionType,
}

#[derive(Clone, Debug)]
pub enum ActionType {
    OpenSettings,
    OpenHistory,
    OpenInfo,
    OpenApp(PathBuf),
    OpenPath(PathBuf),
    OpenUrl(String),
    MathResult(String),
    WebSearch(String),
}