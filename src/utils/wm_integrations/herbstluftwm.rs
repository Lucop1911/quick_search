use crate::utils::{window_manger::WindowManagerBackend};
use anyhow::Result;
use std::process::Command;

pub struct HerbstluftwmBackend;

impl HerbstluftwmBackend {
    pub fn new() -> Self {
        Self
    }
}

impl WindowManagerBackend for HerbstluftwmBackend {
    fn find_window(&self, title: &str) -> Option<String> {
        let output = Command::new("herbstclient")
            .args(["list_clients"])
            .output()
            .ok()?;

        let clients = String::from_utf8_lossy(&output.stdout);
        
        for line in clients.lines() {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if let Some(id) = parts.first() {
                let title_output = Command::new("xdotool")
                    .args(["getwindowname", id])
                    .output()
                    .ok()?;
                
                let window_title = String::from_utf8_lossy(&title_output.stdout);
                if window_title.contains(title) {
                    return Some(id.to_string());
                }
            }
        }
        None
    }

    fn make_float(&self, window_id: &str) -> Result<()> {
        Command::new("herbstclient")
            .args(["set_attr", &format!("clients.{}.floating", window_id), "true"])
            .output()?;
        Ok(())
    }

    fn pin_to_all_workspaces(&self, window_id: &str) -> Result<()> {
        Command::new("herbstclient")
            .args(["set_attr", &format!("clients.{}.sticky", window_id), "true"])
            .output()?;
        Ok(())
    }

    fn focus_window(&self, window_id: &str) -> Result<()> {
        Command::new("herbstclient")
            .args(["jumpto", window_id])
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
        let output = Command::new("herbstclient")
            .args(["monitor_rect", ""])
            .output()
            .ok()?;
        
        let rect = String::from_utf8_lossy(&output.stdout);
        let parts: Vec<&str> = rect.split_whitespace().collect();
        
        if parts.len() >= 4 {
            let width = parts[2].parse::<i32>().ok()?;
            let height = parts[3].parse::<i32>().ok()?;
            return Some((width, height));
        }
        
        None
    }
}