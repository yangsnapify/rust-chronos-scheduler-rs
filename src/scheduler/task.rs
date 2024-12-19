use std::time::Duration;

pub struct Task<'a> {
    id: i32,
    cb: Box<dyn FnMut() + Send>,
    recurring: bool,
    delay: Duration,
    paused: bool,
}

impl Task {
    pub fn new<F>(id: i32, cb: F, recurring: bool, delay: Duration, paused: bool) -> Self
    where 
        F: FnMut() + Send + 'static,
    {
        Task {
            id, 
            cb: Box::new(cb),
            delay,
            recurring, 
            paused
        }
    }
}