use std::sync::{Arc, Mutex};
use std::thread;

#[macro_use]
extern crate fstrings;

pub struct WebRequest {
    work: Box<dyn FnOnce(&str) + Send + Sync>,
}

impl WebRequest {
    pub fn new<F>(f: F) -> WebRequest
    where
        F: FnOnce(&str) + Send + Sync + 'static,
    {
        WebRequest { work: Box::new(f) }
    }
}

pub struct ThreadPool {
    workers: Vec<thread::JoinHandle<()>>,
    queue: Arc<Mutex<Vec<WebRequest>>>,
}

impl ThreadPool {
    pub fn new(num_threads: usize) -> ThreadPool {
        let mut workers = Vec::with_capacity(num_threads);
        let queue = Arc::new(Mutex::new(Vec::<WebRequest>::new()));
    
        for i in 0..num_threads {
            let number = f!("Request {i}");
            let queue_clone = Arc::clone(&queue);
            workers.push(thread::spawn(move || loop {
                let task = queue_clone.lock().unwrap().pop();
                if let Some(task) = task {
                    (task.work)(&number);
                } else {
                    break;
                }
            }));
        }
        ThreadPool { workers, queue }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce(&str) + Send + Sync + 'static,
    {
        let task = WebRequest::new(f);
        self.queue.lock().unwrap().push(task);
    }

    pub fn join(self) {
        for worker in self.workers {
            worker.join().unwrap();
        }
    }
}


