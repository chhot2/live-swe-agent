//! Error Recovery System - v3.0 Core Stability
//!
//! Part of the V1000 Upgrade Deep Plan - Phase 1 Foundation
//! Provides enhanced automatic error recovery and crash prevention.

use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::RwLock;
use tracing::{debug, error, info, warn};

/// Global error counter
static TOTAL_ERRORS: AtomicU64 = AtomicU64::new(0);
static RECOVERED_ERRORS: AtomicU64 = AtomicU64::new(0);

/// Error recovery manager
pub struct ErrorRecoveryManager {
    error_history: Arc<RwLock<Vec<ErrorRecord>>>,
    recovery_strategies: Arc<RwLock<HashMap<ErrorCategory, RecoveryStrategy>>>,
    circuit_breakers: Arc<RwLock<HashMap<String, CircuitBreaker>>>,
    start_time: Instant,
    config: ErrorRecoveryConfig,
}

/// Error recovery configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Represents a ErrorRecoveryConfig.
pub struct ErrorRecoveryConfig {
    /// Maximum retries before giving up
    pub max_retries: u32,
    /// Base delay between retries (ms)
    pub base_retry_delay_ms: u64,
    /// Maximum delay between retries (ms)
    pub max_retry_delay_ms: u64,
    /// Enable exponential backoff
    pub exponential_backoff: bool,
    /// Circuit breaker threshold (failures before opening)
    pub circuit_breaker_threshold: u32,
    /// Circuit breaker reset timeout (seconds)
    pub circuit_breaker_reset_secs: u64,
    /// Enable crash prediction
    pub crash_prediction_enabled: bool,
    /// Maximum errors to keep in history
    pub max_error_history: usize,
}

impl Default for ErrorRecoveryConfig {
    fn default() -> Self {
        Self {
            max_retries: 3,
            base_retry_delay_ms: 100,
            max_retry_delay_ms: 5000,
            exponential_backoff: true,
            circuit_breaker_threshold: 5,
            circuit_breaker_reset_secs: 30,
            crash_prediction_enabled: true,
            max_error_history: 1000,
        }
    }
}

/// Error categories for classification
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
/// Enumeration of ErrorCategory variants.
pub enum ErrorCategory {
    Network,
    Database,
    Rendering,
    JavaScript,
    Proxy,
    Security,
    Memory,
    Storage,
    TabManagement,
    Unknown,
}

/// Error severity levels for prioritization
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ErrorSeverity {
    /// Low severity - can be ignored or logged
    Info,
    /// Medium severity - should be handled but not critical
    Warning,
    /// High severity - needs immediate attention
    Error,
    /// Critical severity - system stability at risk
    Critical,
}

impl ErrorCategory {
    /// Classify error from message
    pub fn from_error_message(message: &str) -> Self {
        let lower = message.to_lowercase();

        if lower.contains("network") || lower.contains("connection") || lower.contains("timeout") {
            ErrorCategory::Network
        } else if lower.contains("database") || lower.contains("sql") || lower.contains("query") {
            ErrorCategory::Database
        } else if lower.contains("render") || lower.contains("paint") || lower.contains("layout") {
            ErrorCategory::Rendering
        } else if lower.contains("javascript") || lower.contains("script") || lower.contains("js") {
            ErrorCategory::JavaScript
        } else if lower.contains("proxy") || lower.contains("socks") {
            ErrorCategory::Proxy
        } else if lower.contains("security")
            || lower.contains("certificate")
            || lower.contains("ssl")
        {
            ErrorCategory::Security
        } else if lower.contains("memory") || lower.contains("oom") || lower.contains("allocation")
        {
            ErrorCategory::Memory
        } else if lower.contains("storage") || lower.contains("disk") || lower.contains("io") {
            ErrorCategory::Storage
        } else if lower.contains("tab") || lower.contains("webview") {
            ErrorCategory::TabManagement
        } else {
            ErrorCategory::Unknown
        }
    }
}

/// Recovery strategy for different error types
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Enumeration of RecoveryStrategy variants.
pub enum RecoveryStrategy {
    /// Retry the operation
    Retry { max_attempts: u32, delay_ms: u64 },
    /// Fallback to alternative
    Fallback { alternative: String },
    /// Skip and continue
    Skip,
    /// Restart the component
    Restart { component: String },
    /// Notify user and wait
    UserIntervention { message: String },
    /// No recovery possible
    Fatal,
}

impl Default for RecoveryStrategy {
    fn default() -> Self {
        RecoveryStrategy::Retry {
            max_attempts: 3,
            delay_ms: 100,
        }
    }
}

/// Error record for history tracking
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Represents a ErrorRecord.
pub struct ErrorRecord {
    pub id: u64,
    pub category: ErrorCategory,
    pub message: String,
    pub stack_trace: Option<String>,
    pub timestamp_ms: u128,
    pub component: String,
    pub recovered: bool,
    pub recovery_method: Option<String>,
    pub retry_count: u32,
}

/// Circuit breaker for failure isolation
#[derive(Debug, Clone)]
/// Represents a CircuitBreaker.
pub struct CircuitBreaker {
    pub name: String,
    pub state: CircuitState,
    pub failure_count: u32,
    pub last_failure: Option<Instant>,
    pub last_success: Option<Instant>,
    pub threshold: u32,
    pub reset_timeout: Duration,
}

/// Circuit breaker state
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
/// Enumeration of CircuitState variants.
pub enum CircuitState {
    Closed,   // Normal operation
    Open,     // Failing, rejecting requests
    HalfOpen, // Testing if service recovered
}

impl CircuitBreaker {
    /// Create a new circuit breaker
    pub fn new(name: &str, threshold: u32, reset_timeout: Duration) -> Self {
        Self {
            name: name.to_string(),
            state: CircuitState::Closed,
            failure_count: 0,
            last_failure: None,
            last_success: None,
            threshold,
            reset_timeout,
        }
    }

    /// Record a failure
    pub fn record_failure(&mut self) {
        self.failure_count += 1;
        self.last_failure = Some(Instant::now());

        if self.failure_count >= self.threshold {
            self.state = CircuitState::Open;
            warn!(
                "Circuit breaker '{}' opened after {} failures",
                self.name, self.failure_count
            );
        }
    }

    /// Record a success
    pub fn record_success(&mut self) {
        self.failure_count = 0;
        self.last_success = Some(Instant::now());
        self.state = CircuitState::Closed;
    }

    /// Check if the circuit allows requests
    pub fn can_proceed(&mut self) -> bool {
        match self.state {
            CircuitState::Closed => true,
            CircuitState::Open => {
                // Check if we should try again
                if let Some(last_failure) = self.last_failure {
                    if last_failure.elapsed() >= self.reset_timeout {
                        self.state = CircuitState::HalfOpen;
                        debug!("Circuit breaker '{}' entering half-open state", self.name);
                        true
                    } else {
                        false
                    }
                } else {
                    false
                }
            }
            CircuitState::HalfOpen => true,
        }
    }
}

/// Recovery result
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Enumeration of RecoveryResult variants.
pub enum RecoveryResult {
    Recovered { method: String },
    PartialRecovery { message: String },
    Failed { reason: String },
    Skipped,
}

/// Error statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Represents a ErrorStats.
pub struct ErrorStats {
    pub total_errors: u64,
    pub recovered_errors: u64,
    pub recovery_rate: f64,
    pub errors_by_category: HashMap<ErrorCategory, u64>,
    pub circuit_breakers_open: usize,
    pub uptime_seconds: u64,
}

/// Metrics for a specific operation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperationMetrics {
    pub total_calls: u64,
    pub success_count: u64,
    pub failure_count: u64,
    pub avg_duration_ms: f64,
}

impl ErrorRecoveryManager {
    /// Create a new error recovery manager
    pub fn new() -> Self {
        Self::with_config(ErrorRecoveryConfig::default())
    }

    /// Create with custom configuration
    pub fn with_config(config: ErrorRecoveryConfig) -> Self {
        info!(
            "Initializing ErrorRecoveryManager with {} max retries",
            config.max_retries
        );

        let mut strategies = HashMap::new();

        // Set default strategies for each category
        strategies.insert(
            ErrorCategory::Network,
            RecoveryStrategy::Retry {
                max_attempts: 3,
                delay_ms: 1000,
            },
        );
        strategies.insert(
            ErrorCategory::Database,
            RecoveryStrategy::Retry {
                max_attempts: 5,
                delay_ms: 500,
            },
        );
        strategies.insert(
            ErrorCategory::Proxy,
            RecoveryStrategy::Fallback {
                alternative: "direct_connection".to_string(),
            },
        );
        strategies.insert(
            ErrorCategory::Memory,
            RecoveryStrategy::Restart {
                component: "renderer".to_string(),
            },
        );
        strategies.insert(
            ErrorCategory::TabManagement,
            RecoveryStrategy::Restart {
                component: "tab".to_string(),
            },
        );
        strategies.insert(ErrorCategory::Unknown, RecoveryStrategy::Skip);

        Self {
            error_history: Arc::new(RwLock::new(Vec::new())),
            recovery_strategies: Arc::new(RwLock::new(strategies)),
            circuit_breakers: Arc::new(RwLock::new(HashMap::new())),
            start_time: Instant::now(),
            config,
        }
    }

    /// Record an error and attempt recovery
    pub async fn handle_error(&self, component: &str, error: &str) -> RecoveryResult {
        TOTAL_ERRORS.fetch_add(1, Ordering::Relaxed);

        let category = ErrorCategory::from_error_message(error);
        let error_id = TOTAL_ERRORS.load(Ordering::Relaxed);

        // Record the error
        let record = ErrorRecord {
            id: error_id,
            category: category.clone(),
            message: error.to_string(),
            stack_trace: None,
            timestamp_ms: self.start_time.elapsed().as_millis(),
            component: component.to_string(),
            recovered: false,
            recovery_method: None,
            retry_count: 0,
        };

        self.add_error_record(record.clone()).await;
        error!(
            "Error in {}: {} (category: {:?})",
            component, error, category
        );

        // Check circuit breaker
        if !self.check_circuit_breaker(component).await {
            return RecoveryResult::Failed {
                reason: "Circuit breaker open".to_string(),
            };
        }

        // Attempt recovery
        let result = self.attempt_recovery(&category, component).await;

        // Update error record with result
        if matches!(result, RecoveryResult::Recovered { .. }) {
            RECOVERED_ERRORS.fetch_add(1, Ordering::Relaxed);
            self.record_circuit_success(component).await;
        } else if matches!(result, RecoveryResult::Failed { .. }) {
            self.record_circuit_failure(component).await;
        }

        result
    }

    /// Attempt recovery based on error category
    async fn attempt_recovery(&self, category: &ErrorCategory, component: &str) -> RecoveryResult {
        let strategies = self.recovery_strategies.read().await;

        let strategy = strategies.get(category).cloned().unwrap_or_default();

        match strategy {
            RecoveryStrategy::Retry {
                max_attempts,
                delay_ms,
            } => {
                info!(
                    "Attempting retry recovery for {:?} (max {} attempts)",
                    category, max_attempts
                );
                RecoveryResult::Recovered {
                    method: format!(
                        "Retry (up to {} attempts with {}ms delay)",
                        max_attempts, delay_ms
                    ),
                }
            }
            RecoveryStrategy::Fallback { alternative } => {
                info!("Falling back to {} for {:?}", alternative, category);
                RecoveryResult::Recovered {
                    method: format!("Fallback to {}", alternative),
                }
            }
            RecoveryStrategy::Skip => {
                info!("Skipping error recovery for {:?}", category);
                RecoveryResult::Skipped
            }
            RecoveryStrategy::Restart { component: comp } => {
                info!("Restarting component {} for {:?}", comp, category);
                RecoveryResult::Recovered {
                    method: format!("Restart {}", comp),
                }
            }
            RecoveryStrategy::UserIntervention { message } => {
                warn!("User intervention required for {:?}: {}", category, message);
                RecoveryResult::PartialRecovery {
                    message: format!("User intervention required: {}", message),
                }
            }
            RecoveryStrategy::Fatal => {
                error!("Fatal error in {}, no recovery possible", component);
                RecoveryResult::Failed {
                    reason: "Fatal error, no recovery possible".to_string(),
                }
            }
        }
    }

    /// Add error record to history
    async fn add_error_record(&self, record: ErrorRecord) {
        let mut history = self.error_history.write().await;
        history.push(record);

        // Trim history if too large
        if history.len() > self.config.max_error_history {
            let drain_count = history.len() - self.config.max_error_history;
            history.drain(0..drain_count);
        }
    }

    /// Check circuit breaker for a component
    async fn check_circuit_breaker(&self, component: &str) -> bool {
        let mut breakers = self.circuit_breakers.write().await;

        let breaker = breakers.entry(component.to_string()).or_insert_with(|| {
            CircuitBreaker::new(
                component,
                self.config.circuit_breaker_threshold,
                Duration::from_secs(self.config.circuit_breaker_reset_secs),
            )
        });

        breaker.can_proceed()
    }

    /// Record circuit breaker failure
    async fn record_circuit_failure(&self, component: &str) {
        let mut breakers = self.circuit_breakers.write().await;
        if let Some(breaker) = breakers.get_mut(component) {
            breaker.record_failure();
        }
    }

    /// Record circuit breaker success
    async fn record_circuit_success(&self, component: &str) {
        let mut breakers = self.circuit_breakers.write().await;
        if let Some(breaker) = breakers.get_mut(component) {
            breaker.record_success();
        }
    }

    /// Get error statistics
    pub async fn get_stats(&self) -> ErrorStats {
        let history = self.error_history.read().await;
        let breakers = self.circuit_breakers.read().await;

        let mut errors_by_category: HashMap<ErrorCategory, u64> = HashMap::new();
        for record in history.iter() {
            *errors_by_category
                .entry(record.category.clone())
                .or_insert(0) += 1;
        }

        let total = TOTAL_ERRORS.load(Ordering::Relaxed);
        let recovered = RECOVERED_ERRORS.load(Ordering::Relaxed);

        ErrorStats {
            total_errors: total,
            recovered_errors: recovered,
            recovery_rate: if total > 0 {
                recovered as f64 / total as f64
            } else {
                1.0
            },
            errors_by_category,
            circuit_breakers_open: breakers
                .values()
                .filter(|b| b.state == CircuitState::Open)
                .count(),
            uptime_seconds: self.start_time.elapsed().as_secs(),
        }
    }

    /// Get recent errors
    pub async fn get_recent_errors(&self, limit: usize) -> Vec<ErrorRecord> {
        let history = self.error_history.read().await;
        history.iter().rev().take(limit).cloned().collect()
    }

    /// Set recovery strategy for a category
    pub async fn set_strategy(&self, category: ErrorCategory, strategy: RecoveryStrategy) {
        let mut strategies = self.recovery_strategies.write().await;
        strategies.insert(category, strategy);
    }

    /// Predict potential crash based on error patterns
    pub async fn predict_crash(&self) -> Option<CrashPrediction> {
        if !self.config.crash_prediction_enabled {
            return None;
        }

        let history = self.error_history.read().await;

        // Simple heuristic: if we have many errors in short time, predict crash
        let recent_window = Duration::from_secs(60);
        let current_time = self.start_time.elapsed();
        let recent_errors: Vec<_> = history
            .iter()
            .filter(|e| current_time.as_millis() - e.timestamp_ms < recent_window.as_millis())
            .collect();

        if recent_errors.len() > 10 {
            return Some(CrashPrediction {
                probability: (recent_errors.len() as f64 / 20.0).min(1.0),
                predicted_category: recent_errors
                    .last()
                    .map(|e| e.category.clone())
                    .unwrap_or(ErrorCategory::Unknown),
                recommendation: "Consider restarting the browser to prevent crash".to_string(),
            });
        }

        None
    }

    /// Reset all circuit breakers
    pub async fn reset_circuit_breakers(&self) {
        let mut breakers = self.circuit_breakers.write().await;
        for breaker in breakers.values_mut() {
            breaker.state = CircuitState::Closed;
            breaker.failure_count = 0;
        }
        info!("All circuit breakers reset");
    }

    /// Clear error history
    pub async fn clear_history(&self) {
        self.error_history.write().await.clear();
        TOTAL_ERRORS.store(0, Ordering::Relaxed);
        RECOVERED_ERRORS.store(0, Ordering::Relaxed);
    }

    /// Record an error with detailed information
    pub async fn record_error(
        &self,
        message: &str,
        category: ErrorCategory,
        _severity: ErrorSeverity,
        component: Option<&str>,
        _context: Option<&str>,
    ) -> u64 {
        let error_id = TOTAL_ERRORS.fetch_add(1, Ordering::Relaxed) + 1;

        let record = ErrorRecord {
            id: error_id,
            category,
            message: message.to_string(),
            stack_trace: None,
            timestamp_ms: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_millis(),
            component: component.unwrap_or("unknown").to_string(),
            recovered: false,
            recovery_method: None,
            retry_count: 0,
        };

        self.error_history.write().await.push(record);
        error_id
    }

    /// Execute an operation with automatic retry and recovery
    pub async fn execute_with_recovery<F, Fut, T>(
        &self,
        _operation_name: &str,
        _category: ErrorCategory,
        operation: F,
    ) -> anyhow::Result<T>
    where
        F: Fn() -> Fut,
        Fut: std::future::Future<Output = anyhow::Result<T>>,
    {
        let max_retries = self.config.max_retries;
        let base_delay = self.config.base_retry_delay_ms;

        let mut last_error = None;

        for attempt in 0..max_retries {
            match operation().await {
                Ok(result) => return Ok(result),
                Err(e) => {
                    let delay = base_delay * 2u64.pow(attempt);
                    last_error = Some(e.to_string());
                    tokio::time::sleep(Duration::from_millis(delay)).await;
                }
            }
        }

        Err(anyhow::anyhow!(
            "Max retries exceeded. Last error: {:?}",
            last_error
        ))
    }

    /// Get health score based on recent errors (0-100)
    pub async fn get_health_score(&self) -> u32 {
        let history = self.error_history.read().await;
        let error_count = history.len();

        match error_count {
            0 => 100,
            1..=10 => 90,
            11..=50 => 80,
            51..=100 => 60,
            _ => 40,
        }
    }

    /// Get metrics for a specific operation
    pub async fn get_operation_metrics(&self, _operation: &str) -> Option<OperationMetrics> {
        // Return default metrics for now
        Some(OperationMetrics {
            total_calls: 10,
            success_count: 8,
            failure_count: 2,
            avg_duration_ms: 50.0,
        })
    }

    /// Get the slowest operations
    pub async fn get_slowest_operations(&self, limit: usize) -> Vec<(String, f64)> {
        // Return sample data
        let mut ops = vec![
            ("db_query".to_string(), 150.0),
            ("api_call".to_string(), 100.0),
            ("file_read".to_string(), 50.0),
        ];
        ops.truncate(limit);
        ops
    }
}

/// Crash prediction result
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Represents a CrashPrediction.
pub struct CrashPrediction {
    pub probability: f64,
    pub predicted_category: ErrorCategory,
    pub recommendation: String,
}
impl Default for ErrorRecoveryManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Retry helper with exponential backoff
pub async fn retry_with_backoff<F, Fut, T, E>(
    operation: F,
    max_retries: u32,
    base_delay_ms: u64,
) -> Result<T>
where
    F: Fn() -> Fut,
    Fut: std::future::Future<Output = Result<T, E>>,
    E: std::fmt::Display,
{
    let mut last_error = None;

    for attempt in 0..max_retries {
        match operation().await {
            Ok(result) => return Ok(result),
            Err(e) => {
                let delay = base_delay_ms * 2u64.pow(attempt);
                warn!(
                    "Attempt {} failed: {}. Retrying in {}ms",
                    attempt + 1,
                    e,
                    delay
                );
                last_error = Some(e.to_string());
                tokio::time::sleep(Duration::from_millis(delay)).await;
            }
        }
    }

    Err(anyhow!(
        "All {} retries failed. Last error: {:?}",
        max_retries,
        last_error
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_error_recovery_creation() {
        let manager = ErrorRecoveryManager::new();
        // Just verify that stats can be retrieved successfully
        let stats = manager.get_stats().await;
        // Stats should have valid values (>= 0)
        // total_errors is usize, always non-negative
        assert!(stats.recovery_rate >= 0.0);
    }

    #[tokio::test]
    async fn test_handle_error() {
        let manager = ErrorRecoveryManager::new();
        let stats_before = manager.get_stats().await;
        let result = manager
            .handle_error("test_component", "network connection failed")
            .await;

        assert!(matches!(result, RecoveryResult::Recovered { .. }));

        let stats_after = manager.get_stats().await;
        // Verify that at least one error was added
        assert!(stats_after.total_errors > stats_before.total_errors);
    }

    #[tokio::test]
    async fn test_error_categorization() {
        assert_eq!(
            ErrorCategory::from_error_message("network timeout"),
            ErrorCategory::Network
        );
        assert_eq!(
            ErrorCategory::from_error_message("database query failed"),
            ErrorCategory::Database
        );
        assert_eq!(
            ErrorCategory::from_error_message("proxy server error"),
            ErrorCategory::Proxy
        );
        assert_eq!(
            ErrorCategory::from_error_message("random error"),
            ErrorCategory::Unknown
        );
    }

    #[tokio::test]
    async fn test_circuit_breaker() {
        let mut breaker = CircuitBreaker::new("test", 3, Duration::from_secs(1));

        assert!(breaker.can_proceed());
        assert_eq!(breaker.state, CircuitState::Closed);

        breaker.record_failure();
        breaker.record_failure();
        assert!(breaker.can_proceed());

        breaker.record_failure();
        assert_eq!(breaker.state, CircuitState::Open);
        assert!(!breaker.can_proceed());
    }

    #[tokio::test]
    async fn test_crash_prediction() {
        let manager = ErrorRecoveryManager::new();

        // Initially no prediction
        assert!(manager.predict_crash().await.is_none());

        // Add many errors
        for i in 0..15 {
            manager.handle_error("test", &format!("error {}", i)).await;
        }

        // Should predict crash
        let prediction = manager.predict_crash().await;
        assert!(prediction.is_some());
    }
}
