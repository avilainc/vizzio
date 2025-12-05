//! Configuration file parsing
//!
//! Provides simple config file parsing supporting KEY=VALUE and KEY: VALUE formats.
//! Ignores comments (# and //) and empty lines.

mod parser;

pub use parser::parse_config_file;
