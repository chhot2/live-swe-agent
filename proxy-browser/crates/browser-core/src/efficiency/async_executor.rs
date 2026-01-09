//! # Async Executor
//!
//! Provides async execution utilities for efficient asynchronous operations.

use std::collections::VecDeque;
use std::sync::{
    atomic::{AtomicBool, AtomicUsize, Ordering},
    Arc, Mutex,
};
use std::time::{Duration, Instant};

/// Task priority levels
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
pub enum TaskPriority {
    Low = 0,
    #[default]
    Normal = 1,
    High = 2,
    Critical = 3,
}

/// A scheduled task
#[derive(Debug)]
pub struct ScheduledTask {
    id: usize,
    name: String,
    priority: TaskPriority,
    scheduled_at: Instant,
    execute_after: Option<Duration>,
}

impl ScheduledTask {
    pub fn new(id: usize, name: String, priority: TaskPriority) -> Self {
        Self {
            id,
            name,
            priority,
            scheduled_at: Instant::now(),
            execute_after: None,
        }
    }
    pub fn with_delay(mut self, delay: Duration) -> Self {
        self.execute_after = Some(delay);
        self
    }
    pub fn is_ready(&self) -> bool {
        self.execute_after
            .map(|d| self.scheduled_at.elapsed() >= d)
            .unwrap_or(true)
    }
    pub fn id(&self) -> usize {
        self.id
    }
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn priority(&self) -> TaskPriority {
        self.priority
    }
}

/// Async executor for managing concurrent tasks
#[derive(Debug)]
pub struct AsyncExecutor {
    tasks: Arc<Mutex<VecDeque<ScheduledTask>>>,
    next_id: AtomicUsize,
    running: AtomicBool,
    completed_count: AtomicUsize,
    max_concurrent: usize,
}

impl AsyncExecutor {
    pub fn new(max_concurrent: usize) -> Self {
        Self {
            tasks: Arc::new(Mutex::new(VecDeque::new())),
            next_id: AtomicUsize::new(0),
            running: AtomicBool::new(false),
            completed_count: AtomicUsize::new(0),
            max_concurrent,
        }
    }

    pub fn schedule(&self, name: &str, priority: TaskPriority) -> usize {
        let id = self.next_id.fetch_add(1, Ordering::Relaxed);
        let task = ScheduledTask::new(id, name.to_string(), priority);
        if let Ok(mut tasks) = self.tasks.lock() {
            let pos = tasks
                .iter()
                .position(|t| t.priority < priority)
                .unwrap_or(tasks.len());
            tasks.insert(pos, task);
        }
        id
    }

    pub fn schedule_delayed(&self, name: &str, priority: TaskPriority, delay: Duration) -> usize {
        let id = self.next_id.fetch_add(1, Ordering::Relaxed);
        let task = ScheduledTask::new(id, name.to_string(), priority).with_delay(delay);
        if let Ok(mut tasks) = self.tasks.lock() {
            tasks.push_back(task);
        }
        id
    }

    pub fn pop_ready(&self) -> Option<ScheduledTask> {
        let mut tasks = self.tasks.lock().ok()?;
        let idx = tasks.iter().position(|t| t.is_ready())?;
        tasks.remove(idx)
    }

    pub fn mark_completed(&self) {
        self.completed_count.fetch_add(1, Ordering::Relaxed);
    }
    pub fn pending_count(&self) -> usize {
        self.tasks.lock().map(|t| t.len()).unwrap_or(0)
    }
    pub fn completed_count(&self) -> usize {
        self.completed_count.load(Ordering::Relaxed)
    }
    pub fn is_running(&self) -> bool {
        self.running.load(Ordering::Relaxed)
    }
    pub fn start(&self) {
        self.running.store(true, Ordering::Relaxed);
    }
    pub fn stop(&self) {
        self.running.store(false, Ordering::Relaxed);
    }

    pub fn stats(&self) -> ExecutorStats {
        ExecutorStats {
            pending: self.pending_count(),
            completed: self.completed_count(),
            max_concurrent: self.max_concurrent,
            running: self.is_running(),
        }
    }
}

impl Default for AsyncExecutor {
    fn default() -> Self {
        Self::new(4)
    }
}

#[derive(Debug, Clone)]
pub struct ExecutorStats {
    pub pending: usize,
    pub completed: usize,
    pub max_concurrent: usize,
    pub running: bool,
}

/// Rate limiter for controlling execution rate
#[derive(Debug)]
pub struct RateLimiter {
    tokens: AtomicUsize,
    max_tokens: usize,
    last_refill: Mutex<Instant>,
    refill_rate: Duration,
}

impl RateLimiter {
    pub fn new(max_tokens: usize, refill_rate: Duration) -> Self {
        Self {
            tokens: AtomicUsize::new(max_tokens),
            max_tokens,
            last_refill: Mutex::new(Instant::now()),
            refill_rate,
        }
    }

    pub fn try_acquire(&self) -> bool {
        self.refill();
        loop {
            let current = self.tokens.load(Ordering::Relaxed);
            if current == 0 {
                return false;
            }
            if self
                .tokens
                .compare_exchange_weak(current, current - 1, Ordering::Relaxed, Ordering::Relaxed)
                .is_ok()
            {
                return true;
            }
        }
    }

    fn refill(&self) {
        if let Ok(mut last) = self.last_refill.lock() {
            let elapsed = last.elapsed();
            if elapsed >= self.refill_rate {
                let tokens_to_add = (elapsed.as_millis() / self.refill_rate.as_millis()) as usize;
                let current = self.tokens.load(Ordering::Relaxed);
                let new_tokens = (current + tokens_to_add).min(self.max_tokens);
                self.tokens.store(new_tokens, Ordering::Relaxed);
                *last = Instant::now();
            }
        }
    }

    pub fn available(&self) -> usize {
        self.refill();
        self.tokens.load(Ordering::Relaxed)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_async_executor() {
        let executor = AsyncExecutor::new(4);
        let id = executor.schedule("test_task", TaskPriority::Normal);
        assert_eq!(id, 0);
        assert_eq!(executor.pending_count(), 1);
    }

    #[test]
    fn test_rate_limiter() {
        let limiter = RateLimiter::new(2, Duration::from_millis(100));
        assert!(limiter.try_acquire());
        assert!(limiter.try_acquire());
        assert!(!limiter.try_acquire());
    }
}
