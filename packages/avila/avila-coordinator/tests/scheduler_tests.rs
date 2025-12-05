/// Integration tests for scheduler implementations
use avila_coordinator::{
    Task, TaskId, Scheduler,
    DeadlineScheduler, WeightedScheduler, FifoScheduler,
    PriorityScheduler, FairScheduler
};

#[test]
fn test_deadline_scheduler_basic() {
    let mut scheduler = DeadlineScheduler::new();

    let task1 = Task::new(TaskId::new(1));
    let task2 = Task::new(TaskId::new(2));
    let task3 = Task::new(TaskId::new(3));
    let tasks = vec![task1, task2, task3];

    // Set deadlines
    scheduler.set_deadline(TaskId::new(1), 100);
    scheduler.set_deadline(TaskId::new(2), 50);
    scheduler.set_deadline(TaskId::new(3), 75);

    // Should select task2 (earliest deadline)
    let selected = scheduler.next_task(&tasks);
    assert_eq!(selected, Some(TaskId::new(2)));
}

#[test]
fn test_deadline_scheduler_with_time() {
    let mut scheduler = DeadlineScheduler::with_time(0);

    let task1 = Task::new(TaskId::new(1));
    let _tasks = vec![task1];    scheduler.set_deadline(TaskId::new(1), 100);

    // Check time to deadline
    assert_eq!(scheduler.time_to_deadline(TaskId::new(1)), 100);
    assert!(!scheduler.is_overdue(TaskId::new(1)));

    // Advance time
    scheduler.advance_time(50);
    assert_eq!(scheduler.time_to_deadline(TaskId::new(1)), 50);

    // Go over deadline
    scheduler.advance_time(60);
    assert_eq!(scheduler.time_to_deadline(TaskId::new(1)), 0);
    assert!(scheduler.is_overdue(TaskId::new(1)));
}

#[test]
fn test_deadline_scheduler_ignores_completed() {
    let mut scheduler = DeadlineScheduler::new();

    let mut task1 = Task::new(TaskId::new(1));
    let task2 = Task::new(TaskId::new(2));
    task1.complete();    let tasks = vec![task1, task2];

    scheduler.set_deadline(TaskId::new(1), 50);
    scheduler.set_deadline(TaskId::new(2), 100);

    // Should skip completed task1 and select task2
    let selected = scheduler.next_task(&tasks);
    assert_eq!(selected, Some(TaskId::new(2)));
}

#[test]
fn test_deadline_scheduler_without_deadlines() {
    let mut scheduler = DeadlineScheduler::new();

    let task1 = Task::new(TaskId::new(1));
    let task2 = Task::new(TaskId::new(2));
    let tasks = vec![task1, task2];

    // No deadlines set - should still select first pending
    let selected = scheduler.next_task(&tasks);
    assert!(selected.is_some());
}

#[test]
fn test_weighted_scheduler_basic() {
    let mut scheduler = WeightedScheduler::new();

    let task1 = Task::new(TaskId::new(1));
    let task2 = Task::new(TaskId::new(2));
    let task3 = Task::new(TaskId::new(3));
    let tasks = vec![task1, task2, task3];

    // Set weights
    scheduler.set_weight(TaskId::new(1), 1);
    scheduler.set_weight(TaskId::new(2), 10);
    scheduler.set_weight(TaskId::new(3), 5);

    // Should select task2 (highest weight)
    let selected = scheduler.next_task(&tasks);
    assert_eq!(selected, Some(TaskId::new(2)));
}

#[test]
fn test_weighted_scheduler_with_default_weight() {
    let mut scheduler = WeightedScheduler::with_default_weight(5);

    let task1 = Task::new(TaskId::new(1));
    let task2 = Task::new(TaskId::new(2));
    let tasks = vec![task1, task2];

    scheduler.set_weight(TaskId::new(1), 10);
    // task2 gets default weight of 5

    // Should select task1 (weight 10 > 5)
    let selected = scheduler.next_task(&tasks);
    assert_eq!(selected, Some(TaskId::new(1)));
}

#[test]
fn test_weighted_scheduler_get_weight() {
    let mut scheduler = WeightedScheduler::with_default_weight(3);

    scheduler.set_weight(TaskId::new(1), 10);

    assert_eq!(scheduler.get_weight(TaskId::new(1)), 10);
    assert_eq!(scheduler.get_weight(TaskId::new(2)), 3); // default weight
}

#[test]
fn test_weighted_scheduler_total_weight() {
    let mut scheduler = WeightedScheduler::new();

    let task1 = Task::new(TaskId::new(1));
    let task2 = Task::new(TaskId::new(2));
    let task3 = Task::new(TaskId::new(3));
    let tasks = vec![task1, task2, task3];

    scheduler.set_weight(TaskId::new(1), 5);
    scheduler.set_weight(TaskId::new(2), 10);
    scheduler.set_weight(TaskId::new(3), 5);

    // Default weight is 1
    let total = scheduler.total_weight(&tasks);
    assert_eq!(total, 5 + 10 + 5);
}

#[test]
fn test_weighted_scheduler_remove_weight() {
    let mut scheduler = WeightedScheduler::with_default_weight(1);

    scheduler.set_weight(TaskId::new(1), 10);
    assert_eq!(scheduler.get_weight(TaskId::new(1)), 10);

    scheduler.remove_weight(TaskId::new(1));
    assert_eq!(scheduler.get_weight(TaskId::new(1)), 1); // reverts to default
}

#[test]
fn test_weighted_scheduler_ignores_completed() {
    let mut scheduler = WeightedScheduler::new();

    let mut task1 = Task::new(TaskId::new(1));
    let task2 = Task::new(TaskId::new(2));
    task1.complete();

    let tasks = vec![task1, task2];

    scheduler.set_weight(TaskId::new(1), 100); // high weight but completed
    scheduler.set_weight(TaskId::new(2), 5);

    // Should skip completed task1 and select task2
    let selected = scheduler.next_task(&tasks);
    assert_eq!(selected, Some(TaskId::new(2)));
}

#[test]
fn test_deadline_scheduler_set_current_time() {
    let mut scheduler = DeadlineScheduler::new();

    scheduler.set_current_time(50);
    assert_eq!(scheduler.current_time(), 50);

    scheduler.set_deadline(TaskId::new(1), 100);
    assert_eq!(scheduler.time_to_deadline(TaskId::new(1)), 50);
}

#[test]
fn test_weighted_scheduler_default() {
    let scheduler1 = WeightedScheduler::default();
    let scheduler2 = WeightedScheduler::new();

    // Both should be equivalent
    let task = Task::new(TaskId::new(1));
    assert_eq!(
        scheduler1.get_weight(task.id),
        scheduler2.get_weight(task.id)
    );
}

#[test]
fn test_deadline_scheduler_default() {
    let scheduler1 = DeadlineScheduler::default();
    let scheduler2 = DeadlineScheduler::new();

    // Both should have current_time = 0
    assert_eq!(scheduler1.current_time(), 0);
    assert_eq!(scheduler2.current_time(), 0);
}

#[test]
fn test_multiple_pending_tasks_deadline() {
    let mut scheduler = DeadlineScheduler::new();

    let task1 = Task::new(TaskId::new(1));
    let mut task2 = Task::new(TaskId::new(2));
    let task3 = Task::new(TaskId::new(3));

    task2.start(); // Not pending

    let tasks = vec![task1, task2, task3];

    scheduler.set_deadline(TaskId::new(1), 50);
    scheduler.set_deadline(TaskId::new(2), 25); // This task is running, not pending
    scheduler.set_deadline(TaskId::new(3), 75);

    // Should select task1 (earliest deadline among pending)
    let selected = scheduler.next_task(&tasks);
    assert_eq!(selected, Some(TaskId::new(1)));
}

#[test]
fn test_multiple_pending_tasks_weighted() {
    let mut scheduler = WeightedScheduler::new();

    let task1 = Task::new(TaskId::new(1));
    let mut task2 = Task::new(TaskId::new(2));
    let task3 = Task::new(TaskId::new(3));

    task2.fail(); // Not pending

    let tasks = vec![task1, task2, task3];

    scheduler.set_weight(TaskId::new(1), 5);
    scheduler.set_weight(TaskId::new(2), 100); // This task failed, not pending
    scheduler.set_weight(TaskId::new(3), 10);

    // Should select task3 (highest weight among pending)
    let selected = scheduler.next_task(&tasks);
    assert_eq!(selected, Some(TaskId::new(3)));
}

#[test]
fn test_all_schedulers_empty_task_list() {
    let mut deadline_scheduler = DeadlineScheduler::new();
    let mut weighted_scheduler = WeightedScheduler::new();
    let mut fifo_scheduler = FifoScheduler::default();
    let mut priority_scheduler = PriorityScheduler::default();
    let mut fair_scheduler = FairScheduler::new();

    let tasks: Vec<Task> = vec![];

    assert_eq!(deadline_scheduler.next_task(&tasks), None);
    assert_eq!(weighted_scheduler.next_task(&tasks), None);
    assert_eq!(fifo_scheduler.next_task(&tasks), None);
    assert_eq!(priority_scheduler.next_task(&tasks), None);
    assert_eq!(fair_scheduler.next_task(&tasks), None);
}

#[test]
fn test_weighted_scheduler_zero_weight_ignored() {
    let mut scheduler = WeightedScheduler::new();

    // Zero weight should not be inserted
    scheduler.set_weight(TaskId::new(1), 0);

    // Should return default weight
    assert_eq!(scheduler.get_weight(TaskId::new(1)), 1);
}

#[test]
fn test_deadline_scheduler_clone() {
    let mut scheduler1 = DeadlineScheduler::new();
    scheduler1.set_deadline(TaskId::new(1), 100);
    scheduler1.set_current_time(50);

    let scheduler2 = scheduler1.clone();

    assert_eq!(scheduler1.current_time(), scheduler2.current_time());
    assert_eq!(
        scheduler1.time_to_deadline(TaskId::new(1)),
        scheduler2.time_to_deadline(TaskId::new(1))
    );
}

#[test]
fn test_weighted_scheduler_clone() {
    let mut scheduler1 = WeightedScheduler::new();
    scheduler1.set_weight(TaskId::new(1), 10);

    let scheduler2 = scheduler1.clone();

    assert_eq!(scheduler1.get_weight(TaskId::new(1)), scheduler2.get_weight(TaskId::new(1)));
}
