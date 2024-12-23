use std::time::Duration;
use std::fmt;

#[derive(Clone)]
pub enum RecurrenceType {
    None,
    Fixed(Duration),
}

#[derive(Clone)]
pub struct Task {
    pub id: i32,
    pub name: String,
    pub delay: Duration,
    pub recurrence: RecurrenceType,
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
        write!(f, "Task ID: {}", self.id)
    }
}

impl Task {
    pub fn new(id: i32, name: String, delay: Duration, recurrence:RecurrenceType ) -> Self
    {
        Task {
            id,
            name,
            delay,
            recurrence
        }
    }
}