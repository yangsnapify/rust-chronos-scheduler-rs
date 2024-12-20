use task_scheduler_rs::{ Scheduler, Task, Channel };
use task_scheduler_rs::scheduler::task::TaskAction;
use std::time::Duration;

fn chan_success(msg: TaskAction) {
    println!("TEST {}", msg);
}
fn chan_err() {
    println!("error");
}

fn main() {
    let mut scheduler = Scheduler::new();
    let mut chan = Channel::new();

    chan.listen(chan_success, chan_err);
    chan.send(TaskAction::Execute);
    chan.send(TaskAction::Execute);
    chan.send(TaskAction::Shutdown);

    let callback: Box<dyn FnMut(&Task)> = Box::new(|task: &Task| {
        println!("Executing task with ID: {}", task.id);
    });
    let _task = scheduler.add_task(callback, true, Duration::new(0, 0), false);
    scheduler.execute();
}
