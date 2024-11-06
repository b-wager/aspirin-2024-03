use std::sync::{mpsc, Arc, Mutex};

use crate::error::ThreadPoolError;

#[derive(Debug)]
pub struct ThreadPool {
    workers: Vec<Worker>,
    threads: Vec<std::thread::JoinHandle<()>>,
    sender: mpsc::Sender<Message>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

enum Message {
    NewJob(Job),
    Terminate,
}

impl ThreadPool {
    /// Create a new LocalThreadPool with num_threads threads.
    ///
    /// Errors:
    /// - If num_threads is 0, return an error
    pub fn new(num_threads: usize) -> Result<ThreadPool, ThreadPoolError> {
        if num_threads == 0 {
            return Err(ThreadPoolError::ZeroThreads);
        }
        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(num_threads);
        let mut threads = Vec::with_capacity(num_threads);

        for id in 0..num_threads {
            let (worker, thread) = Worker::new(id, Arc::clone(&receiver));
            workers.push(worker);
            threads.push(thread);
        }
        Ok(ThreadPool {
            workers,
            threads,
            sender,
        })
    }

    /// Execute the provided function on the thread pool
    ///
    /// Errors:
    /// - If we fail to send a message, report an error
    pub fn execute<F: FnOnce() + Send + 'static>(&self, f: F) -> Result<(), ThreadPoolError> {
        let job = Box::new(f);
        match self.sender.send(Message::NewJob(job)) {
            Ok(_) => Ok(()),
            Err(_) => {
                return Err(ThreadPoolError::Send);
            }
        }
    }

    /// Retrieve any results from the thread pool that have been computed
    pub fn get_results(self) -> Vec<()> {
        // Send a terminate message to all workers
        self.workers.iter().for_each(|_| {
            self.sender.send(Message::Terminate).unwrap();
        });

        // Collect results from all workers
        self.threads
            .into_iter()
            .map(|thread| thread.join().unwrap())
            .collect()
    }
}

#[derive(Debug)]
struct Worker {
    _id: usize,
}

impl Worker {
    fn new(
        id: usize,
        receiver: Arc<Mutex<mpsc::Receiver<Message>>>,
    ) -> (Worker, std::thread::JoinHandle<()>) {
        let thread = std::thread::spawn(move || loop {
            let job = {
                let receiver = receiver.lock().unwrap();
                receiver.recv()
            };

            match job {
                Ok(Message::NewJob(job)) => job(),
                Ok(Message::Terminate) => break,
                Err(_) => break,
            }
        });

        (Worker { _id: id }, thread)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_thread_pool() {
        let pool = ThreadPool::new(4);
        assert!(pool.unwrap().workers.len() == 4);
    }

    #[test]
    fn test_create_thread_pool_with_zero_threads() {
        let pool = ThreadPool::new(0);
        assert!(pool.is_err());
    }

    #[test]
    fn test_execute_tasks_and_get_results() {
        let pool = ThreadPool::new(4).unwrap();
        let _ = pool.execute(|| println!("Task 1"));
        let _ = pool.execute(|| println!("Task 2"));
        let results = pool.get_results();
        assert_eq!(results.len(), 4);
    }

    #[test]
    fn test_tasks_execution() {
        let pool = ThreadPool::new(4).unwrap();
        let result = Arc::new(Mutex::new(0));
        let result_clone = Arc::clone(&result);

        let _ = pool.execute(move || {
            let mut num = result_clone.lock().unwrap();
            *num += 1;
        });

        pool.get_results();

        assert_eq!(*result.lock().unwrap(), 1);
    }

    #[test]
    fn test_multiple_tasks_execution() {
        let pool = ThreadPool::new(4).unwrap();
        let result = Arc::new(Mutex::new(0));
        let result_clone1 = Arc::clone(&result);
        let result_clone2 = Arc::clone(&result);

        let _ = pool.execute(move || {
            let mut num = result_clone1.lock().unwrap();
            *num += 1;
        });

        let _ = pool.execute(move || {
            let mut num = result_clone2.lock().unwrap();
            *num += 2;
        });

        pool.get_results();

        assert_eq!(*result.lock().unwrap(), 3);
    }
}
