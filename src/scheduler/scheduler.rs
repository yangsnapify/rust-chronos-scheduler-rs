use std::collections::HashMap;
use std::time::Duration;
use tokio::time::{sleep as tokio_sleep, interval as tokio_interval};
use tokio::task::JoinHandle;
use crate::scheduler::task::Task;
use crate::scheduler::task::RecurrenceType;

pub struct Scheduler {
    pub task_id_counter: i32,
    pub tasks: HashMap<i32, Task>,
    pub tasks_cb: HashMap<i32, Box<dyn FnMut(&Task) + Send + 'static>>,  // Changed lifetime bound
    pub name_to_id: HashMap<String, i32>,
    running_tasks: HashMap<i32, JoinHandle<()>>,
}

impl Scheduler {
    pub fn new() -> Self {
        Scheduler {
            task_id_counter: 0,
            tasks: HashMap::new(),
            tasks_cb: HashMap::new(),
            name_to_id: HashMap::new(),
            running_tasks: HashMap::new(),
        }
    }

    pub fn add_task<T>(
        &mut self,
        name: String,
        cb: T,
        delay: Duration
    ) -> &mut Self
        where T: FnMut(&Task) + Send + 'static
    {
        self.add_recurring_task(name, cb, delay, RecurrenceType::None)
    }

    pub fn add_recurring_task<T>(
        &mut self,
        name: String,
        cb: T,
        delay: Duration,
        recurrence: RecurrenceType,
    ) -> &mut Self
        where T: FnMut(&Task) + Send + 'static
    {
        let task = Task {
            id: self.task_id_counter,
            name: name.clone(),
            delay,
            recurrence,
        };
        
        self.task_id_counter += 1;
        self.tasks.insert(self.task_id_counter, task);
        self.tasks_cb.insert(self.task_id_counter, Box::new(cb));
        self.name_to_id.insert(name, self.task_id_counter);

        self
    }

    pub async fn execute(&mut self) {
        let mut tasks_to_process = Vec::new();
        
        // Collect tasks to process
        for (key, task) in &self.tasks {
            tasks_to_process.push((*key, task.clone()));
        }

        for (key, task) in tasks_to_process {
            match task.recurrence {
                RecurrenceType::None => {
                    // Handle one-time task
                    let mut cb = self.tasks_cb.remove(&key)
                        .expect("Callback should exist for task");
                    let task_clone = task.clone();
                    
                    tokio::spawn(async move {
                        if !task_clone.delay.is_zero() {
                            tokio_sleep(task_clone.delay).await;
                        }
                        cb(&task_clone);
                    });
                },
                RecurrenceType::Fixed(repeat_interval) => {
                    // Handle recurring task
                    let mut cb = self.tasks_cb.remove(&key)
                        .expect("Callback should exist for task");
                    let task_clone = task.clone();
                    
                    let handle = tokio::spawn(async move {
                        // Handle initial delay
                        if !task_clone.delay.is_zero() {
                            tokio_sleep(task_clone.delay).await;
                        }
                        
                        let mut interval = tokio_interval(repeat_interval);
                        loop {
                            interval.tick().await;
                            cb(&task_clone);
                        }
                    });
                    
                    self.running_tasks.insert(key, handle);
                }
            }
        }
    }

    pub async fn stop(&mut self) {
        for (_, handle) in self.running_tasks.drain() {
            handle.abort();
        }
    }

    pub fn cleanup(&mut self) {
        for (_, handle) in self.running_tasks.drain() {
            handle.abort();
        }
    }

    pub fn remove_task_by_name(&mut self, name: &str) -> &mut Self {
        if let Some(id) = self.name_to_id.remove(name) {
            self.tasks.remove(&id);
            self.tasks_cb.remove(&id);
        }
        self
    }

    pub fn list_tasks(&self) -> Vec<&str> {
        self.tasks
            .values()
            .map(|task| task.name.as_str())
            .collect()
    }
}

impl Drop for Scheduler {
    fn drop(&mut self) {
        self.cleanup();
    }
}
