use std::sync::mpsc; // multi producer single consumer that mean we need a way that worker can own and mutate the reciever of the channel
use std::sync::Arc; // let multiple workers own the reciever
use std::sync::Mutex; // ensure that only one worker gets a job from the receiver at a time
use std::thread;

/// Worker Struct is Responsible for Sending Code from the ThreadPool to a Thread
/// because thread::spawn wanna give the thread a code to execute as soon as the thread is created
/// but we want to create thread and make them wait for the code
///
/// each worker will store a single JoinHandle<()> instance
/// and has a method that will take a closure of code to run and send it to the alreading running thread for execution
pub struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>, // () because closure passed to the threadpool will handle the connection and doesn't return anything
}

impl Worker {
    /// new worker holds an id and the receiving side of the channel
    /// and in this thread, the worker will loop over its receiving side of the channel
    /// and execute the closures of any jobs it receives
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(|| {
            receiver;
        });
        Worker { id, thread }
    }
}

/// Job struct holds the closure we want to send to the worker
struct Job;

/// ThreadPool
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
    /// execute method will send a job from threadpool(sending side) to the Worker instances(receiving side)
    /// which will send the job to its thread.
    ///
    pub fn excute<F>(&self, f: F)
    where
        // FnOnce: because the thread for running a request will ony execute that request's closure one time
        // we use the () after FnOnce because this FnOnce represents a closure that takes no parameters and doesn't return a value
        // Send: to transfer the closure from one thread to another
        // 'static: because we don't know how long the thread will take to excute
        F: FnOnce() + Send + 'static,
    {
    }
}
