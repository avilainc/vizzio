//! # Resources - Resource management and pooling
extern crate alloc;
use alloc::vec::Vec;
use crate::types::{TaskId, TaskError};

/// Resource identifier
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ResourceId(pub u64);

impl ResourceId {
    pub fn new(id: u64) -> Self {
        Self(id)
    }
}

/// Resource state
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ResourceState {
    Available,
    InUse,
    Reserved,
}

/// Generic resource in the pool
#[derive(Clone, Debug)]
pub struct Resource {
    pub id: ResourceId,
    pub state: ResourceState,
    pub owner: Option<TaskId>,
}

impl Resource {
    pub fn new(id: ResourceId) -> Self {
        Self {
            id,
            state: ResourceState::Available,
            owner: None,
        }
    }

    pub fn acquire(&mut self, task_id: TaskId) -> Result<(), TaskError> {
        if self.state == ResourceState::Available {
            self.state = ResourceState::InUse;
            self.owner = Some(task_id);
            Ok(())
        } else {
            Err(TaskError::ValidationFailed)
        }
    }

    pub fn release(&mut self) {
        self.state = ResourceState::Available;
        self.owner = None;
    }
}

/// Resource pool manager
pub struct ResourcePool {
    resources: Vec<Resource>,
    max_size: usize,
}

impl ResourcePool {
    pub fn new(max_size: usize) -> Self {
        Self {
            resources: Vec::new(),
            max_size,
        }
    }

    pub fn add_resource(&mut self, resource: Resource) -> Result<(), TaskError> {
        if self.resources.len() >= self.max_size {
            return Err(TaskError::ValidationFailed);
        }
        self.resources.push(resource);
        Ok(())
    }

    pub fn acquire(&mut self, task_id: TaskId) -> Option<ResourceId> {
        for resource in &mut self.resources {
            if resource.state == ResourceState::Available {
                if resource.acquire(task_id).is_ok() {
                    return Some(resource.id);
                }
            }
        }
        None
    }

    pub fn release(&mut self, resource_id: ResourceId) -> Result<(), TaskError> {
        if let Some(resource) = self.resources.iter_mut().find(|r| r.id == resource_id) {
            resource.release();
            Ok(())
        } else {
            Err(TaskError::NotFound)
        }
    }

    pub fn available_count(&self) -> usize {
        self.resources.iter()
            .filter(|r| r.state == ResourceState::Available)
            .count()
    }

    pub fn in_use_count(&self) -> usize {
        self.resources.iter()
            .filter(|r| r.state == ResourceState::InUse)
            .count()
    }

    pub fn total_count(&self) -> usize {
        self.resources.len()
    }
}

/// Rate limiter for controlling task execution rate
pub struct RateLimiter {
    max_per_period: u32,
    current_count: u32,
    period_start: u64,
    period_duration_ms: u64,
}

impl RateLimiter {
    pub fn new(max_per_period: u32, period_duration_ms: u64) -> Self {
        Self {
            max_per_period,
            current_count: 0,
            period_start: 0,
            period_duration_ms,
        }
    }

    pub fn try_acquire(&mut self, now: u64) -> bool {
        // Reset if new period
        if now - self.period_start >= self.period_duration_ms {
            self.current_count = 0;
            self.period_start = now;
        }

        if self.current_count < self.max_per_period {
            self.current_count += 1;
            true
        } else {
            false
        }
    }

    pub fn reset(&mut self) {
        self.current_count = 0;
        self.period_start = 0;
    }

    pub fn remaining_capacity(&self) -> u32 {
        self.max_per_period.saturating_sub(self.current_count)
    }
}

/// Quota manager for resource limits
pub struct QuotaManager {
    task_quotas: Vec<(TaskId, u32)>,
    global_limit: u32,
    current_usage: u32,
}

impl QuotaManager {
    pub fn new(global_limit: u32) -> Self {
        Self {
            task_quotas: Vec::new(),
            global_limit,
            current_usage: 0,
        }
    }

    pub fn set_task_quota(&mut self, task_id: TaskId, quota: u32) {
        if let Some(entry) = self.task_quotas.iter_mut().find(|(id, _)| *id == task_id) {
            entry.1 = quota;
        } else {
            self.task_quotas.push((task_id, quota));
        }
    }

    pub fn can_allocate(&self, task_id: TaskId, amount: u32) -> bool {
        // Check global limit
        if self.current_usage + amount > self.global_limit {
            return false;
        }

        // Check task-specific quota if exists
        if let Some((_, quota)) = self.task_quotas.iter().find(|(id, _)| *id == task_id) {
            amount <= *quota
        } else {
            true
        }
    }

    pub fn allocate(&mut self, amount: u32) -> Result<(), TaskError> {
        if self.current_usage + amount <= self.global_limit {
            self.current_usage += amount;
            Ok(())
        } else {
            Err(TaskError::ValidationFailed)
        }
    }

    pub fn release(&mut self, amount: u32) {
        self.current_usage = self.current_usage.saturating_sub(amount);
    }

    pub fn available(&self) -> u32 {
        self.global_limit.saturating_sub(self.current_usage)
    }
}

impl Default for QuotaManager {
    fn default() -> Self {
        Self::new(100)
    }
}
