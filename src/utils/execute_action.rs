use crate::utils::{
    helpers::helpers::{self, copy_to_clipboard, is_command_available}, run_commands::run_command, settings_manager::SettingsManager, utils::{ActionType, SearchResult}
};
use std::path::Path;

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

fn open_path_intelligently(path: &Path) {
    // xdg-open for directories
    if path.is_dir() {
        let _ = std::process::Command::new("xdg-open")
            .arg(path)
            .spawn();
        return;
    }

    let settings_manager = SettingsManager::new();
    let settings = settings_manager.load_settings();

    if !settings.text_editor_command.is_empty() {
        if is_command_available(&settings.text_editor_command) {
            let _ = std::process::Command::new(settings.text_editor_command)
                .arg(path)
                .spawn();
            return;
        }
    }

    if let Some(ext) = path.extension() {
        let ext_str = ext.to_string_lossy().to_lowercase();
        
        let text_extensions = vec![
            // Programming languages
            "as", "bas", "c", "clj", "cljc", "cljs", "cpp", "cs", "dart", "edn",
            "erl", "ex", "exs", "fs", "fsi", "fsx", "fsscript", "go", "h",
            "hs", "hpp", "java", "jl", "js", "kts", "kt", "lua", "m", "mm",
            "nix", "php", "proto", "py", "r", "rb", "rs", "scala", "scm",
            "ss", "swift", "ts", "wat", "zig",

            // Markup & documentation
            "adoc", "asciidoc", "bib", "csv", "gmi", "gemini", "latex", "md",
            "mdx", "org", "rst", "tex", "tsv",

            // Web-related
            "astro", "css", "ejs", "hbs", "haml", "html", "jsx", "less",
            "liquid", "sass", "scss", "slim", "svelte", "tsx", "vue",

            // Templates
            "erb", "jinja", "jinja2", "j2", "mustache", "template", "tmpl",

            // Config & data formats
            "cfg", "cnf", "config", "env", "hcl", "ini", "ion", "json",
            "jsonl", "ndjson", "properties", "prop", "rc", "toml", "xml",
            "yaml", "yml",

            // Shells / system
            "bash", "bash_profile", "bashrc", "profile", "service",
            "service.in", "sh", "ssh_config", "unit", "zprofile", "zsh",
            "zshrc",

            // Scripts
            "awk", "bat", "cmd", "ps1", "psd1", "psm1", "sed",

            // Build systems / package manifests
            "bzl", "cargo", "cargo.lock", "cargo.toml", "cmake",
            "cmakelists", "compose", "gradle", "gradle.kts", "go.mod",
            "go.sum", "makefile", "mk", "npmignore", "pom", "sln",
            "vcxproj", "yarnrc",

            // DevOps / container / CI
            "dockerfile", "dockerignore", "editorconfig",
            "gitattributes", "gitignore", "gitmodules",

            // Infrastructure-as-code
            "nomad", "rego", "tf", "tfvars", "vault",

            // SQL & database
            "ddl", "dml", "psql", "sql",

            // Security
            "asc", "crt", "csr", "key", "pem", "pub",

            // Networking
            "ifcfg", "rules",

            // System definitions
            "boot", "conf", "conf.d", "mime.types", "mailcap",

            // Logs & misc text
            "list", "lit", "log", "lst", "note", "notes", "todo",

            // Graph formats
            "dot", "gv",

            // Query languages
            "gql", "graphql", "graphqls",

            // Misc dev formats
            "cson", "diff", "glsl", "hlsl", "lock", "proto", "thrift",
        ];

        if text_extensions.contains(&ext_str.as_str()) {
            let gui_editors = vec![
                "code", "code-oss", "codium", "subl", "sublime_text", "gedit", "kate" , "mousepad", "pluma", "xed",
                "leafpad", "geany", "anjuta", "bluefish", "atom", "nvim-qt", "gvim", "notepadqq", "xed", "micro",
                "lite-xl", "lapce", "zed", "helix-gtk",
            ];


            for editor in gui_editors {
                if is_command_available(editor) {
                    let _ = std::process::Command::new(editor)
                        .arg(path)
                        .spawn();
                    return;
                }
            }

            let terminals: Vec<(&str, &str)> = vec![
                ("kitty", "-e"),
                ("alacritty", "-e"),
                ("wezterm", "start"),
                ("foot", ""),
                ("footclient", ""),
                ("gnome-terminal", "--"),
                ("konsole", "-e"),
                ("xterm", "-e"),
                ("lxterminal", "-e"),
                ("xfce4-terminal", "-e"),
                ("tilix", "-e"),
                ("terminator", "-x"),
                ("tilda", "-c"),
                ("urxvt", "-e"),
                ("st", "-e"),
                ("eterm", "-e"),
                ("deepin-terminal", "-e"),
                ("mate-terminal", "-e"),
                ("qterminal", "-e"),
                ("sakura", "-e"),
                ("guake", "-e"),
            ];

            for (terminal, flag) in terminals {
                if is_command_available(terminal) {
                    let editor = get_terminal_editor();
                    
                    let mut cmd = std::process::Command::new(terminal);
                    if !flag.is_empty() {
                        cmd.arg(flag);
                    }
                    cmd.arg(&editor).arg(path);
                    
                    let _ = cmd.spawn();
                    return;
                }
            }
        }
    }

    let _ = std::process::Command::new("xdg-open")
        .arg(path)
        .spawn();
}

fn get_terminal_editor() -> String {
    if let Ok(editor) = std::env::var("EDITOR") {
        if !editor.is_empty() {
            return editor;
        }
    }

    let editors = vec!["nvim", "vim", "nano", "micro", "vi"];
    for editor in editors {
        if is_command_available(editor) {
            return editor.to_string();
        }
    }

    // Final fallback
    "nano".to_string()
}