use std::time::Duration;
use std::fmt;

pub struct Task {
    pub id: i32,
    pub cb: Box<dyn FnMut(i32)>,
    pub recurring: bool,
    pub delay: Duration,
    pub paused: bool,
}

impl fmt::Display for Task {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Task ID: {} and the pause state {}", self.id, self.paused)
    }
}


impl Task {
    pub fn new<F>(id: i32, cb: F, recurring: bool, delay: Duration, paused: bool) -> Self
    where
        F: FnMut(i32) + 'static
    {
        Task {
            id,
            cb: Box::new(cb),
            recurring,
            delay,
            paused,
        }
    }
}