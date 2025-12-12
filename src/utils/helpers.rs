pub mod helpers {
    pub fn encode(s: &str) -> String {
        s.chars()
            .map(|c| match c {
                'A'..='Z' | 'a'..='z' | '0'..='9' | '-' | '_' | '.' | '~' => c.to_string(),
                ' ' => "+".to_string(),
                _ => format!("%{:02X}", c as u8),
            })
            .collect()
    }
    
    pub fn evaluate_math(expr: &str) -> Option<String> {
        match meval::eval_str(expr) {
            Ok(result) => {
                // Check if the result is meaningful (not just the input)
                if result.is_finite() && !expr.chars().all(|c| c.is_numeric() || c == '.') {
                    Some(format!("{:.6}", result).trim_end_matches('0').trim_end_matches('.').to_string())
                } else {
                    None
                }
            }
            Err(_) => None,
        }
    }

    pub fn is_url(text: &str) -> bool {
        text.starts_with("http://") || 
        text.starts_with("https://") || 
        text.starts_with("www.") ||
        (text.contains('.') && text.split('.').count() >= 2 && !text.contains(' '))
    }

    use arboard::Clipboard;
    pub fn copy_to_clipboard(text: &str) {
        if std::env::var("WAYLAND_DISPLAY").is_ok() {
            // Wayland
            use std::process::{Command, Stdio};
            use std::io::Write;
            
            match Command::new("wl-copy")
                .stdin(Stdio::piped())
                .spawn()
            {
                Ok(mut child) => {
                    if let Some(mut stdin) = child.stdin.take() {
                        let _ = stdin.write_all(text.as_bytes());
                    }
                }
                Err(e) => eprintln!("Failed to copy (Wayland): {}", e),
            }
        } else {
            // X11
            match Clipboard::new() {
                Ok(mut clipboard) => {
                    if let Err(err) = clipboard.set_text(text.to_string()) {
                        eprintln!("Failed to set clipboard text (X11): {}", err);
                    }
                }
                Err(err) => eprintln!("Failed to access clipboard (X11): {}", err),
            }
        }
    }
}