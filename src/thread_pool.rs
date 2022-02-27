use std::sync::{mpsc, Arc, Mutex};

use crate::{
    message::Message,
    result::{ServiceError, ServiceResult},
    worker::Worker,
};

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>,
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        for _ in &self.workers {
            self.sender.send(Message::Terminate).unwrap();
        }

        for worker in &mut self.workers {
            println!("shutting down worker {}", worker.id);
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

impl ThreadPool {
    /// Create a new ThreadPool.
    ///
    /// The size is the number of threads in the pool.
    ///
    /// # Panics
    ///
    /// The `new` function will panic if the size is zero.

    pub fn new(size: usize) -> ServiceResult<Self> {
        if size == 0 {
            return Err(ServiceError::ThreadPoolSizeMustBePositive);
        }

        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            let worker = Worker::new(id, Arc::clone(&receiver));
            workers.push(worker);
        }
        return Ok(Self { workers, sender });
    }

    pub fn execute<F>(&self, f: F) -> ServiceResult<()>
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.send(Message::NewJob(job))?;
        Ok(())
    }
}
