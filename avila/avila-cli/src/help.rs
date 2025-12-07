//! Help text formatting and generation
//!
//! Provides professional, colorized help output with proper alignment and structure.

use crate::app::App;
use crate::arg::Arg;
use crate::colors;

/// Default maximum width for help text
const DEFAULT_MAX_WIDTH: usize = 100;

/// Indentation for different sections
const SECTION_INDENT: &str = "    ";

/// Help formatter configuration
pub struct HelpFormatter {
    max_width: usize,
    colored: bool,
}

impl HelpFormatter {
    pub fn new(colored: bool) -> Self {
        Self {
            max_width: Self::detect_terminal_width(),
            colored,
        }
    }

    pub fn with_max_width(mut self, width: usize) -> Self {
        self.max_width = width;
        self
    }

    /// Detect terminal width, fallback to default
    fn detect_terminal_width() -> usize {
        // In a real implementation, you'd use a crate like `terminal_size`
        // For now, use a sensible default
        DEFAULT_MAX_WIDTH
    }

    /// Format the complete help text for an app
    pub fn format_app_help(&self, app: &App) -> String {
        let mut output = String::new();

        // Header: name, version, description
        self.append_header(&mut output, app);

        // Usage line
        self.append_usage(&mut output, app);

        // Description (expanded about text)
        if !app.about.is_empty() {
            self.append_description(&mut output, &app.about);
        }

        // Options
        if !app.global_args.is_empty() {
            self.append_options(&mut output, &app.global_args);
        }

        // Commands
        if !app.commands.is_empty() {
            self.append_commands(&mut output, app);
        }

        output
    }

    fn append_header(&self, output: &mut String, app: &App) {
        let title = if self.colored {
            format!("{} {}",
                colors::colorize(&app.name, colors::CYAN),
                colors::colorize(&app.version, colors::GREEN)
            )
        } else {
            format!("{} {}", app.name, app.version)
        };

        output.push_str(&title);

        if let Some(ref author) = app.author {
            if self.colored {
                output.push_str(&format!(" - {}", colors::colorize(author, colors::DIM)));
            } else {
                output.push_str(&format!(" - {}", author));
            }
        }

        output.push_str("\n\n");
    }

    fn append_usage(&self, output: &mut String, app: &App) {
        let usage_label = if self.colored {
            colors::colorize("USAGE:", colors::YELLOW)
        } else {
            "USAGE:".to_string()
        };

        output.push_str(&usage_label);
        output.push('\n');
        output.push_str(SECTION_INDENT);

        // App name
        if self.colored {
            output.push_str(&colors::colorize(&app.name, colors::GREEN));
        } else {
            output.push_str(&app.name);
        }

        // Options indicator
        if !app.global_args.is_empty() {
            let options = if self.colored {
                colors::colorize("[OPTIONS]", colors::BLUE)
            } else {
                "[OPTIONS]".to_string()
            };
            output.push_str(&format!(" {}", options));
        }

        // Positional args (if any required args without -- or -)
        let positional_args: Vec<&Arg> = app.global_args.iter()
            .filter(|a| a.required && a.short.is_none() && a.long.is_empty())
            .collect();

        for arg in positional_args {
            let arg_name = if self.colored {
                colors::colorize(&format!("<{}>", arg.name.to_uppercase()), colors::CYAN)
            } else {
                format!("<{}>", arg.name.to_uppercase())
            };
            output.push_str(&format!(" {}", arg_name));
        }

        // Subcommands indicator
        if !app.commands.is_empty() {
            let cmd = if self.colored {
                colors::colorize("[COMMAND]", colors::MAGENTA)
            } else {
                "[COMMAND]".to_string()
            };
            output.push_str(&format!(" {}", cmd));
        }

        output.push_str("\n\n");
    }

    fn append_description(&self, output: &mut String, description: &str) {
        let label = if self.colored {
            colors::colorize("DESCRIPTION:", colors::YELLOW)
        } else {
            "DESCRIPTION:".to_string()
        };

        output.push_str(&label);
        output.push('\n');

        // Word wrap description
        let wrapped = self.wrap_text(description, self.max_width - 4);
        for line in wrapped.lines() {
            output.push_str(SECTION_INDENT);
            output.push_str(line);
            output.push('\n');
        }
        output.push('\n');
    }

    fn append_options(&self, output: &mut String, args: &[Arg]) {
        let label = if self.colored {
            colors::colorize("OPTIONS:", colors::YELLOW)
        } else {
            "OPTIONS:".to_string()
        };

        output.push_str(&label);
        output.push('\n');

        // Calculate column widths for alignment
        let max_flag_width = args.iter()
            .map(|a| self.format_arg_flags(a).len())
            .max()
            .unwrap_or(20)
            .min(30); // Cap at 30 chars

        for arg in args {
            if arg.hidden {
                continue;
            }

            output.push_str(SECTION_INDENT);

            let flags = self.format_arg_flags(arg);
            let flags_colored = if self.colored {
                self.colorize_arg_flags(&flags)
            } else {
                flags.clone()
            };

            output.push_str(&flags_colored);

            // Padding for alignment
            let padding = max_flag_width.saturating_sub(flags.len());
            output.push_str(&" ".repeat(padding + 4));

            // Help text
            output.push_str(&arg.help);

            // Additional info
            if arg.required {
                let req = if self.colored {
                    colors::colorize(" (required)", colors::RED)
                } else {
                    " (required)".to_string()
                };
                output.push_str(&req);
            }

            if let Some(ref default) = arg.default_value {
                let def = if self.colored {
                    format!(" {}", colors::colorize(&format!("[default: {}]", default), colors::DIM))
                } else {
                    format!(" [default: {}]", default)
                };
                output.push_str(&def);
            }

            if !arg.possible_values.is_empty() {
                let vals = arg.possible_values.join(", ");
                let poss = if self.colored {
                    format!(" {}", colors::colorize(&format!("[possible: {}]", vals), colors::BLUE))
                } else {
                    format!(" [possible: {}]", vals)
                };
                output.push_str(&poss);
            }

            output.push('\n');
        }

        output.push('\n');
    }

    fn append_commands(&self, output: &mut String, app: &App) {
        let label = if self.colored {
            colors::colorize("COMMANDS:", colors::YELLOW)
        } else {
            "COMMANDS:".to_string()
        };

        output.push_str(&label);
        output.push('\n');

        // Calculate max command name width
        let max_cmd_width = app.commands.iter()
            .map(|c| c.name.len())
            .max()
            .unwrap_or(10)
            .min(20);

        for cmd in &app.commands {
            output.push_str(SECTION_INDENT);

            let cmd_name = if self.colored {
                colors::colorize(&cmd.name, colors::MAGENTA)
            } else {
                cmd.name.clone()
            };

            output.push_str(&cmd_name);

            // Padding
            let padding = max_cmd_width.saturating_sub(cmd.name.len());
            output.push_str(&" ".repeat(padding + 4));

            output.push_str(&cmd.about);
            output.push('\n');
        }

        output.push('\n');
        output.push_str(&format!("Run '{} <COMMAND> --help' for more information on a command.\n", app.name));
    }

    fn format_arg_flags(&self, arg: &Arg) -> String {
        let mut flags = String::new();

        if let Some(ref short) = arg.short {
            flags.push('-');
            flags.push_str(short);
        }

        if !arg.long.is_empty() {
            if !flags.is_empty() {
                flags.push_str(", ");
            }
            flags.push_str("--");
            flags.push_str(&arg.long);
        }

        if arg.takes_value {
            let value_name = arg.name.to_uppercase();
            flags.push_str(&format!(" <{}>", value_name));
        }

        flags
    }

    fn colorize_arg_flags(&self, flags: &str) -> String {
        // Color short/long flags in yellow, value placeholders in cyan
        let mut result = String::new();
        let mut in_value = false;

        for c in flags.chars() {
            if c == '<' {
                in_value = true;
                result.push_str(&colors::colorize(&c.to_string(), colors::CYAN));
            } else if c == '>' {
                in_value = false;
                result.push_str(&colors::colorize(&c.to_string(), colors::CYAN));
            } else if in_value {
                result.push_str(&colors::colorize(&c.to_string(), colors::CYAN));
            } else if c == '-' || c.is_alphanumeric() {
                result.push_str(&colors::colorize(&c.to_string(), colors::YELLOW));
            } else {
                result.push(c);
            }
        }

        result
    }

    fn wrap_text(&self, text: &str, width: usize) -> String {
        let mut result = String::new();
        let mut current_line = String::new();

        for word in text.split_whitespace() {
            if current_line.len() + word.len() + 1 > width {
                if !current_line.is_empty() {
                    result.push_str(&current_line);
                    result.push('\n');
                    current_line.clear();
                }
            }

            if !current_line.is_empty() {
                current_line.push(' ');
            }
            current_line.push_str(word);
        }

        if !current_line.is_empty() {
            result.push_str(&current_line);
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::App;
    use crate::Arg;

    #[test]
    fn test_help_formatter_creation() {
        let formatter = HelpFormatter::new(true);
        assert!(formatter.colored);
        assert_eq!(formatter.max_width, DEFAULT_MAX_WIDTH);
    }

    #[test]
    fn test_format_arg_flags() {
        let formatter = HelpFormatter::new(false);

        let arg = Arg::new("name")
            .short('n')
            .long("name")
            .takes_value(true);

        let flags = formatter.format_arg_flags(&arg);
        assert_eq!(flags, "-n, --name <NAME>");
    }

    #[test]
    fn test_wrap_text() {
        let formatter = HelpFormatter::new(false);
        let text = "This is a very long line that should be wrapped at the specified width";
        let wrapped = formatter.wrap_text(text, 30);

        for line in wrapped.lines() {
            assert!(line.len() <= 30);
        }
    }

    #[test]
    fn test_format_app_help() {
        let app = App::new("test")
            .version("1.0.0")
            .about("A test application")
            .arg(Arg::new("verbose").short('v').help("Enable verbose mode"));

        let formatter = HelpFormatter::new(false);
        let help = formatter.format_app_help(&app);

        assert!(help.contains("test 1.0.0"));
        assert!(help.contains("USAGE:"));
        assert!(help.contains("OPTIONS:"));
        assert!(help.contains("-v"));
        assert!(help.contains("verbose"));
    }
}
