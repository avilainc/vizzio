use crate::types::Alert;
use crate::handler::AlertHandler;
use std::sync::{Arc, RwLock};

/// Alert manager for handling and dispatching alerts
#[derive(Clone)]
pub struct AlertManager {
    handlers: Arc<RwLock<Vec<Box<dyn AlertHandler>>>>,
}

impl Default for AlertManager {
    fn default() -> Self {
        Self::new()
    }
}

impl AlertManager {
    /// Creates a new AlertManager
    pub fn new() -> Self {
        Self {
            handlers: Arc::new(RwLock::new(Vec::new())),
        }
    }

    /// Adds a handler to the manager
    pub fn add_handler(&self, handler: Box<dyn AlertHandler>) {
        let mut handlers = self.handlers.write().unwrap();
        handlers.push(handler);
    }

    /// Removes all handlers
    pub fn clear_handlers(&self) {
        let mut handlers = self.handlers.write().unwrap();
        handlers.clear();
    }

    /// Returns the number of registered handlers
    pub fn handler_count(&self) -> usize {
        let handlers = self.handlers.read().unwrap();
        handlers.len()
    }

    /// Dispatches an alert to all registered handlers
    pub fn dispatch(&self, alert: Alert) {
        let handlers = self.handlers.read().unwrap();
        for handler in handlers.iter() {
            handler.handle(&alert);
        }
    }

    /// Convenience method to dispatch an alert
    pub fn alert(&self, alert: Alert) {
        self.dispatch(alert);
    }

    /// Dispatches multiple alerts in sequence
    pub fn dispatch_batch(&self, alerts: impl IntoIterator<Item = Alert>) {
        for alert in alerts {
            self.dispatch(alert);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::AlertLevel;
    use crate::handler::CallbackHandler;
    use std::sync::atomic::{AtomicUsize, Ordering};

    #[test]
    fn test_manager_creation() {
        let manager = AlertManager::new();
        assert_eq!(manager.handler_count(), 0);
    }

    #[test]
    fn test_manager_add_handler() {
        let manager = AlertManager::new();
        let counter = Arc::new(AtomicUsize::new(0));
        let counter_clone = counter.clone();

        let handler = CallbackHandler::new(move |_alert| {
            counter_clone.fetch_add(1, Ordering::SeqCst);
        });

        manager.add_handler(Box::new(handler));
        assert_eq!(manager.handler_count(), 1);
    }

    #[test]
    fn test_manager_dispatch() {
        let manager = AlertManager::new();
        let counter = Arc::new(AtomicUsize::new(0));
        let counter_clone = counter.clone();

        let handler = CallbackHandler::new(move |_alert| {
            counter_clone.fetch_add(1, Ordering::SeqCst);
        });

        manager.add_handler(Box::new(handler));
        manager.dispatch(Alert::info("Test"));

        assert_eq!(counter.load(Ordering::SeqCst), 1);
    }

    #[test]
    fn test_manager_multiple_handlers() {
        let manager = AlertManager::new();
        let counter1 = Arc::new(AtomicUsize::new(0));
        let counter2 = Arc::new(AtomicUsize::new(0));

        let c1 = counter1.clone();
        let c2 = counter2.clone();

        manager.add_handler(Box::new(CallbackHandler::new(move |_| {
            c1.fetch_add(1, Ordering::SeqCst);
        })));

        manager.add_handler(Box::new(CallbackHandler::new(move |_| {
            c2.fetch_add(1, Ordering::SeqCst);
        })));

        manager.dispatch(Alert::warning("Test"));

        assert_eq!(counter1.load(Ordering::SeqCst), 1);
        assert_eq!(counter2.load(Ordering::SeqCst), 1);
    }

    #[test]
    fn test_manager_clear() {
        let manager = AlertManager::new();
        let counter = Arc::new(AtomicUsize::new(0));
        let counter_clone = counter.clone();

        manager.add_handler(Box::new(CallbackHandler::new(move |_| {
            counter_clone.fetch_add(1, Ordering::SeqCst);
        })));

        assert_eq!(manager.handler_count(), 1);

        manager.clear_handlers();
        assert_eq!(manager.handler_count(), 0);
    }

    #[test]
    fn test_manager_batch_dispatch() {
        let manager = AlertManager::new();
        let counter = Arc::new(AtomicUsize::new(0));
        let counter_clone = counter.clone();

        manager.add_handler(Box::new(CallbackHandler::new(move |_| {
            counter_clone.fetch_add(1, Ordering::SeqCst);
        })));

        let alerts = vec![
            Alert::info("Test 1"),
            Alert::warning("Test 2"),
            Alert::error("Test 3"),
        ];

        manager.dispatch_batch(alerts);
        assert_eq!(counter.load(Ordering::SeqCst), 3);
    }
}
