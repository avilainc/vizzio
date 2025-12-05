//! Task management system
//! 
//! Pure Rust implementation of async tasks without external dependencies.

use std::future::Future;
use std::pin::Pin;
use std::sync::atomic::{AtomicU64, AtomicU8, Ordering};
use std::sync::Arc;
use std::task::{Context, Poll};

/// Task ID counter
static TASK_ID_COUNTER: AtomicU64 = AtomicU64::new(1);

/// Task states
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum TaskState {
    /// Task is ready to run
    Ready = 0,
    /// Task is currently running
    Running = 1,
    /// Task is waiting (blocked)
    Waiting = 2,
    /// Task completed successfully
    Completed = 3,
    /// Task was cancelled
    Cancelled = 4,
}

impl From<u8> for TaskState {
    fn from(value: u8) -> Self {
        match value {
            0 => TaskState::Ready,
            1 => TaskState::Running,
            2 => TaskState::Waiting,
            3 => TaskState::Completed,
            4 => TaskState::Cancelled,
            _ => TaskState::Ready,
        }
    }
}

/// Task priority levels
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Priority {
    Low = 0,
    Normal = 1,
    High = 2,
    Critical = 3,
}

/// Task metadata
#[derive(Debug)]
pub struct TaskMetadata {
    pub id: u64,
    pub name: Option<String>,
    pub priority: Priority,
    pub created_at: std::time::Instant,
    pub started_at: Option<std::time::Instant>,
    pub completed_at: Option<std::time::Instant>,
}

/// Boxed future type
pub type BoxFuture = Pin<Box<dyn Future<Output = ()> + Send + 'static>>;

/// Task structure
pub struct Task {
    id: u64,
    future: BoxFuture,
    state: Arc<AtomicU8>,
    priority: Priority,
    metadata: TaskMetadata,
}

impl Task {
    /// Create a new task
    pub fn new<F>(future: F) -> Self
    where
        F: Future<Output = ()> + Send + 'static,
    {
        Self::with_priority(future, Priority::Normal)
    }

    /// Create a new task with priority
    pub fn with_priority<F>(future: F, priority: Priority) -> Self
    where
        F: Future<Output = ()> + Send + 'static,
    {
        let id = TASK_ID_COUNTER.fetch_add(1, Ordering::SeqCst);
        
        Self {
            id,
            future: Box::pin(future),
            state: Arc::new(AtomicU8::new(TaskState::Ready as u8)),
            priority,
            metadata: TaskMetadata {
                id,
                name: None,
                priority,
                created_at: std::time::Instant::now(),
                started_at: None,
                completed_at: None,
            },
        }
    }

    /// Create a named task
    pub fn named<F>(name: impl Into<String>, future: F) -> Self
    where
        F: Future<Output = ()> + Send + 'static,
    {
        let mut task = Self::new(future);
        task.metadata.name = Some(name.into());
        task
    }

    /// Get task ID
    pub fn id(&self) -> u64 {
        self.id
    }

    /// Get task state
    pub fn state(&self) -> TaskState {
        TaskState::from(self.state.load(Ordering::SeqCst))
    }

    /// Set task state
    pub fn set_state(&self, state: TaskState) {
        self.state.store(state as u8, Ordering::SeqCst);
    }

    /// Get task priority
    pub fn priority(&self) -> Priority {
        self.priority
    }

    /// Check if task is ready
    pub fn is_ready(&self) -> bool {
        self.state() == TaskState::Ready
    }

    /// Check if task is running
    pub fn is_running(&self) -> bool {
        self.state() == TaskState::Running
    }

    /// Check if task is completed
    pub fn is_completed(&self) -> bool {
        matches!(self.state(), TaskState::Completed | TaskState::Cancelled)
    }

    /// Poll the task
    pub fn poll(&mut self, context: &mut Context<'_>) -> Poll<()> {
        // Set state to running if it was ready
        if self.state() == TaskState::Ready {
            self.set_state(TaskState::Running);
            self.metadata.started_at = Some(std::time::Instant::now());
        }

        // Poll the future
        match self.future.as_mut().poll(context) {
            Poll::Ready(()) => {
                self.set_state(TaskState::Completed);
                self.metadata.completed_at = Some(std::time::Instant::now());
                Poll::Ready(())
            }
            Poll::Pending => {
                self.set_state(TaskState::Waiting);
                Poll::Pending
            }
        }
    }

    /// Cancel the task
    pub fn cancel(&mut self) {
        self.set_state(TaskState::Cancelled);
        self.metadata.completed_at = Some(std::time::Instant::now());
    }

    /// Get task metadata
    pub fn metadata(&self) -> &TaskMetadata {
        &self.metadata
    }

    /// Get task execution time
    pub fn execution_time(&self) -> Option<std::time::Duration> {
        match (self.metadata.started_at, self.metadata.completed_at) {
            (Some(start), Some(end)) => Some(end.duration_since(start)),
            _ => None,
        }
    }
}

impl std::fmt::Debug for Task {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Task")
            .field("id", &self.id)
            .field("state", &self.state())
            .field("priority", &self.priority)
            .field("name", &self.metadata.name)
            .finish()
    }
}

/// Task handle for external reference
#[derive(Clone)]
pub struct TaskHandle {
    id: u64,
    state: Arc<AtomicU8>,
}

impl TaskHandle {
    /// Create a new task handle
    pub(crate) fn new(id: u64, state: Arc<AtomicU8>) -> Self {
        Self { id, state }
    }

    /// Get task ID
    pub fn id(&self) -> u64 {
        self.id
    }

    /// Get task state
    pub fn state(&self) -> TaskState {
        TaskState::from(self.state.load(Ordering::SeqCst))
    }

    /// Check if task is completed
    pub fn is_completed(&self) -> bool {
        matches!(self.state(), TaskState::Completed | TaskState::Cancelled)
    }

    /// Wait for task completion
    pub fn wait(&self) {
        while !self.is_completed() {
            std::thread::yield_now();
        }
    }
}
