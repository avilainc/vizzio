//! Error types for CLI parsing
//!
//! Provides structured error types for CLI parsing and validation failures.

use std::fmt;

/// CLI error type
#[derive(Debug, Clone, PartialEq)]
pub enum CliError {
    /// Required argument missing
    MissingRequired(String),

    /// Invalid value for argument
    InvalidValue {
        arg: String,
        value: String,
        reason: String,
    },

    /// Argument conflicts with another
    Conflict {
        arg1: String,
        arg2: String,
    },

    /// Required argument missing (depends on another)
    MissingRequirement {
        arg: String,
        required: String,
    },

    /// Argument group validation failed
    GroupValidation(String),

    /// Custom validation error
    ValidationFailed {
        arg: String,
        message: String,
    },

    /// Unknown argument (with suggestions)
    UnknownArgument {
        arg: String,
        suggestions: Vec<String>,
    },

    /// Unknown command (with suggestions)
    UnknownCommand {
        command: String,
        suggestions: Vec<String>,
    },

    /// Invalid value from possible_values (with suggestions)
    InvalidPossibleValue {
        arg: String,
        value: String,
        possible_values: Vec<String>,
        suggestions: Vec<String>,
    },

    /// Missing value for argument that takes a value
    MissingValue(String),
}

impl fmt::Display for CliError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CliError::MissingRequired(arg) => {
                write!(f, "Error: --{} is required", arg)
            }
            CliError::InvalidValue { arg, value, reason } => {
                write!(f, "Error: invalid value '{}' for --{}: {}", value, arg, reason)
            }
            CliError::Conflict { arg1, arg2 } => {
                write!(f, "Error: --{} conflicts with --{}", arg1, arg2)
            }
            CliError::MissingRequirement { arg, required } => {
                write!(f, "Error: --{} requires --{}", arg, required)
            }
            CliError::GroupValidation(msg) => {
                write!(f, "{}", msg)
            }
            CliError::ValidationFailed { arg, message } => {
                write!(f, "Error: validation failed for --{}: {}", arg, message)
            }
            CliError::UnknownArgument { arg, suggestions } => {
                write!(f, "Error: unknown argument '{}'", arg)?;
                if !suggestions.is_empty() {
                    write!(f, "\n\nDid you mean one of these?\n")?;
                    for suggestion in suggestions {
                        write!(f, "  {}\n", suggestion)?;
                    }
                }
                Ok(())
            }
            CliError::UnknownCommand { command, suggestions } => {
                write!(f, "Error: unknown command '{}'", command)?;
                if !suggestions.is_empty() {
                    write!(f, "\n\nDid you mean one of these?\n")?;
                    for suggestion in suggestions {
                        write!(f, "  {}\n", suggestion)?;
                    }
                }
                Ok(())
            }
            CliError::InvalidPossibleValue { arg, value, possible_values, suggestions } => {
                write!(f, "Error: invalid value '{}' for --{}", value, arg)?;
                write!(f, "\n\nPossible values: {}", possible_values.join(", "))?;
                if !suggestions.is_empty() {
                    write!(f, "\n\nDid you mean one of these?\n")?;
                    for suggestion in suggestions {
                        write!(f, "  {}\n", suggestion)?;
                    }
                }
                Ok(())
            }
            CliError::MissingValue(arg) => {
                write!(f, "Error: --{} requires a value", arg)
            }
        }
    }
}

impl std::error::Error for CliError {}

impl CliError {
    /// Format error with colors (ANSI escape codes)
    pub fn colored(&self) -> String {
        use crate::colors::{colorize, RED, CYAN, YELLOW, GREEN};

        match self {
            CliError::MissingRequired(arg) => {
                format!("Error: {} is required", colorize(&format!("--{}", arg), RED))
            }
            CliError::InvalidValue { arg, value, reason } => {
                format!(
                    "Error: invalid value {} for {}: {}",
                    colorize(&format!("'{}'", value), RED),
                    colorize(&format!("--{}", arg), CYAN),
                    reason
                )
            }
            CliError::Conflict { arg1, arg2 } => {
                format!(
                    "Error: {} conflicts with {}",
                    colorize(&format!("--{}", arg1), RED),
                    colorize(&format!("--{}", arg2), RED)
                )
            }
            CliError::MissingRequirement { arg, required } => {
                format!(
                    "Error: {} requires {}",
                    colorize(&format!("--{}", arg), CYAN),
                    colorize(&format!("--{}", required), YELLOW)
                )
            }
            CliError::GroupValidation(msg) => msg.clone(),
            CliError::ValidationFailed { arg, message } => {
                format!(
                    "Error: validation failed for {}: {}",
                    colorize(&format!("--{}", arg), CYAN),
                    colorize(message, RED)
                )
            }
            CliError::UnknownArgument { arg, suggestions } => {
                let mut output = format!("Error: unknown argument {}", colorize(&format!("'{}'", arg), RED));
                if !suggestions.is_empty() {
                    output.push_str(&format!("\n\n{}", colorize("Did you mean one of these?", YELLOW)));
                    for suggestion in suggestions {
                        output.push_str(&format!("\n  {}", colorize(suggestion, GREEN)));
                    }
                }
                output
            }
            CliError::UnknownCommand { command, suggestions } => {
                let mut output = format!("Error: unknown command {}", colorize(&format!("'{}'", command), RED));
                if !suggestions.is_empty() {
                    output.push_str(&format!("\n\n{}", colorize("Did you mean one of these?", YELLOW)));
                    for suggestion in suggestions {
                        output.push_str(&format!("\n  {}", colorize(suggestion, GREEN)));
                    }
                }
                output
            }
            CliError::InvalidPossibleValue { arg, value, possible_values, suggestions } => {
                let mut output = format!(
                    "Error: invalid value {} for {}",
                    colorize(&format!("'{}'", value), RED),
                    colorize(&format!("--{}", arg), CYAN)
                );
                output.push_str(&format!("\n\nPossible values: {}", colorize(&possible_values.join(", "), CYAN)));
                if !suggestions.is_empty() {
                    output.push_str(&format!("\n\n{}", colorize("Did you mean one of these?", YELLOW)));
                    for suggestion in suggestions {
                        output.push_str(&format!("\n  {}", colorize(suggestion, GREEN)));
                    }
                }
                output
            }
            CliError::MissingValue(arg) => {
                format!("Error: {} requires a value", colorize(&format!("--{}", arg), RED))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        let err = CliError::MissingRequired("config".to_string());
        assert_eq!(err.to_string(), "Error: --config is required");
    }

    #[test]
    fn test_invalid_value_error() {
        let err = CliError::InvalidValue {
            arg: "port".to_string(),
            value: "abc".to_string(),
            reason: "must be a number".to_string(),
        };
        assert!(err.to_string().contains("invalid value"));
        assert!(err.to_string().contains("port"));
    }

    #[test]
    fn test_conflict_error() {
        let err = CliError::Conflict {
            arg1: "json".to_string(),
            arg2: "yaml".to_string(),
        };
        assert!(err.to_string().contains("conflicts"));
    }

    #[test]
    fn test_error_is_error_trait() {
        let err = CliError::UnknownArgument {
            arg: "test".to_string(),
            suggestions: vec![],
        };
        let _: &dyn std::error::Error = &err;
    }
}
