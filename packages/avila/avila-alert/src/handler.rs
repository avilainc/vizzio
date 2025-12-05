use crate::types::{Alert, AlertLevel};
use crate::formatter::AlertFormatter;

/// Trait for handling alerts
pub trait AlertHandler: Send + Sync {
    fn handle(&self, alert: &Alert);
}

/// Console handler that prints alerts to stdout/stderr
#[derive(Debug)]
pub struct ConsoleHandler<F: AlertFormatter> {
    formatter: F,
    min_level: AlertLevel,
}

impl<F: AlertFormatter> ConsoleHandler<F> {
    pub fn new(formatter: F) -> Self {
        Self {
            formatter,
            min_level: AlertLevel::Trace,
        }
    }

    pub fn with_min_level(mut self, level: AlertLevel) -> Self {
        self.min_level = level;
        self
    }
}

impl<F: AlertFormatter + Send + Sync> AlertHandler for ConsoleHandler<F> {
    fn handle(&self, alert: &Alert) {
        if alert.level >= self.min_level {
            let formatted = self.formatter.format(alert);

            if alert.level >= AlertLevel::Error {
                eprintln!("{}", formatted);
            } else {
                println!("{}", formatted);
            }
        }
    }
}

/// Callback handler that calls a custom function
pub struct CallbackHandler<F>
where
    F: Fn(&Alert) + Send + Sync,
{
    callback: F,
    min_level: AlertLevel,
}

impl<F> CallbackHandler<F>
where
    F: Fn(&Alert) + Send + Sync,
{
    pub fn new(callback: F) -> Self {
        Self {
            callback,
            min_level: AlertLevel::Trace,
        }
    }

    pub fn with_min_level(mut self, level: AlertLevel) -> Self {
        self.min_level = level;
        self
    }
}

impl<F> AlertHandler for CallbackHandler<F>
where
    F: Fn(&Alert) + Send + Sync,
{
    fn handle(&self, alert: &Alert) {
        if alert.level >= self.min_level {
            (self.callback)(alert);
        }
    }
}

/// Multi-handler that dispatches to multiple handlers
#[derive(Default)]
pub struct MultiHandler {
    handlers: Vec<Box<dyn AlertHandler>>,
}

impl MultiHandler {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_handler(mut self, handler: Box<dyn AlertHandler>) -> Self {
        self.handlers.push(handler);
        self
    }

    pub fn handler_count(&self) -> usize {
        self.handlers.len()
    }
}

impl AlertHandler for MultiHandler {
    fn handle(&self, alert: &Alert) {
        for handler in &self.handlers {
            handler.handle(alert);
        }
    }
}

/// Filter handler that only passes alerts matching a predicate
pub struct FilterHandler<F, H>
where
    F: Fn(&Alert) -> bool + Send + Sync,
    H: AlertHandler,
{
    filter: F,
    handler: H,
}

impl<F, H> FilterHandler<F, H>
where
    F: Fn(&Alert) -> bool + Send + Sync,
    H: AlertHandler,
{
    pub fn new(filter: F, handler: H) -> Self {
        Self { filter, handler }
    }
}

impl<F, H> AlertHandler for FilterHandler<F, H>
where
    F: Fn(&Alert) -> bool + Send + Sync,
    H: AlertHandler,
{
    fn handle(&self, alert: &Alert) {
        if (self.filter)(alert) {
            self.handler.handle(alert);
        }
    }
}

/// Buffer handler that collects alerts in memory
#[derive(Debug, Clone)]
pub struct BufferHandler {
    buffer: std::sync::Arc<std::sync::Mutex<Vec<Alert>>>,
    max_size: Option<usize>,
}

impl BufferHandler {
    pub fn new() -> Self {
        Self {
            buffer: std::sync::Arc::new(std::sync::Mutex::new(Vec::new())),
            max_size: None,
        }
    }

    pub fn with_max_size(mut self, size: usize) -> Self {
        self.max_size = Some(size);
        self
    }

    pub fn get_alerts(&self) -> Vec<Alert> {
        self.buffer.lock().unwrap().clone()
    }

    pub fn clear(&self) {
        self.buffer.lock().unwrap().clear();
    }

    pub fn len(&self) -> usize {
        self.buffer.lock().unwrap().len()
    }

    pub fn is_empty(&self) -> bool {
        self.buffer.lock().unwrap().is_empty()
    }
}

impl Default for BufferHandler {
    fn default() -> Self {
        Self::new()
    }
}

impl AlertHandler for BufferHandler {
    fn handle(&self, alert: &Alert) {
        let mut buffer = self.buffer.lock().unwrap();
        
        if let Some(max) = self.max_size {
            if buffer.len() >= max {
                buffer.remove(0);
            }
        }
        
        buffer.push(alert.clone());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::formatter::SimpleFormatter;
    use std::sync::{Arc, atomic::{AtomicUsize, Ordering}};

    #[test]
    fn test_console_handler_min_level() {
        let handler = ConsoleHandler::new(SimpleFormatter)
            .with_min_level(AlertLevel::Warning);

        let info_alert = Alert::info("This should not be printed");
        let warning_alert = Alert::warning("This should be printed");

        // These won't actually print in tests, but we're testing the structure
        handler.handle(&info_alert);
        handler.handle(&warning_alert);
    }

    #[test]
    fn test_callback_handler() {
        let counter = Arc::new(AtomicUsize::new(0));
        let counter_clone = counter.clone();

        let handler = CallbackHandler::new(move |_| {
            counter_clone.fetch_add(1, Ordering::SeqCst);
        });

        handler.handle(&Alert::info("Test"));
        assert_eq!(counter.load(Ordering::SeqCst), 1);
    }

    #[test]
    fn test_multi_handler() {
        let counter1 = Arc::new(AtomicUsize::new(0));
        let counter2 = Arc::new(AtomicUsize::new(0));
        
        let c1 = counter1.clone();
        let c2 = counter2.clone();

        let handler = MultiHandler::new()
            .add_handler(Box::new(CallbackHandler::new(move |_| {
                c1.fetch_add(1, Ordering::SeqCst);
            })))
            .add_handler(Box::new(CallbackHandler::new(move |_| {
                c2.fetch_add(1, Ordering::SeqCst);
            })));

        assert_eq!(handler.handler_count(), 2);
        
        handler.handle(&Alert::warning("Test"));
        
        assert_eq!(counter1.load(Ordering::SeqCst), 1);
        assert_eq!(counter2.load(Ordering::SeqCst), 1);
    }

    #[test]
    fn test_filter_handler() {
        let counter = Arc::new(AtomicUsize::new(0));
        let counter_clone = counter.clone();

        let callback = CallbackHandler::new(move |_| {
            counter_clone.fetch_add(1, Ordering::SeqCst);
        });

        let handler = FilterHandler::new(
            |alert| alert.level >= AlertLevel::Error,
            callback,
        );

        handler.handle(&Alert::info("Should not trigger"));
        handler.handle(&Alert::error("Should trigger"));
        handler.handle(&Alert::critical("Should trigger"));

        assert_eq!(counter.load(Ordering::SeqCst), 2);
    }

    #[test]
    fn test_buffer_handler() {
        let handler = BufferHandler::new();

        assert_eq!(handler.len(), 0);
        assert!(handler.is_empty());

        handler.handle(&Alert::info("Test 1"));
        handler.handle(&Alert::warning("Test 2"));

        assert_eq!(handler.len(), 2);
        assert!(!handler.is_empty());

        let alerts = handler.get_alerts();
        assert_eq!(alerts.len(), 2);
        assert_eq!(alerts[0].message, "Test 1");
        assert_eq!(alerts[1].message, "Test 2");

        handler.clear();
        assert_eq!(handler.len(), 0);
    }

    #[test]
    fn test_buffer_handler_max_size() {
        let handler = BufferHandler::new().with_max_size(2);

        handler.handle(&Alert::info("Test 1"));
        handler.handle(&Alert::info("Test 2"));
        handler.handle(&Alert::info("Test 3"));

        assert_eq!(handler.len(), 2);
        
        let alerts = handler.get_alerts();
        assert_eq!(alerts[0].message, "Test 2");
        assert_eq!(alerts[1].message, "Test 3");
    }
}
