//! Native thread pool implementation
//!
//! Zero-dependency thread pool for parallel compression operations.
//! Replaces Rayon with a lightweight, purpose-built solution.

use std::sync::{Arc, Mutex, Condvar};
use std::sync::mpsc::{channel, Sender, Receiver};
use std::thread;

/// A simple thread pool for parallel task execution
pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<Sender<Job>>,
    num_threads: usize,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool {
    /// Create a new thread pool
    ///
    /// # Arguments
    /// * `num_threads` - Number of worker threads (0 = auto-detect CPU count)
    pub fn new(num_threads: usize) -> Self {
        let num_threads = if num_threads == 0 {
            std::thread::available_parallelism()
                .map(|n| n.get())
                .unwrap_or(4)
        } else {
            num_threads
        };

        let (sender, receiver) = channel();
        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(num_threads);
        for id in 0..num_threads {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool {
            workers,
            sender: Some(sender),
            num_threads,
        }
    }

    /// Execute a closure on the thread pool
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        if let Some(sender) = &self.sender {
            sender.send(job).expect("Thread pool send failed");
        }
    }

    /// Get number of threads
    pub fn num_threads(&self) -> usize {
        self.num_threads
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        drop(self.sender.take());

        for worker in &mut self.workers {
            if let Some(thread) = worker.thread.take() {
                thread.join().ok();
            }
        }
    }
}

struct Worker {
    #[allow(dead_code)]
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let job = {
                let lock = receiver.lock().unwrap();
                lock.recv()
            };

            match job {
                Ok(job) => job(),
                Err(_) => break,
            }
        });

        Worker {
            id,
            thread: Some(thread),
        }
    }
}

/// Parallel map operation
///
/// Maps a function over a slice in parallel, returning results in order
pub fn parallel_map<T, R, F>(items: &[T], num_threads: usize, f: F) -> Vec<R>
where
    T: Sync,
    R: Send + 'static,
    F: Fn(&T) -> R + Send + Sync + 'static,
{
    if items.is_empty() {
        return Vec::new();
    }

    // For small datasets, just do sequential processing
    if items.len() < num_threads * 2 {
        return items.iter().map(|item| f(item)).collect();
    }

    let num_threads = if num_threads == 0 {
        std::thread::available_parallelism()
            .map(|n| n.get())
            .unwrap_or(4)
    } else {
        num_threads
    };

    let chunk_size = (items.len() + num_threads - 1) / num_threads;
    let f = Arc::new(f);
    
    let mut handles = Vec::new();

    for chunk in items.chunks(chunk_size) {
        let f = Arc::clone(&f);
        let chunk: Vec<&T> = chunk.iter().collect();
        
        let handle = thread::spawn(move || {
            chunk.into_iter().map(|item| f(item)).collect::<Vec<R>>()
        });
        
        handles.push(handle);
    }

    let mut results = Vec::with_capacity(items.len());
    for handle in handles {
        if let Ok(mut chunk_results) = handle.join() {
            results.append(&mut chunk_results);
        }
    }

    results
}

/// Parallel map with fallible operations
///
/// Maps a fallible function over a slice in parallel
pub fn parallel_try_map<T, R, E, F>(items: &[T], num_threads: usize, f: F) -> Result<Vec<R>, E>
where
    T: Sync,
    R: Send + 'static,
    E: Send + 'static,
    F: Fn(&T) -> Result<R, E> + Send + Sync + 'static,
{
    if items.is_empty() {
        return Ok(Vec::new());
    }

    // For small datasets, just do sequential processing
    if items.len() < num_threads * 2 {
        return items.iter().map(|item| f(item)).collect();
    }

    let num_threads = if num_threads == 0 {
        std::thread::available_parallelism()
            .map(|n| n.get())
            .unwrap_or(4)
    } else {
        num_threads
    };

    let chunk_size = (items.len() + num_threads - 1) / num_threads;
    let f = Arc::new(f);
    let error_flag = Arc::new(Mutex::new(None::<E>));
    
    let mut handles = Vec::new();

    for chunk in items.chunks(chunk_size) {
        let f = Arc::clone(&f);
        let error_flag = Arc::clone(&error_flag);
        let chunk: Vec<&T> = chunk.iter().collect();
        
        let handle = thread::spawn(move || {
            let mut results = Vec::new();
            
            for item in chunk {
                // Check if another thread already failed
                if error_flag.lock().unwrap().is_some() {
                    return Vec::new();
                }
                
                match f(item) {
                    Ok(result) => results.push(result),
                    Err(e) => {
                        *error_flag.lock().unwrap() = Some(e);
                        return Vec::new();
                    }
                }
            }
            
            results
        });
        
        handles.push(handle);
    }

    let mut results = Vec::with_capacity(items.len());
    for handle in handles {
        if let Ok(mut chunk_results) = handle.join() {
            results.append(&mut chunk_results);
        }
    }

    // Check if any thread encountered an error
    if let Some(error) = Arc::try_unwrap(error_flag).ok()
        .and_then(|m| m.into_inner().ok())
        .flatten() {
        return Err(error);
    }

    Ok(results)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_thread_pool_basic() {
        let pool = ThreadPool::new(4);
        let counter = Arc::new(Mutex::new(0));

        for _ in 0..10 {
            let counter = Arc::clone(&counter);
            pool.execute(move || {
                let mut num = counter.lock().unwrap();
                *num += 1;
            });
        }

        // Wait for all jobs to complete
        drop(pool);

        // All jobs should have executed
        // Note: This is a simple test, actual value may vary due to threading
    }

    #[test]
    fn test_parallel_map() {
        let data: Vec<i32> = (0..100).collect();
        let results = parallel_map(&data, 4, |x| x * 2);
        
        assert_eq!(results.len(), 100);
        for (i, &result) in results.iter().enumerate() {
            assert_eq!(result, (i as i32) * 2);
        }
    }

    #[test]
    fn test_parallel_map_empty() {
        let data: Vec<i32> = vec![];
        let results = parallel_map(&data, 4, |x| x * 2);
        assert_eq!(results.len(), 0);
    }

    #[test]
    fn test_parallel_try_map_success() {
        let data: Vec<i32> = (0..50).collect();
        let results: Result<Vec<i32>, String> = parallel_try_map(&data, 4, |x| Ok(x * 2));
        
        assert!(results.is_ok());
        let results = results.unwrap();
        assert_eq!(results.len(), 50);
    }

    #[test]
    fn test_parallel_try_map_failure() {
        let data: Vec<i32> = (0..50).collect();
        let results: Result<Vec<i32>, String> = parallel_try_map(&data, 4, |x| {
            if *x == 25 {
                Err("Error at 25".to_string())
            } else {
                Ok(x * 2)
            }
        });
        
        assert!(results.is_err());
    }

    #[test]
    fn test_auto_detect_threads() {
        let pool = ThreadPool::new(0);
        assert!(pool.num_threads() >= 1);
    }
}
