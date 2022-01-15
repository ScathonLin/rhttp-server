use std::{
    sync::{Arc, mpsc, Mutex},
    future::Future,
    sync::mpsc::{Receiver, Sender},
    thread,
    thread::JoinHandle,
};

type Task = Box<dyn FnOnce() + Send + 'static>;

pub struct ThreadPool {
    workers: Vec<Worker>,
    task_sender: Sender<Task>,
}

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        if size > 512 {
            panic!("#size must between 1 and 2024");
        }

        let (task_sender, task_receiver) = mpsc::channel();
        let task_receiver_arc = Arc::new(Mutex::new(task_receiver));

        let mut workers = Vec::with_capacity(size);
        for i in 0..size {
            workers.push(Worker::new(i, Arc::clone(&task_receiver_arc)));
        }
        ThreadPool {
            workers,
            task_sender,
        }
    }

    pub fn execute(&self, task: Task) {
        self.task_sender.send(task).unwrap();
    }
}

struct Worker {
    id: usize,
    thread_handler: JoinHandle<()>,
}

impl Worker {
    fn new(id: usize, task_receiver: Arc<Mutex<Receiver<Task>>>) -> Worker {
        let thread_handler = thread::spawn(move || loop {
            let task = task_receiver.lock().unwrap().recv().unwrap();
            task();
        }
        );
        Worker {
            id,
            thread_handler,
        }
    }
}