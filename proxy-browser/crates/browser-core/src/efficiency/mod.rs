//! # Efficiency Optimization Module
//!
//! This module provides comprehensive efficiency optimizations for the proxy browser,
//! addressing performance concerns across all components.

pub mod async_executor;
pub mod buffer_pool;
pub mod cache_manager;
pub mod connection_pool;
pub mod cpu_optimizer;
pub mod memory_optimizer;

pub use async_executor::{AsyncExecutor, ExecutorStats, RateLimiter, ScheduledTask, TaskPriority};
pub use buffer_pool::{BufferPool, PooledBuffer};
pub use cache_manager::OptimizedCacheManager;
pub use connection_pool::{ConnectionPool, ConnectionPoolStats, PooledConnection};
pub use cpu_optimizer::{unrolled_loop, BatchProcessor, CpuOptimizer, ParallelProcessor};
pub use memory_optimizer::{MemoryOptimizer, MemoryPressure, MemoryStats, MemoryThresholds};

use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

/// Global efficiency metrics tracker
#[derive(Debug, Default)]
pub struct EfficiencyMetrics {
    pub cache_hits: AtomicUsize,
    pub cache_misses: AtomicUsize,
    pub buffer_reuses: AtomicUsize,
    pub memory_saved_bytes: AtomicUsize,
    pub cpu_cycles_saved: AtomicUsize,
}

impl EfficiencyMetrics {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn record_cache_hit(&self) {
        self.cache_hits.fetch_add(1, Ordering::Relaxed);
    }
    pub fn record_cache_miss(&self) {
        self.cache_misses.fetch_add(1, Ordering::Relaxed);
    }
    pub fn record_buffer_reuse(&self) {
        self.buffer_reuses.fetch_add(1, Ordering::Relaxed);
    }
    pub fn record_memory_saved(&self, bytes: usize) {
        self.memory_saved_bytes.fetch_add(bytes, Ordering::Relaxed);
    }

    pub fn cache_hit_rate(&self) -> f64 {
        let hits = self.cache_hits.load(Ordering::Relaxed);
        let misses = self.cache_misses.load(Ordering::Relaxed);
        let total = hits + misses;
        if total == 0 {
            0.0
        } else {
            hits as f64 / total as f64
        }
    }

    pub fn get_stats(&self) -> EfficiencyStats {
        EfficiencyStats {
            cache_hits: self.cache_hits.load(Ordering::Relaxed),
            cache_misses: self.cache_misses.load(Ordering::Relaxed),
            buffer_reuses: self.buffer_reuses.load(Ordering::Relaxed),
            memory_saved_bytes: self.memory_saved_bytes.load(Ordering::Relaxed),
            cpu_cycles_saved: self.cpu_cycles_saved.load(Ordering::Relaxed),
        }
    }
}

/// Snapshot of efficiency statistics
#[derive(Debug, Clone)]
pub struct EfficiencyStats {
    pub cache_hits: usize,
    pub cache_misses: usize,
    pub buffer_reuses: usize,
    pub memory_saved_bytes: usize,
    pub cpu_cycles_saved: usize,
}

/// Configuration for efficiency optimizations
#[derive(Debug, Clone)]
pub struct EfficiencyConfig {
    pub enable_memory_pooling: bool,
    pub enable_connection_pooling: bool,
    pub enable_caching: bool,
    pub enable_lazy_init: bool,
    pub enable_batch_processing: bool,
    pub buffer_pool_size: usize,
    pub connection_pool_size: usize,
    pub cache_ttl_seconds: u64,
    pub max_batch_size: usize,
}

impl Default for EfficiencyConfig {
    fn default() -> Self {
        Self {
            enable_memory_pooling: true,
            enable_connection_pooling: true,
            enable_caching: true,
            enable_lazy_init: true,
            enable_batch_processing: true,
            buffer_pool_size: 1024,
            connection_pool_size: 100,
            cache_ttl_seconds: 300,
            max_batch_size: 100,
        }
    }
}

/// Main efficiency optimization manager
pub struct EfficiencyManager {
    config: EfficiencyConfig,
    metrics: Arc<EfficiencyMetrics>,
    buffer_pool: Option<BufferPool>,
    cache_manager: Option<OptimizedCacheManager>,
    connection_pool: Option<ConnectionPool>,
}

impl EfficiencyManager {
    pub fn new(config: EfficiencyConfig) -> Self {
        let metrics = Arc::new(EfficiencyMetrics::new());
        let buffer_pool = if config.enable_memory_pooling {
            Some(BufferPool::new(
                config.buffer_pool_size,
                Arc::clone(&metrics),
            ))
        } else {
            None
        };
        let cache_manager = if config.enable_caching {
            Some(OptimizedCacheManager::new(
                config.cache_ttl_seconds,
                Arc::clone(&metrics),
            ))
        } else {
            None
        };
        let connection_pool = if config.enable_connection_pooling {
            Some(ConnectionPool::new(
                config.connection_pool_size,
                Arc::clone(&metrics),
            ))
        } else {
            None
        };

        Self {
            config,
            metrics,
            buffer_pool,
            cache_manager,
            connection_pool,
        }
    }

    pub fn get_metrics(&self) -> Arc<EfficiencyMetrics> {
        Arc::clone(&self.metrics)
    }
    pub fn get_buffer_pool(&self) -> Option<&BufferPool> {
        self.buffer_pool.as_ref()
    }
    pub fn get_cache_manager(&self) -> Option<&OptimizedCacheManager> {
        self.cache_manager.as_ref()
    }
    pub fn get_connection_pool(&self) -> Option<&ConnectionPool> {
        self.connection_pool.as_ref()
    }
    pub fn config(&self) -> &EfficiencyConfig {
        &self.config
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_efficiency_metrics() {
        let metrics = EfficiencyMetrics::new();
        metrics.record_cache_hit();
        metrics.record_cache_hit();
        metrics.record_cache_miss();
        assert_eq!(metrics.cache_hits.load(Ordering::Relaxed), 2);
        assert_eq!(metrics.cache_misses.load(Ordering::Relaxed), 1);
    }

    #[test]
    fn test_efficiency_config_default() {
        let config = EfficiencyConfig::default();
        assert!(config.enable_memory_pooling);
        assert!(config.enable_caching);
    }

    #[test]
    fn test_efficiency_manager() {
        let config = EfficiencyConfig::default();
        let manager = EfficiencyManager::new(config);
        assert!(manager.get_buffer_pool().is_some());
        assert!(manager.get_cache_manager().is_some());
        assert!(manager.get_connection_pool().is_some());
    }
}
