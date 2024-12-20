use std::collections::HashMap;
use std::time::Duration;
use crate::scheduler::task::Task;

pub struct Scheduler<'a> {
    task_id_counter: i32,
    tasks: HashMap<i32, Task>,
    tasks_cb: HashMap<i32,  Box<dyn FnMut(&Task) + 'a>>
}

impl <'a> Scheduler<'a> {
    pub fn new() -> Self {
        Scheduler {
            task_id_counter: 0,
            tasks: HashMap::new(),
            tasks_cb: HashMap::new()
        }
    }

    pub fn add_task(&mut self, cb: Box<dyn FnMut(&Task) + 'a>, recurring: bool, delay: Duration, paused: bool) -> &Task
    {
        self.task_id_counter += 1;
        let task = Task::new(self.task_id_counter, recurring, delay, paused);
        self.tasks_cb.insert(self.task_id_counter, cb);
        self.tasks.insert(self.task_id_counter, task);
        self.tasks.get(&self.task_id_counter).unwrap()
    }

    pub fn execute(&mut self) {
        for (_key, task) in &mut self.tasks {
            let temp_tasks_cb  = self.tasks_cb.get_mut(&_key).unwrap();
            temp_tasks_cb(task);  
        }
    }
}
