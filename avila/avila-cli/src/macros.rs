//! Declarative macros for CLI definition
//!
//! Provides convenient macros for quickly defining CLI applications
//! and arguments with minimal boilerplate.

/// Macro helper for rapid CLI definition
///
/// # Example
/// ```no_run
/// use avila_cli::{cli, arg, Arg};
///
/// let app = cli!("myapp" => {
///     version: "1.0.0",
///     about: "My awesome CLI",
///     args: [
///         arg!("verbose", short: 'v'),
///         arg!("output", takes_value: true)
///     ]
/// });
/// ```
#[macro_export]
macro_rules! cli {
    ($name:expr => {
        version: $version:expr,
        about: $about:expr,
        args: [$($arg:expr),* $(,)?]
    }) => {
        {
            let mut app = $crate::App::new($name)
                .version($version)
                .about($about);
            $(
                app = app.arg($arg);
            )*
            app
        }
    };
}

/// Macro helper for argument definition
///
/// # Examples
/// ```no_run
/// use avila_cli::{arg, Arg};
///
/// // Simple flag
/// let flag = arg!("verbose");
///
/// // With short form
/// let with_short = arg!("verbose", short: 'v');
///
/// // Required argument with value
/// let required = arg!("config", required);
///
/// // Complex argument
/// let complex = arg!("port",
///     short: 'p',
///     help: "Port number",
///     takes_value: true,
///     default: "8080"
/// );
/// ```
#[macro_export]
macro_rules! arg {
    ($name:expr) => {
        $crate::Arg::new($name)
    };
    ($name:expr, short: $short:expr) => {
        $crate::Arg::new($name).short($short)
    };
    ($name:expr, required) => {
        $crate::Arg::new($name).required(true).takes_value(true)
    };
    ($name:expr, $($key:ident: $value:expr),* $(,)?) => {
        {
            let mut a = $crate::Arg::new($name);
            $(
                a = arg!(@set a, $key: $value);
            )*
            a
        }
    };
    (@set $arg:expr, short: $short:expr) => { $arg.short($short) };
    (@set $arg:expr, help: $help:expr) => { $arg.help($help) };
    (@set $arg:expr, required: $req:expr) => { $arg.required($req) };
    (@set $arg:expr, takes_value: $tv:expr) => { $arg.takes_value($tv) };
    (@set $arg:expr, default: $def:expr) => { $arg.default_value($def) };
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_arg_macro_simple() {
        let arg = arg!("test");
        assert_eq!(arg.name, "test");
    }

    #[test]
    fn test_arg_macro_with_short() {
        let arg = arg!("test", short: 't');
        assert_eq!(arg.short, Some("t".to_string()));
    }

    #[test]
    fn test_arg_macro_required() {
        let arg = arg!("test", required);
        assert_eq!(arg.required, true);
        assert_eq!(arg.takes_value, true);
    }
}
