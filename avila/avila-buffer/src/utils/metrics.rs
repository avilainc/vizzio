//! Performance metrics and monitoring

/// Buffer performance metrics
#[derive(Debug, Clone, Default)]
pub struct BufferMetrics {
    /// Total bytes written
    pub bytes_written: u64,

    /// Total bytes read
    pub bytes_read: u64,

    /// Number of write operations
    pub write_operations: u64,

    /// Number of read operations
    pub read_operations: u64,

    /// Peak capacity reached
    pub peak_capacity: usize,

    /// Number of reallocations
    pub reallocations: u64,
}

impl BufferMetrics {
    /// Create new metrics
    pub fn new() -> Self {
        Self::default()
    }

    /// Reset all metrics
    pub fn reset(&mut self) {
        *self = Self::default();
    }

    /// Average bytes per write operation
    pub fn avg_write_size(&self) -> f64 {
        if self.write_operations == 0 {
            0.0
        } else {
            self.bytes_written as f64 / self.write_operations as f64
        }
    }

    /// Average bytes per read operation
    pub fn avg_read_size(&self) -> f64 {
        if self.read_operations == 0 {
            0.0
        } else {
            self.bytes_read as f64 / self.read_operations as f64
        }
    }
}

// Full metrics implementation to be added in Phase 2
