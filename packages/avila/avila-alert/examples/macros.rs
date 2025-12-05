// Macros example - Pure Rust implementation

use avila_alert::{info, debug, warning, error, critical, AlertManager, ConsoleHandler, DetailedFormatter};

fn main() {
    println!("=== Macro Examples ===\n");

    let manager = AlertManager::new();
    manager.add_handler(Box::new(
        ConsoleHandler::new(DetailedFormatter::new())
    ));

    // Simple string messages
    manager.dispatch(info!("Application initialized"));
    manager.dispatch(debug!("Loading configuration"));
    manager.dispatch(warning!("Cache miss"));
    manager.dispatch(error!("Invalid input"));
    manager.dispatch(critical!("System crash imminent"));

    println!("\n=== Formatted Messages ===\n");

    // Formatted messages
    let port = 8080;
    manager.dispatch(info!("Server listening on port {}", port));

    let username = "alice";
    let login_count = 42;
    manager.dispatch(info!("User {} logged in (total logins: {})", username, login_count));

    let cpu_usage = 95.7;
    manager.dispatch(warning!("High CPU usage: {:.1}%", cpu_usage));

    let error_code = 500;
    let error_msg = "Internal Server Error";
    manager.dispatch(error!("HTTP {}: {}", error_code, error_msg));

    println!("\n=== Complex Formatting ===\n");

    // More complex formatting
    let items = vec!["apple", "banana", "cherry"];
    manager.dispatch(debug!("Processing {} items: {:?}", items.len(), items));

    let config = ("timeout", 30);
    manager.dispatch(info!("Configuration: {} = {} seconds", config.0, config.1));
}

