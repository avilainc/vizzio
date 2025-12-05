//! # Coordinator - Main task coordination logic
extern crate alloc;
use alloc::vec::Vec;
use crate::task::{Task, TaskState};
use crate::types::{TaskId, TaskError};

pub struct Coordinator {
    pub tasks: Vec<Task>,
}

impl Coordinator {
    pub fn new() -> Self {
        Self { tasks: Vec::new() }
    }

    pub fn submit(&mut self, id: u64) {
        let task_id = TaskId::new(id);
        self.tasks.push(Task::new(task_id));
    }

    pub fn submit_with_priority(&mut self, id: u64, priority: crate::priority::Priority) {
        let task_id = TaskId::new(id);
        self.tasks.push(Task::with_priority(task_id, priority));
    }

    pub fn start(&mut self, id: u64) -> Result<(), TaskError> {
        let task_id = TaskId::new(id);
        if let Some(task) = self.tasks.iter_mut().find(|t| t.id == task_id) {
            task.start();
            Ok(())
        } else {
            Err(TaskError::NotFound)
        }
    }

    pub fn complete(&mut self, id: u64) -> Result<(), TaskError> {
        let task_id = TaskId::new(id);
        if let Some(task) = self.tasks.iter_mut().find(|t| t.id == task_id) {
            task.complete();
            Ok(())
        } else {
            Err(TaskError::NotFound)
        }
    }

    pub fn fail(&mut self, id: u64) -> Result<(), TaskError> {
        let task_id = TaskId::new(id);
        if let Some(task) = self.tasks.iter_mut().find(|t| t.id == task_id) {
            task.fail();
            Ok(())
        } else {
            Err(TaskError::NotFound)
        }
    }

    pub fn get_task(&self, id: u64) -> Option<&Task> {
        let task_id = TaskId::new(id);
        self.tasks.iter().find(|t| t.id == task_id)
    }

    pub fn tasks_by_state(&self, state: TaskState) -> Vec<&Task> {
        self.tasks.iter().filter(|t| t.state == state).collect()
    }

    pub fn remove_task(&mut self, id: u64) -> Result<Task, TaskError> {
        let task_id = TaskId::new(id);
        if let Some(pos) = self.tasks.iter().position(|t| t.id == task_id) {
            Ok(self.tasks.remove(pos))
        } else {
            Err(TaskError::NotFound)
        }
    }

    pub fn clear_completed(&mut self) -> usize {
        let initial_count = self.tasks.len();
        self.tasks.retain(|t| t.state != TaskState::Completed);
        initial_count - self.tasks.len()
    }

    pub fn task_count(&self) -> usize {
        self.tasks.len()
    }

    pub fn task_count_by_state(&self, state: TaskState) -> usize {
        self.tasks.iter().filter(|t| t.state == state).count()
    }

    pub fn iter(&self) -> impl Iterator<Item = &Task> {
        self.tasks.iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut Task> {
        self.tasks.iter_mut()
    }
}

impl Default for Coordinator {
    fn default() -> Self {
        Self::new()
    }
}
