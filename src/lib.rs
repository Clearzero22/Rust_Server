use std::sync::{Arc, mpsc, Mutex};
use std::thread;

pub struct ThreadPoll {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;
impl ThreadPoll {
    /// 创建线程池
    ///
    /// # Panics
    ///
    /// `new` 函数会在 size 为0 时触犯 panic
    pub fn new(size: usize) -> ThreadPoll {
        assert!(size > 0);
        let (sender, receiver)= mpsc::channel();

        let receiver = Arc::new(Mutex::new(receiver));
        let mut workers = Vec::with_capacity(size);
        for id in 0..size {
            // 创建线程并将它们储存到数组
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }
        ThreadPoll {
            workers,
            sender
        }
    }

    pub fn execute<F>(&self, f: F)
        where F: FnOnce() + Send +'static
        {
            let job = Box::new(f);
            self.sender.send(job).unwrap();
        }
}

struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || {
            loop {
                let job = receiver.lock().unwrap().recv().unwrap();
                println!("Worker {} got a job; executing.", id);
                job();
            }
        });
        Worker {
            id,
            thread
        }
    }
}