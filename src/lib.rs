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
    thread: Option<thread::JoinHandle<()>>, // () because closure passed to the threadpool will handle the connection and doesn't return anything
}

impl Worker {
    /// new worker holds an id and the receiving side of the channel
    /// and in this thread, the worker will loop over its receiving side of the channel
    /// and execute the closures of any jobs it receives
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let message = receiver
            .lock()
            .expect("mutex is in poisoned state, which can happen if some other thread panicked while holding the lock rather than releasing the lock")
            .recv()
            .expect("the thread holding the sending side of the channel might have shut down");
            match message {
                Message::NewJob(job) => {
                    println!("Worker {} got a job; executing.", id);
                    job.call_box();
                }
                Message::Terminate => {
                    println!("Worker {} was told to terminate.", id);
                    break;
                }
            }
        });
        Worker {
            id,
            thread: Some(thread),
        }
    }
}

/// trait helps moving the closure out of the box
trait FnBox {
    /// similar to the call methods in the other Fn* traits
    /// except that it takes self:Box<Self> to take ownership of self
    /// and move the value out of the Box<T>
    fn call_box(self: Box<Self>);
}
impl<F: FnOnce()> FnBox for F {
    /// any FnOnce() closures can use call_box method
    /// call_box uses (*self)() to move the closure out of the box<T> and call the closure
    fn call_box(self: Box<Self>) {
        (*self)()
    }
}

/// Job type holds the closure we want to send to the worker
/// type alias for a -Box- trait object that holds the type of closure that execute receives
type Job = Box<dyn FnBox + Send + 'static>;

/// threads in the worker will listen to message enum to whether
/// (do a new job) or (exit the loop and stop)
enum Message {
    NewJob(Job),
    Terminate,
}

/// ThreadPool
pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>,
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
        let job = Box::new(f);

        self.sender.send(Message::NewJob(job)).unwrap();
    }
}

/// implement Drop trait to provide a graceful shutdown of ThreadPool
/// threads should all join to make sure they finish their work
/// so that threads don't shutdown in the middle of processing requests
impl Drop for ThreadPool {
    /// send one terminate message to each worker
    /// and once to call join on each worker's thread
    ///
    /// note: we used two loops because if we tried to send a message and join immediately in the same loop,
    ///       we couldn't guarantee that the worker in the current iteration
    ///       would be the one to get the message from the channel (Deadlock!)
    fn drop(&mut self) {
        println!("Sending terminate message to all workers");
        for _ in &mut self.workers {
            self.sender.send(Message::Terminate).unwrap();
        }
        println!("shutting down all workers");
        for worker in &mut self.workers {
            println!("shutting down worker {}", worker.id);

            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}
