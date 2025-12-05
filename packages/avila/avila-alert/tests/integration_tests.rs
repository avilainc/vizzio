use avila_alert::{Alert, AlertLevel, AlertBuilder, AlertManager, AlertFormatter, AlertHandler};
use avila_alert::{SimpleFormatter, DetailedFormatter, JsonFormatter, CompactFormatter};
use avila_alert::{CallbackHandler, BufferHandler, FilterHandler};

#[test]
fn test_alert_creation() {
    let alert = Alert::info("Test message");
    assert_eq!(alert.level, AlertLevel::Info);
    assert_eq!(alert.message, "Test message");
    assert!(alert.timestamp.is_some());
}

#[test]
fn test_alert_without_timestamp() {
    let alert = Alert::new_without_timestamp(AlertLevel::Debug, "Test");
    assert_eq!(alert.level, AlertLevel::Debug);
    assert!(alert.timestamp.is_none());
}

#[test]
fn test_alert_levels() {
    let trace = Alert::trace("trace");
    let debug = Alert::debug("debug");
    let info = Alert::info("info");
    let warning = Alert::warning("warning");
    let error = Alert::error("error");
    let critical = Alert::critical("critical");

    assert_eq!(trace.level, AlertLevel::Trace);
    assert_eq!(debug.level, AlertLevel::Debug);
    assert_eq!(info.level, AlertLevel::Info);
    assert_eq!(warning.level, AlertLevel::Warning);
    assert_eq!(error.level, AlertLevel::Error);
    assert_eq!(critical.level, AlertLevel::Critical);
}

#[test]
fn test_alert_with_tags() {
    let alert = Alert::info("Test")
        .with_tag("tag1")
        .with_tag("tag2");

    assert_eq!(alert.tags.len(), 2);
    assert!(alert.tags.contains(&"tag1".to_string()));
    assert!(alert.tags.contains(&"tag2".to_string()));
}

#[test]
fn test_alert_with_tags_multiple() {
    let alert = Alert::info("Test")
        .with_tags(vec!["tag1", "tag2", "tag3"]);

    assert_eq!(alert.tags.len(), 3);
}

#[test]
fn test_alert_with_context() {
    let alert = Alert::error("Test")
        .with_context("key1", "value1")
        .with_context("key2", "value2");

    assert_eq!(alert.context.len(), 2);
    assert_eq!(alert.context.get("key1").unwrap(), "value1");
    assert_eq!(alert.context.get("key2").unwrap(), "value2");
}

#[test]
fn test_alert_level_ordering() {
    assert!(AlertLevel::Trace < AlertLevel::Debug);
    assert!(AlertLevel::Debug < AlertLevel::Info);
    assert!(AlertLevel::Info < AlertLevel::Warning);
    assert!(AlertLevel::Warning < AlertLevel::Error);
    assert!(AlertLevel::Error < AlertLevel::Critical);
}

#[test]
fn test_alert_level_display() {
    assert_eq!(AlertLevel::Info.to_string(), "INFO");
    assert_eq!(AlertLevel::Warning.to_string(), "WARNING");
    assert_eq!(AlertLevel::Error.to_string(), "ERROR");
}

#[test]
fn test_alert_level_all() {
    let levels = AlertLevel::all();
    assert_eq!(levels.len(), 6);
    assert_eq!(levels[0], AlertLevel::Trace);
    assert_eq!(levels[5], AlertLevel::Critical);
}

#[test]
fn test_builder_basic() {
    let alert = AlertBuilder::new()
        .level(AlertLevel::Info)
        .message("Test")
        .build()
        .unwrap();

    assert_eq!(alert.level, AlertLevel::Info);
    assert_eq!(alert.message, "Test");
}

#[test]
fn test_builder_with_tags() {
    let alert = AlertBuilder::new()
        .level(AlertLevel::Warning)
        .message("Test")
        .tag("auth")
        .tags(vec!["security", "login"])
        .build()
        .unwrap();

    assert_eq!(alert.tags.len(), 3);
    assert!(alert.tags.contains(&"auth".to_string()));
    assert!(alert.tags.contains(&"security".to_string()));
}

#[test]
fn test_builder_missing_level() {
    let result = AlertBuilder::new()
        .message("Test")
        .build();

    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "Alert level is required");
}

#[test]
fn test_builder_missing_message() {
    let result = AlertBuilder::new()
        .level(AlertLevel::Info)
        .build();

    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "Alert message is required");
}

#[test]
fn test_timestamp_creation() {
    use avila_alert::Timestamp;
    
    let ts = Timestamp::now();
    assert!(ts.as_secs() > 0);
}

#[test]
fn test_timestamp_formatting() {
    use avila_alert::Timestamp;
    
    let ts = Timestamp::now();
    let formatted = ts.format_iso8601();
    
    assert!(formatted.contains("T"));
    assert!(formatted.ends_with("Z"));
    assert!(formatted.len() >= 19); // YYYY-MM-DDTHH:MM:SSZ
}

#[test]
fn test_simple_formatter() {
    let alert = Alert::info("Test message");
    let formatter = SimpleFormatter;
    let output = formatter.format(&alert);
    
    assert!(output.contains("INFO"));
    assert!(output.contains("Test message"));
}

#[test]
fn test_detailed_formatter() {
    let alert = Alert::warning("Test")
        .with_tag("test")
        .with_context("user", "john");
    
    let formatter = DetailedFormatter::new();
    let output = formatter.format(&alert);
    
    assert!(output.contains("WARNING"));
    assert!(output.contains("test"));
    assert!(output.contains("user=john"));
}

#[test]
fn test_json_formatter() {
    let alert = Alert::error("JSON test")
        .with_tag("json")
        .with_context("format", "test");
    
    let formatter = JsonFormatter::new();
    let output = formatter.format(&alert);
    
    assert!(output.contains("\"level\""));
    assert!(output.contains("\"message\""));
    assert!(output.contains("ERROR"));
    assert!(output.contains("JSON test"));
}

#[test]
fn test_json_formatter_pretty() {
    let alert = Alert::info("Test");
    let formatter = JsonFormatter::new().pretty();
    let output = formatter.format(&alert);
    
    assert!(output.contains("\n"));
    assert!(output.contains("  ")); // Indentation
}

#[test]
fn test_compact_formatter() {
    let alert = Alert::warning("Compact test");
    let formatter = CompactFormatter;
    let output = formatter.format(&alert);
    
    assert_eq!(output, "WARNING: Compact test");
}

#[test]
fn test_buffer_handler() {
    let buffer = BufferHandler::new();
    
    buffer.handle(&Alert::info("Test 1"));
    buffer.handle(&Alert::warning("Test 2"));
    buffer.handle(&Alert::error("Test 3"));
    
    assert_eq!(buffer.len(), 3);
    
    let alerts = buffer.get_alerts();
    assert_eq!(alerts[0].message, "Test 1");
    assert_eq!(alerts[1].message, "Test 2");
    assert_eq!(alerts[2].message, "Test 3");
}

#[test]
fn test_buffer_handler_max_size() {
    let buffer = BufferHandler::new().with_max_size(2);
    
    buffer.handle(&Alert::info("Test 1"));
    buffer.handle(&Alert::info("Test 2"));
    buffer.handle(&Alert::info("Test 3"));
    
    assert_eq!(buffer.len(), 2);
    
    let alerts = buffer.get_alerts();
    assert_eq!(alerts[0].message, "Test 2");
    assert_eq!(alerts[1].message, "Test 3");
}

#[test]
fn test_manager_integration() {
    use std::sync::{Arc, atomic::{AtomicUsize, Ordering}};
    
    let manager = AlertManager::new();
    let counter = Arc::new(AtomicUsize::new(0));
    let counter_clone = counter.clone();
    
    manager.add_handler(Box::new(CallbackHandler::new(move |_| {
        counter_clone.fetch_add(1, Ordering::SeqCst);
    })));
    
    manager.dispatch(Alert::info("Test 1"));
    manager.dispatch(Alert::warning("Test 2"));
    manager.dispatch(Alert::error("Test 3"));
    
    assert_eq!(counter.load(Ordering::SeqCst), 3);
}

#[test]
fn test_filter_handler_integration() {
    use std::sync::{Arc, atomic::{AtomicUsize, Ordering}};
    
    let counter = Arc::new(AtomicUsize::new(0));
    let counter_clone = counter.clone();
    
    let callback = CallbackHandler::new(move |_| {
        counter_clone.fetch_add(1, Ordering::SeqCst);
    });
    
    let handler = FilterHandler::new(
        |alert| alert.level >= AlertLevel::Error,
        callback,
    );
    
    handler.handle(&Alert::info("Should not count"));
    handler.handle(&Alert::warning("Should not count"));
    handler.handle(&Alert::error("Should count"));
    handler.handle(&Alert::critical("Should count"));
    
    assert_eq!(counter.load(Ordering::SeqCst), 2);
}

#[test]
fn test_macros() {
    use avila_alert::{info, warning, error};
    
    let alert1 = info!("Test info");
    assert_eq!(alert1.level, AlertLevel::Info);
    assert_eq!(alert1.message, "Test info");
    
    let value = 42;
    let alert2 = warning!("Value: {}", value);
    assert_eq!(alert2.level, AlertLevel::Warning);
    assert!(alert2.message.contains("42"));
    
    let alert3 = error!("Error code: {}", 500);
    assert_eq!(alert3.level, AlertLevel::Error);
    assert!(alert3.message.contains("500"));
}

