//! # Dependencies - Task dependency management
extern crate alloc;
use alloc::vec::Vec;
use crate::types::TaskId;

#[derive(Clone, Debug, PartialEq)]
pub struct TaskDependency {
    pub task_id: TaskId,
    pub depends_on: Vec<TaskId>,
}

impl TaskDependency {
    pub fn new(task_id: TaskId) -> Self {
        Self {
            task_id,
            depends_on: Vec::new(),
        }
    }

    pub fn add_dependency(&mut self, depends_on: TaskId) {
        self.depends_on.push(depends_on);
    }
}

pub struct DependencyGraph {
    dependencies: Vec<TaskDependency>,
}

impl DependencyGraph {
    pub fn new() -> Self {
        Self {
            dependencies: Vec::new(),
        }
    }

    pub fn add_dependency(&mut self, task_id: TaskId, depends_on: TaskId) {
        if let Some(dep) = self.dependencies.iter_mut().find(|d| d.task_id == task_id) {
            dep.add_dependency(depends_on);
        } else {
            let mut new_dep = TaskDependency::new(task_id);
            new_dep.add_dependency(depends_on);
            self.dependencies.push(new_dep);
        }
    }

    pub fn get_dependencies(&self, task_id: TaskId) -> Option<&Vec<TaskId>> {
        self.dependencies
            .iter()
            .find(|d| d.task_id == task_id)
            .map(|d| &d.depends_on)
    }

    pub fn has_cycle(&self) -> bool {
        for dep in &self.dependencies {
            let mut visited = Vec::new();
            if self.has_cycle_recursive(dep.task_id, &mut visited) {
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

        if let Some(deps) = self.get_dependencies(task_id) {
            for dep_id in deps {
                if self.has_cycle_recursive(*dep_id, visited) {
                    return true;
                }
            }
        }

        visited.pop();
        false
    }

    pub fn can_execute(&self, task_id: TaskId, completed_tasks: &[TaskId]) -> bool {
        if let Some(deps) = self.get_dependencies(task_id) {
            deps.iter().all(|dep| completed_tasks.contains(dep))
        } else {
            true
        }
    }

    pub fn get_ready_tasks(&self, all_tasks: &[TaskId], completed_tasks: &[TaskId]) -> Vec<TaskId> {
        all_tasks.iter()
            .filter(|task_id| !completed_tasks.contains(task_id))
            .filter(|task_id| self.can_execute(**task_id, completed_tasks))
            .copied()
            .collect()
    }
}

impl Default for DependencyGraph {
    fn default() -> Self {
        Self::new()
    }
}
