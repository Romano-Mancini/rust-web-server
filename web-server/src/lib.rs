use std::{
    sync::{
        Arc, Mutex,
        mpsc::{self, Receiver},
    },
    thread::{self, JoinHandle},
};

/// Pool of threads of fixed size.
///
/// Preferred to the one thread/one request
/// architecture in order to prevent DoS attacks.
pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

struct Worker {
    _id: usize,
    handle: JoinHandle<()>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<Receiver<Job>>>) -> Self {
        Worker {
            _id: id,
            handle: thread::spawn(move || {
                loop {
                    let received = receiver.lock().unwrap().recv();

                    match received {
                        Ok(job) => {
                            println!("Worker with id = {id} started.");
                            job();
                        }
                        Err(_) => {
                            println!("Worker with id = {id} disconnected.");
                            break;
                        }
                    }
                }
            }),
        }
    }
}

impl ThreadPool {
    /// Create a new ThreadPool.
    ///
    /// Size is the number of threads in the pool.
    ///
    /// # Panics
    ///
    /// The `new` function panics if size is zero.
    pub fn new(size: usize) -> Self {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();
        let mut workers = Vec::with_capacity(size);
        let receiver = Arc::new(Mutex::new(receiver));

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool {
            workers,
            sender: Some(sender),
        }
    }

    /// Assigns a closure to a thread in the pool.
    ///
    /// `f` is a closure of type `FnOnce()`.
    pub fn assign<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        self.sender.as_ref().unwrap().send(Box::new(f)).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        drop(self.sender.take());
        for worker in self.workers.drain(..) {
            worker.handle.join().unwrap();
        }
    }
}
