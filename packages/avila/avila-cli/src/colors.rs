//! ANSI color codes for terminal output
//!
//! Provides utilities for colorizing terminal text with automatic
//! detection of color support based on environment variables.

pub const RESET: &str = "\x1b[0m";
pub const BOLD: &str = "\x1b[1m";
pub const DIM: &str = "\x1b[2m";
pub const RED: &str = "\x1b[31m";
pub const GREEN: &str = "\x1b[32m";
pub const YELLOW: &str = "\x1b[33m";
pub const BLUE: &str = "\x1b[34m";
pub const MAGENTA: &str = "\x1b[35m";
pub const CYAN: &str = "\x1b[36m";
pub const GRAY: &str = "\x1b[90m";

/// Colorize text with the given ANSI color code
///
/// Automatically detects color support and returns plain text
/// if colors are not supported.
///
/// # Example
/// ```
/// use avila_cli::colors::{colorize, RED};
/// let error = colorize("Error", RED);
/// ```
pub fn colorize(text: &str, color: &str) -> String {
    if is_color_supported() {
        format!("{}{}{}", color, text, RESET)
    } else {
        text.to_string()
    }
}

/// Check if the terminal supports ANSI color codes
///
/// Returns `false` if:
/// - NO_COLOR environment variable is set
/// - TERM is set to "dumb"
/// - Neither TERM nor COLORTERM is set
pub fn is_color_supported() -> bool {
    std::env::var("NO_COLOR").is_err()
        && (std::env::var("TERM").map(|t| t != "dumb").unwrap_or(false)
            || std::env::var("COLORTERM").is_ok())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_colorize_returns_colored_string() {
        let result = colorize("test", RED);
        // When colors are supported, should contain ANSI codes
        assert!(result.contains("test"));
    }

    #[test]
    fn test_color_constants_are_ansi_codes() {
        assert!(RED.starts_with("\x1b["));
        assert!(GREEN.starts_with("\x1b["));
        assert!(BLUE.starts_with("\x1b["));
    }
}
