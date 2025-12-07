use avila_alert::{
    Alert, AlertLevel, AlertManager, AlertHandler,
    ConsoleHandler, SimpleFormatter, DetailedFormatter, JsonFormatter, CompactFormatter,
};

fn main() {
    println!("=== Formatter Examples ===\n");

    // Example 1: Simple Formatter
    println!("1. Simple Formatter:");
    let manager1 = AlertManager::new();
    manager1.add_handler(Box::new(ConsoleHandler::new(SimpleFormatter)));
    manager1.dispatch(Alert::info("Simple formatted message"));
    println!();

    // Example 2: Detailed Formatter (default)
    println!("2. Detailed Formatter (with all features):");
    let manager2 = AlertManager::new();
    let formatter2 = DetailedFormatter::new();
    manager2.add_handler(Box::new(ConsoleHandler::new(formatter2)));

    let alert = Alert::warning("Detailed message")
        .with_tag("example")
        .with_context("user", "alice");
    manager2.dispatch(alert);
    println!();

    // Example 3: Detailed Formatter (no emoji)
    println!("3. Detailed Formatter (no emoji):");
    let manager3 = AlertManager::new();
    let formatter3 = DetailedFormatter::new()
        .with_emoji(false);
    manager3.add_handler(Box::new(ConsoleHandler::new(formatter3)));
    manager3.dispatch(Alert::error("No emoji here"));
    println!();

    // Example 4: Detailed Formatter (minimal)
    println!("4. Detailed Formatter (minimal - no tags/context):");
    let manager4 = AlertManager::new();
    let formatter4 = DetailedFormatter::new()
        .with_tags(false)
        .with_context(false);
    manager4.add_handler(Box::new(ConsoleHandler::new(formatter4)));

    let alert = Alert::info("Minimal format")
        .with_tag("ignored")
        .with_context("also", "ignored");
    manager4.dispatch(alert);
    println!();

    // Example 5: JSON Formatter
    println!("5. JSON Formatter:");
    let manager5 = AlertManager::new();
    let formatter5 = JsonFormatter::new();
    manager5.add_handler(Box::new(ConsoleHandler::new(formatter5)));

    let alert = Alert::critical("JSON formatted")
        .with_tag("json")
        .with_context("format", "compact");
    manager5.dispatch(alert);
    println!();

    // Example 6: Pretty JSON Formatter
    println!("6. Pretty JSON Formatter:");
    let manager6 = AlertManager::new();
    let formatter6 = JsonFormatter::new().pretty();
    manager6.add_handler(Box::new(ConsoleHandler::new(formatter6)));

    let alert = Alert::debug("Pretty JSON")
        .with_tag("json")
        .with_tag("pretty")
        .with_context("indented", "true");
    manager6.dispatch(alert);
    println!();

    // Example 7: Compact Formatter
    println!("7. Compact Formatter:");
    let manager7 = AlertManager::new();
    let formatter7 = CompactFormatter;
    manager7.add_handler(Box::new(ConsoleHandler::new(formatter7)));
    manager7.dispatch(Alert::warning("Compact one-liner format"));
}
