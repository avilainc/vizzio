//! Argument definitions and related types
//!
//! Provides the core types for defining command-line arguments,
//! including groups, validators, and value sources.

/// Custom validator function type
pub type Validator = fn(&str) -> Result<(), String>;

/// Argument value source tracking
#[derive(Debug, Clone, PartialEq)]
pub enum ValueSource {
    CommandLine,
    Environment,
    ConfigFile,
    Default,
}

/// Argument group for mutual exclusion or requirements
#[derive(Clone)]
pub struct ArgGroup {
    pub(crate) name: String,
    pub(crate) args: Vec<String>,
    pub(crate) required: bool,
    pub(crate) multiple: bool,
}

impl ArgGroup {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            args: Vec::new(),
            required: false,
            multiple: false,
        }
    }

    pub fn args(mut self, args: &[&str]) -> Self {
        self.args = args.iter().map(|s| s.to_string()).collect();
        self
    }

    pub fn required(mut self, req: bool) -> Self {
        self.required = req;
        self
    }

    pub fn multiple(mut self, mult: bool) -> Self {
        self.multiple = mult;
        self
    }
}

/// Command-line argument specification
///
/// Defines a flag or option with optional short/long forms.
/// Can be boolean (flag) or value-taking (option).
pub struct Arg {
    pub(crate) name: String,
    pub(crate) long: String,
    pub(crate) short: Option<String>,
    pub(crate) help: String,
    pub(crate) takes_value: bool,
    pub(crate) required: bool,
    pub(crate) default_value: Option<String>,
    pub(crate) possible_values: Vec<String>,
    pub(crate) validator: Option<Validator>,
    pub(crate) env_var: Option<String>,
    pub(crate) hidden: bool,
    pub(crate) conflicts_with: Vec<String>,
    pub(crate) requires: Vec<String>,
    pub(crate) multiple_values: bool,
    pub(crate) value_delimiter: Option<char>,
    pub(crate) min_values: Option<usize>,
    pub(crate) max_values: Option<usize>,
}

impl Arg {
    pub fn new(name: impl Into<String>) -> Self {
        let name = name.into();
        Self {
            long: name.clone(),
            name,
            short: None,
            help: String::new(),
            takes_value: false,
            required: false,
            default_value: None,
            possible_values: Vec::new(),
            validator: None,
            env_var: None,
            hidden: false,
            conflicts_with: Vec::new(),
            requires: Vec::new(),
            multiple_values: false,
            value_delimiter: None,
            min_values: None,
            max_values: None,
        }
    }

    pub fn long(mut self, long: impl Into<String>) -> Self {
        self.long = long.into();
        self
    }

    pub fn short(mut self, short: char) -> Self {
        self.short = Some(short.to_string());
        self
    }

    pub fn help(mut self, help: impl Into<String>) -> Self {
        self.help = help.into();
        self
    }

    pub fn takes_value(mut self, takes: bool) -> Self {
        self.takes_value = takes;
        self
    }

    pub fn required(mut self, req: bool) -> Self {
        self.required = req;
        self
    }

    /// Set a default value for the argument
    ///
    /// # Example
    /// ```no_run
    /// # use avila_cli::Arg;
    /// Arg::new("port")
    ///     .takes_value(true)
    ///     .default_value("8080");
    /// ```
    pub fn default_value(mut self, value: impl Into<String>) -> Self {
        self.default_value = Some(value.into());
        self
    }

    /// Restrict possible values
    ///
    /// # Example
    /// ```no_run
    /// # use avila_cli::Arg;
    /// Arg::new("format")
    ///     .takes_value(true)
    ///     .possible_values(&["json", "yaml", "toml"]);
    /// ```
    pub fn possible_values(mut self, values: &[&str]) -> Self {
        self.possible_values = values.iter().map(|s| s.to_string()).collect();
        self
    }

    /// Add a custom validator function
    ///
    /// # Example
    /// ```no_run
    /// # use avila_cli::Arg;
    /// Arg::new("port")
    ///     .takes_value(true)
    ///     .validator(|v| {
    ///         v.parse::<u16>()
    ///             .map(|_| ())
    ///             .map_err(|_| "must be a valid port number".to_string())
    ///     });
    /// ```
    pub fn validator(mut self, f: Validator) -> Self {
        self.validator = Some(f);
        self
    }

    /// Read value from specific environment variable
    pub fn env(mut self, var: impl Into<String>) -> Self {
        self.env_var = Some(var.into());
        self
    }

    /// Hide this argument from help output
    pub fn hidden(mut self, hide: bool) -> Self {
        self.hidden = hide;
        self
    }

    /// This argument conflicts with another
    pub fn conflicts_with(mut self, arg: impl Into<String>) -> Self {
        self.conflicts_with.push(arg.into());
        self
    }

    /// This argument requires another to be present
    pub fn requires(mut self, arg: impl Into<String>) -> Self {
        self.requires.push(arg.into());
        self
    }

    /// Allow multiple values for this argument
    ///
    /// # Example
    /// ```no_run
    /// # use avila_cli::Arg;
    /// Arg::new("files")
    ///     .takes_value(true)
    ///     .multiple_values(true);
    /// // Usage: --files file1.txt --files file2.txt
    /// ```
    pub fn multiple_values(mut self, multiple: bool) -> Self {
        self.multiple_values = multiple;
        self
    }

    /// Set delimiter for splitting multiple values from single input
    ///
    /// # Example
    /// ```no_run
    /// # use avila_cli::Arg;
    /// Arg::new("tags")
    ///     .takes_value(true)
    ///     .value_delimiter(',');
    /// // Usage: --tags rust,cli,parser
    /// // Results in: ["rust", "cli", "parser"]
    /// ```
    pub fn value_delimiter(mut self, delimiter: char) -> Self {
        self.value_delimiter = Some(delimiter);
        self.multiple_values = true; // Auto-enable multiple values
        self
    }

    /// Set minimum number of values required
    ///
    /// # Example
    /// ```no_run
    /// # use avila_cli::Arg;
    /// Arg::new("inputs")
    ///     .takes_value(true)
    ///     .multiple_values(true)
    ///     .min_values(2);
    /// ```
    pub fn min_values(mut self, min: usize) -> Self {
        self.min_values = Some(min);
        self.multiple_values = true;
        self
    }

    /// Set maximum number of values allowed
    ///
    /// # Example
    /// ```no_run
    /// # use avila_cli::Arg;
    /// Arg::new("choices")
    ///     .takes_value(true)
    ///     .multiple_values(true)
    ///     .max_values(3);
    /// ```
    pub fn max_values(mut self, max: usize) -> Self {
        self.max_values = Some(max);
        self.multiple_values = true;
        self
    }

    /// Set exact number of values required (min and max equal)
    ///
    /// # Example
    /// ```no_run
    /// # use avila_cli::Arg;
    /// Arg::new("coords")
    ///     .takes_value(true)
    ///     .number_of_values(2);
    /// // Usage: --coords 10 20
    /// ```
    pub fn number_of_values(mut self, num: usize) -> Self {
        self.min_values = Some(num);
        self.max_values = Some(num);
        self.multiple_values = true;
        self
    }
}

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
    fn test_arg_group_creation() {
        let group = ArgGroup::new("test")
            .args(&["arg1", "arg2"])
            .required(true)
            .multiple(false);

        assert_eq!(group.name, "test");
        assert_eq!(group.args.len(), 2);
        assert_eq!(group.required, true);
        assert_eq!(group.multiple, false);
    }

    #[test]
    fn test_value_source_variants() {
        let sources = vec![
            ValueSource::CommandLine,
            ValueSource::Environment,
            ValueSource::ConfigFile,
            ValueSource::Default,
        ];
        assert_eq!(sources.len(), 4);
    }
}
