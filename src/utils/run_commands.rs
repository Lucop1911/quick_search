use std::process::Command;

use anyhow::{Ok, Result};

use crate::utils::{helpers::helpers::is_command_available, settings_manager::SettingsManager};

pub fn run_command(command: &str) -> Result<()> {
    let settings_manager = SettingsManager::new();
    let settings = settings_manager.load_settings();
    let shell = std::env::var("SHELL").unwrap_or_else(|_| "/bin/bash".into());

    let shell_cmd = format!("{command}; exec {shell}");

    // User-defined terminal
    if !settings.terminal_command.is_empty() {
        Command::new(&settings.terminal_command)
            .arg("-e")
            .arg(&shell)
            .arg("-c")
            .arg(&shell_cmd)
            .spawn()?;
        return Ok(());
    }

    const TERMINALS: &[(&str, &[&str])] = &[
        ("kitty", &["-e"]),
        ("alacritty", &["-e"]),
        ("wezterm", &["start"]),
        ("foot", &[""]),
        ("footclient", &[""]),
        ("gnome-terminal", &["--"]),
        ("konsole", &["-e"]),
        ("xterm", &["-e"]),
        ("lxterminal", &["-e"]),
        ("xfce4-terminal", &["-e"]),
        ("tilix", &["-e"]),
        ("terminator", &["-x"]),
        ("tilda", &["-c"]),
        ("urxvt", &["-e"]),
        ("st", &["-e"]),
        ("eterm", &["-e"]),
        ("deepin-terminal", &["-e"]),
        ("mate-terminal", &["-e"]),
        ("qterminal", &["-e"]),
        ("sakura", &["-e"]),
        ("guake", &["-e"]),
    ];

    for (terminal, flags) in TERMINALS {
        if is_command_available(terminal) {
            Command::new(terminal)
                .args(*flags)
                .arg(&shell)
                .arg("-c")
                .arg(&shell_cmd)
                .spawn()?;

            return Ok(());
        }
    }

    Ok(())
}