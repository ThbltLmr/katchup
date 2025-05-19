use std::{
    sync::{
        mpsc::{channel, Receiver, Sender},
        Arc, Mutex,
    },
    thread::{Builder, JoinHandle},
};

pub type Job = Box<dyn FnOnce() + Send + 'static>;

#[derive(Debug)]
pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Sender<Job>,
}

#[derive(Debug)]
pub enum PoolCreationError {
    PoolSizeError,
    Error,
}

// Flag: implementation based on Rust Book: https://doc.rust-lang.org/book/ch21-00-final-project-a-web-server.html
impl ThreadPool {
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);

        self.sender.send(job).unwrap();
    }

    pub fn build(size: usize) -> Result<ThreadPool, PoolCreationError> {
        if size == 0 {
            return Err(PoolCreationError::PoolSizeError);
        }

        let (sender, receiver) = channel();

        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver))?);
        }

        Ok(ThreadPool { workers, sender })
    }
}

#[derive(Debug)]
struct Worker {
    id: usize,
    thread: JoinHandle<()>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<Receiver<Job>>>) -> Result<Worker, PoolCreationError> {
        match Builder::new().spawn(move || loop {
            let job = receiver
                .lock()
                .expect("Failed to acquire mutex lock!")
                .recv()
                .expect("Channel sender has stopped");

            job();
        }) {
            Ok(thread) => Ok(Worker { id, thread }),
            Err(_) => Err(PoolCreationError::Error),
        }
    }
}
