use std::collections::HashMap;
use std::time::Duration;
use tokio::time::sleep as tokio_sleep;
use crate::scheduler::task::Task;

pub struct Scheduler<'a> {
    pub task_id_counter: i32,
    pub tasks: HashMap<i32, Task>,
    pub tasks_cb: HashMap<i32, Box<dyn FnMut(&Task) + 'a + Send>>,
    pub name_to_id: HashMap<String, i32>,
}

impl<'a> Scheduler<'a> {
    pub fn new() -> Self {
        Scheduler {
            task_id_counter: 0,
            tasks: HashMap::new(),
            tasks_cb: HashMap::new(),
            name_to_id: HashMap::new(),
        }
    }

    pub fn add_task<T>(
        &mut self,
        name: String,
        cb: T,
        delay: Duration
    ) -> &mut Self
        where T: FnMut(&Task) + 'a + Send
    {
        let task = Task {
            id: self.task_id_counter,
            name: name.clone(),
            delay,
        };
        self.task_id_counter += 1;
        self.tasks.insert(self.task_id_counter, task);
        self.tasks_cb.insert(self.task_id_counter, Box::new(cb));
        self.name_to_id.insert(name, self.task_id_counter);

        self
    }

    pub async fn execute(&mut self) {
        let mut tasks_to_remove = Vec::new();
        for (key, task) in &mut self.tasks {
            if !task.delay.is_zero() {
                tokio_sleep(task.delay).await;
            }

            if let Some(cb) = self.tasks_cb.get_mut(key) {
                cb(task);
                tasks_to_remove.push(*key);
            }
        }

        for task_id in &tasks_to_remove {
            self.tasks.remove(&task_id);
            self.tasks_cb.remove(&task_id);
        }
    }

    pub fn get_last_task_id(&self) -> Option<i32> {
        if self.task_id_counter > 0 { Some(self.task_id_counter) } else { None }
    }

    pub fn remove_task(&mut self, task_id: i32) -> &mut Self {
        self.tasks.retain(|_, task| task.id != task_id);
        self
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
