//! # Types - Core type definitions

/// Unique identifier for tasks
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub struct TaskId(pub u64);

impl TaskId {
    pub fn new(id: u64) -> Self {
        Self(id)
    }

    pub fn as_u64(&self) -> u64 {
        self.0
    }
}

/// Result type for task operations
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TaskResult {
    Success,
    Failure(TaskError),
}

/// Error types for task operations
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TaskError {
    NotFound,
    InvalidState,
    DependencyFailed,
    RetryExhausted,
    ValidationFailed,
    DuplicateId,
    CircularDependency,
    InvalidTransition { from: &'static str, to: &'static str },
}

impl TaskError {
    pub fn message(&self) -> &'static str {
        match self {
            TaskError::NotFound => "Task not found",
            TaskError::InvalidState => "Invalid task state",
            TaskError::DependencyFailed => "Task dependency failed",
            TaskError::RetryExhausted => "Retry attempts exhausted",
            TaskError::ValidationFailed => "Validation failed",
            TaskError::DuplicateId => "Duplicate task ID",
            TaskError::CircularDependency => "Circular dependency detected",
            TaskError::InvalidTransition { .. } => "Invalid state transition",
        }
    }
}
