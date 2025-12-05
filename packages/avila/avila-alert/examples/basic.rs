use avila_alert::{Alert, AlertManager, ConsoleHandler, SimpleFormatter};

fn main() {
    println!("=== Basic Alert Example ===\n");

    // Create a manager
    let manager = AlertManager::new();

    // Add a console handler
    manager.add_handler(Box::new(ConsoleHandler::new(SimpleFormatter)));

    // Create and dispatch various alerts
    manager.dispatch(Alert::info("Application started successfully"));
    manager.dispatch(Alert::warning("Configuration file not found, using defaults"));
    manager.dispatch(Alert::error("Failed to connect to database"));
    manager.dispatch(Alert::critical("System out of memory!"));

    println!("\n=== Using convenience constructors ===\n");

    manager.dispatch(Alert::trace("Entering function foo()"));
    manager.dispatch(Alert::debug("Variable x = 42"));
    manager.dispatch(Alert::info("Processing request #1234"));
}
