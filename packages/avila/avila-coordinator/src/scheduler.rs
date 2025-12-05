//! # Scheduler - Task scheduling strategies
//!
//! This module provides multiple scheduling strategies for task execution.
//! Each scheduler implements the `Scheduler` trait and can be used to select
//! the next task to execute from a list of pending tasks.
//!
//! Available schedulers:
//! - `FifoScheduler`: First-In-First-Out (basic FIFO ordering)
//! - `PriorityScheduler`: Priority-based (highest priority first)
//! - `FairScheduler`: Fair scheduling (round-robin)
//! - `DeadlineScheduler`: Deadline-based (earliest deadline first)
//! - `WeightedScheduler`: Weight-based (highest weight first)
extern crate alloc;
use alloc::vec::Vec;
use alloc::collections::BTreeMap;
use crate::task::{Task, TaskState};
use crate::types::TaskId;

/// Trait for task scheduling strategies
pub trait Scheduler {
    fn next_task(&mut self, tasks: &[Task]) -> Option<TaskId>;
}

/// First-In-First-Out scheduler
pub struct FifoScheduler;

impl Scheduler for FifoScheduler {
    fn next_task(&mut self, tasks: &[Task]) -> Option<TaskId> {
        tasks.iter()
            .find(|t| t.state == TaskState::Pending)
            .map(|t| t.id)
    }
}

impl Default for FifoScheduler {
    fn default() -> Self {
        Self
    }
}

/// Priority-based scheduler (highest priority first)
pub struct PriorityScheduler;

impl Scheduler for PriorityScheduler {
    fn next_task(&mut self, tasks: &[Task]) -> Option<TaskId> {
        tasks.iter()
            .filter(|t| t.state == TaskState::Pending)
            .max_by_key(|t| t.priority)
            .map(|t| t.id)
    }
}

impl Default for PriorityScheduler {
    fn default() -> Self {
        Self
    }
}

/// Fair scheduler using round-robin approach
pub struct FairScheduler {
    last_index: usize,
}

impl FairScheduler {
    pub fn new() -> Self {
        Self { last_index: 0 }
    }
}

impl Scheduler for FairScheduler {
    fn next_task(&mut self, tasks: &[Task]) -> Option<TaskId> {
        if tasks.is_empty() {
            return None;
        }

        let pending_tasks: Vec<&Task> = tasks.iter()
            .filter(|t| t.state == TaskState::Pending)
            .collect();

        if pending_tasks.is_empty() {
            return None;
        }

        self.last_index = (self.last_index + 1) % pending_tasks.len();
        Some(pending_tasks[self.last_index].id)
    }
}

impl Default for FairScheduler {
    fn default() -> Self {
        Self::new()
    }
}

/// Deadline-based scheduler (earliest deadline first)
///
/// This scheduler selects tasks based on their deadline timestamps.
/// Tasks with earlier deadlines are executed first (EDF - Earliest Deadline First).
/// Tasks without a deadline are treated as having infinite deadline.
///
/// # Example
/// ```ignore
/// let mut scheduler = DeadlineScheduler::new();
/// let next_task = scheduler.next_task(&tasks);
/// ```
#[derive(Clone, Debug)]
pub struct DeadlineScheduler {
    deadlines: BTreeMap<TaskId, u64>,
    current_time: u64,
}

impl DeadlineScheduler {
    /// Create a new deadline scheduler with current time = 0
    pub fn new() -> Self {
        Self {
            deadlines: BTreeMap::new(),
            current_time: 0,
        }
    }

    /// Create a new deadline scheduler with a specific current time
    pub fn with_time(current_time: u64) -> Self {
        Self {
            deadlines: BTreeMap::new(),
            current_time,
        }
    }

    /// Set the deadline for a task (in milliseconds)
    pub fn set_deadline(&mut self, task_id: TaskId, deadline: u64) {
        self.deadlines.insert(task_id, deadline);
    }

    /// Remove deadline for a task
    pub fn remove_deadline(&mut self, task_id: TaskId) {
        self.deadlines.remove(&task_id);
    }

    /// Advance the current time (for simulation purposes)
    pub fn advance_time(&mut self, delta_ms: u64) {
        self.current_time += delta_ms;
    }

    /// Set the absolute current time
    pub fn set_current_time(&mut self, time: u64) {
        self.current_time = time;
    }

    /// Get the current time
    pub fn current_time(&self) -> u64 {
        self.current_time
    }

    /// Check if a task has exceeded its deadline
    pub fn is_overdue(&self, task_id: TaskId) -> bool {
        self.deadlines
            .get(&task_id)
            .map(|deadline| *deadline < self.current_time)
            .unwrap_or(false)
    }

    /// Get time remaining until deadline (returns 0 if overdue)
    pub fn time_to_deadline(&self, task_id: TaskId) -> u64 {
        self.deadlines
            .get(&task_id)
            .map(|deadline| {
                if *deadline > self.current_time {
                    deadline - self.current_time
                } else {
                    0
                }
            })
            .unwrap_or(u64::MAX)
    }
}

impl Default for DeadlineScheduler {
    fn default() -> Self {
        Self::new()
    }
}

impl Scheduler for DeadlineScheduler {
    fn next_task(&mut self, tasks: &[Task]) -> Option<TaskId> {
        tasks
            .iter()
            .filter(|t| t.state == TaskState::Pending)
            .min_by_key(|t| {
                // Tasks with deadline get priority, sorted by deadline
                // Tasks without deadline are sorted last
                self.deadlines
                    .get(&t.id)
                    .copied()
                    .unwrap_or(u64::MAX)
            })
            .map(|t| t.id)
    }
}

/// Weighted scheduler (weight-based priority with load balancing)
///
/// This scheduler selects tasks based on custom weights assigned to each task.
/// Tasks with higher weights are prioritized over lower-weight tasks.
/// It also balances load by considering the number of pending tasks.
///
/// # Example
/// ```ignore
/// let mut scheduler = WeightedScheduler::new();
/// scheduler.set_weight(task_id, 10);
/// let next_task = scheduler.next_task(&tasks);
/// ```
#[derive(Clone, Debug)]
pub struct WeightedScheduler {
    weights: BTreeMap<TaskId, u32>,
    default_weight: u32,
}

impl WeightedScheduler {
    /// Create a new weighted scheduler with default weight of 1
    pub fn new() -> Self {
        Self {
            weights: BTreeMap::new(),
            default_weight: 1,
        }
    }

    /// Create a new weighted scheduler with custom default weight
    pub fn with_default_weight(default_weight: u32) -> Self {
        Self {
            weights: BTreeMap::new(),
            default_weight,
        }
    }

    /// Set the weight for a specific task
    pub fn set_weight(&mut self, task_id: TaskId, weight: u32) {
        if weight > 0 {
            self.weights.insert(task_id, weight);
        }
    }

    /// Remove weight assignment for a task (reverts to default)
    pub fn remove_weight(&mut self, task_id: TaskId) {
        self.weights.remove(&task_id);
    }

    /// Get the weight for a task
    pub fn get_weight(&self, task_id: TaskId) -> u32 {
        self.weights
            .get(&task_id)
            .copied()
            .unwrap_or(self.default_weight)
    }

    /// Set the default weight for new tasks
    pub fn set_default_weight(&mut self, weight: u32) {
        if weight > 0 {
            self.default_weight = weight;
        }
    }

    /// Get the total weight of all pending tasks
    pub fn total_weight(&self, tasks: &[Task]) -> u32 {
        tasks
            .iter()
            .filter(|t| t.state == TaskState::Pending)
            .map(|t| self.get_weight(t.id))
            .sum()
    }
}

impl Default for WeightedScheduler {
    fn default() -> Self {
        Self::new()
    }
}

impl Scheduler for WeightedScheduler {
    fn next_task(&mut self, tasks: &[Task]) -> Option<TaskId> {
        tasks
            .iter()
            .filter(|t| t.state == TaskState::Pending)
            .max_by_key(|t| self.get_weight(t.id))
            .map(|t| t.id)
    }
}
