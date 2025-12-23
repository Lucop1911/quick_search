use crate::utils::{window_manger::WindowManagerBackend};
use anyhow::Result;
use std::process::Command;

pub struct HyprlandBackend;

impl HyprlandBackend {
    pub fn new() -> Self {
        Self
    }
}

impl WindowManagerBackend for HyprlandBackend {
    fn find_window(&self, title: &str) -> Option<String> {
        let output = Command::new("hyprctl")
            .args(["clients", "-j"])
            .output()
            .ok()?;

        if !output.status.success() {
            return None;
        }

        let clients: serde_json::Value = serde_json::from_slice(&output.stdout).ok()?;
        
        for client in clients.as_array()? {
            let client_title = client["title"].as_str()?;
            if client_title.contains(title) {
                return Some(client["address"].as_str()?.to_string());
            }
        }
        None
    }

    fn make_float(&self, window_id: &str) -> Result<()> {
        Command::new("hyprctl")
            .args(["dispatch", "togglefloating", &format!("address:{}", window_id)])
            .output()?;
        Ok(())
    }

    fn pin_to_all_workspaces(&self, window_id: &str) -> Result<()> {
        Command::new("hyprctl")
            .args(["dispatch", "pin", &format!("address:{}", window_id)])
            .output()?;
        Ok(())
    }

    fn focus_window(&self, window_id: &str) -> Result<()> {
        Command::new("hyprctl")
            .args(["dispatch", "focuswindow", &format!("address:{}", window_id)])
            .output()?;
        Ok(())
    }

    fn move_to_position(&self, window_id: &str, x: i32, y: i32) -> Result<()> {
        Command::new("hyprctl")
            .args(["dispatch", "movewindowpixel", 
                   &format!("exact {} {},address:{}", x, y, window_id)])
            .output()?;
        Ok(())
    }

    fn get_screen_dimensions(&self) -> Option<(i32, i32)> {
        let output = Command::new("hyprctl")
            .args(["monitors", "-j"])
            .output()
            .ok()?;
        
        if !output.status.success() {
            return None;
        }

        let monitors: serde_json::Value = serde_json::from_slice(&output.stdout).ok()?;
        if let Some(monitor) = monitors.as_array()?.first() {
            let width = monitor["width"].as_i64()? as i32;
            let height = monitor["height"].as_i64()? as i32;
            return Some((width, height));
        }
        
        None
    }
}