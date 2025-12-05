//! Avila Browser CLI
//!
//! Command-line interface for controlling the Avila Browser

use avila_browser::cli::{parse_args, print_usage, CliRunner};
use std::env;
use std::process;

#[tokio::main]
async fn main() {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env()
                .add_directive(tracing::Level::INFO.into()),
        )
        .init();

    // Parse command-line arguments
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        print_usage();
        process::exit(1);
    }

    // Check for help flag
    if args.iter().any(|arg| arg == "-h" || arg == "--help") {
        print_usage();
        process::exit(0);
    }

    // Check for verbose flag
    let verbose = args.iter().any(|arg| arg == "-v" || arg == "--verbose");

    // Parse command
    let command = match parse_args(args) {
        Ok(cmd) => cmd,
        Err(e) => {
            eprintln!("Error: {}", e);
            eprintln!();
            print_usage();
            process::exit(1);
        }
    };

    // Execute command
    let runner = CliRunner::new(verbose);
    if let Err(e) = runner.execute(command).await {
        eprintln!("Error executing command: {}", e);
        process::exit(1);
    }
}
