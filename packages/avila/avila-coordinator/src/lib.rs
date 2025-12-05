//! # avila-coordinator - Task Coordination
//!
//! A modular task coordination library for managing task lifecycles,
//! dependencies, scheduling, and metrics.

extern crate alloc;

// Core modules
pub mod types;
pub mod task;
pub mod coordinator;

// Feature modules
pub mod priority;
pub mod dependencies;
pub mod scheduler;
pub mod metrics;
pub mod events;
pub mod retry;
pub mod validation;
pub mod concurrent;
pub mod builder;
pub mod workflow;
pub mod resources;
pub mod serde_support;

// Re-exports for convenience
pub use types::{TaskId, TaskResult, TaskError};
pub use task::{Task, TaskState};
pub use coordinator::Coordinator;
pub use priority::Priority;
pub use dependencies::{TaskDependency, DependencyGraph};
pub use scheduler::{Scheduler, FifoScheduler, PriorityScheduler, FairScheduler};
pub use metrics::{TaskMetrics, MetricsCollector};
pub use events::{TaskEvent, EventHandler, EventBus};
pub use retry::{RetryPolicy, TaskRetryInfo, BackoffStrategy, RetryManager};
pub use validation::{StateValidator, IdValidator, PreCondition, AlwaysValid};
pub use builder::{CoordinatorBuilder, AdvancedCoordinator};
pub use metrics::{Timestamp, Duration, ExecutionRecord};
pub use workflow::{WorkflowNode, Workflow, WorkflowExecution};
pub use resources::{ResourceId, Resource, ResourceState, ResourcePool, RateLimiter, QuotaManager};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_coordinator() {
        let mut coord = Coordinator::new();
        coord.submit(1);
        coord.complete(1).unwrap();
        assert_eq!(coord.tasks[0].state, TaskState::Completed);
    }

    #[test]
    fn test_task_state_transitions() {
        let mut task = Task::new(TaskId::new(1));
        assert_eq!(task.state, TaskState::Pending);

        task.start();
        assert_eq!(task.state, TaskState::Running);

        task.complete();
        assert_eq!(task.state, TaskState::Completed);
    }

    #[test]
    fn test_priority() {
        assert!(Priority::Critical > Priority::High);
        assert!(Priority::High > Priority::Normal);
        assert!(Priority::Normal > Priority::Low);
    }

    #[test]
    fn test_dependency_graph() {
        let mut graph = DependencyGraph::new();
        let task1 = TaskId::new(1);
        let task2 = TaskId::new(2);

        graph.add_dependency(task2, task1);
        let deps = graph.get_dependencies(task2).unwrap();
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0], task1);
    }

    #[test]
    fn test_retry_policy() {
        let mut policy = RetryPolicy::new(3);
        assert!(policy.can_retry());

        policy.increment();
        policy.increment();
        policy.increment();
        assert!(!policy.can_retry());
    }

    #[test]
    fn test_metrics() {
        let mut collector = MetricsCollector::new();
        let task_id = TaskId::new(1);

        let metrics = collector.get_or_create(task_id);
        metrics.record_attempt();
        metrics.record_success();

        assert_eq!(metrics.attempts, 1);
        assert_eq!(metrics.success_count, 1);
    }

    #[test]
    fn test_coordinator_enhancements() {
        let mut coord = Coordinator::new();
        coord.submit(1);
        coord.submit(2);

        assert_eq!(coord.task_count(), 2);
        assert_eq!(coord.task_count_by_state(TaskState::Pending), 2);

        coord.complete(1).unwrap();
        let removed = coord.clear_completed();
        assert_eq!(removed, 1);
        assert_eq!(coord.task_count(), 1);
    }

    #[test]
    fn test_priority_scheduler() {
        use Priority::*;
        let mut coord = Coordinator::new();
        coord.submit_with_priority(1, Low);
        coord.submit_with_priority(2, Critical);
        coord.submit_with_priority(3, Normal);

        let mut scheduler = PriorityScheduler::default();
        let next = scheduler.next_task(&coord.tasks);
        assert_eq!(next, Some(TaskId::new(2))); // Critical priority
    }

    #[test]
    fn test_dependency_cycle_detection() {
        let mut graph = DependencyGraph::new();
        let t1 = TaskId::new(1);
        let t2 = TaskId::new(2);
        let t3 = TaskId::new(3);

        graph.add_dependency(t2, t1);
        graph.add_dependency(t3, t2);
        graph.add_dependency(t1, t3); // Creates cycle

        assert!(graph.has_cycle());
    }

    #[test]
    fn test_retry_backoff() {
        let policy = RetryPolicy::new(3)
            .with_strategy(BackoffStrategy::Exponential)
            .with_base_delay(100);

        assert_eq!(policy.calculate_delay(), 100);

        let mut policy = policy;
        policy.increment();
        assert_eq!(policy.calculate_delay(), 200);

        policy.increment();
        assert_eq!(policy.calculate_delay(), 400);
    }

    #[test]
    fn test_event_bus() {
        use alloc::boxed::Box;

        struct Counter { count: u32 }
        impl EventHandler for Counter {
            fn on_event(&mut self, _event: &TaskEvent) {
                self.count += 1;
            }
        }

        let mut bus = EventBus::new();
        bus.subscribe(Box::new(Counter { count: 0 }));

        let event = TaskEvent::Submitted(TaskId::new(1));
        bus.publish(&event);

        assert_eq!(bus.handler_count(), 1);
    }

    #[test]
    fn test_state_validator() {
        assert!(StateValidator::can_transition(TaskState::Pending, TaskState::Running).is_ok());
        assert!(StateValidator::can_transition(TaskState::Running, TaskState::Completed).is_ok());
        assert!(StateValidator::can_transition(TaskState::Pending, TaskState::Completed).is_err());
    }

    #[test]
    fn test_id_validator() {
        let mut validator = IdValidator::new();
        let id = TaskId::new(1);

        assert!(validator.register(id).is_ok());
        assert!(validator.register(id).is_err()); // Duplicate
        assert!(validator.is_registered(id));

        validator.unregister(id);
        assert!(!validator.is_registered(id));
    }

    #[test]
    fn test_coordinator_builder() {
        let advanced = CoordinatorBuilder::new()
            .with_events()
            .with_retry_management()
            .with_id_validation()
            .build();

        assert!(advanced.event_bus.is_some());
        assert!(advanced.retry_manager.is_some());
        assert!(advanced.id_validator.is_some());
    }

    #[test]
    fn test_workflow_dag() {
        use alloc::string::ToString;
        let mut workflow = Workflow::new("test".to_string());

        let node1 = WorkflowNode::new(TaskId::new(1));
        let node2 = WorkflowNode::new(TaskId::new(2));
        let node3 = WorkflowNode::new(TaskId::new(3));

        workflow.add_node(node1).unwrap();
        workflow.add_node(node2).unwrap();
        workflow.add_node(node3).unwrap();

        workflow.add_edge(TaskId::new(1), TaskId::new(2)).unwrap();
        workflow.add_edge(TaskId::new(2), TaskId::new(3)).unwrap();

        let order = workflow.execution_order().unwrap();
        assert_eq!(order[0], TaskId::new(1));
        assert_eq!(order[2], TaskId::new(3));
    }

    #[test]
    fn test_resource_pool() {
        let mut pool = ResourcePool::new(5);

        let res1 = Resource::new(ResourceId::new(1));
        let res2 = Resource::new(ResourceId::new(2));

        pool.add_resource(res1).unwrap();
        pool.add_resource(res2).unwrap();

        assert_eq!(pool.available_count(), 2);

        let acquired = pool.acquire(TaskId::new(1));
        assert!(acquired.is_some());
        assert_eq!(pool.available_count(), 1);

        pool.release(acquired.unwrap()).unwrap();
        assert_eq!(pool.available_count(), 2);
    }

    #[test]
    fn test_rate_limiter() {
        let mut limiter = RateLimiter::new(3, 1000);

        assert!(limiter.try_acquire(0));
        assert!(limiter.try_acquire(0));
        assert!(limiter.try_acquire(0));
        assert!(!limiter.try_acquire(0)); // Limit reached

        // New period
        assert!(limiter.try_acquire(1000));
    }

    #[test]
    fn test_quota_manager() {
        let mut quota = QuotaManager::new(100);

        quota.set_task_quota(TaskId::new(1), 30);
        assert!(quota.can_allocate(TaskId::new(1), 30));
        assert!(!quota.can_allocate(TaskId::new(1), 31));

        quota.allocate(50).unwrap();
        assert_eq!(quota.available(), 50);

        quota.release(20);
        assert_eq!(quota.available(), 70);
    }

    #[test]
    fn test_metrics_aggregations() {
        let mut collector = MetricsCollector::new();

        let t1 = TaskId::new(1);
        let t2 = TaskId::new(2);

        let m1 = collector.get_or_create(t1);
        m1.record_attempt();
        m1.record_success();

        let m2 = collector.get_or_create(t2);
        m2.record_attempt();
        m2.record_failure();

        assert_eq!(collector.total_tasks(), 2);
        assert_eq!(collector.total_successes(), 1);
        assert_eq!(collector.total_failures(), 1);
        assert_eq!(collector.overall_success_rate(), 0.5);
    }
}
