//! # Scheduler - Task scheduling strategies
extern crate alloc;
use alloc::vec::Vec;
use crate::task::{Task, TaskState};
use crate::types::TaskId;
use crate::priority::Priority;

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
