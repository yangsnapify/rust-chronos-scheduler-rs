use std::time::Duration;
use std::fmt;

#[derive(Clone)]
pub struct Task {
    pub id: i32,
    pub recurring: bool,
    pub delay: Duration,
    pub paused: bool,
}

#[derive(Clone)]
pub enum TaskAction {
    Execute,
    Paused,
    Shutdown,
}

impl fmt::Display for TaskAction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TaskAction::Execute => write!(f, "Execute"),
            TaskAction::Paused => write!(f, "Paused"),
            TaskAction::Shutdown => write!(f, "Shuwdown"),
        }
    }
}

impl fmt::Display for Task {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Task ID: {} and the pause state {}", self.id, self.paused)
    }
}

impl Task {
    pub fn new(id: i32, recurring: bool, delay: Duration, paused: bool) -> Self
    {
        Task {
            id,
            recurring,
            delay,
            paused,
        }
    }
}