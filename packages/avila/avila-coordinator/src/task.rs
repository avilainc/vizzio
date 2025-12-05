//! # Task - Task definition and state management
extern crate alloc;
use crate::types::TaskId;
use crate::priority::Priority;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TaskState {
    Pending,
    Running,
    Completed,
    Failed,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Task {
    pub id: TaskId,
    pub state: TaskState,
    pub priority: Priority,
}

impl Task {
    pub fn new(id: TaskId) -> Self {
        Self {
            id,
            state: TaskState::Pending,
            priority: Priority::default(),
        }
    }

    pub fn with_priority(id: TaskId, priority: Priority) -> Self {
        Self {
            id,
            state: TaskState::Pending,
            priority,
        }
    }

    pub fn start(&mut self) {
        self.state = TaskState::Running;
    }

    pub fn complete(&mut self) {
        self.state = TaskState::Completed;
    }

    pub fn fail(&mut self) {
        self.state = TaskState::Failed;
    }

    pub fn set_priority(&mut self, priority: Priority) {
        self.priority = priority;
    }
}
