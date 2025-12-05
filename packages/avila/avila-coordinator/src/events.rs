//! # Events - Task event system
extern crate alloc;
use alloc::vec::Vec;
use alloc::boxed::Box;
use crate::types::TaskId;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TaskEvent {
    Submitted(TaskId),
    Started(TaskId),
    Completed(TaskId),
    Failed(TaskId),
    Retrying(TaskId, u32),
    DependencyResolved(TaskId),
}

/// Trait for handling task events
pub trait EventHandler {
    fn on_event(&mut self, event: &TaskEvent);
}

/// Event bus for managing multiple event handlers
pub struct EventBus {
    handlers: Vec<Box<dyn EventHandler>>,
}

impl EventBus {
    pub fn new() -> Self {
        Self {
            handlers: Vec::new(),
        }
    }

    pub fn subscribe(&mut self, handler: Box<dyn EventHandler>) {
        self.handlers.push(handler);
    }

    pub fn publish(&mut self, event: &TaskEvent) {
        for handler in &mut self.handlers {
            handler.on_event(event);
        }
    }

    pub fn handler_count(&self) -> usize {
        self.handlers.len()
    }
}

impl Default for EventBus {
    fn default() -> Self {
        Self::new()
    }
}
