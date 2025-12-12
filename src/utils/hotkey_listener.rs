#[cfg(target_os = "windows")]
pub fn start_hotkey_listener() {
    use std::ptr::null_mut;
    use std::mem::zeroed;

    use winapi::shared::minwindef::UINT;
    use winapi::um::winuser::{RegisterHotKey, UnregisterHotKey, GetMessageW, TranslateMessage, DispatchMessageW, WM_HOTKEY, MSG, MOD_ALT};

    unsafe {
        let id: i32 = 1;
        // Register Alt+S as global hotkey
        if RegisterHotKey(null_mut(), id, MOD_ALT as UINT, 'S' as UINT) == 0 {
            eprintln!("Failed to register global hotkey (Alt+S)");
            return;
        }

        // Message loop waiting for WM_HOTKEY
        let mut msg: MSG = zeroed();
        loop {
            let ret = GetMessageW(&mut msg, null_mut(), 0, 0);
            if ret == 0 {
                break; // WM_QUIT
            }

            if msg.message == WM_HOTKEY {
                // Spawn a temporary window instance that does NOT become the resident listener
                if let Ok(exe) = std::env::current_exe() {
                    let _ = std::process::Command::new(exe)
                        .arg("--temp-window")
                        .spawn();
                }
            }

            TranslateMessage(&msg);
            DispatchMessageW(&msg);
        }

        let _ = UnregisterHotKey(null_mut(), id);
    }
}

#[cfg(target_os = "linux")]
#[allow(dead_code)]
pub fn start_hotkey_listener() {
    // No-op on non-Windows platforms
}
