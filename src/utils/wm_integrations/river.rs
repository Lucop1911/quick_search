use crate::utils::window_manger::WindowManagerBackend;
use anyhow::Result;
use std::process::Command;

// Not tested
pub struct RiverBackend;

impl RiverBackend {
    pub fn new() -> Self {
        Self
    }
}

impl WindowManagerBackend for RiverBackend {
    fn find_window(&self, title: &str) -> Option<String> {
        // River doesn't have great IPC yet, i'll try to use lswt (list wayland toplevels) if available
        let output = Command::new("lswt")
            .output()
            .ok()?;

        let windows = String::from_utf8_lossy(&output.stdout);
        for line in windows.lines() {
            if line.contains(title) {
                // Extract window ID from the line
                if let Some(id) = line.split_whitespace().next() {
                    return Some(id.to_string());
                }
            }
        }
        None
    }

    fn make_float(&self, _window_id: &str) -> Result<()> {
        // River doesn't support programmatic floating yet
        Ok(())
    }

    fn pin_to_all_workspaces(&self, _window_id: &str) -> Result<()> {
        // River doesn't support sticky windows yet
        Ok(())
    }

    fn focus_window(&self, _window_id: &str) -> Result<()> {
        // River doesn't have window focusing by ID yet
        Ok(())
    }

    fn move_to_position(&self, _window_id: &str, _x: i32, _y: i32) -> Result<()> {
        // River doesn't support precise window positioning yet
        Ok(())
    }

    fn get_screen_dimensions(&self) -> Option<(i32, i32)> {
        // No reliable way to get screen dimensions in River yet
        // Return a default size
        Some((1920, 1080))
    }
}