use std::sync::{Arc, Barrier, Mutex};

use anyhow::Result;
use rand::Rng;

mod error;
mod thread_pool;

/// Generate a random vector of size capacity filled with random i64s
fn random_vec(capacity: usize) -> Vec<i64> {
    let mut vec = vec![0; capacity];
    rand::thread_rng().fill(&mut vec[..]);
    vec
}

/// Merge two sorted subarrays into a single sorted array
fn merge(left: &[i64], right: &[i64]) -> Vec<i64> {
    let mut result = Vec::with_capacity(left.len() + right.len());
    let (mut i, mut j) = (0, 0);

    while i < left.len() && j < right.len() {
        if left[i] < right[j] {
            result.push(left[i]);
            i += 1;
        } else {
            result.push(right[j]);
            j += 1;
        }
    }

    result.extend_from_slice(&left[i..]);
    result.extend_from_slice(&right[j..]);

    result
}

/// Parallel merge sort using a thread pool
fn merge_sort(
    pool: Arc<thread_pool::ThreadPool>,
    data: Arc<Mutex<Vec<i64>>>,
    barrier: Arc<Barrier>,
) {
    let mut data = data.lock().unwrap();
    if data.len() <= 1 {
        barrier.wait();
        return;
    }

    let mid = data.len() / 2;
    let (left, right) = data.split_at_mut(mid);

    let left = Arc::new(Mutex::new(left.to_vec()));
    let right = Arc::new(Mutex::new(right.to_vec()));

    {
        let pool_clone = Arc::clone(&pool);
        let right_clone = Arc::clone(&right);
        let barrier_clone = Arc::clone(&barrier);
        pool.execute(move || merge_sort(pool_clone, right_clone, barrier_clone))
            .unwrap();
    }

    {
        let pool_clone = Arc::clone(&pool);
        let left_clone = Arc::clone(&left);
        let barrier_clone = Arc::clone(&barrier);
        pool.execute(move || merge_sort(pool_clone, left_clone, barrier_clone))
            .unwrap();
    }

    barrier.wait();

    let left_sorted = left.lock().unwrap().to_vec();
    let right_sorted = right.lock().unwrap().to_vec();
    let result = merge(&left_sorted, &right_sorted);

    data.copy_from_slice(&result);
}

fn main() -> Result<()> {
    let data = random_vec(10);
    let pool = Arc::new(thread_pool::ThreadPool::new(4)?);
    let barrier = Arc::new(Barrier::new(3));

    let data = Arc::new(Mutex::new(data));
    {
        merge_sort(Arc::clone(&pool), data.clone(), Arc::clone(&barrier));
    }
    drop(data);
    drop(barrier);

    match Arc::try_unwrap(pool) {
        Ok(pool) => {
            pool.get_results();
        }
        Err(_) => {
            eprintln!("Failed to unwrap Arc<ThreadPool> because there are still other references.");
        }
    }

    Ok(())
}
