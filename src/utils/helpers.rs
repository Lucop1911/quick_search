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
        // Try to evaluate as math expression
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
}