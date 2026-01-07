use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::{mem, thread};
struct Worker {
    thread: thread::JoinHandle<()>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Option<i32>>>>) -> Self {
        let thread = thread::spawn(move || loop {
            let Some(job) = receiver.lock().unwrap().recv().unwrap() else {
                println!("worker {} exited", id);
                break;
            };
            println!("worker {} received job {}", id, job);
            thread::sleep(std::time::Duration::from_secs(1));
        });
        Self { thread }
    }
}

struct WorkerPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Option<i32>>,
}

impl WorkerPool {
    fn new(size: usize) -> Self {
        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));
        let mut workers = Vec::with_capacity(size);
        for id in 0..size {
            workers.push(Worker::new(id, receiver.clone()));
        }
        Self { workers, sender }
    }

    fn execute(&self, job: i32) {
        self.sender.send(Some(job)).unwrap();
    }
}

impl Drop for WorkerPool {
    fn drop(&mut self) {
        // send all workers a signal to exit
        for _ in &self.workers {
            self.sender.send(None).unwrap();
        }
        // wait for all workers to exit
        for worker in mem::take(&mut self.workers) {
            worker.thread.join().unwrap();
        }
    }
}

pub fn test() {
    let pool = WorkerPool::new(10);
    for i in 0..100 {
        pool.execute(i);
    }
}
