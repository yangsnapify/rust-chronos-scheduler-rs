use task_scheduler_rs::{Scheduler};
use std::time::Duration;

fn main() {
    let mut scheduler = Scheduler::new();

    let task = scheduler.add_task(
        Box::new(||{
             println!("Executing task with id:");
        }),
        true,
        Duration::new(5, 0),
        false,
    );
    println!("{}", task);
}
