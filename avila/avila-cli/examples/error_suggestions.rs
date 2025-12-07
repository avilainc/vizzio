//! Error suggestions demonstration
//!
//! This example demonstrates the "Did you mean?" feature that provides
//! helpful suggestions when users make typos in:
//! - Argument names
//! - Command names
//! - Possible values
//!
//! Try running with intentional typos:
//!   cargo run --example error_suggestions -- --naem John
//!   cargo run --example error_suggestions -- isntall
//!   cargo run --example error_suggestions -- --format josn

use avila_cli::{App, Arg, Command};

fn main() {
    let matches = App::new("error-demo")
        .version("1.0.0")
        .about("Demonstrating error suggestions with 'Did you mean?'")
        .arg(
            Arg::new("name")
                .short('n')
                .long("name")
                .help("Your name")
                .takes_value(true)
        )
        .arg(
            Arg::new("age")
                .short('a')
                .long("age")
                .help("Your age")
                .takes_value(true)
        )
        .arg(
            Arg::new("email")
                .short('e')
                .long("email")
                .help("Your email address")
                .takes_value(true)
        )
        .arg(
            Arg::new("format")
                .short('f')
                .long("format")
                .help("Output format")
                .takes_value(true)
                .possible_values(&["json", "yaml", "toml", "xml"])
        )
        .command(
            Command::new("install")
                .about("Install a package")
        )
        .command(
            Command::new("remove")
                .about("Remove a package")
        )
        .command(
            Command::new("list")
                .about("List packages")
        )
        .parse();

    println!("✓ All arguments parsed successfully!");

    if let Some(name) = matches.value_of("name") {
        println!("Name: {}", name);
    }

    if let Some(format) = matches.value_of("format") {
        println!("Format: {}", format);
    }

    if let Some(cmd) = matches.subcommand() {
        println!("Command: {}", cmd);
    }

    println!("\n🎯 Try these examples to see error suggestions:");
    println!("  cargo run --example error_suggestions -- --naem John");
    println!("    (suggests: --name)");
    println!();
    println!("  cargo run --example error_suggestions -- isntall");
    println!("    (suggests: install)");
    println!();
    println!("  cargo run --example error_suggestions -- --format josn");
    println!("    (suggests: json)");
    println!();
    println!("  cargo run --example error_suggestions -- --emai test@example.com");
    println!("    (suggests: --email)");
}
