use std::thread;

pub struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl Worker {
    fn new(id: usize) -> Worker {
        let thread = thread::spawn(|| {});
        Worker { id, thread }
    }
}

pub struct ThreadPool {
    workers: Vec<Worker>,
}

impl ThreadPool {
    /// Create a new ThreadPool
    ///
    /// The size is the number of threads in the pool
    ///
    /// # Panics
    /// The `new` function will panic if the size is zero
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let mut workers = Vec::with_capacity(size);
        for id in 0..size {
            // create some threads and store them in a vector
            workers.push(Worker::new(id));
        }
        ThreadPool { threads }
    }

    pub fn excute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static, // we use the () after FnOnce because this FnOnce represents a closure that takes no parameters and doesn't return a value
    {
    }
}
