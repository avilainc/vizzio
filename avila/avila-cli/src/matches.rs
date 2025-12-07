//! Parse result and argument matching
//!
//! Contains the Matches type that holds parsed command-line arguments
//! and provides various query methods for accessing values.

use std::collections::HashMap;
use crate::arg::{Arg, ValueSource};

/// Parse result containing matched arguments
///
/// Uses HashMap for O(1) argument lookups.
/// Stores the active subcommand and all parsed argument values.
pub struct Matches {
    pub(crate) command: Option<String>,
    pub(crate) args: HashMap<String, Option<String>>,
    pub(crate) multi_args: HashMap<String, Vec<String>>,
    pub(crate) values: Vec<String>,
    pub(crate) sources: HashMap<String, ValueSource>,
}

impl Matches {
    pub(crate) fn new() -> Self {
        Self {
            command: None,
            args: HashMap::new(),
            multi_args: HashMap::new(),
            values: Vec::new(),
            sources: HashMap::new(),
        }
    }

    pub fn subcommand(&self) -> Option<&str> {
        self.command.as_deref()
    }

    pub fn is_present(&self, name: &str) -> bool {
        self.args.contains_key(name)
    }

    pub fn value_of(&self, name: &str) -> Option<&str> {
        self.args.get(name)?.as_deref()
    }

    pub fn values(&self) -> &[String] {
        &self.values
    }

    /// Get the source of where the value came from
    pub fn value_source(&self, name: &str) -> Option<&ValueSource> {
        self.sources.get(name)
    }

    /// Get parsed value as specific type
    ///
    /// # Example
    /// ```no_run
    /// # use avila_cli::*;
    /// # fn main() {
    /// # // Assume matches is already created from App::parse()
    /// # }
    /// ```
    pub fn value_as<T>(&self, name: &str) -> Option<T>
    where
        T: std::str::FromStr,
    {
        self.value_of(name)?.parse().ok()
    }

    /// Check if any of the given argument names is present
    ///
    /// # Example
    /// ```no_run
    /// # use avila_cli::*;
    /// # fn main() {
    /// # // Assume matches is already created from App::parse()
    /// # }
    /// ```
    pub fn any_present(&self, names: &[&str]) -> bool {
        names.iter().any(|name| self.is_present(name))
    }

    /// Check if all of the given argument names are present
    pub fn all_present(&self, names: &[&str]) -> bool {
        names.iter().all(|name| self.is_present(name))
    }

    /// Get value or return a default
    pub fn value_or<'a>(&'a self, name: &str, default: &'a str) -> &'a str {
        self.value_of(name).unwrap_or(default)
    }

    /// Get the number of positional arguments
    pub fn values_count(&self) -> usize {
        self.values.len()
    }

    /// Get multiple values for an argument that accepts multiple values
    ///
    /// # Example
    /// ```no_run
    /// # use avila_cli::*;
    /// # let matches = Matches::new();
    /// if let Some(files) = matches.values_of("files") {
    ///     for file in files {
    ///         println!("File: {}", file);
    ///     }
    /// }
    /// ```
    pub fn values_of(&self, name: &str) -> Option<&[String]> {
        self.multi_args.get(name).map(|v| v.as_slice())
    }

    /// Get number of times an argument was provided
    ///
    /// Useful for counting flags like -v, -vv, -vvv
    ///
    /// # Example
    /// ```no_run
    /// # use avila_cli::*;
    /// # let matches = Matches::new();
    /// let verbosity = matches.occurrences_of("verbose");
    /// match verbosity {
    ///     0 => println!("Normal output"),
    ///     1 => println!("Verbose output"),
    ///     _ => println!("Very verbose output"),
    /// }
    /// ```
    pub fn occurrences_of(&self, name: &str) -> usize {
        self.multi_args.get(name).map(|v| v.len()).unwrap_or_else(|| {
            if self.is_present(name) { 1 } else { 0 }
        })
    }

    pub(crate) fn parse_args_list(&mut self, arg_defs: &[Arg], args: &[String]) {
        let mut i = 0;
        while i < args.len() {
            let arg = &args[i];

            if arg.starts_with("--") {
                let key = &arg[2..];
                if let Some(arg_def) = arg_defs.iter().find(|a| a.long == key) {
                    if arg_def.takes_value && i + 1 < args.len() {
                        let value = args[i + 1].clone();

                        if arg_def.multiple_values {
                            // Handle delimiter if specified
                            if let Some(delim) = arg_def.value_delimiter {
                                let split_values: Vec<String> = value.split(delim)
                                    .map(|s| s.trim().to_string())
                                    .collect();
                                self.multi_args.entry(arg_def.name.clone())
                                    .or_insert_with(Vec::new)
                                    .extend(split_values);
                            } else {
                                self.multi_args.entry(arg_def.name.clone())
                                    .or_insert_with(Vec::new)
                                    .push(value);
                            }
                        } else {
                            self.args.insert(arg_def.name.clone(), Some(value));
                        }
                        i += 2;
                    } else {
                        if arg_def.multiple_values {
                            // Count flag occurrences
                            self.multi_args.entry(arg_def.name.clone())
                                .or_insert_with(Vec::new)
                                .push(String::new());
                        } else {
                            self.args.insert(arg_def.name.clone(), None);
                        }
                        i += 1;
                    }
                } else {
                    i += 1;
                }
            } else if arg.starts_with('-') && arg.len() == 2 {
                let short = &arg[1..];
                if let Some(arg_def) = arg_defs.iter().find(|a| a.short.as_deref() == Some(short)) {
                    if arg_def.takes_value && i + 1 < args.len() {
                        let value = args[i + 1].clone();

                        if arg_def.multiple_values {
                            if let Some(delim) = arg_def.value_delimiter {
                                let split_values: Vec<String> = value.split(delim)
                                    .map(|s| s.trim().to_string())
                                    .collect();
                                self.multi_args.entry(arg_def.name.clone())
                                    .or_insert_with(Vec::new)
                                    .extend(split_values);
                            } else {
                                self.multi_args.entry(arg_def.name.clone())
                                    .or_insert_with(Vec::new)
                                    .push(value);
                            }
                        } else {
                            self.args.insert(arg_def.name.clone(), Some(value));
                        }
                        i += 2;
                    } else {
                        if arg_def.multiple_values {
                            self.multi_args.entry(arg_def.name.clone())
                                .or_insert_with(Vec::new)
                                .push(String::new());
                        } else {
                            self.args.insert(arg_def.name.clone(), None);
                        }
                        i += 1;
                    }
                } else {
                    i += 1;
                }
            } else {
                self.values.push(arg.clone());
                i += 1;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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

    #[test]
    fn test_multiple_values() {
        let mut matches = Matches::new();
        matches.multi_args.insert(
            "files".to_string(),
            vec!["file1.txt".to_string(), "file2.txt".to_string()]
        );

        let files = matches.values_of("files").unwrap();
        assert_eq!(files.len(), 2);
        assert_eq!(files[0], "file1.txt");
        assert_eq!(files[1], "file2.txt");
    }

    #[test]
    fn test_occurrences() {
        let mut matches = Matches::new();
        matches.multi_args.insert(
            "verbose".to_string(),
            vec![String::new(), String::new(), String::new()]
        );

        assert_eq!(matches.occurrences_of("verbose"), 3);
        assert_eq!(matches.occurrences_of("debug"), 0);
    }
}
