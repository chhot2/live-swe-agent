//! # CPU Optimizer

use std::collections::VecDeque;
use std::sync::{
    atomic::{AtomicBool, AtomicUsize, Ordering},
    Arc, Mutex,
};
use std::time::{Duration, Instant};

#[inline]
pub fn likely(b: bool) -> bool {
    if !b {
        std::hint::spin_loop();
    }
    b
}
#[inline]
pub fn unlikely(b: bool) -> bool {
    if b {
        std::hint::spin_loop();
    }
    b
}

#[derive(Debug)]
pub struct BatchProcessor<T> {
    queue: VecDeque<T>,
    batch_size: usize,
    processed_count: AtomicUsize,
}

impl<T> BatchProcessor<T> {
    pub fn new(batch_size: usize) -> Self {
        Self {
            queue: VecDeque::with_capacity(batch_size * 2),
            batch_size,
            processed_count: AtomicUsize::new(0),
        }
    }
    pub fn push(&mut self, item: T) {
        self.queue.push_back(item);
    }
    pub fn is_batch_ready(&self) -> bool {
        self.queue.len() >= self.batch_size
    }
    pub fn len(&self) -> usize {
        self.queue.len()
    }
    pub fn is_empty(&self) -> bool {
        self.queue.is_empty()
    }

    pub fn process_batch<F>(&mut self, mut processor: F) -> usize
    where
        F: FnMut(Vec<T>),
    {
        let count = self.queue.len().min(self.batch_size);
        if count == 0 {
            return 0;
        }
        let batch: Vec<T> = self.queue.drain(..count).collect();
        processor(batch);
        self.processed_count.fetch_add(count, Ordering::Relaxed);
        count
    }

    pub fn flush<F>(&mut self, mut processor: F) -> usize
    where
        F: FnMut(Vec<T>),
    {
        let mut total = 0;
        while !self.queue.is_empty() {
            total += self.process_batch(&mut processor);
        }
        total
    }
}

#[derive(Debug)]
pub struct CpuOptimizer {
    measurements: Arc<Mutex<Vec<(String, Duration)>>>,
    enabled: AtomicBool,
}

impl CpuOptimizer {
    pub fn new() -> Self {
        Self {
            measurements: Arc::new(Mutex::new(Vec::new())),
            enabled: AtomicBool::new(true),
        }
    }

    pub fn measure<F, R>(&self, operation: &str, f: F) -> R
    where
        F: FnOnce() -> R,
    {
        if !self.enabled.load(Ordering::Relaxed) {
            return f();
        }
        let start = Instant::now();
        let result = f();
        if let Ok(mut m) = self.measurements.lock() {
            m.push((operation.to_string(), start.elapsed()));
            if m.len() > 1000 {
                m.drain(..500);
            }
        }
        result
    }

    pub fn average_duration(&self, operation: &str) -> Option<Duration> {
        let m = self.measurements.lock().ok()?;
        let matching: Vec<_> = m.iter().filter(|(op, _)| op == operation).collect();
        if matching.is_empty() {
            return None;
        }
        let total: Duration = matching.iter().map(|(_, d)| *d).sum();
        Some(total / matching.len() as u32)
    }

    pub fn set_enabled(&self, enabled: bool) {
        self.enabled.store(enabled, Ordering::Relaxed);
    }
    pub fn clear(&self) {
        if let Ok(mut m) = self.measurements.lock() {
            m.clear();
        }
    }
}

impl Default for CpuOptimizer {
    fn default() -> Self {
        Self::new()
    }
}

pub struct ParallelProcessor {
    thread_count: usize,
}

impl ParallelProcessor {
    pub fn new() -> Self {
        Self {
            thread_count: std::thread::available_parallelism()
                .map(|p| p.get())
                .unwrap_or(4),
        }
    }
    pub fn with_threads(count: usize) -> Self {
        Self {
            thread_count: count.max(1),
        }
    }
    pub fn thread_count(&self) -> usize {
        self.thread_count
    }

    pub fn process_parallel<T, F, R>(&self, items: Vec<T>, processor: F) -> Vec<R>
    where
        T: Send + Clone + 'static,
        R: Send + 'static,
        F: Fn(T) -> R + Send + Sync + Clone + 'static,
    {
        let chunk_size = (items.len() / self.thread_count).max(1);
        let chunks: Vec<Vec<T>> = items
            .into_iter()
            .collect::<Vec<_>>()
            .chunks(chunk_size)
            .map(|c| c.to_vec())
            .collect();
        let handles: Vec<_> = chunks
            .into_iter()
            .map(|chunk| {
                let proc = processor.clone();
                std::thread::spawn(move || chunk.into_iter().map(proc).collect::<Vec<_>>())
            })
            .collect();
        handles
            .into_iter()
            .filter_map(|h| h.join().ok())
            .flatten()
            .collect()
    }
}

impl Default for ParallelProcessor {
    fn default() -> Self {
        Self::new()
    }
}

#[inline]
pub fn unrolled_loop<F>(count: usize, mut f: F)
where
    F: FnMut(usize),
{
    let chunks = count / 4;
    let remainder = count % 4;
    for i in 0..chunks {
        let base = i * 4;
        f(base);
        f(base + 1);
        f(base + 2);
        f(base + 3);
    }
    for i in 0..remainder {
        f(chunks * 4 + i);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_batch_processor() {
        let mut proc: BatchProcessor<i32> = BatchProcessor::new(3);
        proc.push(1);
        proc.push(2);
        proc.push(3);
        assert!(proc.is_batch_ready());
    }

    #[test]
    fn test_unrolled_loop() {
        let mut sum = 0;
        unrolled_loop(10, |i| sum += i);
        assert_eq!(sum, 45);
    }
}
