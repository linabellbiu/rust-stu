use std::sync::{Arc, mpsc, Mutex};
use std::thread;

pub struct ThreadPool {
    // workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}


type Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool {
    /// 创建线程池。
    ///
    /// 线程池中线程的数量。
    ///
    /// # Panics
    ///
    /// `new` 函数在 size 为 0 时会 panic。
    pub fn new(size: usize) -> ThreadPool {
        if size <= 0 {
            panic!("Thread pool size")
        }

        let (sender, receiver) = mpsc::channel();

        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        for i in 0..size {
            workers.push(Worker::new(i, Arc::clone(&receiver)));
        }

        ThreadPool {
            //workers
            sender
        }
    }

    pub fn execute<F>(&self, f: F)
        where
            F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);

        self.sender.send(job).unwrap();
    }
}

struct Worker {
    // pub id: usize,
    // pub thead: thread::JoinHandle<()>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        thread::spawn(move || {
            loop {
                let job = receiver.lock().expect("线程被锁").recv().unwrap();

                println!("Worker {} got a job; executing.", id);

                job()
            }
        });

        Worker {
            // id,
            // thead
        }
    }
}
