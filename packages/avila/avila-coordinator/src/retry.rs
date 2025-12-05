//! # Retry - Retry policies and resilience
extern crate alloc;
use alloc::vec::Vec;
use crate::types::TaskId;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BackoffStrategy {
    Linear,
    Exponential,
    Fixed,
}

#[derive(Clone, Debug)]
pub struct RetryPolicy {
    pub max_attempts: u32,
    pub current_attempt: u32,
    pub strategy: BackoffStrategy,
    pub base_delay_ms: u64,
}

impl RetryPolicy {
    pub fn new(max_attempts: u32) -> Self {
        Self {
            max_attempts,
            current_attempt: 0,
            strategy: BackoffStrategy::Fixed,
            base_delay_ms: 1000,
        }
    }

    pub fn with_strategy(mut self, strategy: BackoffStrategy) -> Self {
        self.strategy = strategy;
        self
    }

    pub fn with_base_delay(mut self, delay_ms: u64) -> Self {
        self.base_delay_ms = delay_ms;
        self
    }

    pub fn can_retry(&self) -> bool {
        self.current_attempt < self.max_attempts
    }

    pub fn increment(&mut self) {
        self.current_attempt += 1;
    }

    pub fn reset(&mut self) {
        self.current_attempt = 0;
    }

    pub fn calculate_delay(&self) -> u64 {
        match self.strategy {
            BackoffStrategy::Fixed => self.base_delay_ms,
            BackoffStrategy::Linear => self.base_delay_ms * (self.current_attempt as u64 + 1),
            BackoffStrategy::Exponential => {
                self.base_delay_ms * 2u64.pow(self.current_attempt)
            }
        }
    }
}

impl Default for RetryPolicy {
    fn default() -> Self {
        Self::new(3)
    }
}

#[derive(Clone, Debug)]
pub struct TaskRetryInfo {
    pub task_id: TaskId,
    pub policy: RetryPolicy,
}

impl TaskRetryInfo {
    pub fn new(task_id: TaskId, max_attempts: u32) -> Self {
        Self {
            task_id,
            policy: RetryPolicy::new(max_attempts),
        }
    }

    pub fn with_policy(task_id: TaskId, policy: RetryPolicy) -> Self {
        Self {
            task_id,
            policy,
        }
    }
}

/// Manager for tracking retry information across multiple tasks
pub struct RetryManager {
    retries: Vec<TaskRetryInfo>,
}

impl RetryManager {
    pub fn new() -> Self {
        Self {
            retries: Vec::new(),
        }
    }

    pub fn register(&mut self, info: TaskRetryInfo) {
        self.retries.push(info);
    }

    pub fn get_mut(&mut self, task_id: TaskId) -> Option<&mut TaskRetryInfo> {
        self.retries.iter_mut().find(|r| r.task_id == task_id)
    }

    pub fn remove(&mut self, task_id: TaskId) -> Option<TaskRetryInfo> {
        if let Some(pos) = self.retries.iter().position(|r| r.task_id == task_id) {
            Some(self.retries.remove(pos))
        } else {
            None
        }
    }
}

impl Default for RetryManager {
    fn default() -> Self {
        Self::new()
    }
}
