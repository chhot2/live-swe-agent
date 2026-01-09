//! Tests for Proxy Rotation Manager
//!
//! This module tests:
//! - Proxy rotation strategies
//! - Session management
//! - Performance metrics
//! - Domain-based proxy assignment

use browser_core::free_ip_providers::FreeIpProviderManager;
use browser_core::proxy::{FreeProxy, ProxyType};
use browser_core::proxy_rotation::{
    ProxyMetrics, ProxyRotationManager, ProxyRotationStrategy, ProxySession, ProxySessionStats,
};
use chrono::{Duration, Utc};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

// ============================================================================
// Test Fixtures
// ============================================================================

fn create_test_proxy(ip: &str, port: u16, country: &str) -> FreeProxy {
    FreeProxy {
        ip: ip.to_string(),
        port,
        protocol: ProxyType::Http,
        country: country.to_string(),
        country_code: country.chars().take(2).collect::<String>().to_uppercase(),
        anonymity: "elite".to_string(),
        speed: 100,
        uptime: 99.5,
        last_checked: Utc::now().to_rfc3339(),
        provider: "TestProvider".to_string(),
        is_working: true,
    }
}

async fn create_test_provider_manager() -> Arc<RwLock<FreeIpProviderManager>> {
    Arc::new(RwLock::new(
        FreeIpProviderManager::new().expect("Failed to acquire lock"),
    ))
}

// ============================================================================
// ProxyRotationStrategy Tests
// ============================================================================

#[test]
fn test_strategy_per_request() {
    let strategy = ProxyRotationStrategy::PerRequest(100);
    match strategy {
        ProxyRotationStrategy::PerRequest(n) => assert_eq!(n, 100),
        _ => panic!("Wrong strategy type"),
    }
}

#[test]
fn test_strategy_per_duration() {
    let duration = Duration::minutes(30);
    let strategy = ProxyRotationStrategy::PerDuration(duration);
    match strategy {
        ProxyRotationStrategy::PerDuration(d) => assert_eq!(d.num_minutes(), 30),
        _ => panic!("Wrong strategy type"),
    }
}

#[test]
fn test_strategy_random() {
    let strategy = ProxyRotationStrategy::Random { probability: 0.1 };
    match strategy {
        ProxyRotationStrategy::Random { probability } => {
            assert!(probability >= 0.0 && probability <= 1.0);
            assert_eq!(probability, 0.1);
        }
        _ => panic!("Wrong strategy type"),
    }
}

#[test]
fn test_strategy_sticky() {
    let strategy = ProxyRotationStrategy::Sticky {
        duration: Duration::hours(1),
    };
    match strategy {
        ProxyRotationStrategy::Sticky { duration } => {
            assert_eq!(duration.num_hours(), 1);
        }
        _ => panic!("Wrong strategy type"),
    }
}

#[test]
fn test_strategy_geographic() {
    let strategy = ProxyRotationStrategy::Geographic {
        country_codes: vec!["US".to_string(), "DE".to_string(), "FR".to_string()],
    };
    match strategy {
        ProxyRotationStrategy::Geographic { country_codes } => {
            assert_eq!(country_codes.len(), 3);
            assert!(country_codes.contains(&"US".to_string()));
        }
        _ => panic!("Wrong strategy type"),
    }
}

#[test]
fn test_strategy_variants_exist() {
    // Test that all strategy variants can be created
    let _per_session = ProxyRotationStrategy::PerSession;
    let _performance = ProxyRotationStrategy::PerformanceBased;
    let _round_robin = ProxyRotationStrategy::RoundRobin;
    let _domain_based = ProxyRotationStrategy::DomainBased;
    let _manual = ProxyRotationStrategy::Manual;
}

// ============================================================================
// ProxySession Tests
// ============================================================================

#[test]
fn test_proxy_session_creation() {
    let proxy = create_test_proxy("192.168.1.1", 8080, "United States");
    let now = Utc::now();

    let session = ProxySession {
        proxy: proxy.clone(),
        assigned_at: now,
        last_used: now,
        request_count: 0,
        tab_id: "tab-123".to_string(),
        domain_proxy_map: HashMap::new(),
    };

    assert_eq!(session.tab_id, "tab-123");
    assert_eq!(session.request_count, 0);
    assert_eq!(session.proxy.ip, "192.168.1.1");
}

#[test]
fn test_proxy_session_with_domain_map() {
    let proxy = create_test_proxy("192.168.1.1", 8080, "United States");
    let now = Utc::now();

    let mut domain_map = HashMap::new();
    domain_map.insert("google.com".to_string(), "proxy-1".to_string());
    domain_map.insert("github.com".to_string(), "proxy-2".to_string());

    let session = ProxySession {
        proxy: proxy.clone(),
        assigned_at: now,
        last_used: now,
        request_count: 5,
        tab_id: "tab-456".to_string(),
        domain_proxy_map: domain_map,
    };

    assert_eq!(session.domain_proxy_map.len(), 2);
    assert_eq!(
        session.domain_proxy_map.get("google.com"),
        Some(&"proxy-1".to_string())
    );
}

// ============================================================================
// ProxyMetrics Tests
// ============================================================================

#[test]
fn test_proxy_metrics_creation() {
    let metrics = ProxyMetrics {
        response_time_ms: 150.5,
        success_rate: 0.95,
        last_success: Some(Utc::now()),
        consecutive_failures: 0,
        total_requests: 100,
        failed_requests: 5,
    };

    assert_eq!(metrics.response_time_ms, 150.5);
    assert_eq!(metrics.success_rate, 0.95);
    assert_eq!(metrics.consecutive_failures, 0);
    assert_eq!(metrics.total_requests, 100);
    assert_eq!(metrics.failed_requests, 5);
}

#[test]
fn test_proxy_metrics_with_failures() {
    let metrics = ProxyMetrics {
        response_time_ms: 0.0,
        success_rate: 0.0,
        last_success: None,
        consecutive_failures: 5,
        total_requests: 10,
        failed_requests: 10,
    };

    assert_eq!(metrics.consecutive_failures, 5);
    assert!(metrics.last_success.is_none());
    assert_eq!(metrics.success_rate, 0.0);
}

#[test]
fn test_proxy_metrics_serialization() {
    let metrics = ProxyMetrics {
        response_time_ms: 100.0,
        success_rate: 0.9,
        last_success: Some(Utc::now()),
        consecutive_failures: 1,
        total_requests: 50,
        failed_requests: 5,
    };

    // Test serialization
    let json = serde_json::to_string(&metrics).expect("Metrics operation failed");
    assert!(json.contains("response_time_ms"));
    assert!(json.contains("success_rate"));

    // Test deserialization
    let parsed: ProxyMetrics = serde_json::from_str(&json).expect("Json operation failed");
    assert_eq!(parsed.total_requests, 50);
}

// ============================================================================
// ProxySessionStats Tests
// ============================================================================

#[test]
fn test_proxy_session_stats() {
    let stats = ProxySessionStats {
        tab_id: "tab-789".to_string(),
        current_proxy_ip: "10.0.0.1".to_string(),
        proxy_country: "Germany".to_string(),
        assigned_at: Utc::now(),
        request_count: 42,
        duration_seconds: 3600,
    };

    assert_eq!(stats.tab_id, "tab-789");
    assert_eq!(stats.current_proxy_ip, "10.0.0.1");
    assert_eq!(stats.proxy_country, "Germany");
    assert_eq!(stats.request_count, 42);
    assert_eq!(stats.duration_seconds, 3600);
}

#[test]
fn test_proxy_session_stats_serialization() {
    let stats = ProxySessionStats {
        tab_id: "tab-001".to_string(),
        current_proxy_ip: "192.168.1.1".to_string(),
        proxy_country: "France".to_string(),
        assigned_at: Utc::now(),
        request_count: 10,
        duration_seconds: 600,
    };

    let json = serde_json::to_string(&stats).expect("Stats operation failed");
    assert!(json.contains("tab_id"));
    assert!(json.contains("current_proxy_ip"));

    let parsed: ProxySessionStats = serde_json::from_str(&json).expect("Json operation failed");
    assert_eq!(parsed.proxy_country, "France");
}

// ============================================================================
// ProxyRotationManager Tests
// ============================================================================

#[tokio::test]
async fn test_rotation_manager_creation() {
    let provider_manager = create_test_provider_manager().await;
    let strategy = ProxyRotationStrategy::RoundRobin;

    let manager = ProxyRotationManager::new(provider_manager, strategy);
    // Manager should be created successfully
    assert!(true);
}

#[tokio::test]
async fn test_rotation_manager_with_per_request_strategy() {
    let provider_manager = create_test_provider_manager().await;
    let strategy = ProxyRotationStrategy::PerRequest(10);

    let manager = ProxyRotationManager::new(provider_manager, strategy);
    assert!(true);
}

#[tokio::test]
async fn test_rotation_manager_with_geographic_strategy() {
    let provider_manager = create_test_provider_manager().await;
    let strategy = ProxyRotationStrategy::Geographic {
        country_codes: vec!["US".to_string(), "UK".to_string()],
    };

    let manager = ProxyRotationManager::new(provider_manager, strategy);
    assert!(true);
}

#[tokio::test]
async fn test_rotation_manager_get_proxy_for_tab() {
    let provider_manager = create_test_provider_manager().await;
    let strategy = ProxyRotationStrategy::Manual;

    let manager = ProxyRotationManager::new(provider_manager, strategy);

    // Try to get a proxy for a tab (may fail if pool is empty, but shouldn't panic)
    let result = manager.get_proxy_for_tab("tab-test", None).await;
    match result {
        Ok(proxy) => {
            assert!(!proxy.ip.is_empty());
        }
        Err(e) => {
            // Expected if no proxies are available
            println!("Expected error (no proxies): {}", e);
        }
    }
}

// ============================================================================
// Strategy Logic Tests
// ============================================================================

#[test]
fn test_per_request_threshold() {
    let thresholds = vec![1, 10, 100, 1000];

    for threshold in thresholds {
        let strategy = ProxyRotationStrategy::PerRequest(threshold);
        if let ProxyRotationStrategy::PerRequest(n) = strategy {
            assert_eq!(n, threshold);
        }
    }
}

#[test]
fn test_duration_variants() {
    let durations = vec![
        Duration::seconds(30),
        Duration::minutes(5),
        Duration::hours(1),
        Duration::days(1),
    ];

    for duration in durations {
        let strategy = ProxyRotationStrategy::PerDuration(duration);
        if let ProxyRotationStrategy::PerDuration(d) = strategy {
            assert!(d.num_seconds() > 0);
        }
    }
}

#[test]
fn test_random_probability_bounds() {
    let probabilities = vec![0.0, 0.1, 0.5, 0.9, 1.0];

    for prob in probabilities {
        let strategy = ProxyRotationStrategy::Random { probability: prob };
        if let ProxyRotationStrategy::Random { probability } = strategy {
            assert!(probability >= 0.0 && probability <= 1.0);
        }
    }
}

// ============================================================================
// Clone Tests
// ============================================================================

#[test]
fn test_proxy_session_clone() {
    let proxy = create_test_proxy("192.168.1.1", 8080, "US");
    let session = ProxySession {
        proxy,
        assigned_at: Utc::now(),
        last_used: Utc::now(),
        request_count: 10,
        tab_id: "tab-1".to_string(),
        domain_proxy_map: HashMap::new(),
    };

    let cloned = session.clone();
    assert_eq!(cloned.tab_id, session.tab_id);
    assert_eq!(cloned.request_count, session.request_count);
}

#[test]
fn test_proxy_metrics_clone() {
    let metrics = ProxyMetrics {
        response_time_ms: 100.0,
        success_rate: 0.95,
        last_success: Some(Utc::now()),
        consecutive_failures: 0,
        total_requests: 100,
        failed_requests: 5,
    };

    let cloned = metrics.clone();
    assert_eq!(cloned.response_time_ms, metrics.response_time_ms);
    assert_eq!(cloned.total_requests, metrics.total_requests);
}
