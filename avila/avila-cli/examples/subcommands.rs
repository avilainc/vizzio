//! Subcommands example
//!
//! Demonstrates how to use subcommands:
//! - Defining multiple commands
//! - Command-specific arguments
//! - Detecting which command was used
//!
//! Run with:
//!   cargo run --example subcommands -- install package-name
//!   cargo run --example subcommands -- remove package-name --force
//!   cargo run --example subcommands -- list --all

use avila_cli::{App, Arg, Command};

fn main() {
    let matches = App::new("package-manager")
        .version("1.0.0")
        .about("A package manager CLI example")
        .command(
            Command::new("install")
                .about("Install a package")
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
                        .help("Install as dev dependency")
                )
        )
        .command(
            Command::new("remove")
                .about("Remove a package")
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
                .about("List installed packages")
                .arg(
                    Arg::new("all")
                        .short('a')
                        .long("all")
                        .help("Show all packages including dependencies")
                )
        )
        .parse();

    match matches.subcommand() {
        Some("install") => {
            let package = matches.value_of("package").unwrap();
            println!("Installing package: {}", package);

            if matches.is_present("dev") {
                println!("Installing as dev dependency");
            }
        }
        Some("remove") => {
            let package = matches.value_of("package").unwrap();

            if matches.is_present("force") {
                println!("Force removing package: {}", package);
            } else {
                println!("Removing package: {}", package);
            }
        }
        Some("list") => {
            println!("Installed packages:");

            if matches.is_present("all") {
                println!("  - package1 (v1.0.0)");
                println!("  - package2 (v2.1.0)");
                println!("  - dependency1 (v0.5.0)");
            } else {
                println!("  - package1 (v1.0.0)");
                println!("  - package2 (v2.1.0)");
            }
        }
        _ => {
            println!("No command specified. Use --help for usage information.");
        }
    }
}
