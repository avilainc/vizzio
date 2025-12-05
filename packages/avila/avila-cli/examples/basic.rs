//! Basic CLI example
//!
//! Demonstrates the fundamental features of Ávila CLI:
//! - Creating an App
//! - Adding arguments with short/long forms
//! - Parsing command-line arguments
//! - Accessing values
//!
//! Run with: cargo run --example basic -- --name John --age 30 -v

use avila_cli::{App, Arg};

fn main() {
    let matches = App::new("basic")
        .version("1.0.0")
        .about("A basic CLI example demonstrating Ávila CLI features")
        .author("Your Name")
        .arg(
            Arg::new("name")
                .short('n')
                .long("name")
                .help("Your name")
                .takes_value(true)
                .required(true)
        )
        .arg(
            Arg::new("age")
                .short('a')
                .long("age")
                .help("Your age")
                .takes_value(true)
                .validator(|v| {
                    v.parse::<u8>()
                        .map(|_| ())
                        .map_err(|_| "age must be a valid number".to_string())
                })
        )
        .arg(
            Arg::new("verbose")
                .short('v')
                .long("verbose")
                .help("Enable verbose output")
        )
        .parse();

    // Access values
    let name = matches.value_of("name").unwrap();
    println!("Hello, {}!", name);

    if let Some(age) = matches.value_of("age") {
        println!("You are {} years old", age);
    }

    if matches.is_present("verbose") {
        println!("Verbose mode enabled");
        println!("Debug info: argument sources, etc.");
    }
}
