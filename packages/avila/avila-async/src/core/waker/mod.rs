//! Waker system for task notification
//! 
//! Pure Rust implementation without external dependencies.

use std::sync::{Arc, Mutex};
use std::task::{RawWaker, RawWakerVTable, Waker};
use std::collections::VecDeque;

/// Waker data shared between tasks
struct WakerData {
    /// Queue of tasks ready to be woken
    ready_queue: Mutex<VecDeque<u64>>,
}

impl WakerData {
    fn new() -> Self {
        Self {
            ready_queue: Mutex::new(VecDeque::new()),
        }
    }

    fn wake(&self, task_id: u64) {
        if let Ok(mut queue) = self.ready_queue.lock() {
            queue.push_back(task_id);
        }
    }

    fn take_ready(&self) -> Option<u64> {
        self.ready_queue
            .lock()
            .ok()
            .and_then(|mut queue| queue.pop_front())
    }
}

/// Waker implementation for Avila Async
pub struct AvilaWaker {
    data: Arc<WakerData>,
}

impl AvilaWaker {
    /// Create a new waker
    pub fn new() -> Self {
        Self {
            data: Arc::new(WakerData::new()),
        }
    }

    /// Wake a task by ID
    pub fn wake_task(&self, task_id: u64) {
        self.data.wake(task_id);
    }

    /// Get next ready task
    pub fn next_ready(&self) -> Option<u64> {
        self.data.take_ready()
    }

    /// Create a Waker for a specific task
    pub fn create_waker(&self, task_id: u64) -> Waker {
        let data = Arc::clone(&self.data);
        let raw_waker = create_raw_waker(task_id, data);
        unsafe { Waker::from_raw(raw_waker) }
    }
}

impl Default for AvilaWaker {
    fn default() -> Self {
        Self::new()
    }
}

/// Create a raw waker for a task
fn create_raw_waker(task_id: u64, data: Arc<WakerData>) -> RawWaker {
    // Pack task_id and Arc into a single pointer
    let ptr = Box::into_raw(Box::new((task_id, data)));
    
    RawWaker::new(
        ptr as *const (),
        &RawWakerVTable::new(
            clone_waker,
            wake_waker,
            wake_by_ref_waker,
            drop_waker,
        ),
    )
}

/// Clone the waker
unsafe fn clone_waker(ptr: *const ()) -> RawWaker {
    let data_ptr = ptr as *const (u64, Arc<WakerData>);
    let (task_id, data) = &*data_ptr;
    create_raw_waker(*task_id, Arc::clone(data))
}

/// Wake the task (consuming the waker)
unsafe fn wake_waker(ptr: *const ()) {
    let data_ptr = ptr as *mut (u64, Arc<WakerData>);
    let (task_id, data) = Box::from_raw(data_ptr);
    data.wake(task_id);
}

/// Wake the task (by reference)
unsafe fn wake_by_ref_waker(ptr: *const ()) {
    let data_ptr = ptr as *const (u64, Arc<WakerData>);
    let (task_id, data) = &*data_ptr;
    data.wake(*task_id);
}

/// Drop the waker
unsafe fn drop_waker(ptr: *const ()) {
    let data_ptr = ptr as *mut (u64, Arc<WakerData>);
    drop(Box::from_raw(data_ptr));
}

/// Waker pool for efficient waker reuse
pub struct WakerPool {
    pool: Mutex<Vec<Arc<WakerData>>>,
    capacity: usize,
}

impl WakerPool {
    /// Create a new waker pool
    pub fn new(capacity: usize) -> Self {
        Self {
            pool: Mutex::new(Vec::with_capacity(capacity)),
            capacity,
        }
    }

    /// Get or create a waker
    pub fn get(&self) -> AvilaWaker {
        if let Ok(mut pool) = self.pool.lock() {
            if let Some(data) = pool.pop() {
                return AvilaWaker { data };
            }
        }
        AvilaWaker::new()
    }

    /// Return a waker to the pool
    pub fn recycle(&self, waker: AvilaWaker) {
        if let Ok(mut pool) = self.pool.lock() {
            if pool.len() < self.capacity {
                pool.push(waker.data);
            }
        }
    }

    /// Get pool size
    pub fn size(&self) -> usize {
        self.pool.lock().map(|p| p.len()).unwrap_or(0)
    }
}

impl Default for WakerPool {
    fn default() -> Self {
        Self::new(1024)
    }
}

/// Waker statistics
#[derive(Debug, Default, Clone)]
pub struct WakerStats {
    pub wakes: u64,
    pub clones: u64,
    pub drops: u64,
}

impl WakerStats {
    pub fn new() -> Self {
        Self::default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_waker_creation() {
        let waker = AvilaWaker::new();
        let task_waker = waker.create_waker(1);
        task_waker.wake_by_ref();
        
        assert_eq!(waker.next_ready(), Some(1));
    }

    #[test]
    fn test_waker_pool() {
        let pool = WakerPool::new(10);
        let waker = pool.get();
        assert_eq!(pool.size(), 0);
        
        pool.recycle(waker);
        assert_eq!(pool.size(), 1);
    }
}
