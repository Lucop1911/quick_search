use hyprland::data::Clients;
use hyprland::dispatch::{Dispatch, DispatchType, WindowIdentifier};
use hyprland::prelude::*;
use hyprland::shared::Address;
use std::process::Command;

pub struct HyprlandIntegration {
    window_address: Option<Address>,
}

impl HyprlandIntegration {
    pub fn new() -> Self {
        Self {
            window_address: None,
        }
    }

    pub fn find_window_by_title(&mut self, title: &str) -> Option<Address> {
        if let Ok(clients) = Clients::get() {
            for client in clients {
                if client.title.contains(title) || client.class.contains("quick_search") {
                    self.window_address = Some(client.address.clone());
                    return Some(client.address);
                }
            }
        }
        None
    }

    pub fn make_float(&self) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(addr) = &self.window_address {
            let identifier = WindowIdentifier::Address(addr.clone());
            Dispatch::call(DispatchType::ToggleFloating(Some(identifier)))?;
        }
        Ok(())
    }

    pub fn pin_to_all_workspaces(&self) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(addr) = &self.window_address {
            let addr_str = format!("{}", addr);
            Command::new("hyprctl")
                .args(["dispatch", "pin", &format!("address:{}", addr_str)])
                .output()?;
        }
        Ok(())
    }

    pub fn focus_window(&self) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(addr) = &self.window_address {
            let identifier = WindowIdentifier::Address(addr.clone());
            Dispatch::call(DispatchType::FocusWindow(identifier))?;
        }
        Ok(())
    }

    pub fn _center_window(&self) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(addr) = &self.window_address {
            let addr_str = format!("{}", addr);
            Command::new("hyprctl")
                .args(["dispatch", "centerwindow", &format!("address:{}", addr_str)])
                .output()?;
        }
        Ok(())
    }

    pub fn move_window_to_top_center(&self, width: i32) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(addr) = &self.window_address {
            let output = Command::new("hyprctl")
                .args(["monitors", "-j"])
                .output()?;
            
            if output.status.success() {
                let monitors: serde_json::Value = serde_json::from_slice(&output.stdout)?;
                if let Some(monitor) = monitors.as_array().and_then(|arr| arr.first()) {
                    if let Some(mon_width) = monitor["width"].as_i64() {
                        let x_pos = (mon_width as i32 - width) / 2;
                        let y_pos = 20;
                        
                        let addr_str = format!("{}", addr);
                        Command::new("hyprctl")
                            .args(["dispatch", "movewindowpixel", &format!("exact {} {},address:{}", x_pos, y_pos, addr_str)])
                            .output()?;
                    }
                }
            }
        }
        Ok(())
    }
    
    pub fn setup_launcher_window(&mut self, title: &str, width: i32) -> Result<(), Box<dyn std::error::Error>> {
        std::thread::sleep(std::time::Duration::from_millis(100));
        
        if self.find_window_by_title(title).is_some() {
            self.make_float()?;
            self.pin_to_all_workspaces()?;
            self.move_window_to_top_center(width)?;
            self.focus_window()?;
        }
        
        Ok(())
    }
}