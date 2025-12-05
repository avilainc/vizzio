//! Task executor
//! 
//! Pure Rust implementation of task execution engine.

use std::collections::VecDeque;
use std::sync::{Arc, Mutex};
use std::task::Context;

use super::task::Task;
use super::waker::AvilaWaker;

/// Per-worker queue
struct WorkerQueue {
    tasks: Mutex<VecDeque<Task>>,
}

impl WorkerQueue {
    fn new() -> Self {
        Self {
            tasks: Mutex::new(VecDeque::new()),
        }
    }

    fn push(&self, task: Task) {
        if let Ok(mut tasks) = self.tasks.lock() {
            tasks.push_back(task);
        }
    }

    fn pop(&self) -> Option<Task> {
        self.tasks.lock().ok()?.pop_front()
    }

    fn steal(&self) -> Option<Task> {
        // Steal from the back (LIFO for stealing)
        self.tasks.lock().ok()?.pop_back()
    }

    fn len(&self) -> usize {
        self.tasks.lock().map(|t| t.len()).unwrap_or(0)
    }

    fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

/// Executor statistics
#[derive(Debug, Default, Clone)]
pub struct ExecutorStats {
    pub tasks_executed: u64,
    pub tasks_completed: u64,
    pub tasks_stolen: u64,
    pub poll_count: u64,
}

/// Task executor
pub struct Executor {
    /// Per-worker queues
    queues: Vec<Arc<WorkerQueue>>,
    /// Waker for task notifications
    waker: Arc<AvilaWaker>,
    /// Statistics
    stats: Arc<Mutex<ExecutorStats>>,
}

impl Executor {
    /// Create a new executor
    pub fn new(num_workers: usize) -> Self {
        let queues = (0..num_workers)
            .map(|_| Arc::new(WorkerQueue::new()))
            .collect();

        Self {
            queues,
            waker: Arc::new(AvilaWaker::new()),
            stats: Arc::new(Mutex::new(ExecutorStats::default())),
        }
    }

    /// Submit a task to a specific worker
    pub fn submit(&self, task: Task, worker_id: usize) {
        if worker_id < self.queues.len() {
            self.queues[worker_id].push(task);
        }
    }

    /// Execute a task on a worker
    pub fn execute(&self, mut task: Task, worker_id: usize) {
        let task_id = task.id();
        let waker = self.waker.create_waker(task_id);
        let mut context = Context::from_waker(&waker);

        // Update stats
        if let Ok(mut stats) = self.stats.lock() {
            stats.tasks_executed += 1;
            stats.poll_count += 1;
        }

        // Poll the task
        match task.poll(&mut context) {
            std::task::Poll::Ready(()) => {
                // Task completed
                if let Ok(mut stats) = self.stats.lock() {
                    stats.tasks_completed += 1;
                }
            }
            std::task::Poll::Pending => {
                // Task not ready, re-queue it
                self.submit(task, worker_id);
            }
        }
    }

    /// Try to get next task for a worker
    pub fn next_task(&self, worker_id: usize) -> Option<Task> {
        if worker_id >= self.queues.len() {
            return None;
        }

        // Try local queue first
        if let Some(task) = self.queues[worker_id].pop() {
            return Some(task);
        }

        // Try stealing from other workers
        self.try_steal(worker_id)
    }

    /// Try to steal work from other workers
    fn try_steal(&self, worker_id: usize) -> Option<Task> {
        let num_workers = self.queues.len();
        
        // Start from the next worker
        for i in 1..num_workers {
            let target = (worker_id + i) % num_workers;
            
            if let Some(task) = self.queues[target].steal() {
                // Update stats
                if let Ok(mut stats) = self.stats.lock() {
                    stats.tasks_stolen += 1;
                }
                return Some(task);
            }
        }

        None
    }

    /// Get queue length for a worker
    pub fn queue_len(&self, worker_id: usize) -> usize {
        if worker_id < self.queues.len() {
            self.queues[worker_id].len()
        } else {
            0
        }
    }

    /// Get total queue length
    pub fn total_queue_len(&self) -> usize {
        self.queues.iter().map(|q| q.len()).sum()
    }

    /// Check if all queues are empty
    pub fn is_idle(&self) -> bool {
        self.queues.iter().all(|q| q.is_empty())
    }

    /// Get executor statistics
    pub fn stats(&self) -> ExecutorStats {
        self.stats.lock().unwrap().clone()
    }

    /// Reset statistics
    pub fn reset_stats(&self) {
        if let Ok(mut stats) = self.stats.lock() {
            *stats = ExecutorStats::default();
        }
    }
}

/// Executor builder for advanced configuration
pub struct ExecutorBuilder {
    num_workers: usize,
}

impl ExecutorBuilder {
    /// Create a new executor builder
    pub fn new() -> Self {
        Self {
            num_workers: num_cpus(),
        }
    }

    /// Set the number of workers
    pub fn num_workers(mut self, n: usize) -> Self {
        self.num_workers = n.max(1);
        self
    }

    /// Build the executor
    pub fn build(self) -> Executor {
        Executor::new(self.num_workers)
    }
}

impl Default for ExecutorBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// Get number of CPUs
fn num_cpus() -> usize {
    std::thread::available_parallelism()
        .map(|n| n.get())
        .unwrap_or(4)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicBool, Ordering};

    #[test]
    fn test_executor_creation() {
        let executor = Executor::new(4);
        assert_eq!(executor.queues.len(), 4);
        assert!(executor.is_idle());
    }

    #[test]
    fn test_task_submission() {
        let executor = Executor::new(2);
        let ran = Arc::new(AtomicBool::new(false));
        let ran_clone = Arc::clone(&ran);
        
        let task = Task::new(async move {
            ran_clone.store(true, Ordering::SeqCst);
        });
        
        executor.submit(task, 0);
        assert_eq!(executor.queue_len(0), 1);
    }

    #[test]
    fn test_work_stealing() {
        let executor = Executor::new(2);
        
        // Add tasks to worker 0
        for i in 0..5 {
            let task = Task::named(format!("task-{}", i), async {});
            executor.submit(task, 0);
        }
        
        assert_eq!(executor.queue_len(0), 5);
        assert_eq!(executor.queue_len(1), 0);
        
        // Worker 1 should be able to steal
        let stolen = executor.next_task(1);
        assert!(stolen.is_some());
    }
}
