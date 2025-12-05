//! # Concurrent - Thread-safe coordination
//!
//! Note: This module requires std for Arc and Mutex.
//! For no_std environments, this module should not be used.

#[cfg(feature = "std")]
pub mod sync {
    extern crate std;
    use std::sync::{Arc, Mutex};
    use std::thread;
    use crate::coordinator::Coordinator;
    use crate::types::TaskError;

    /// Thread-safe wrapper for Coordinator
    pub struct ConcurrentCoordinator {
        inner: Arc<Mutex<Coordinator>>,
    }

    impl ConcurrentCoordinator {
        pub fn new() -> Self {
            Self {
                inner: Arc::new(Mutex::new(Coordinator::new())),
            }
        }

        pub fn clone_handle(&self) -> Self {
            Self {
                inner: Arc::clone(&self.inner),
            }
        }

        pub fn submit(&self, id: u64) {
            if let Ok(mut coord) = self.inner.lock() {
                coord.submit(id);
            }
        }

        pub fn complete(&self, id: u64) -> Result<(), TaskError> {
            if let Ok(mut coord) = self.inner.lock() {
                coord.complete(id)
            } else {
                Err(TaskError::InvalidState)
            }
        }

        pub fn task_count(&self) -> usize {
            if let Ok(coord) = self.inner.lock() {
                coord.task_count()
            } else {
                0
            }
        }
    }

    impl Default for ConcurrentCoordinator {
        fn default() -> Self {
            Self::new()
        }
    }

    /// Worker pool for parallel task execution
    pub struct WorkerPool {
        worker_count: usize,
    }

    impl WorkerPool {
        pub fn new(worker_count: usize) -> Self {
            Self { worker_count }
        }

        pub fn spawn<F>(&self, task: F) -> thread::JoinHandle<()>
        where
            F: FnOnce() + Send + 'static,
        {
            thread::spawn(task)
        }

        pub fn worker_count(&self) -> usize {
            self.worker_count
        }
    }

    impl Default for WorkerPool {
        fn default() -> Self {
            Self::new(4)
        }
    }
}
