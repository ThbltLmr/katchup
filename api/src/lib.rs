use std::thread::{Builder, JoinHandle};

pub struct ThreadPool {
    threads: Vec<JoinHandle<()>>,
}

pub enum PoolCreationError {
    PoolSizeError,
    Error,
}

impl ThreadPool {
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
    }

    pub fn build(size: usize) -> Result<ThreadPool, PoolCreationError> {
        if size == 0 {
            return Err(PoolCreationError::PoolSizeError);
        }

        let mut threads = Vec::with_capacity(size);

        for _ in 0..size {
            match Builder::new().spawn(|| {
                // Thread work would go here
            }) {
                Ok(thread) => threads.push(thread),
                Err(_) => return Err(PoolCreationError::Error),
            }
        }

        Ok(ThreadPool { threads })
    }
}
