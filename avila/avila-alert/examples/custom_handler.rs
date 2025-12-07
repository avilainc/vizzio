use avila_alert::{
    Alert, AlertLevel, AlertManager, AlertHandler,
    CallbackHandler, MultiHandler, ConsoleHandler, SimpleFormatter,
};
use std::sync::{Arc, Mutex};

fn main() {
    println!("=== Custom Handler Examples ===\n");

    let manager = AlertManager::new();

    // Example 1: Callback handler for logging to a file (simulated)
    let log_file = Arc::new(Mutex::new(Vec::<String>::new()));
    let log_file_clone = log_file.clone();

    let file_handler = CallbackHandler::new(move |alert| {
        let mut logs = log_file_clone.lock().unwrap();
        logs.push(format!("[{}] {}", alert.level, alert.message));
        println!("📝 Logged to file: {}", alert.message);
    });

    // Example 2: Callback handler for metrics (simulated)
    let metrics = Arc::new(Mutex::new(std::collections::HashMap::<AlertLevel, usize>::new()));
    let metrics_clone = metrics.clone();

    let metrics_handler = CallbackHandler::new(move |alert| {
        let mut m = metrics_clone.lock().unwrap();
        *m.entry(alert.level).or_insert(0) += 1;
    });

    // Example 3: Callback handler for critical alerts only
    let critical_handler = CallbackHandler::new(|alert| {
        println!("🚨 CRITICAL ALERT DETECTED: {}", alert.message);
        // Here you could send to PagerDuty, Slack, etc.
    }).with_min_level(AlertLevel::Critical);

    // Example 4: Multi-handler combining console + custom handlers
    let multi = MultiHandler::new()
        .add_handler(Box::new(ConsoleHandler::new(SimpleFormatter)))
        .add_handler(Box::new(file_handler))
        .add_handler(Box::new(metrics_handler))
        .add_handler(Box::new(critical_handler));

    manager.add_handler(Box::new(multi));

    // Dispatch various alerts
    println!("Dispatching alerts...\n");

    manager.dispatch(Alert::info("Application started"));
    manager.dispatch(Alert::warning("High memory usage"));
    manager.dispatch(Alert::error("Connection timeout"));
    manager.dispatch(Alert::critical("System failure - immediate action required!"));
    manager.dispatch(Alert::debug("Debug information"));

    // Show metrics
    println!("\n=== Metrics Report ===");
    let m = metrics.lock().unwrap();
    for (level, count) in m.iter() {
        println!("{}: {}", level, count);
    }

    // Show logged messages
    println!("\n=== File Log Contents ===");
    let logs = log_file.lock().unwrap();
    for (i, log) in logs.iter().enumerate() {
        println!("{}: {}", i + 1, log);
    }
}
