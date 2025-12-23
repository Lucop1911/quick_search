use crate::utils::{window_manger::WindowManagerBackend, wm_integrations::generic_x11::GenericX11Backend};
use anyhow::Result;

// Not tested
pub struct QtileBackend {
    x11: GenericX11Backend,
}

impl QtileBackend {
    pub fn new() -> Self {
        Self {
            x11: GenericX11Backend::new(),
        }
    }
}

impl WindowManagerBackend for QtileBackend {
    fn find_window(&self, title: &str) -> Option<String> {
        self.x11.find_window(title)
    }

    fn make_float(&self, window_id: &str) -> Result<()> {
        self.x11.make_float(window_id)
    }

    fn pin_to_all_workspaces(&self, window_id: &str) -> Result<()> {
        self.x11.pin_to_all_workspaces(window_id)
    }

    fn focus_window(&self, window_id: &str) -> Result<()> {
        self.x11.focus_window(window_id)
    }

    fn move_to_position(&self, window_id: &str, x: i32, y: i32) -> Result<()> {
        self.x11.move_to_position(window_id, x, y)
    }

    fn get_screen_dimensions(&self) -> Option<(i32, i32)> {
        self.x11.get_screen_dimensions()
    }
}
