use task_scheduler_rs::{Scheduler, Task};
use std::time::Duration;

fn main() {
    let mut scheduler = Scheduler::new();
    let callback: Box<dyn FnMut(&Task)> = Box::new(|task: &Task| {
        println!("Executing task with ID: {}", task.id);
    });
    let _task = scheduler.add_task(callback, true, Duration::new(0, 0), false);

    scheduler.execute();
}
