//! Help formatting demonstration
//!
//! This example demonstrates the improved help text formatting with:
//! - Colorized output
//! - Proper alignment
//! - Section headers
//! - Additional info (required, default, possible values)
//!
//! Run with:
//!   cargo run --example help_demo -- --help
//!   cargo run --example help_demo -- install --help

use avila_cli::{App, Arg, Command};

fn main() {
    App::new("help-demo")
        .version("2.1.0")
        .author("Ávila CLI Team")
        .about("A comprehensive demonstration of the improved help system with professional formatting, colorization, and detailed information display")
        .arg(
            Arg::new("config")
                .short('c')
                .long("config")
                .help("Path to configuration file")
                .takes_value(true)
                .default_value("config.toml")
        )
        .arg(
            Arg::new("verbose")
                .short('v')
                .long("verbose")
                .help("Enable verbose output mode")
        )
        .arg(
            Arg::new("output")
                .short('o')
                .long("output")
                .help("Output file path")
                .takes_value(true)
                .required(true)
        )
        .arg(
            Arg::new("format")
                .short('f')
                .long("format")
                .help("Output format")
                .takes_value(true)
                .possible_values(&["json", "yaml", "toml", "xml"])
                .default_value("json")
        )
        .arg(
            Arg::new("threads")
                .short('t')
                .long("threads")
                .help("Number of worker threads")
                .takes_value(true)
                .default_value("4")
        )
        .command(
            Command::new("install")
                .about("Install packages with dependencies")
                .arg(
                    Arg::new("package")
                        .help("Package name to install")
                        .takes_value(true)
                        .required(true)
                )
                .arg(
                    Arg::new("dev")
                        .short('d')
                        .long("dev")
                        .help("Install as development dependency")
                )
        )
        .command(
            Command::new("remove")
                .about("Remove installed packages")
                .arg(
                    Arg::new("package")
                        .help("Package name to remove")
                        .takes_value(true)
                        .required(true)
                )
                .arg(
                    Arg::new("force")
                        .short('f')
                        .long("force")
                        .help("Force removal without confirmation")
                )
        )
        .command(
            Command::new("list")
                .about("List all installed packages")
                .arg(
                    Arg::new("all")
                        .short('a')
                        .long("all")
                        .help("Show all packages including dependencies")
                )
        )
        .parse();

    println!("✓ Application started!");
    println!("\n💡 Tip: Run with --help to see the beautifully formatted help text");
}
