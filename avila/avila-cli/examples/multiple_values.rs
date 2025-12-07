//! Multiple values example
//!
//! Demonstrates handling multiple values for arguments:
//! - Multiple occurrences of same argument
//! - Value delimiters (comma-separated)
//! - Counting occurrences (verbosity levels)
//! - Min/max value constraints
//!
//! Run with:
//!   cargo run --example multiple_values -- --file input1.txt --file input2.txt
//!   cargo run --example multiple_values -- --tags rust,cli,parser
//!   cargo run --example multiple_values -- -vvv

use avila_cli::{App, Arg};

fn main() {
    let matches = App::new("multi")
        .version("1.0.0")
        .about("Example demonstrating multiple values")
        .arg(
            Arg::new("files")
                .short('f')
                .long("file")
                .help("Input files")
                .takes_value(true)
                .multiple_values(true)
        )
        .arg(
            Arg::new("tags")
                .short('t')
                .long("tags")
                .help("Tags (comma-separated)")
                .takes_value(true)
                .value_delimiter(',')
        )
        .arg(
            Arg::new("verbose")
                .short('v')
                .long("verbose")
                .help("Increase verbosity (-v, -vv, -vvv)")
                .multiple_values(true)
        )
        .arg(
            Arg::new("coords")
                .short('c')
                .long("coords")
                .help("X and Y coordinates")
                .takes_value(true)
                .number_of_values(2)
        )
        .parse();

    // Multiple files
    if let Some(files) = matches.values_of("files") {
        println!("Processing {} files:", files.len());
        for file in files {
            println!("  - {}", file);
        }
    }

    // Comma-separated tags
    if let Some(tags) = matches.values_of("tags") {
        println!("\nTags: {}", tags.join(", "));
    }

    // Verbosity level based on occurrences
    let verbosity = matches.occurrences_of("verbose");
    match verbosity {
        0 => println!("\nNormal output"),
        1 => println!("\nVerbose output (-v)"),
        2 => println!("\nVery verbose output (-vv)"),
        _ => println!("\nDebug output (-vvv)"),
    }

    // Fixed number of values
    if let Some(coords) = matches.values_of("coords") {
        println!("\nCoordinates: x={}, y={}", coords[0], coords[1]);
    }
}
