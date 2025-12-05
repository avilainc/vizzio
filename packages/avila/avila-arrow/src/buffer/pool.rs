//! Memory pooling for efficient buffer allocation

use std::sync::Arc;
use std::sync::Mutex;

/// Memory pool for buffer allocation
pub struct MemoryPool {
    pools: Arc<Mutex<Vec<Vec<u8>>>>,
}

impl MemoryPool {
    pub fn new() -> Self {
        Self {
            pools: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub fn allocate(&self, size: usize) -> Vec<u8> {
        let mut pools = self.pools.lock().unwrap();
        pools.pop().unwrap_or_else(|| Vec::with_capacity(size))
    }

    pub fn release(&self, buffer: Vec<u8>) {
        let mut pools = self.pools.lock().unwrap();
        pools.push(buffer);
    }
}

impl Default for MemoryPool {
    fn default() -> Self {
        Self::new()
    }
}
