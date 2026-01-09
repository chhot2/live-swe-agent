//! Connection Pool Implementation
//!
//! Provides efficient connection management through pooling to reduce
//! connection establishment overhead and improve network performance.

use std::collections::VecDeque;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use super::EfficiencyMetrics;

/// Represents a pooled connection
#[derive(Debug)]
pub struct PooledConnection {
    pub id: usize,
    pub host: String,
    pub port: u16,
    pub created_at: Instant,
    pub last_used: Instant,
    pub is_healthy: bool,
}

impl PooledConnection {
    pub fn new(id: usize, host: String, port: u16) -> Self {
        let now = Instant::now();
        Self {
            id,
            host,
            port,
            created_at: now,
            last_used: now,
            is_healthy: true,
        }
    }

    pub fn is_stale(&self, max_idle: Duration) -> bool {
        self.last_used.elapsed() > max_idle
    }

    pub fn touch(&mut self) {
        self.last_used = Instant::now();
    }

    pub fn age(&self) -> Duration {
        self.created_at.elapsed()
    }
}

/// Connection pool for managing reusable connections
#[derive(Debug)]
pub struct ConnectionPool {
    connections: Mutex<VecDeque<PooledConnection>>,
    max_size: usize,
    max_idle: Duration,
    max_age: Duration,
    id_counter: AtomicUsize,
    metrics: Arc<EfficiencyMetrics>,
    in_use: AtomicUsize,
}

impl ConnectionPool {
    pub fn new(max_size: usize, metrics: Arc<EfficiencyMetrics>) -> Self {
        Self {
            connections: Mutex::new(VecDeque::with_capacity(max_size)),
            max_size,
            max_idle: Duration::from_secs(300),
            max_age: Duration::from_secs(3600),
            id_counter: AtomicUsize::new(0),
            metrics,
            in_use: AtomicUsize::new(0),
        }
    }

    pub fn acquire(&self, host: &str, port: u16) -> PooledConnection {
        let mut connections = self.connections.lock().unwrap_or_else(|e| e.into_inner());

        let position = connections.iter().position(|c| {
            c.host == host && c.port == port && c.is_healthy && !c.is_stale(self.max_idle)
        });

        if let Some(pos) = position {
            let mut conn = connections.remove(pos).expect("Connection exists");
            conn.touch();
            self.metrics.record_buffer_reuse();
            self.in_use.fetch_add(1, Ordering::Relaxed);
            return conn;
        }

        let id = self.id_counter.fetch_add(1, Ordering::Relaxed);
        self.in_use.fetch_add(1, Ordering::Relaxed);
        PooledConnection::new(id, host.to_string(), port)
    }

    pub fn release(&self, mut connection: PooledConnection) {
        self.in_use.fetch_sub(1, Ordering::Relaxed);

        if !connection.is_healthy || connection.age() > self.max_age {
            return;
        }

        connection.touch();

        let mut connections = self.connections.lock().unwrap_or_else(|e| e.into_inner());
        if connections.len() < self.max_size {
            connections.push_back(connection);
        }
    }

    pub fn available(&self) -> usize {
        self.connections
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .len()
    }

    pub fn in_use(&self) -> usize {
        self.in_use.load(Ordering::Relaxed)
    }

    pub fn stats(&self) -> ConnectionPoolStats {
        let connections = self.connections.lock().unwrap_or_else(|e| e.into_inner());
        ConnectionPoolStats {
            available: connections.len(),
            in_use: self.in_use.load(Ordering::Relaxed),
            max_size: self.max_size,
            total_created: self.id_counter.load(Ordering::Relaxed),
        }
    }

    pub fn clear(&self) {
        self.connections
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .clear();
    }
}

#[derive(Debug, Clone)]
pub struct ConnectionPoolStats {
    pub available: usize,
    pub in_use: usize,
    pub max_size: usize,
    pub total_created: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_connection_pool() {
        let metrics = Arc::new(EfficiencyMetrics::default());
        let pool = ConnectionPool::new(10, metrics);

        let conn = pool.acquire("localhost", 8080);
        assert_eq!(conn.host, "localhost");
        pool.release(conn);
        assert_eq!(pool.available(), 1);
    }
}
