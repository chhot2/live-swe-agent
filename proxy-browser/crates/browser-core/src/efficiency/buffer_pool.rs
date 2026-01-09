//! # Buffer Pool Implementation
//!
//! Provides efficient buffer management with pooling to reduce memory allocations.

use super::EfficiencyMetrics;
use std::collections::VecDeque;
use std::sync::{Arc, Mutex};

/// A pooled buffer that can be reused
#[derive(Debug)]
pub struct PooledBuffer {
    data: Vec<u8>,
    pool: Arc<Mutex<VecDeque<Vec<u8>>>>,
    metrics: Arc<EfficiencyMetrics>,
}

impl PooledBuffer {
    pub fn new(
        capacity: usize,
        pool: Arc<Mutex<VecDeque<Vec<u8>>>>,
        metrics: Arc<EfficiencyMetrics>,
    ) -> Self {
        Self {
            data: Vec::with_capacity(capacity),
            pool,
            metrics,
        }
    }

    pub fn as_slice(&self) -> &[u8] {
        &self.data
    }
    pub fn as_mut_slice(&mut self) -> &mut [u8] {
        &mut self.data
    }
    pub fn capacity(&self) -> usize {
        self.data.capacity()
    }
    pub fn len(&self) -> usize {
        self.data.len()
    }
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
    pub fn clear(&mut self) {
        self.data.clear();
    }
    pub fn extend_from_slice(&mut self, data: &[u8]) {
        self.data.extend_from_slice(data);
    }
    pub fn write(&mut self, data: &[u8]) {
        self.data.clear();
        self.data.extend_from_slice(data);
    }
}

impl Drop for PooledBuffer {
    fn drop(&mut self) {
        let mut buffer = std::mem::take(&mut self.data);
        buffer.clear();
        if let Ok(mut pool) = self.pool.lock() {
            if pool.len() < 1024 {
                pool.push_back(buffer);
                self.metrics.record_buffer_reuse();
            }
        }
    }
}

/// Buffer pool for efficient memory management
#[derive(Debug)]
pub struct BufferPool {
    pool: Arc<Mutex<VecDeque<Vec<u8>>>>,
    default_capacity: usize,
    metrics: Arc<EfficiencyMetrics>,
}

impl BufferPool {
    pub fn new(initial_size: usize, metrics: Arc<EfficiencyMetrics>) -> Self {
        let pool = Arc::new(Mutex::new(VecDeque::with_capacity(initial_size)));
        if let Ok(mut p) = pool.lock() {
            for _ in 0..initial_size.min(64) {
                p.push_back(Vec::with_capacity(8192));
            }
        }
        Self {
            pool,
            default_capacity: 8192,
            metrics,
        }
    }

    pub fn acquire(&self) -> PooledBuffer {
        let data = self
            .pool
            .lock()
            .ok()
            .and_then(|mut p| p.pop_front())
            .unwrap_or_else(|| Vec::with_capacity(self.default_capacity));
        PooledBuffer {
            data,
            pool: Arc::clone(&self.pool),
            metrics: Arc::clone(&self.metrics),
        }
    }

    pub fn acquire_with_capacity(&self, capacity: usize) -> PooledBuffer {
        let data = self
            .pool
            .lock()
            .ok()
            .and_then(|mut p| {
                p.iter()
                    .position(|b| b.capacity() >= capacity)
                    .and_then(|i| p.remove(i))
            })
            .unwrap_or_else(|| Vec::with_capacity(capacity));
        PooledBuffer {
            data,
            pool: Arc::clone(&self.pool),
            metrics: Arc::clone(&self.metrics),
        }
    }

    pub fn size(&self) -> usize {
        self.pool.lock().map(|p| p.len()).unwrap_or(0)
    }
    pub fn clear(&self) {
        if let Ok(mut p) = self.pool.lock() {
            p.clear();
        }
    }
}

impl Clone for BufferPool {
    fn clone(&self) -> Self {
        Self {
            pool: Arc::clone(&self.pool),
            default_capacity: self.default_capacity,
            metrics: Arc::clone(&self.metrics),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_buffer_pool() {
        let metrics = Arc::new(EfficiencyMetrics::default());
        let pool = BufferPool::new(10, metrics);
        let mut buffer = pool.acquire();
        buffer.extend_from_slice(b"hello");
        assert_eq!(buffer.len(), 5);
    }
}
