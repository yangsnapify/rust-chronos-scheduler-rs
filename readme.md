# Task Scheduler RS

A flexible, async task scheduler for Rust that supports both one-time and recurring tasks using tokio, with thread-safe implementation.

## Features

- **Async Task Scheduling**: Built on tokio for efficient asynchronous execution
- **Thread-safe**: Uses `Arc` and `Mutex` for safe concurrent access
- **Task Types**:
  - One-time tasks with delay
  - Recurring tasks with customizable intervals
- **Task Management**:
  - Add and remove tasks dynamically
  - List active tasks
  - Automatic cleanup of completed tasks

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
task-scheduler-rs = "0.4.0"
tokio = { version = "1.0", features = ["full"] }
```

### Basic Example

```rust
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::time::Duration;
use task_scheduler_rs::{Scheduler, Task};
use task_scheduler_rs::scheduler::task::RecurrenceType;

#[tokio::main]
async fn main() {
    // Create a thread-safe scheduler
    let scheduler = Arc::new(Mutex::new(Scheduler::new()));
    
    // Add tasks with lock
    {
        let mut scheduler = scheduler.lock().await;
        
        // Add a one-time task
        scheduler.add_task(
            "one_time_task".to_string(),
            |task| println!("Executing task: {}", task.name),
            Duration::from_secs(5)  // Runs after 5 seconds
        );
        
        // Add a recurring task
        scheduler.add_recurring_task(
            "recurring_task".to_string(),
            |task| println!("Running recurring task: {}", task.name),
            Duration::from_secs(0),  // No initial delay
            RecurrenceType::Fixed(Duration::from_secs(2))  // Repeats every 2 seconds
        );
    }

    // Run scheduler in separate task
    let scheduler_clone = Arc::clone(&scheduler);
    tokio::spawn(async move {
        let mut scheduler = scheduler_clone.lock().await;
        scheduler.execute().await;
    });
}
```

### Task Management Example

```rust
// Remove a task
let scheduler_clone = Arc::clone(&scheduler);
tokio::spawn(async move {
    tokio::time::sleep(Duration::from_secs(5)).await;
    let mut scheduler = scheduler_clone.lock().await;
    scheduler.remove_task_by_name("one_time_task");
    println!("Current tasks: {:?}", scheduler.list_tasks());
});
```

### Scheduler Methods

- `new()`: Create a new scheduler instance
- `add_task(name: String, callback: impl FnMut(&Task) + Send + 'static, delay: Duration)`: Add a one-time task
- `add_recurring_task(name: String, callback: impl FnMut(&Task) + Send + 'static, delay: Duration, recurrence: RecurrenceType)`: Add a recurring task
- `execute()`: Start executing scheduled tasks
- `remove_task_by_name(name: &str)`: Remove a task by its name
- `list_tasks()`: Get a list of all task names
- `stop()`: Stop all running tasks

### Task Types

```rust
pub enum RecurrenceType {
    None,               // One-time task
    Fixed(Duration),    // Recurring task with fixed interval
}
```

## Thread Safety

The scheduler is designed to be thread-safe using `Arc<Mutex<Scheduler>>`. This allows:
- Concurrent access from multiple tasks
- Safe task management across different threads
- Dynamic task addition/removal while the scheduler is running

## Best Practices

1. Always use proper locking with `.lock().await`
2. Keep lock durations as short as possible
3. Clone the Arc for different tasks that need scheduler access
4. Handle potential task panics in callbacks

## Requirements

- Rust edition 2021 or later
- tokio with `full` features enabled

## Important Note

This is a learning project as I'm still getting familiar with Rust. The current implementation focuses on basic functionality and might not include advanced features or optimizations. If you're looking for a production-ready task scheduler with more advanced features, please consider using other established crates like:

- [tokio-cron-scheduler](https://crates.io/crates/tokio-cron-scheduler)
- [delay-timer](https://crates.io/crates/delay-timer)
- [scheduled-thread-pool](https://crates.io/crates/scheduled-thread-pool)


## Others example
```
#[tokio::main]
async fn main() {
    let scheduler = Arc::new(Mutex::new(Scheduler::new()));
    {
        let mut scheduler = scheduler.lock().await; // Lock for adding tasks
        scheduler.add_task(
            "daily_backup".to_string(),
            Box::new(|task: &Task| println!("Backing up... {}", task.name)),
            Duration::from_secs(1),
        );
        scheduler.add_task(
            "hourly_check".to_string(),
            Box::new(|task: &Task| println!("Checking... {}", task.name)),
            Duration::from_secs(2),
        );
        scheduler.add_recurring_task(
            "recurring_task".to_string(),
            |task| println!("Running task: {}", task.name),
            Duration::from_secs(0),  // initial delay
            RecurrenceType::Fixed(Duration::from_secs(5))  // repeat every 5 seconds
        );
    };


    let scheduler_clone = Arc::clone(&scheduler);
    // Task 1: Execute the scheduler
    tokio::spawn(async move {
        let mut scheduler = scheduler_clone.lock().await;
        scheduler.execute().await;
    });


    let scheduler_clone = Arc::clone(&scheduler);
    // Task 2: Remove 'daily_backup' after 5 seconds
    tokio::spawn(async move {
        sleep(Duration::from_secs(5)).await; // Wait 5 seconds
        let mut scheduler = scheduler_clone.lock().await;
        scheduler.remove_task_by_name("daily_backup");
        println!("Removed 'daily_backup'");
        println!("Current tasks: {:?}", scheduler.list_tasks());
    });


    // Main thread keeps running
    loop {
        sleep(Duration::from_secs(1)).await;
        println!("Main thread is still running...");
    }
}
```