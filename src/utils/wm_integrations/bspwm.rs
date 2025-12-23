use crate::utils::{window_manger::WindowManagerBackend};
use anyhow::Result;
use std::process::Command;

// Not tested
pub struct BspwmBackend;

impl BspwmBackend {
    pub fn new() -> Self {
        Self
    }
}

impl WindowManagerBackend for BspwmBackend {
    fn find_window(&self, title: &str) -> Option<String> {
        let output = Command::new("bspc")
            .args(["query", "-N", "-n", ".window"])
            .output()
            .ok()?;

        let window_ids = String::from_utf8_lossy(&output.stdout);
        
        for id in window_ids.lines() {
            let name_output = Command::new("xdotool")
                .args(["getwindowname", id])
                .output()
                .ok()?;
            
            let window_title = String::from_utf8_lossy(&name_output.stdout);
            if window_title.contains(title) {
                return Some(id.to_string());
            }
        }
        None
    }

    fn make_float(&self, window_id: &str) -> Result<()> {
        Command::new("bspc")
            .args(["node", window_id, "-t", "floating"])
            .output()?;
        Ok(())
    }

    fn pin_to_all_workspaces(&self, window_id: &str) -> Result<()> {
        Command::new("bspc")
            .args(["node", window_id, "-g", "sticky=on"])
            .output()?;
        Ok(())
    }

    fn focus_window(&self, window_id: &str) -> Result<()> {
        Command::new("bspc")
            .args(["node", window_id, "-f"])
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