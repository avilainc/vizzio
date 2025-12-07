//! Ávila CLI Parser - Ultra-Optimized v1.0.0
//!
//! Zero-dependency command-line argument parser with advanced features.
//! Provides compile-time type safety, constant-time lookups, and professional-grade features.
//!
//! Features:
//! - Zero dependencies (pure Rust std)
//! - Colored output (ANSI escape codes)
//! - Shell completion generation (bash, zsh, fish, powershell)
//! - Argument groups and validation
//! - Custom validators
//! - Environment variable fallback
//! - Config file parsing (TOML-like)
//! - Lazy evaluation
//! - Macro helpers for rapid development
//! - Performance optimized (O(1) lookups, zero-copy parsing)

// Public modules
pub mod colors;
pub mod arg;
pub mod app;
pub mod matches;
pub mod completion;
pub mod config;
pub mod validation;
pub mod error;
pub mod suggestions;
pub mod help;

// Macro module (macros need to be at crate root)
#[macro_use]
mod macros;

// Re-exports for convenience
pub use app::{App, Command};
pub use arg::{Arg, ArgGroup, Validator, ValueSource};
pub use matches::Matches;
pub use completion::Shell;
pub use error::CliError;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_arg_creation() {
        let arg = Arg::new("test")
            .long("test")
            .short('t')
            .help("Test argument")
            .takes_value(true);

        assert_eq!(arg.name, "test");
        assert_eq!(arg.long, "test");
        assert_eq!(arg.short, Some("t".to_string()));
    }

    #[test]
    fn test_command_creation() {
        let cmd = Command::new("test")
            .about("Test command")
            .arg(Arg::new("arg1"));

        assert_eq!(cmd.name, "test");
        assert_eq!(cmd.args.len(), 1);
    }

    #[test]
    fn test_value_as_parsing() {
        let mut matches = Matches::new();
        matches.args.insert("port".to_string(), Some("8080".to_string()));

        let port: u16 = matches.value_as("port").unwrap();
        assert_eq!(port, 8080);
    }

    #[test]
    fn test_any_present() {
        let mut matches = Matches::new();
        matches.args.insert("verbose".to_string(), None);

        assert!(matches.any_present(&["verbose", "debug"]));
        assert!(!matches.any_present(&["quiet", "silent"]));
    }

    #[test]
    fn test_all_present() {
        let mut matches = Matches::new();
        matches.args.insert("verbose".to_string(), None);
        matches.args.insert("debug".to_string(), None);

        assert!(matches.all_present(&["verbose", "debug"]));
        assert!(!matches.all_present(&["verbose", "debug", "trace"]));
    }

    #[test]
    fn test_value_or_default() {
        let matches = Matches::new();
        assert_eq!(matches.value_or("port", "8080"), "8080");
    }

    #[test]
    fn test_values_count() {
        let mut matches = Matches::new();
        matches.values.push("file1".to_string());
        matches.values.push("file2".to_string());

        assert_eq!(matches.values_count(), 2);
    }
}

