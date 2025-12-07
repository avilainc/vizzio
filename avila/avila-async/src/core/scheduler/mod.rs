//! Task scheduler with multiple strategies
//! 
//! Pure Rust implementation supporting FIFO, priority, and work-stealing scheduling.

use std::collections::{BinaryHeap, VecDeque};
use std::cmp::Ordering as CmpOrdering;
use std::sync::{Arc, Mutex};

use super::task::{BoxFuture, Priority, Task};

/// Scheduling strategy
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SchedulingStrategy {
    /// First-In-First-Out
    FIFO,
    /// Priority-based scheduling
    Priority,
    /// Round-robin
    RoundRobin,
    /// Work-stealing
    WorkStealing,
}

/// Wrapper for priority queue ordering
struct PriorityTask {
    task: Task,
    priority: Priority,
    sequence: u64,
}

impl PartialEq for PriorityTask {
    fn eq(&self, other: &Self) -> bool {
        self.priority == other.priority && self.sequence == other.sequence
    }
}

impl Eq for PriorityTask {}

impl PartialOrd for PriorityTask {
    fn partial_cmp(&self, other: &Self) -> Option<CmpOrdering> {
        Some(self.cmp(other))
    }
}

impl Ord for PriorityTask {
    fn cmp(&self, other: &Self) -> CmpOrdering {
        // Higher priority first, then FIFO (lower sequence first)
        match other.priority.cmp(&self.priority) {
            CmpOrdering::Equal => self.sequence.cmp(&other.sequence),
            other => other,
        }
    }
}

/// Per-worker scheduler queue
struct SchedulerQueue {
    fifo: VecDeque<Task>,
    priority: BinaryHeap<PriorityTask>,
    sequence: u64,
}

impl SchedulerQueue {
    fn new() -> Self {
        Self {
            fifo: VecDeque::new(),
            priority: BinaryHeap::new(),
            sequence: 0,
        }
    }

    fn push_fifo(&mut self, task: Task) {
        self.fifo.push_back(task);
    }

    fn push_priority(&mut self, task: Task) {
        let priority = task.priority();
        let sequence = self.sequence;
        self.sequence += 1;
        
        self.priority.push(PriorityTask {
            task,
            priority,
            sequence,
        });
    }

    fn pop_fifo(&mut self) -> Option<Task> {
        self.fifo.pop_front()
    }

    fn pop_priority(&mut self) -> Option<Task> {
        self.priority.pop().map(|pt| pt.task)
    }

    fn len_fifo(&self) -> usize {
        self.fifo.len()
    }

    fn len_priority(&self) -> usize {
        self.priority.len()
    }

    fn is_empty(&self) -> bool {
        self.fifo.is_empty() && self.priority.is_empty()
    }
}

/// Scheduler statistics
#[derive(Debug, Default, Clone)]
pub struct SchedulerStats {
    pub tasks_scheduled: u64,
    pub tasks_dispatched: u64,
    pub strategy_switches: u64,
}

/// Main scheduler
pub struct Scheduler {
    /// Per-worker queues
    queues: Vec<Arc<Mutex<SchedulerQueue>>>,
    /// Current strategy
    strategy: Arc<Mutex<SchedulingStrategy>>,
    /// Round-robin counter
    rr_counter: Arc<Mutex<usize>>,
    /// Statistics
    stats: Arc<Mutex<SchedulerStats>>,
}

impl Scheduler {
    /// Create a new scheduler
    pub fn new(num_workers: usize) -> Self {
        let queues = (0..num_workers)
            .map(|_| Arc::new(Mutex::new(SchedulerQueue::new())))
            .collect();

        Self {
            queues,
            strategy: Arc::new(Mutex::new(SchedulingStrategy::WorkStealing)),
            rr_counter: Arc::new(Mutex::new(0)),
            stats: Arc::new(Mutex::new(SchedulerStats::default())),
        }
    }

    /// Create scheduler with specific strategy
    pub fn with_strategy(num_workers: usize, strategy: SchedulingStrategy) -> Self {
        let mut scheduler = Self::new(num_workers);
        *scheduler.strategy.lock().unwrap() = strategy;
        scheduler
    }

    /// Schedule a task
    pub fn schedule(&self, future: BoxFuture) {
        let task = Task::new(future);
        self.schedule_task(task);
    }

    /// Schedule a task with priority
    pub fn schedule_with_priority(&self, future: BoxFuture, priority: Priority) {
        let task = Task::with_priority(future, priority);
        self.schedule_task(task);
    }

    /// Schedule a named task
    pub fn schedule_named(&self, name: impl Into<String>, future: BoxFuture) {
        let task = Task::named(name, future);
        self.schedule_task(task);
    }

    /// Internal task scheduling
    fn schedule_task(&self, task: Task) {
        let strategy = *self.strategy.lock().unwrap();
        
        // Update stats
        if let Ok(mut stats) = self.stats.lock() {
            stats.tasks_scheduled += 1;
        }

        match strategy {
            SchedulingStrategy::FIFO => self.schedule_fifo(task),
            SchedulingStrategy::Priority => self.schedule_priority(task),
            SchedulingStrategy::RoundRobin => self.schedule_round_robin(task),
            SchedulingStrategy::WorkStealing => self.schedule_work_stealing(task),
        }
    }

    /// Schedule using FIFO to worker 0
    fn schedule_fifo(&self, task: Task) {
        if let Ok(mut queue) = self.queues[0].lock() {
            queue.push_fifo(task);
        }
    }

    /// Schedule using priority to worker 0
    fn schedule_priority(&self, task: Task) {
        if let Ok(mut queue) = self.queues[0].lock() {
            queue.push_priority(task);
        }
    }

    /// Schedule using round-robin
    fn schedule_round_robin(&self, task: Task) {
        let worker_id = {
            let mut counter = self.rr_counter.lock().unwrap();
            let id = *counter % self.queues.len();
            *counter = (*counter + 1) % self.queues.len();
            id
        };

        if let Ok(mut queue) = self.queues[worker_id].lock() {
            queue.push_fifo(task);
        }
    }

    /// Schedule using work-stealing (least loaded)
    fn schedule_work_stealing(&self, task: Task) {
        // Find the least loaded worker
        let worker_id = self.find_least_loaded();

        if let Ok(mut queue) = self.queues[worker_id].lock() {
            queue.push_priority(task);
        }
    }

    /// Find the least loaded worker
    fn find_least_loaded(&self) -> usize {
        let mut min_load = usize::MAX;
        let mut min_worker = 0;

        for (i, queue) in self.queues.iter().enumerate() {
            if let Ok(q) = queue.lock() {
                let load = q.len_fifo() + q.len_priority();
                if load < min_load {
                    min_load = load;
                    min_worker = i;
                }
            }
        }

        min_worker
    }

    /// Get next task for a worker
    pub fn next_task(&self, worker_id: usize) -> Option<Task> {
        if worker_id >= self.queues.len() {
            return None;
        }

        let strategy = *self.strategy.lock().unwrap();

        let task = match strategy {
            SchedulingStrategy::Priority | SchedulingStrategy::WorkStealing => {
                self.queues[worker_id]
                    .lock()
                    .ok()?
                    .pop_priority()
            }
            _ => {
                self.queues[worker_id]
                    .lock()
                    .ok()?
                    .pop_fifo()
            }
        };

        if task.is_some() {
            if let Ok(mut stats) = self.stats.lock() {
                stats.tasks_dispatched += 1;
            }
        }

        task
    }

    /// Change scheduling strategy
    pub fn set_strategy(&self, strategy: SchedulingStrategy) {
        *self.strategy.lock().unwrap() = strategy;
        
        if let Ok(mut stats) = self.stats.lock() {
            stats.strategy_switches += 1;
        }
    }

    /// Get current strategy
    pub fn strategy(&self) -> SchedulingStrategy {
        *self.strategy.lock().unwrap()
    }

    /// Get queue length for a worker
    pub fn queue_len(&self, worker_id: usize) -> usize {
        if worker_id >= self.queues.len() {
            return 0;
        }

        self.queues[worker_id]
            .lock()
            .map(|q| q.len_fifo() + q.len_priority())
            .unwrap_or(0)
    }

    /// Get total queue length
    pub fn total_queue_len(&self) -> usize {
        self.queues
            .iter()
            .filter_map(|q| q.lock().ok())
            .map(|q| q.len_fifo() + q.len_priority())
            .sum()
    }

    /// Check if scheduler is idle
    pub fn is_idle(&self) -> bool {
        self.queues
            .iter()
            .filter_map(|q| q.lock().ok())
            .all(|q| q.is_empty())
    }

    /// Get scheduler statistics
    pub fn stats(&self) -> SchedulerStats {
        self.stats.lock().unwrap().clone()
    }

    /// Reset statistics
    pub fn reset_stats(&self) {
        *self.stats.lock().unwrap() = SchedulerStats::default();
    }
}

/// Scheduler builder
pub struct SchedulerBuilder {
    num_workers: usize,
    strategy: SchedulingStrategy,
}

impl SchedulerBuilder {
    pub fn new() -> Self {
        Self {
            num_workers: num_cpus(),
            strategy: SchedulingStrategy::WorkStealing,
        }
    }

    pub fn num_workers(mut self, n: usize) -> Self {
        self.num_workers = n.max(1);
        self
    }

    pub fn strategy(mut self, strategy: SchedulingStrategy) -> Self {
        self.strategy = strategy;
        self
    }

    pub fn build(self) -> Scheduler {
        Scheduler::with_strategy(self.num_workers, self.strategy)
    }
}

impl Default for SchedulerBuilder {
    fn default() -> Self {
        Self::new()
    }
}

fn num_cpus() -> usize {
    std::thread::available_parallelism()
        .map(|n| n.get())
        .unwrap_or(4)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scheduler_creation() {
        let scheduler = Scheduler::new(4);
        assert!(scheduler.is_idle());
    }

    #[test]
    fn test_scheduling_strategies() {
        let scheduler = Scheduler::new(2);
        
        scheduler.set_strategy(SchedulingStrategy::FIFO);
        assert_eq!(scheduler.strategy(), SchedulingStrategy::FIFO);
        
        scheduler.set_strategy(SchedulingStrategy::Priority);
        assert_eq!(scheduler.strategy(), SchedulingStrategy::Priority);
    }

    #[test]
    fn test_priority_scheduling() {
        let scheduler = Scheduler::with_strategy(1, SchedulingStrategy::Priority);
        
        scheduler.schedule_with_priority(Box::pin(async {}), Priority::Low);
        scheduler.schedule_with_priority(Box::pin(async {}), Priority::Critical);
        scheduler.schedule_with_priority(Box::pin(async {}), Priority::Normal);
        
        // Critical should come first
        let task = scheduler.next_task(0).unwrap();
        assert_eq!(task.priority(), Priority::Critical);
    }
}
