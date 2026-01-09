//! Memory Optimizer Implementation
//!
//! Provides memory optimization utilities including allocation tracking,
//! memory pressure monitoring, and garbage collection hints.

use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::Arc;

use super::EfficiencyMetrics;

#[derive(Debug, Clone)]
pub struct MemoryThresholds {
    pub warning: usize,
    pub critical: usize,
    pub target: usize,
}

impl Default for MemoryThresholds {
    fn default() -> Self {
        Self {
            warning: 512 * 1024 * 1024,
            critical: 1024 * 1024 * 1024,
            target: 256 * 1024 * 1024,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MemoryPressure {
    Normal,
    Warning,
    Critical,
}

#[derive(Debug)]
pub struct MemoryOptimizer {
    allocated_bytes: AtomicUsize,
    peak_bytes: AtomicUsize,
    allocation_count: AtomicUsize,
    deallocation_count: AtomicUsize,
    thresholds: MemoryThresholds,
    tracking_enabled: AtomicBool,
    metrics: Arc<EfficiencyMetrics>,
}

impl MemoryOptimizer {
    pub fn new(metrics: Arc<EfficiencyMetrics>) -> Self {
        Self {
            allocated_bytes: AtomicUsize::new(0),
            peak_bytes: AtomicUsize::new(0),
            allocation_count: AtomicUsize::new(0),
            deallocation_count: AtomicUsize::new(0),
            thresholds: MemoryThresholds::default(),
            tracking_enabled: AtomicBool::new(true),
            metrics,
        }
    }

    pub fn with_thresholds(thresholds: MemoryThresholds, metrics: Arc<EfficiencyMetrics>) -> Self {
        Self {
            allocated_bytes: AtomicUsize::new(0),
            peak_bytes: AtomicUsize::new(0),
            allocation_count: AtomicUsize::new(0),
            deallocation_count: AtomicUsize::new(0),
            thresholds,
            tracking_enabled: AtomicBool::new(true),
            metrics,
        }
    }

    pub fn record_allocation(&self, size: usize) {
        if !self.tracking_enabled.load(Ordering::Relaxed) {
            return;
        }

        let current = self.allocated_bytes.fetch_add(size, Ordering::Relaxed) + size;
        self.allocation_count.fetch_add(1, Ordering::Relaxed);

        let mut peak = self.peak_bytes.load(Ordering::Relaxed);
        while current > peak {
            match self.peak_bytes.compare_exchange_weak(
                peak,
                current,
                Ordering::Relaxed,
                Ordering::Relaxed,
            ) {
                Ok(_) => break,
                Err(p) => peak = p,
            }
        }
    }

    pub fn record_deallocation(&self, size: usize) {
        if !self.tracking_enabled.load(Ordering::Relaxed) {
            return;
        }

        self.allocated_bytes.fetch_sub(size, Ordering::Relaxed);
        self.deallocation_count.fetch_add(1, Ordering::Relaxed);
        self.metrics.record_memory_saved(size);
    }

    pub fn allocated(&self) -> usize {
        self.allocated_bytes.load(Ordering::Relaxed)
    }

    pub fn peak(&self) -> usize {
        self.peak_bytes.load(Ordering::Relaxed)
    }

    pub fn pressure(&self) -> MemoryPressure {
        let current = self.allocated();
        if current >= self.thresholds.critical {
            MemoryPressure::Critical
        } else if current >= self.thresholds.warning {
            MemoryPressure::Warning
        } else {
            MemoryPressure::Normal
        }
    }

    pub fn is_above_target(&self) -> bool {
        self.allocated() > self.thresholds.target
    }

    pub fn enable_tracking(&self) {
        self.tracking_enabled.store(true, Ordering::Relaxed);
    }

    pub fn disable_tracking(&self) {
        self.tracking_enabled.store(false, Ordering::Relaxed);
    }

    pub fn reset(&self) {
        self.allocated_bytes.store(0, Ordering::Relaxed);
        self.peak_bytes.store(0, Ordering::Relaxed);
        self.allocation_count.store(0, Ordering::Relaxed);
        self.deallocation_count.store(0, Ordering::Relaxed);
    }

    pub fn stats(&self) -> MemoryStats {
        MemoryStats {
            allocated_bytes: self.allocated_bytes.load(Ordering::Relaxed),
            peak_bytes: self.peak_bytes.load(Ordering::Relaxed),
            allocation_count: self.allocation_count.load(Ordering::Relaxed),
            deallocation_count: self.deallocation_count.load(Ordering::Relaxed),
            pressure: self.pressure(),
        }
    }

    pub fn suggest_gc(&self) -> bool {
        matches!(
            self.pressure(),
            MemoryPressure::Warning | MemoryPressure::Critical
        )
    }
}

#[derive(Debug, Clone)]
pub struct MemoryStats {
    pub allocated_bytes: usize,
    pub peak_bytes: usize,
    pub allocation_count: usize,
    pub deallocation_count: usize,
    pub pressure: MemoryPressure,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_memory_optimizer() {
        let metrics = Arc::new(EfficiencyMetrics::default());
        let optimizer = MemoryOptimizer::new(metrics);

        optimizer.record_allocation(1024);
        assert_eq!(optimizer.allocated(), 1024);

        optimizer.record_deallocation(512);
        assert_eq!(optimizer.allocated(), 512);
    }

    #[test]
    fn test_memory_pressure() {
        let metrics = Arc::new(EfficiencyMetrics::default());
        let thresholds = MemoryThresholds {
            warning: 100,
            critical: 200,
            target: 50,
        };
        let optimizer = MemoryOptimizer::with_thresholds(thresholds, metrics);

        assert_eq!(optimizer.pressure(), MemoryPressure::Normal);

        optimizer.record_allocation(150);
        assert_eq!(optimizer.pressure(), MemoryPressure::Warning);
    }
}
