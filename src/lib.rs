use std::sync::mpsc; // multi producer single consumer that mean we need a way that worker can own and mutate the reciever of the channel
use std::sync::Arc; // let multiple workers own the reciever
use std::sync::Mutex; // ensure that only one worker gets a job from the receiver at a time
use std::thread;

pub struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(|| {
            receiver;
        });
        Worker { id, thread }
    }
}

struct Job;

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

impl ThreadPool {
    /// Create a new ThreadPool that holds a vector of workers
    /// and the sending end of the channel
    ///
    /// The size is the number of threads in the pool
    ///
    /// # Panics
    /// The `new` function will panic if the size is zero
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);
        let (sender, receiver) = mpsc::channel(); // channel act as a queque for jobs
        let receiver = Arc::new(Mutex::new(receiver)); // put the receiving end of the channel in an Arc and a Mutex

        let mut workers = Vec::with_capacity(size);
        for id in 0..size {
            // create some workers and store them in a vector
            // for each new worker, clone the Arc to bump the reference count so the workers
            // can share ownership of the receiving end
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }
        ThreadPool { workers, sender }
    }

    pub fn excute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static, // we use the () after FnOnce because this FnOnce represents a closure that takes no parameters and doesn't return a value
    {
    }
}
