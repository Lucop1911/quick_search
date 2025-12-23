use crate::utils::{window_manger::WindowManagerBackend};
use anyhow::Result;
use std::process::Command;

// Not tested
pub struct I3Backend;

impl I3Backend {
    pub fn new() -> Self {
        Self
    }

    fn find_window_in_tree(node: &serde_json::Value, title: &str) -> Option<String> {
        if let Some(window_title) = node["name"].as_str() {
            if window_title.contains(title) {
                if let Some(id) = node["id"].as_i64() {
                    return Some(id.to_string());
                }
            }
        }

        if let Some(nodes) = node["nodes"].as_array() {
            for child in nodes {
                if let Some(id) = Self::find_window_in_tree(child, title) {
                    return Some(id);
                }
            }
        }

        if let Some(floating_nodes) = node["floating_nodes"].as_array() {
            for child in floating_nodes {
                if let Some(id) = Self::find_window_in_tree(child, title) {
                    return Some(id);
                }
            }
        }

        None
    }
}

impl WindowManagerBackend for I3Backend {
    fn find_window(&self, title: &str) -> Option<String> {
        let output = Command::new("i3-msg")
            .args(["-t", "get_tree"])
            .output()
            .ok()?;

        if !output.status.success() {
            return None;
        }

        let tree: serde_json::Value = serde_json::from_slice(&output.stdout).ok()?;
        Self::find_window_in_tree(&tree, title)
    }

    fn make_float(&self, window_id: &str) -> Result<()> {
        Command::new("i3-msg")
            .arg(format!("[con_id=\"{}\"] floating enable", window_id))
            .output()?;
        Ok(())
    }

    fn pin_to_all_workspaces(&self, window_id: &str) -> Result<()> {
        Command::new("i3-msg")
            .arg(format!("[con_id=\"{}\"] sticky enable", window_id))
            .output()?;
        Ok(())
    }

    fn focus_window(&self, window_id: &str) -> Result<()> {
        Command::new("i3-msg")
            .arg(format!("[con_id=\"{}\"] focus", window_id))
            .output()?;
        Ok(())
    }

    fn move_to_position(&self, window_id: &str, x: i32, y: i32) -> Result<()> {
        Command::new("i3-msg")
            .arg(format!("[con_id=\"{}\"] move position {} {}", window_id, x, y))
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