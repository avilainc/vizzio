//! # Validation - Task validation rules
extern crate alloc;
use alloc::vec::Vec;
use crate::task::TaskState;
use crate::types::{TaskError, TaskId};

pub struct StateValidator;

impl StateValidator {
    pub fn can_transition(from: TaskState, to: TaskState) -> Result<(), TaskError> {
        match (from, to) {
            (TaskState::Pending, TaskState::Running) => Ok(()),
            (TaskState::Running, TaskState::Completed) => Ok(()),
            (TaskState::Running, TaskState::Failed) => Ok(()),
            (TaskState::Failed, TaskState::Pending) => Ok(()), // retry
            _ => Err(TaskError::InvalidTransition {
                from: Self::state_name(from),
                to: Self::state_name(to),
            }),
        }
    }

    fn state_name(state: TaskState) -> &'static str {
        match state {
            TaskState::Pending => "Pending",
            TaskState::Running => "Running",
            TaskState::Completed => "Completed",
            TaskState::Failed => "Failed",
        }
    }

    pub fn validate_transition_chain(states: &[TaskState]) -> Result<(), TaskError> {
        for window in states.windows(2) {
            Self::can_transition(window[0], window[1])?;
        }
        Ok(())
    }
}

/// Validator for task ID uniqueness
pub struct IdValidator {
    used_ids: Vec<TaskId>,
}

impl IdValidator {
    pub fn new() -> Self {
        Self {
            used_ids: Vec::new(),
        }
    }

    pub fn register(&mut self, id: TaskId) -> Result<(), TaskError> {
        if self.used_ids.contains(&id) {
            Err(TaskError::DuplicateId)
        } else {
            self.used_ids.push(id);
            Ok(())
        }
    }

    pub fn unregister(&mut self, id: TaskId) -> bool {
        if let Some(pos) = self.used_ids.iter().position(|&i| i == id) {
            self.used_ids.remove(pos);
            true
        } else {
            false
        }
    }

    pub fn is_registered(&self, id: TaskId) -> bool {
        self.used_ids.contains(&id)
    }

    pub fn clear(&mut self) {
        self.used_ids.clear();
    }
}

impl Default for IdValidator {
    fn default() -> Self {
        Self::new()
    }
}

/// Pre-condition validation rules
pub trait PreCondition {
    fn check(&self) -> Result<(), TaskError>;
}

pub struct AlwaysValid;

impl PreCondition for AlwaysValid {
    fn check(&self) -> Result<(), TaskError> {
        Ok(())
    }
}
