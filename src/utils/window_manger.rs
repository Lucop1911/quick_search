use anyhow::Result;
use std::process::Command;

use crate::utils::wm_integrations::{awesome, bspwm, dwm, generic_x11, herbstluftwm, hyprland, i3, leftwm, qtile, river, sway, xmonad};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum WindowManager {
    Hyprland,
    I3,
    Sway,
    Bspwm,
    Qtile,
    Awesome,
    Xmonad,
    Dwm,
    Herbstluftwm,
    Leftwm,
    River,
    Unknown,
}

pub trait WindowManagerBackend {
    fn find_window(&self, title: &str) -> Option<String>;
    fn make_float(&self, window_id: &str) -> Result<()>;
    fn pin_to_all_workspaces(&self, window_id: &str) -> Result<()>;
    fn focus_window(&self, window_id: &str) -> Result<()>;
    fn move_to_position(&self, window_id: &str, x: i32, y: i32) -> Result<()>;
    fn get_screen_dimensions(&self) -> Option<(i32, i32)>;
}

pub struct WindowManagerIntegration {
    _wm: WindowManager,
    backend: Box<dyn WindowManagerBackend>,
    window_id: Option<String>,
}

impl WindowManagerIntegration {
    pub fn new() -> Self {
        let _wm = Self::detect_window_manager();
        let backend = Self::create_backend(_wm);
        
        Self {
            _wm,
            backend,
            window_id: None,
        }
    }

    fn detect_window_manager() -> WindowManager {
        // I check env variables first
        if std::env::var("HYPRLAND_INSTANCE_SIGNATURE").is_ok() {
            return WindowManager::Hyprland;
        }
        
        if let Ok(desktop) = std::env::var("XDG_CURRENT_DESKTOP") {
            let desktop_lower = desktop.to_lowercase();
            if desktop_lower.contains("sway") {
                return WindowManager::Sway;
            }
            if desktop_lower.contains("i3") {
                return WindowManager::I3;
            }
        }

        if let Ok(session) = std::env::var("DESKTOP_SESSION") {
            let session_lower = session.to_lowercase();
            if session_lower.contains("i3") {
                return WindowManager::I3;
            }
            if session_lower.contains("sway") {
                return WindowManager::Sway;
            }
            if session_lower.contains("bspwm") {
                return WindowManager::Bspwm;
            }
            if session_lower.contains("qtile") {
                return WindowManager::Qtile;
            }
            if session_lower.contains("awesome") {
                return WindowManager::Awesome;
            }
        }

        // If nothing is found i'll check for processes
        let wm_processes = vec![
            ("hyprland", WindowManager::Hyprland),
            ("sway", WindowManager::Sway),
            ("i3", WindowManager::I3),
            ("bspwm", WindowManager::Bspwm),
            ("qtile", WindowManager::Qtile),
            ("awesome", WindowManager::Awesome),
            ("xmonad", WindowManager::Xmonad),
            ("dwm", WindowManager::Dwm),
            ("herbstluftwm", WindowManager::Herbstluftwm),
            ("leftwm", WindowManager::Leftwm),
            ("river", WindowManager::River),
        ];

        for (process_name, wm_type) in wm_processes {
            if Self::is_process_running(process_name) {
                return wm_type;
            }
        }

        WindowManager::Unknown
    }

    fn is_process_running(name: &str) -> bool {
        Command::new("pgrep")
            .arg("-x")
            .arg(name)
            .output()
            .map(|output| output.status.success())
            .unwrap_or(false)
    }

    fn create_backend(wm: WindowManager) -> Box<dyn WindowManagerBackend> {
        match wm {
            WindowManager::Hyprland => Box::new(hyprland::HyprlandBackend::new()),
            WindowManager::I3 => Box::new(i3::I3Backend::new()),
            WindowManager::Sway => Box::new(sway::SwayBackend::new()),
            WindowManager::Bspwm => Box::new(bspwm::BspwmBackend::new()),
            WindowManager::Qtile => Box::new(qtile::QtileBackend::new()),
            WindowManager::Awesome => Box::new(awesome::AwesomeBackend::new()),
            WindowManager::Xmonad => Box::new(xmonad::XmonadBackend::new()),
            WindowManager::Dwm => Box::new(dwm::DwmBackend::new()),
            WindowManager::Herbstluftwm => Box::new(herbstluftwm::HerbstluftwmBackend::new()),
            WindowManager::Leftwm => Box::new(leftwm::LeftwmBackend::new()),
            WindowManager::River => Box::new(river::RiverBackend::new()),
            WindowManager::Unknown => Box::new(generic_x11::GenericX11Backend::new()),
        }
    }

    pub fn find_window_by_title(&mut self, title: &str) -> Result<()> {
        self.window_id = self.backend.find_window(title);
        Ok(())
    }

    pub fn make_float(&self) -> Result<()> {
        if let Some(ref window_id) = self.window_id {
            self.backend.make_float(window_id)?;
        }
        Ok(())
    }

    pub fn pin_to_all_workspaces(&self) -> Result<()> {
        if let Some(ref window_id) = self.window_id {
            self.backend.pin_to_all_workspaces(window_id)?;
        }
        Ok(())
    }

    pub fn focus_window(&self) -> Result<()> {
        if let Some(ref window_id) = self.window_id {
            self.backend.focus_window(window_id)?;
        }
        Ok(())
    }

    pub fn move_window_to_top_center(&self, width: i32) -> Result<()> {
        if let Some(ref window_id) = self.window_id {
            if let Some((screen_width, _)) = self.backend.get_screen_dimensions() {
                let x_pos = (screen_width - width) / 2;
                let y_pos = 20;
                self.backend.move_to_position(window_id, x_pos, y_pos)?;
            }
        }
        Ok(())
    }

    pub fn setup_launcher_window(&mut self, title: &str, width: i32) -> Result<()> {
        std::thread::sleep(std::time::Duration::from_millis(150));
        
        self.find_window_by_title(title)?;
        
        if self.window_id.is_some() {
            let _ = self.make_float();
            let _ = self.pin_to_all_workspaces();
            let _ = self.move_window_to_top_center(width);
            let _ = self.focus_window();
        }
        
        Ok(())
    }
}