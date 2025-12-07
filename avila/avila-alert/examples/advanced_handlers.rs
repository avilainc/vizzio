// Example demonstrating advanced handler features - Pure Rust

use avila_alert::{
    Alert, AlertLevel, AlertManager,
    ConsoleHandler, SimpleFormatter, BufferHandler, FilterHandler, CallbackHandler,
};

fn main() {
    println!("=== Advanced Handler Examples ===\n");

    // Example 1: BufferHandler for collecting alerts
    println!("1. Buffer Handler:");
    let manager = AlertManager::new();
    let buffer = BufferHandler::new().with_max_size(5);
    let buffer_clone = buffer.clone();

    manager.add_handler(Box::new(buffer));

    manager.dispatch(Alert::info("Message 1"));
    manager.dispatch(Alert::warning("Message 2"));
    manager.dispatch(Alert::error("Message 3"));

    println!("Buffered {} alerts", buffer_clone.len());
    for (i, alert) in buffer_clone.get_alerts().iter().enumerate() {
        println!("  {}: [{}] {}", i + 1, alert.level, alert.message);
    }
    println!();

    // Example 2: FilterHandler for selective processing
    println!("2. Filter Handler (only errors and critical):");
    let manager2 = AlertManager::new();

    let console = ConsoleHandler::new(SimpleFormatter);
    let filtered = FilterHandler::new(
        |alert| alert.level >= AlertLevel::Error,
        console,
    );

    manager2.add_handler(Box::new(filtered));

    manager2.dispatch(Alert::info("This won't be shown"));
    manager2.dispatch(Alert::warning("This won't be shown either"));
    manager2.dispatch(Alert::error("This WILL be shown"));
    manager2.dispatch(Alert::critical("This WILL be shown too"));
    println!();

    // Example 3: FilterHandler with tag matching
    println!("3. Filter Handler (only 'security' tagged alerts):");
    let manager3 = AlertManager::new();

    let console = ConsoleHandler::new(SimpleFormatter);
    let security_filter = FilterHandler::new(
        |alert| alert.tags.contains(&"security".to_string()),
        console,
    );

    manager3.add_handler(Box::new(security_filter));

    manager3.dispatch(Alert::info("Normal log").with_tag("app"));
    manager3.dispatch(Alert::warning("Login attempt").with_tag("security"));
    manager3.dispatch(Alert::error("Unauthorized access").with_tag("security"));
    println!();

    // Example 4: Combining handlers
    println!("4. Combined Handlers:");
    let manager4 = AlertManager::new();

    // Buffer for all alerts
    let all_buffer = BufferHandler::new();
    let all_buffer_clone = all_buffer.clone();

    // Buffer only for critical alerts
    let critical_buffer = BufferHandler::new();
    let critical_buffer_clone = critical_buffer.clone();
    let critical_filter = FilterHandler::new(
        |alert| alert.level == AlertLevel::Critical,
        critical_buffer,
    );

    // Console output for warnings and above
    let console = ConsoleHandler::new(SimpleFormatter)
        .with_min_level(AlertLevel::Warning);

    manager4.add_handler(Box::new(all_buffer));
    manager4.add_handler(Box::new(critical_filter));
    manager4.add_handler(Box::new(console));

    manager4.dispatch(Alert::trace("Trace message"));
    manager4.dispatch(Alert::info("Info message"));
    manager4.dispatch(Alert::warning("Warning message"));
    manager4.dispatch(Alert::error("Error message"));
    manager4.dispatch(Alert::critical("Critical message"));

    println!("\nTotal alerts buffered: {}", all_buffer_clone.len());
    println!("Critical alerts: {}", critical_buffer_clone.len());
    println!();

    // Example 5: Custom callback with state
    println!("5. Stateful Callback Handler:");
    let manager5 = AlertManager::new();

    use std::sync::{Arc, Mutex};
    use std::collections::HashMap;

    let stats = Arc::new(Mutex::new(HashMap::<AlertLevel, usize>::new()));
    let stats_clone = stats.clone();

    let stats_handler = CallbackHandler::new(move |alert| {
        let mut s = stats_clone.lock().unwrap();
        *s.entry(alert.level).or_insert(0) += 1;
    });

    manager5.add_handler(Box::new(stats_handler));

    // Dispatch various alerts
    for _ in 0..3 {
        manager5.dispatch(Alert::info("Info"));
    }
    for _ in 0..2 {
        manager5.dispatch(Alert::warning("Warning"));
    }
    for _ in 0..5 {
        manager5.dispatch(Alert::error("Error"));
    }
    manager5.dispatch(Alert::critical("Critical"));

    println!("Alert statistics:");
    let final_stats = stats.lock().unwrap();
    for level in AlertLevel::all() {
        if let Some(count) = final_stats.get(&level) {
            println!("  {}: {}", level, count);
        }
    }
}
