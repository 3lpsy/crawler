use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::thread::spawn;
use std::thread::JoinHandle;

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Sender<Job>,
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
            println!("Creating worker {}", n);
            workers.push(Worker::new(n, Arc::clone(&rx)));
        }
        ThreadPool {
            workers: workers,
            sender: tx,
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

    pub fn join(self) {
        println!("Joining");
        for worker in self.workers {
            worker.join();
        }
    }
}

struct Worker {
    _id: usize,
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
            _id: id,
            handle: handle,
        }
    }
    fn join(self) {
        self.handle.join().expect("Worker failed to join");
    }
}

type Job = Box<dyn FnOnce() + Send + 'static>;
