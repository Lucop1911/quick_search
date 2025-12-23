use crate::utils::window_manger::WindowManagerBackend;
use anyhow::Result;
use std::process::Command;

// Not tested
pub struct SwayBackend;

impl SwayBackend {
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

impl WindowManagerBackend for SwayBackend {
    fn find_window(&self, title: &str) -> Option<String> {
        let output = Command::new("swaymsg")
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
        Command::new("swaymsg")
            .arg(format!("[con_id=\"{}\"] floating enable", window_id))
            .output()?;
        Ok(())
    }

    fn pin_to_all_workspaces(&self, window_id: &str) -> Result<()> {
        Command::new("swaymsg")
            .arg(format!("[con_id=\"{}\"] sticky enable", window_id))
            .output()?;
        Ok(())
    }

    fn focus_window(&self, window_id: &str) -> Result<()> {
        Command::new("swaymsg")
            .arg(format!("[con_id=\"{}\"] focus", window_id))
            .output()?;
        Ok(())
    }

    fn move_to_position(&self, window_id: &str, x: i32, y: i32) -> Result<()> {
        Command::new("swaymsg")
            .arg(format!("[con_id=\"{}\"] move position {} {}", window_id, x, y))
            .output()?;
        Ok(())
    }

    fn get_screen_dimensions(&self) -> Option<(i32, i32)> {
        let output = Command::new("swaymsg")
            .args(["-t", "get_outputs"])
            .output()
            .ok()?;
        
        if !output.status.success() {
            return None;
        }

        let outputs: serde_json::Value = serde_json::from_slice(&output.stdout).ok()?;
        if let Some(output) = outputs.as_array()?.iter().find(|o| o["focused"].as_bool().unwrap_or(false)) {
            let rect = &output["rect"];
            let width = rect["width"].as_i64()? as i32;
            let height = rect["height"].as_i64()? as i32;
            return Some((width, height));
        }
        
        None
    }
}