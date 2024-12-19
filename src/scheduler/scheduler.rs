use std::collections::HashMap;
use std::time::Duration;
use crate::scheduler::task::Task;

pub struct Scheduler {
    task_id_counter: i32,
    tasks: HashMap<i32, Task>,
}

impl Scheduler {
    pub fn new() -> Self {
        Scheduler {
            task_id_counter: 0,
            tasks: HashMap::new(),
        }
    }

    pub fn add_task(&mut self, cb: Box<dyn Fn()>, recurring: bool, delay: Duration, paused: bool) -> &Task

    {
        self.task_id_counter += 1;
        let task = Task::new(self.task_id_counter, cb, recurring, delay, paused);
        self.tasks.insert(self.task_id_counter, task);
        self.tasks.get(&self.task_id_counter).unwrap()
    }
}
