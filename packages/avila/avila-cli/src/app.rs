//! Application and command definitions
//!
//! Core types for defining CLI applications and subcommands.

use std::env;
use crate::arg::{Arg, ArgGroup, ValueSource};
use crate::matches::Matches;
use crate::completion::Shell;
use crate::colors;

/// Subcommand definition
///
/// Represents a distinct command with its own argument schema.
/// Commands are parsed from the first positional argument.
pub struct Command {
    pub(crate) name: String,
    pub(crate) about: String,
    pub(crate) args: Vec<Arg>,
}

impl Command {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            about: String::new(),
            args: Vec::new(),
        }
    }

    pub fn about(mut self, about: impl Into<String>) -> Self {
        self.about = about.into();
        self
    }

    pub fn arg(mut self, arg: Arg) -> Self {
        self.args.push(arg);
        self
    }
}

/// Command-line application parser
///
/// Stack-allocated structure that defines the command-line interface schema.
/// All fields use heap-allocated collections for dynamic argument counts,
/// but the parser itself is deterministic and type-safe.
pub struct App {
    pub(crate) name: String,
    pub(crate) version: String,
    pub(crate) about: String,
    pub(crate) author: Option<String>,
    pub(crate) commands: Vec<Command>,
    pub(crate) global_args: Vec<Arg>,
    pub(crate) groups: Vec<ArgGroup>,
    pub(crate) colored_help: bool,
    pub(crate) config_file: Option<String>,
    pub(crate) env_prefix: Option<String>,
}

impl App {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            version: "1.0.0".to_string(),
            about: String::new(),
            author: None,
            commands: Vec::new(),
            global_args: Vec::new(),
            groups: Vec::new(),
            colored_help: true,
            config_file: None,
            env_prefix: None,
        }
    }

    pub fn version(mut self, version: impl Into<String>) -> Self {
        self.version = version.into();
        self
    }

    pub fn about(mut self, about: impl Into<String>) -> Self {
        self.about = about.into();
        self
    }

    pub fn author(mut self, author: impl Into<String>) -> Self {
        self.author = Some(author.into());
        self
    }

    pub fn command(mut self, cmd: Command) -> Self {
        self.commands.push(cmd);
        self
    }

    pub fn arg(mut self, arg: Arg) -> Self {
        self.global_args.push(arg);
        self
    }

    pub fn group(mut self, group: ArgGroup) -> Self {
        self.groups.push(group);
        self
    }

    pub fn colored_help(mut self, colored: bool) -> Self {
        self.colored_help = colored;
        self
    }

    /// Enable config file parsing (TOML-like format)
    pub fn config_file(mut self, path: impl Into<String>) -> Self {
        self.config_file = Some(path.into());
        self
    }

    /// Set environment variable prefix for fallback
    /// Example: prefix "MYAPP" allows MYAPP_PORT=8080
    pub fn env_prefix(mut self, prefix: impl Into<String>) -> Self {
        self.env_prefix = Some(prefix.into());
        self
    }

    /// Generate shell completion script
    pub fn generate_completion(&self, shell: Shell) -> String {
        crate::completion::generate(shell, &self.name, &self.global_args, &self.commands)
    }

    /// Print formatted help text
    pub fn print_help(&self) {
        let formatter = crate::help::HelpFormatter::new(self.colored_help);
        let help_text = formatter.format_app_help(self);
        println!("{}", help_text);
    }

    pub fn parse(self) -> Matches {
        let args: Vec<String> = env::args().skip(1).collect();
        self.parse_args(&args)
    }

    fn parse_args(self, args: &[String]) -> Matches {
        let mut matches = Matches::new();

        if args.is_empty() {
            return matches;
        }

        // Check for help/version
        if args[0] == "--help" || args[0] == "-h" {
            self.print_help();
            std::process::exit(0);
        }
        if args[0] == "--version" || args[0] == "-V" {
            println!("{} {}", self.name, self.version);
            std::process::exit(0);
        }

        // Parse command with suggestions if not found
        if let Some(cmd) = self.commands.iter().find(|c| c.name == args[0]) {
            matches.command = Some(args[0].clone());
            matches.parse_args_list(&cmd.args, &args[1..]);
        } else if !args[0].starts_with('-') && !self.commands.is_empty() {
            // Unknown command - provide suggestions
            let command_names: Vec<&str> = self.commands.iter()
                .map(|c| c.name.as_str())
                .collect();
            let suggestions = crate::suggestions::find_similar_commands(&args[0], &command_names, 2);

            let err = crate::error::CliError::UnknownCommand {
                command: args[0].clone(),
                suggestions,
            };

            if self.colored_help {
                eprintln!("{}", err.colored());
            } else {
                eprintln!("{}", err);
            }
            std::process::exit(1);
        } else {
            matches.parse_args_list(&self.global_args, args);
        }

        // Apply defaults and validate
        self.apply_defaults_and_validate(&mut matches);

        matches
    }

    fn apply_defaults_and_validate(&self, matches: &mut Matches) {
        // Load config file if specified
        let config_values = if let Some(ref path) = self.config_file {
            crate::config::parse_config_file(path)
        } else {
            std::collections::HashMap::new()
        };

        for arg in &self.global_args {
            let arg_name = &arg.name;

            // Priority order: CLI > Specific Env > Prefix Env > Config > Default
            if !matches.is_present(arg_name) {
                // Try specific environment variable first
                if let Some(ref env_var) = arg.env_var {
                    if let Ok(env_val) = env::var(env_var) {
                        matches.args.insert(arg_name.clone(), Some(env_val));
                        matches.sources.insert(arg_name.clone(), ValueSource::Environment);
                        continue;
                    }
                }

                // Try prefix-based environment variable
                if let Some(ref prefix) = self.env_prefix {
                    let env_key = format!("{}_{}", prefix.to_uppercase(), arg.long.to_uppercase());
                    if let Ok(env_val) = env::var(&env_key) {
                        matches.args.insert(arg_name.clone(), Some(env_val));
                        matches.sources.insert(arg_name.clone(), ValueSource::Environment);
                        continue;
                    }
                }

                // Try config file
                if let Some(config_val) = config_values.get(arg_name) {
                    matches.args.insert(arg_name.clone(), Some(config_val.clone()));
                    matches.sources.insert(arg_name.clone(), ValueSource::ConfigFile);
                    continue;
                }

                // Apply default value
                if arg.default_value.is_some() {
                    matches.args.insert(arg_name.clone(), arg.default_value.clone());
                    matches.sources.insert(arg_name.clone(), ValueSource::Default);
                }
            } else {
                // Mark as command line source if present
                matches.sources.insert(arg_name.clone(), ValueSource::CommandLine);
            }

            // Check required
            if arg.required && !matches.is_present(arg_name) {
                let msg = if self.colored_help {
                    format!("Error: {} is required", colors::colorize(&format!("--{}", arg.long), colors::RED))
                } else {
                    format!("Error: --{} is required", arg.long)
                };
                eprintln!("{}", msg);
                std::process::exit(1);
            }

            // Validate possible values with suggestions
            if !arg.possible_values.is_empty() {
                if let Some(value) = matches.value_of(&arg.name) {
                    if !arg.possible_values.iter().any(|v| v == value) {
                        // Generate suggestions for similar values
                        let possible_refs: Vec<&str> = arg.possible_values.iter()
                            .map(|s| s.as_str())
                            .collect();
                        let suggestions = crate::suggestions::find_similar_values(
                            value,
                            &possible_refs,
                            2
                        );

                        let err = crate::error::CliError::InvalidPossibleValue {
                            arg: arg.name.clone(),
                            value: value.to_string(),
                            possible_values: arg.possible_values.clone(),
                            suggestions,
                        };

                        if self.colored_help {
                            eprintln!("{}", err.colored());
                        } else {
                            eprintln!("{}", err);
                        }
                        std::process::exit(1);
                    }
                }
            }

            // Execute custom validator if present
            if let Some(validator) = &arg.validator {
                if let Some(value) = matches.value_of(&arg.name) {
                    if let Err(err) = validator(value) {
                        let msg = if self.colored_help {
                            format!(
                                "Error: validation failed for {}: {}",
                                colors::colorize(&format!("--{}", arg.long), colors::CYAN),
                                colors::colorize(&err, colors::RED)
                            )
                        } else {
                            format!("Error: validation failed for --{}: {}", arg.long, err)
                        };
                        eprintln!("{}", msg);
                        std::process::exit(1);
                    }
                }
            }

            // Check conflicts
            for conflict in &arg.conflicts_with {
                if matches.is_present(arg_name) && matches.is_present(conflict) {
                    let msg = if self.colored_help {
                        format!(
                            "Error: {} conflicts with {}",
                            colors::colorize(&format!("--{}", arg.long), colors::RED),
                            colors::colorize(&format!("--{}", conflict), colors::RED)
                        )
                    } else {
                        format!("Error: --{} conflicts with --{}", arg.long, conflict)
                    };
                    eprintln!("{}", msg);
                    std::process::exit(1);
                }
            }

            // Check requirements
            for required in &arg.requires {
                if matches.is_present(arg_name) && !matches.is_present(required) {
                    let msg = if self.colored_help {
                        format!(
                            "Error: {} requires {}",
                            colors::colorize(&format!("--{}", arg.long), colors::CYAN),
                            colors::colorize(&format!("--{}", required), colors::YELLOW)
                        )
                    } else {
                        format!("Error: --{} requires --{}", arg.long, required)
                    };
                    eprintln!("{}", msg);
                    std::process::exit(1);
                }
            }
        }

        // Validate argument groups
        if let Err(err) = crate::validation::validate_groups(&self.groups, matches, self.colored_help) {
            eprintln!("{}", err);
            std::process::exit(1);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_command_creation() {
        let cmd = Command::new("test")
            .about("Test command")
            .arg(Arg::new("arg1"));

        assert_eq!(cmd.name, "test");
        assert_eq!(cmd.args.len(), 1);
    }

    #[test]
    fn test_app_creation() {
        let app = App::new("myapp")
            .version("1.0.0")
            .about("Test app");

        assert_eq!(app.name, "myapp");
        assert_eq!(app.version, "1.0.0");
    }
}
