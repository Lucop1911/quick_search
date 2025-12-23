use crate::utils::{window_manger::WindowManagerBackend};
use anyhow::Result;
use std::process::Command;

pub struct GenericX11Backend;

impl GenericX11Backend {
    pub fn new() -> Self {
        Self
    }
}

impl WindowManagerBackend for GenericX11Backend {
    fn find_window(&self, title: &str) -> Option<String> {
        let output = Command::new("xdotool")
            .args(["search", "--name", title])
            .output()
            .ok()?;

        if !output.status.success() {
            return None;
        }

        let window_id = String::from_utf8_lossy(&output.stdout)
            .lines()
            .next()?
            .to_string();
        
        Some(window_id)
    }

    fn make_float(&self, _window_id: &str) -> Result<()> {
        // Generic X11 doesn't have a standard way to make windows float
        // This depends on the WM's configuration
        Ok(())
    }

    fn pin_to_all_workspaces(&self, _window_id: &str) -> Result<()> {
        // Generic X11 doesn't have a standard way to pin windows
        // This depends on the WM's configuration
        Ok(())
    }

    fn focus_window(&self, window_id: &str) -> Result<()> {
        Command::new("xdotool")
            .args(["windowactivate", window_id])
            .output()?;
        Ok(())
    }

    fn move_to_position(&self, window_id: &str, x: i32, y: i32) -> Result<()> {
        Command::new("xdotool")
            .args(["windowmove", window_id, &x.to_string(), &y.to_string()])
            .output()?;
        Ok(())
    }

    fn get_screen_dimensions(&self) -> Option<(i32, i32)> {
        let output = Command::new("xdpyinfo")
            .output()
            .ok()?;
        
        let screen_info = String::from_utf8_lossy(&output.stdout);
        for line in screen_info.lines() {
            if line.contains("dimensions:") {
                if let Some(dims) = line.split_whitespace().nth(1) {
                    let parts: Vec<&str> = dims.split('x').collect();
                    if parts.len() == 2 {
                        let width = parts[0].parse::<i32>().ok()?;
                        let height = parts[1].parse::<i32>().ok()?;
                        return Some((width, height));
                    }
                }
            }
        }
        None
    }
}
