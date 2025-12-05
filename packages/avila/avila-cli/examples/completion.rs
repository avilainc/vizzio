//! Shell completion example
//!
//! Demonstrates generating shell completion scripts:
//! - Bash completion
//! - Zsh completion
//! - Fish completion
//! - PowerShell completion
//!
//! Run with:
//!   cargo run --example completion -- bash > completion.bash
//!   cargo run --example completion -- zsh > _completion
//!   cargo run --example completion -- fish > completion.fish
//!   cargo run --example completion -- powershell > completion.ps1

use avila_cli::{App, Arg, Shell};

fn main() {
    let app = App::new("myapp")
        .version("1.0.0")
        .about("A CLI application with shell completions")
        .arg(
            Arg::new("config")
                .short('c')
                .long("config")
                .help("Configuration file")
                .takes_value(true)
        )
        .arg(
            Arg::new("verbose")
                .short('v')
                .long("verbose")
                .help("Enable verbose output")
        )
        .arg(
            Arg::new("format")
                .short('f')
                .long("format")
                .help("Output format")
                .takes_value(true)
                .possible_values(&["json", "yaml", "toml"])
        );

    // Parse args to see which shell completion to generate
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <shell>", args[0]);
        eprintln!("Shells: bash, zsh, fish, powershell");
        std::process::exit(1);
    }

    let shell = match args[1].as_str() {
        "bash" => Shell::Bash,
        "zsh" => Shell::Zsh,
        "fish" => Shell::Fish,
        "powershell" | "pwsh" => Shell::PowerShell,
        _ => {
            eprintln!("Unknown shell: {}", args[1]);
            eprintln!("Supported shells: bash, zsh, fish, powershell");
            std::process::exit(1);
        }
    };

    // Generate and print completion script
    let completion_script = app.generate_completion(shell);
    println!("{}", completion_script);

    eprintln!("\n# Installation instructions:");
    match shell {
        Shell::Bash => {
            eprintln!("# sudo cp completion.bash /etc/bash_completion.d/myapp");
            eprintln!("# Or add to ~/.bashrc:");
            eprintln!("# source /path/to/completion.bash");
        }
        Shell::Zsh => {
            eprintln!("# sudo cp _completion /usr/local/share/zsh/site-functions/_myapp");
            eprintln!("# Then run: compinit");
        }
        Shell::Fish => {
            eprintln!("# cp completion.fish ~/.config/fish/completions/myapp.fish");
        }
        Shell::PowerShell => {
            eprintln!("# Add to your PowerShell profile:");
            eprintln!("# . /path/to/completion.ps1");
        }
    }
}
