use crate::server::App;
use std::net::TcpStream;
use std::sync::{mpsc, Arc, Mutex};
use std::thread;

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>,
}

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

type Job = Box<dyn FnOnce() -> TcpStream + Send + 'static>;

impl Worker {
    fn new<F>(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>, factory: &F) -> Worker
    where
        F: Fn() -> App + Send + Clone + 'static,
    {
        let app = factory();
        let thread = thread::spawn(move || loop {
            println!("Worker {id} waiting for job.");

            let message = receiver.lock().unwrap().recv();

            match message {
                Ok(job) => {
                    println!("Worker {id} got a job; executing.");
                    let stream = job();
                    app.call(stream);
                }
                Err(_) => {
                    println!("Worker {id} disconnected; shutting down.");
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

impl ThreadPool {
    pub fn new<F>(size: usize, factory: F) -> ThreadPool
    where
        F: Fn() -> App + Send + Clone + 'static,
    {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();

        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);
        let factory = Box::new(factory);

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver), &factory));
        }

        ThreadPool {
            workers,
            sender: Some(sender),
        }
    }
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() -> TcpStream + Send + 'static,
    {
        let job = Box::new(f);

        self.sender.as_ref().unwrap().send(job).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        drop(self.sender.take());

        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);

            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}
