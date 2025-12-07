//! # Metrics - Task execution metrics
extern crate alloc;
use alloc::vec::Vec;
use crate::types::TaskId;

/// Simple timestamp representation (milliseconds since epoch)
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Timestamp(pub u64);

impl Timestamp {
    pub fn now() -> Self {
        // In no_std, this needs to be provided externally
        // For now, return 0 as placeholder
        Self(0)
    }

    pub fn elapsed_since(&self, earlier: Timestamp) -> Duration {
        Duration(self.0.saturating_sub(earlier.0))
    }
}

/// Duration in milliseconds
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Duration(pub u64);

impl Duration {
    pub fn as_millis(&self) -> u64 {
        self.0
    }

    pub fn as_secs(&self) -> u64 {
        self.0 / 1000
    }
}

#[derive(Clone, Debug)]
pub struct ExecutionRecord {
    pub started_at: Timestamp,
    pub completed_at: Option<Timestamp>,
    pub duration: Option<Duration>,
    pub success: bool,
}

impl ExecutionRecord {
    pub fn new(started_at: Timestamp) -> Self {
        Self {
            started_at,
            completed_at: None,
            duration: None,
            success: false,
        }
    }

    pub fn complete(&mut self, timestamp: Timestamp, success: bool) {
        self.completed_at = Some(timestamp);
        self.duration = Some(timestamp.elapsed_since(self.started_at));
        self.success = success;
    }
}

#[derive(Clone, Debug)]
pub struct TaskMetrics {
    pub task_id: TaskId,
    pub attempts: u32,
    pub success_count: u32,
    pub failure_count: u32,
    pub created_at: Timestamp,
    pub first_started_at: Option<Timestamp>,
    pub last_completed_at: Option<Timestamp>,
    pub total_duration: Duration,
    pub executions: Vec<ExecutionRecord>,
}

impl TaskMetrics {
    pub fn new(task_id: TaskId) -> Self {
        Self {
            task_id,
            attempts: 0,
            success_count: 0,
            failure_count: 0,
            created_at: Timestamp::now(),
            first_started_at: None,
            last_completed_at: None,
            total_duration: Duration(0),
            executions: Vec::new(),
        }
    }

    pub fn record_attempt(&mut self) {
        self.attempts += 1;
        if self.first_started_at.is_none() {
            self.first_started_at = Some(Timestamp::now());
        }
        self.executions.push(ExecutionRecord::new(Timestamp::now()));
    }

    pub fn record_success(&mut self) {
        self.success_count += 1;
        let now = Timestamp::now();
        self.last_completed_at = Some(now);
        if let Some(exec) = self.executions.last_mut() {
            exec.complete(now, true);
            if let Some(duration) = exec.duration {
                self.total_duration = Duration(self.total_duration.0 + duration.0);
            }
        }
    }

    pub fn record_failure(&mut self) {
        self.failure_count += 1;
        let now = Timestamp::now();
        self.last_completed_at = Some(now);
        if let Some(exec) = self.executions.last_mut() {
            exec.complete(now, false);
            if let Some(duration) = exec.duration {
                self.total_duration = Duration(self.total_duration.0 + duration.0);
            }
        }
    }

    pub fn average_duration(&self) -> Option<Duration> {
        let completed = self.success_count + self.failure_count;
        if completed > 0 {
            Some(Duration(self.total_duration.0 / completed as u64))
        } else {
            None
        }
    }

    pub fn success_rate(&self) -> f32 {
        let total = self.success_count + self.failure_count;
        if total > 0 {
            self.success_count as f32 / total as f32
        } else {
            0.0
        }
    }

    pub fn failure_rate(&self) -> f32 {
        1.0 - self.success_rate()
    }
}

pub struct MetricsCollector {
    metrics: Vec<TaskMetrics>,
}

impl MetricsCollector {
    pub fn new() -> Self {
        Self {
            metrics: Vec::new(),
        }
    }

    pub fn get_or_create(&mut self, task_id: TaskId) -> &mut TaskMetrics {
        if let Some(pos) = self.metrics.iter().position(|m| m.task_id == task_id) {
            &mut self.metrics[pos]
        } else {
            self.metrics.push(TaskMetrics::new(task_id));
            self.metrics.last_mut().unwrap()
        }
    }

    pub fn get(&self, task_id: TaskId) -> Option<&TaskMetrics> {
        self.metrics.iter().find(|m| m.task_id == task_id)
    }

    pub fn total_tasks(&self) -> usize {
        self.metrics.len()
    }

    pub fn total_attempts(&self) -> u32 {
        self.metrics.iter().map(|m| m.attempts).sum()
    }

    pub fn total_successes(&self) -> u32 {
        self.metrics.iter().map(|m| m.success_count).sum()
    }

    pub fn total_failures(&self) -> u32 {
        self.metrics.iter().map(|m| m.failure_count).sum()
    }

    pub fn overall_success_rate(&self) -> f32 {
        let successes = self.total_successes();
        let failures = self.total_failures();
        let total = successes + failures;
        if total > 0 {
            successes as f32 / total as f32
        } else {
            0.0
        }
    }

    pub fn overall_average_duration(&self) -> Option<Duration> {
        let total_duration: u64 = self.metrics.iter()
            .map(|m| m.total_duration.0)
            .sum();
        let completed = self.total_successes() + self.total_failures();
        if completed > 0 {
            Some(Duration(total_duration / completed as u64))
        } else {
            None
        }
    }

    pub fn clear(&mut self) {
        self.metrics.clear();
    }
}

impl Default for MetricsCollector {
    fn default() -> Self {
        Self::new()
    }
}
