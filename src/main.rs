use task_scheduler_rs::{ Scheduler, Task, Channel };
use task_scheduler_rs::scheduler::task::TaskAction;
use std::{thread, time};
use std::time::Duration;

fn chan_success(msg: TaskAction) {
    println!("Msg {}", msg);
}
fn chan_err() {
    println!("End");
}

fn main() {
    let mut scheduler = Scheduler::new();
    let mut chan = Channel::new();

    chan.listen(chan_success, chan_err);
    chan.send(TaskAction::Execute);
    thread::sleep(Duration::from_secs(1));

    let callback: Box<dyn FnMut(&Task)> = Box::new(|task: &Task| {
        println!("Executing task with ID: {}", task.id);
    });
    let _task = scheduler.add_task(callback, true, Duration::new(0, 0), false);
    scheduler.execute();

    chan.send(TaskAction::Shutdown);
    thread::sleep(Duration::from_millis(100));
    chan.send(TaskAction::Execute);
    thread::sleep(Duration::from_secs(1)); 
}
