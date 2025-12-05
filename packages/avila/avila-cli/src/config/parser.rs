//! Config file parser implementation

use std::collections::HashMap;
use std::fs;

/// Parse simple config file (KEY=VALUE or KEY: VALUE format)
///
/// Supports:
/// - KEY=VALUE format
/// - KEY: VALUE format
/// - Comments starting with # or //
/// - Empty lines
/// - Quoted values (strips quotes)
///
/// # Example
/// ```text
/// # Config file
/// port=8080
/// host: localhost
/// debug = true
/// ```
pub fn parse_config_file(path: &str) -> HashMap<String, String> {
    let mut config = HashMap::new();

    if let Ok(contents) = fs::read_to_string(path) {
        for line in contents.lines() {
            let line = line.trim();

            // Skip comments and empty lines
            if line.is_empty() || line.starts_with('#') || line.starts_with("//") {
                continue;
            }

            // Parse KEY=VALUE or KEY: VALUE
            let parts: Vec<&str> = if line.contains('=') {
                line.splitn(2, '=').collect()
            } else if line.contains(':') {
                line.splitn(2, ':').collect()
            } else {
                continue;
            };

            if parts.len() == 2 {
                let key = parts[0].trim().to_lowercase();
                let value = parts[1].trim().trim_matches('"').to_string();
                config.insert(key, value);
            }
        }
    }

    config
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_empty_config() {
        // Non-existent file should return empty map
        let config = parse_config_file("non_existent.conf");
        assert!(config.is_empty());
    }

    #[test]
    fn test_parse_format() {
        // Test that both formats are supported
        use std::io::Write;
        let mut file = std::fs::File::create("test_config.txt").unwrap();
        writeln!(file, "key1=value1").unwrap();
        writeln!(file, "key2: value2").unwrap();
        writeln!(file, "# comment").unwrap();
        writeln!(file, "key3 = value3").unwrap();
        drop(file);

        let config = parse_config_file("test_config.txt");
        assert_eq!(config.get("key1"), Some(&"value1".to_string()));
        assert_eq!(config.get("key2"), Some(&"value2".to_string()));
        assert_eq!(config.get("key3"), Some(&"value3".to_string()));

        std::fs::remove_file("test_config.txt").ok();
    }
}
