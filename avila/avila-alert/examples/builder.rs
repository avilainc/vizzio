use avila_alert::{AlertBuilder, AlertLevel, AlertManager, ConsoleHandler, DetailedFormatter};

fn main() {
    println!("=== Builder Pattern Example ===\n");

    let manager = AlertManager::new();

    // Use detailed formatter with emoji
    let formatter = DetailedFormatter::new()
        .with_emoji(true)
        .with_tags(true)
        .with_context(true);

    manager.add_handler(Box::new(ConsoleHandler::new(formatter)));

    // Build a simple alert
    let alert1 = AlertBuilder::new()
        .level(AlertLevel::Info)
        .message("User logged in")
        .build()
        .unwrap();

    manager.dispatch(alert1);

    // Build a complex alert with tags and context
    let alert2 = AlertBuilder::new()
        .level(AlertLevel::Warning)
        .message("Failed login attempt")
        .tag("security")
        .tag("authentication")
        .context("user", "john.doe")
        .context("ip", "192.168.1.100")
        .context("attempts", "3")
        .build()
        .unwrap();

    manager.dispatch(alert2);

    // Build an error with multiple tags
    let alert3 = AlertBuilder::new()
        .level(AlertLevel::Error)
        .message("Database connection pool exhausted")
        .tags(vec!["database", "performance", "critical"])
        .context_map(vec![
            ("pool_size", "10"),
            ("active_connections", "10"),
            ("waiting_requests", "25"),
        ])
        .build()
        .unwrap();

    manager.dispatch(alert3);
}
