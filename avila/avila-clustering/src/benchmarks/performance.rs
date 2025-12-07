//! Tracking de performance (tempo/memória)

use std::time::{Duration, Instant};

pub struct PerformanceTracker {
    start_time: Option<Instant>,
    measurements: Vec<(String, Duration)>,
}

impl PerformanceTracker {
    pub fn new() -> Self {
        Self {
            start_time: None,
            measurements: Vec::new(),
        }
    }

    pub fn start(&mut self, label: String) {
        self.start_time = Some(Instant::now());
    }

    pub fn stop(&mut self, label: String) {
        if let Some(start) = self.start_time {
            let duration = start.elapsed();
            self.measurements.push((label, duration));
            self.start_time = None;
        }
    }

    pub fn get_measurements(&self) -> &[(String, Duration)] {
        &self.measurements
    }

    pub fn report(&self) -> String {
        let mut lines = vec!["Performance Report:".to_string()];
        for (label, duration) in &self.measurements {
            lines.push(format!("  {}: {:.3}s", label, duration.as_secs_f64()));
        }
        lines.join("\n")
    }
}

impl Default for PerformanceTracker {
    fn default() -> Self {
        Self::new()
    }
}
