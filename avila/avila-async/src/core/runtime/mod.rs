//! Runtime initialization and configuration
//! 
//! Pure Rust implementation without external dependencies.

use std::sync::Arc;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::thread;
use std::time::Duration;

use super::executor::Executor;
use super::scheduler::Scheduler;

/// Runtime configuration
#[derive(Debug, Clone)]
pub struct RuntimeConfig {
    /// Number of worker threads
    pub worker_threads: usize,
    /// Stack size per thread (bytes)
    pub stack_size: usize,
    /// Enable work stealing
    pub work_stealing: bool,
    /// Thread name prefix
    pub thread_name: String,
}

impl Default for RuntimeConfig {
    fn default() -> Self {
        Self {
            worker_threads: num_cpus(),
            stack_size: 2 * 1024 * 1024, // 2MB
            work_stealing: true,
            thread_name: "avila-worker".to_string(),
        }
    }
}

/// Main runtime structure
pub struct Runtime {
    config: RuntimeConfig,
    scheduler: Arc<Scheduler>,
    executor: Arc<Executor>,
    is_running: Arc<AtomicBool>,
    active_tasks: Arc<AtomicUsize>,
}

impl Runtime {
    /// Create a new runtime with default configuration
    pub fn new() -> Self {
        Self::with_config(RuntimeConfig::default())
    }

    /// Create a new runtime with custom configuration
    pub fn with_config(config: RuntimeConfig) -> Self {
        let scheduler = Arc::new(Scheduler::new(config.worker_threads));
        let executor = Arc::new(Executor::new(config.worker_threads));
        
        Self {
            config,
            scheduler,
            executor,
            is_running: Arc::new(AtomicBool::new(false)),
            active_tasks: Arc::new(AtomicUsize::new(0)),
        }
    }

    /// Start the runtime
    pub fn start(&self) {
        self.is_running.store(true, Ordering::SeqCst);
        
        // Spawn worker threads
        for i in 0..self.config.worker_threads {
            let scheduler = Arc::clone(&self.scheduler);
            let executor = Arc::clone(&self.executor);
            let is_running = Arc::clone(&self.is_running);
            let thread_name = format!("{}-{}", self.config.thread_name, i);
            
            thread::Builder::new()
                .name(thread_name)
                .stack_size(self.config.stack_size)
                .spawn(move || {
                    worker_loop(scheduler, executor, is_running, i);
                })
                .expect("Failed to spawn worker thread");
        }
    }

    /// Stop the runtime gracefully
    pub fn shutdown(&self) {
        self.is_running.store(false, Ordering::SeqCst);
        
        // Wait for all tasks to complete
        while self.active_tasks.load(Ordering::SeqCst) > 0 {
            thread::sleep(Duration::from_millis(10));
        }
    }

    /// Get the number of active tasks
    pub fn active_tasks(&self) -> usize {
        self.active_tasks.load(Ordering::SeqCst)
    }

    /// Spawn a new task
    pub fn spawn<F>(&self, future: F)
    where
        F: std::future::Future<Output = ()> + Send + 'static,
    {
        self.active_tasks.fetch_add(1, Ordering::SeqCst);
        self.scheduler.schedule(Box::pin(future));
    }

    /// Block on a future until completion
    pub fn block_on<F>(&self, future: F) -> F::Output
    where
        F: std::future::Future,
    {
        // Start runtime if not running
        if !self.is_running.load(Ordering::SeqCst) {
            self.start();
        }

        // Use a simple blocking executor for the main future
        futures_lite::future::block_on(future)
    }
}

impl Default for Runtime {
    fn default() -> Self {
        Self::new()
    }
}

/// Worker thread loop
fn worker_loop(
    scheduler: Arc<Scheduler>,
    executor: Arc<Executor>,
    is_running: Arc<AtomicBool>,
    worker_id: usize,
) {
    while is_running.load(Ordering::SeqCst) {
        // Try to get a task from the scheduler
        if let Some(task) = scheduler.next_task(worker_id) {
            // Execute the task
            executor.execute(task, worker_id);
        } else {
            // No tasks available, yield
            thread::yield_now();
        }
    }
}

/// Get the number of CPU cores (pure Rust implementation)
fn num_cpus() -> usize {
    thread::available_parallelism()
        .map(|n| n.get())
        .unwrap_or(4)
}

// Simple futures implementation for block_on
mod futures_lite {
    pub mod future {
        use std::future::Future;
        use std::pin::Pin;
        use std::task::{Context, Poll, RawWaker, RawWakerVTable, Waker};

        pub fn block_on<F: Future>(mut future: F) -> F::Output {
            let mut future = unsafe { Pin::new_unchecked(&mut future) };
            
            loop {
                let waker = dummy_waker();
                let mut context = Context::from_waker(&waker);
                
                match future.as_mut().poll(&mut context) {
                    Poll::Ready(output) => return output,
                    Poll::Pending => std::thread::yield_now(),
                }
            }
        }

        fn dummy_waker() -> Waker {
            unsafe fn clone(_: *const ()) -> RawWaker {
                dummy_raw_waker()
            }
            unsafe fn wake(_: *const ()) {}
            unsafe fn wake_by_ref(_: *const ()) {}
            unsafe fn drop(_: *const ()) {}

            fn dummy_raw_waker() -> RawWaker {
                RawWaker::new(
                    std::ptr::null(),
                    &RawWakerVTable::new(clone, wake, wake_by_ref, drop),
                )
            }

            unsafe { Waker::from_raw(dummy_raw_waker()) }
        }
    }
}
