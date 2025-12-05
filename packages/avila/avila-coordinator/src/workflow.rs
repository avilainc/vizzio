//! # Workflow - DAG-based workflow engine
extern crate alloc;
use alloc::vec::Vec;
use alloc::string::String;
use crate::types::{TaskId, TaskError};
use crate::task::TaskState;

/// Workflow node representing a task in the DAG
#[derive(Clone, Debug, PartialEq)]
pub struct WorkflowNode {
    pub task_id: TaskId,
    pub dependencies: Vec<TaskId>,
    pub name: Option<String>,
}

impl WorkflowNode {
    pub fn new(task_id: TaskId) -> Self {
        Self {
            task_id,
            dependencies: Vec::new(),
            name: None,
        }
    }

    pub fn with_name(mut self, name: String) -> Self {
        self.name = Some(name);
        self
    }

    pub fn add_dependency(&mut self, dep_id: TaskId) {
        if !self.dependencies.contains(&dep_id) {
            self.dependencies.push(dep_id);
        }
    }
}

/// Directed Acyclic Graph (DAG) workflow
pub struct Workflow {
    nodes: Vec<WorkflowNode>,
    name: String,
}

impl Workflow {
    pub fn new(name: String) -> Self {
        Self {
            nodes: Vec::new(),
            name,
        }
    }

    pub fn add_node(&mut self, node: WorkflowNode) -> Result<(), TaskError> {
        // Check for duplicate IDs
        if self.nodes.iter().any(|n| n.task_id == node.task_id) {
            return Err(TaskError::DuplicateId);
        }
        self.nodes.push(node);
        Ok(())
    }

    pub fn add_edge(&mut self, from: TaskId, to: TaskId) -> Result<(), TaskError> {
        // Find the 'to' node and add 'from' as dependency
        if let Some(node) = self.nodes.iter_mut().find(|n| n.task_id == to) {
            node.add_dependency(from);

            // Check for cycles
            if self.has_cycle() {
                // Remove the dependency we just added
                node.dependencies.retain(|&id| id != from);
                return Err(TaskError::CircularDependency);
            }
            Ok(())
        } else {
            Err(TaskError::NotFound)
        }
    }

    pub fn has_cycle(&self) -> bool {
        for node in &self.nodes {
            let mut visited = Vec::new();
            if self.has_cycle_recursive(node.task_id, &mut visited) {
                return true;
            }
        }
        false
    }

    fn has_cycle_recursive(&self, task_id: TaskId, visited: &mut Vec<TaskId>) -> bool {
        if visited.contains(&task_id) {
            return true;
        }

        visited.push(task_id);

        if let Some(node) = self.nodes.iter().find(|n| n.task_id == task_id) {
            for &dep_id in &node.dependencies {
                if self.has_cycle_recursive(dep_id, visited) {
                    return true;
                }
            }
        }

        visited.pop();
        false
    }

    /// Topological sort to get execution order
    pub fn execution_order(&self) -> Result<Vec<TaskId>, TaskError> {
        if self.has_cycle() {
            return Err(TaskError::CircularDependency);
        }

        let mut result = Vec::new();
        let mut visited = Vec::new();

        for node in &self.nodes {
            if !visited.contains(&node.task_id) {
                self.topological_sort(node.task_id, &mut visited, &mut result);
            }
        }

        result.reverse();
        Ok(result)
    }

    fn topological_sort(&self, task_id: TaskId, visited: &mut Vec<TaskId>, result: &mut Vec<TaskId>) {
        visited.push(task_id);

        if let Some(node) = self.nodes.iter().find(|n| n.task_id == task_id) {
            for &dep_id in &node.dependencies {
                if !visited.contains(&dep_id) {
                    self.topological_sort(dep_id, visited, result);
                }
            }
        }

        result.push(task_id);
    }

    pub fn get_ready_tasks(&self, completed: &[TaskId]) -> Vec<TaskId> {
        self.nodes.iter()
            .filter(|node| !completed.contains(&node.task_id))
            .filter(|node| node.dependencies.iter().all(|dep| completed.contains(dep)))
            .map(|node| node.task_id)
            .collect()
    }

    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }

    pub fn get_node(&self, task_id: TaskId) -> Option<&WorkflowNode> {
        self.nodes.iter().find(|n| n.task_id == task_id)
    }
}

/// Workflow execution context
pub struct WorkflowExecution {
    workflow: Workflow,
    completed_tasks: Vec<TaskId>,
    failed_tasks: Vec<TaskId>,
    current_tasks: Vec<TaskId>,
}

impl WorkflowExecution {
    pub fn new(workflow: Workflow) -> Self {
        Self {
            workflow,
            completed_tasks: Vec::new(),
            failed_tasks: Vec::new(),
            current_tasks: Vec::new(),
        }
    }

    pub fn next_batch(&mut self) -> Vec<TaskId> {
        let ready = self.workflow.get_ready_tasks(&self.completed_tasks);
        self.current_tasks = ready.clone();
        ready
    }

    pub fn mark_completed(&mut self, task_id: TaskId) {
        if !self.completed_tasks.contains(&task_id) {
            self.completed_tasks.push(task_id);
        }
        self.current_tasks.retain(|&id| id != task_id);
    }

    pub fn mark_failed(&mut self, task_id: TaskId) {
        if !self.failed_tasks.contains(&task_id) {
            self.failed_tasks.push(task_id);
        }
        self.current_tasks.retain(|&id| id != task_id);
    }

    pub fn is_complete(&self) -> bool {
        self.completed_tasks.len() + self.failed_tasks.len() == self.workflow.node_count()
    }

    pub fn has_failures(&self) -> bool {
        !self.failed_tasks.is_empty()
    }

    pub fn progress(&self) -> f32 {
        let total = self.workflow.node_count();
        if total > 0 {
            self.completed_tasks.len() as f32 / total as f32
        } else {
            0.0
        }
    }
}
