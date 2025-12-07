//! Window operations for streaming data

use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WindowType {
    Tumbling { size: Duration },
    Sliding { size: Duration, slide: Duration },
    Session { gap: Duration },
    Count { count: usize },
}

#[derive(Debug, Clone)]
pub struct Window<T> {
    pub window_type: WindowType,
    pub data: Vec<T>,
}

impl<T> Window<T> {
    pub fn new(window_type: WindowType) -> Self {
        Self {
            window_type,
            data: Vec::new(),
        }
    }

    pub fn add(&mut self, item: T) {
        self.data.push(item);
    }

    pub fn is_complete(&self) -> bool {
        match &self.window_type {
            WindowType::Count { count } => self.data.len() >= *count,
            _ => false, // TODO: Implement time-based window completion
        }
    }

    pub fn flush(&mut self) -> Vec<T> {
        std::mem::take(&mut self.data)
    }
}

/// Windowed stream aggregation
pub struct WindowedStream<T> {
    windows: Vec<Window<T>>,
}

impl<T> WindowedStream<T> {
    pub fn new() -> Self {
        Self {
            windows: Vec::new(),
        }
    }

    pub fn add_window(&mut self, window: Window<T>) {
        self.windows.push(window);
    }

    pub fn process(&mut self, item: T) {
        // TODO: Distribute item to appropriate windows
    }

    pub fn get_complete_windows(&mut self) -> Vec<Vec<T>> {
        // TODO: Return and remove completed windows
        Vec::new()
    }
}

impl<T> Default for WindowedStream<T> {
    fn default() -> Self {
        Self::new()
    }
}
