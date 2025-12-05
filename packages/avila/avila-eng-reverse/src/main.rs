// Core modules
mod core;
mod internal;

// Analysis modules
mod analysis;
mod malware;
mod vuln;
mod ctf;

// New modules from blueprint
mod plugin;
mod cache;
mod emulation;
mod ml;
mod threat_intel;
mod formats;
mod tui;
mod web;
mod reporting;

// CLI
mod cli;

use anyhow::Result;
use clap::Parser;
use cli::Cli;
use tracing::info;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_env_filter("deriax=debug,info")
        .with_target(false)
        .init();

    info!("🔬 Deriax - Advanced Reverse Engineering Tool v{}", env!("CARGO_PKG_VERSION"));
    info!("   \"Derivar até o último exponente\"");
    info!("");
    info!("📦 Loaded modules:");
    info!("   ✓ Plugin System");
    info!("   ✓ Cache Layer");
    info!("   ✓ Static Analysis");
    info!("   ✓ Dynamic Analysis");
    info!("   ✓ Code Emulation");
    info!("   ✓ ML Detection");
    info!("   ✓ Threat Intelligence");
    info!("   ✓ Multi-format Support");
    info!("   ✓ Malware Detection");
    info!("   ✓ Vulnerability Scanner");
    info!("   ✓ CTF Tools");
    info!("");

    // Parse CLI arguments
    let cli = Cli::parse();

    // Execute command
    cli::execute(cli).await?;

    Ok(())
}
