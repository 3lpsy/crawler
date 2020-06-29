use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::{Arc, Condvar, Mutex};
use std::thread::spawn;
use std::thread::JoinHandle;

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Sender<Job>,
    count: Arc<(Mutex<usize>, Condvar)>,
}

impl ThreadPool {
    pub fn new(threads: usize) -> Self {
        // immediately create required threads
        // not efficient but whatevs
        let (tx, crx) = channel();
        // need exclusive ownership over receiver => mutex
        let mrx = Mutex::new(crx);
        // need arc for thread safe reference counter
        let rx = Arc::new(mrx);

        let mut workers = Vec::with_capacity(threads);

        for n in 0..threads {
            workers.push(Worker::new(n, Arc::clone(&rx)));
        }
        ThreadPool {
            workers: workers,
            sender: tx,
            count: Arc::new((Mutex::new(0), Condvar::new())),
        }
    }

    // work needs to be static so that it rust knows it must live forwever
    pub fn execute<F>(&self, callback: F)
    where
        F: FnOnce() + Send + 'static,
    {
        // unwrap just   assumes the receiver will always be there
        // not great but meh
        self.sender
            .send(Box::new(callback))
            .expect("Thread shut down too early");
    }

    pub fn join(&self) {
        println!("joining")
    }
}

struct Worker {
    id: usize,
    handle: JoinHandle<()>,
}

impl Worker {
    fn new(id: usize, rx: Arc<Mutex<Receiver<Job>>>) -> Self {
        let handle = spawn(move || loop {
            let result = rx.lock().unwrap().recv();
            match result {
                Ok(rx) => {
                    println!("Worker {} got a job; executing.", id);
                    rx()
                }
                Err(_) => {
                    println!("Worker {} signing off", id);
                    break;
                }
            }
        });
        Worker {
            id: id,
            handle: handle,
        }
    }
}

type Job = Box<dyn FnOnce() + Send + 'static>;
