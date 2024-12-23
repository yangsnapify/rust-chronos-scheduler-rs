use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::time::{Duration, sleep};
use task_scheduler_rs::{Scheduler, Task};

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
