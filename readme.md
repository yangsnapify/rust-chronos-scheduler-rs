# simple-task-scheduler-rs

A simple task scheduler for Rust with support for recurring tasks, pause/resume functionality, and task management. This library allows you to schedule tasks with a specified delay, execute them once, or make them recurring. You can also pause and resume tasks as needed.

## Features

- **Add one-time tasks**: Execute a task after a specified delay.
- **Recurring tasks**: Schedule tasks that repeat at regular intervals.
- **Pause tasks**: Pause tasks without removing them from the scheduler.
- **Resume tasks**: Resume paused tasks from where they left off.
- **Task removal**: Remove tasks by their ID.


```
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

```