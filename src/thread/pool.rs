use std::sync::{Arc, mpsc, Mutex};
use std::sync::mpsc::Receiver;
use std::thread;
use std::thread::JoinHandle;
use log::debug;

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Task>
}

pub struct Worker {
    id: usize,
    handle: JoinHandle<()>,
}

type Task = Box<dyn FnOnce() + Send + 'static>;

impl Worker {
    pub fn new(id: usize, receiver: Arc<Mutex<Receiver<Task>>>) -> Worker {
        let handle = thread::spawn(move || loop {
            let task = receiver.lock().unwrap().recv().unwrap();
            debug!("Worker thread {} received a request", id);
            task();
        });
        debug!("Worker thread {} created", id);
        Worker { id, handle }
    }
}


impl ThreadPool {
    pub fn new(max_threads: usize) -> Self {
        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));
        let mut workers = Vec::with_capacity(max_threads);

        for id in 0..max_threads {
               workers.push(Worker::new(id, Arc::clone(&receiver)))
        }

        ThreadPool {
            workers,
            sender
        }
    }

    pub fn execute<F>(&mut self, f: F) where F: FnOnce() + Send + 'static {
        let task = Box::new(f);
        self.sender.send(task).unwrap()
    }
}