//! Performance profiling and benchmarking tools

use std::time::{Duration, Instant};
use std::fmt;

/// Performance metrics for rendering
#[derive(Debug, Clone)]
pub struct PerfMetrics {
    /// Total render time
    pub total_time: Duration,
    /// Path tracing time
    pub trace_time: Duration,
    /// Memory used (MB)
    pub memory_mb: f64,
    /// Rays per second
    pub rays_per_sec: f64,
    /// Samples per second
    pub samples_per_sec: f64,
}

impl fmt::Display for PerfMetrics {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Total: {:.2}s | Trace: {:.2}s | Memory: {:.1}MB | Rays: {:.1}M/s | Samples: {:.1}M/s",
            self.total_time.as_secs_f64(),
            self.trace_time.as_secs_f64(),
            self.memory_mb,
            self.rays_per_sec / 1e6,
            self.samples_per_sec / 1e6,
        )
    }
}

/// GPU vs CPU performance comparison
#[derive(Debug, Clone)]
pub struct BenchmarkResult {
    /// GPU metrics
    pub gpu: PerfMetrics,
    /// CPU metrics
    pub cpu: PerfMetrics,
    /// Speedup factor
    pub speedup: f64,
}

impl fmt::Display for BenchmarkResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "GPU: {} | CPU: {} | Speedup: {:.1}x",
            self.gpu, self.cpu, self.speedup
        )
    }
}

/// Simple performance timer
pub struct PerfTimer {
    start: Instant,
}

impl PerfTimer {
    /// Start new timer
    pub fn start() -> Self {
        Self {
            start: Instant::now(),
        }
    }

    /// Get elapsed time
    pub fn elapsed(&self) -> Duration {
        self.start.elapsed()
    }

    /// Reset timer
    pub fn reset(&mut self) {
        self.start = Instant::now();
    }

    /// Get elapsed as f64 seconds
    pub fn secs(&self) -> f64 {
        self.elapsed().as_secs_f64()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;

    #[test]
    fn test_perf_timer() {
        let timer = PerfTimer::start();
        thread::sleep(Duration::from_millis(10));
        let elapsed = timer.elapsed();
        assert!(elapsed.as_millis() >= 10);
    }

    #[test]
    fn test_perf_metrics_display() {
        let metrics = PerfMetrics {
            total_time: Duration::from_secs(1),
            trace_time: Duration::from_millis(900),
            memory_mb: 256.0,
            rays_per_sec: 1e9,
            samples_per_sec: 1e8,
        };
        let display = metrics.to_string();
        assert!(display.contains("1.00s"));
    }
}
