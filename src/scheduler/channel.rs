use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;
use std::marker::Send;
use crate::scheduler::task::TaskAction;

pub struct Channel {
    pub task_sender: Arc<Mutex<mpsc::Sender<TaskAction>>>,
    pub task_receiver: Arc<Mutex<mpsc::Receiver<TaskAction>>>
}

impl Channel {
    pub fn new() -> Channel {
        let (sender, receiver) = mpsc::channel();
        Channel {
            task_sender: Arc::new(Mutex::new(sender)),
            task_receiver: Arc::new(Mutex::new(receiver)),
        }
    }

    pub fn listen<F, E>(&mut self, handler: F, err: E)
        where F: Fn(TaskAction) + Send + 'static, E: Fn() + Send + 'static
    {
        let cloned = self.task_receiver.clone();
        thread::spawn(move || {
            loop {
                let tr = cloned.lock().unwrap();
                match tr.recv() {
                    Ok(task) => {
                        if matches!(task, TaskAction::Shutdown) {
                            break;
                        }
                        handler(task);
                    }
                    Err(e) => {
                        println!("Error receiving: {:?}", e);
                        err();
                        break;
                    }
                }
            }
        });
    }

    pub fn send(&mut self, val: TaskAction) {
        let _ = self.task_sender.lock().unwrap().send(val);
    }
}