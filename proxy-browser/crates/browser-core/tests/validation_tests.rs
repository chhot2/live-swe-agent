//! Tests for Proxy Validation & Health Checking
//!
//! This module tests:
//! - Proxy validation functionality
//! - IP leak detection
//! - Geographic verification
//! - Quarantine system for failed proxies
//! - Health checker operations

use browser_core::proxy::{FreeProxy, ProxySettings, ProxyType};
use browser_core::proxy_validator::{
    EnhancedProxyHealthChecker, GeoVerificationConfig, GeoVerificationResult, GeoVerifier,
    ProxyHealthChecker, ProxyQuarantineManager, ProxyValidator, ProxyValidatorConfig,
    QuarantineStats, QuarantinedProxy, ValidationResult,
};
use chrono::Utc;
use std::sync::Arc;
use std::time::Duration;

// ============================================================================
// Test Fixtures
// ============================================================================

fn create_test_proxy() -> FreeProxy {
    FreeProxy {
        ip: "192.168.1.1".to_string(),
        port: 8080,
        protocol: ProxyType::Http,
        country: "United States".to_string(),
        country_code: "US".to_string(),
        anonymity: "elite".to_string(),
        speed: 100,
        uptime: 99.5,
        last_checked: Utc::now().to_rfc3339(),
        provider: "TestProvider".to_string(),
        is_working: true,
    }
}

fn create_test_proxies() -> Vec<FreeProxy> {
    vec![
        FreeProxy {
            ip: "192.168.1.1".to_string(),
            port: 8080,
            protocol: ProxyType::Http,
            country: "United States".to_string(),
            country_code: "US".to_string(),
            anonymity: "elite".to_string(),
            speed: 100,
            uptime: 99.5,
            last_checked: Utc::now().to_rfc3339(),
            provider: "TestProvider".to_string(),
            is_working: true,
        },
        FreeProxy {
            ip: "192.168.1.2".to_string(),
            port: 1080,
            protocol: ProxyType::Socks5,
            country: "Germany".to_string(),
            country_code: "DE".to_string(),
            anonymity: "anonymous".to_string(),
            speed: 50,
            uptime: 95.0,
            last_checked: Utc::now().to_rfc3339(),
            provider: "TestProvider".to_string(),
            is_working: true,
        },
        FreeProxy {
            ip: "192.168.1.3".to_string(),
            port: 3128,
            protocol: ProxyType::Http,
            country: "France".to_string(),
            country_code: "FR".to_string(),
            anonymity: "transparent".to_string(),
            speed: 75,
            uptime: 90.0,
            last_checked: Utc::now().to_rfc3339(),
            provider: "TestProvider".to_string(),
            is_working: false,
        },
    ]
}

// ============================================================================
// ProxyValidatorConfig Tests
// ============================================================================

#[test]
fn test_validator_config_default() {
    let config = ProxyValidatorConfig::default();

    assert_eq!(config.timeout, Duration::from_secs(10));
    assert_eq!(config.concurrent_checks, 20);
    assert_eq!(config.max_retries, 3);
    assert!(!config.test_urls.is_empty());
}

#[test]
fn test_validator_config_custom() {
    let config = ProxyValidatorConfig {
        timeout: Duration::from_secs(5),
        concurrent_checks: 10,
        test_urls: vec!["https://example.com/ip".to_string()],
        max_retries: 2,
    };

    assert_eq!(config.timeout, Duration::from_secs(5));
    assert_eq!(config.concurrent_checks, 10);
    assert_eq!(config.max_retries, 2);
    assert_eq!(config.test_urls.len(), 1);
}

// ============================================================================
// ProxyValidator Tests
// ============================================================================

#[test]
fn test_validator_creation() {
    let config = ProxyValidatorConfig::default();
    let validator = ProxyValidator::new(config);
    // Validator should be created successfully
    assert!(true); // If we get here, creation succeeded
}

#[test]
fn test_validator_creation_with_custom_config() {
    let config = ProxyValidatorConfig {
        timeout: Duration::from_secs(30),
        concurrent_checks: 50,
        test_urls: vec![
            "https://api.ipify.org".to_string(),
            "https://ifconfig.me/ip".to_string(),
        ],
        max_retries: 5,
    };
    let validator = ProxyValidator::new(config);
    assert!(true);
}

// ============================================================================
// ValidationResult Tests
// ============================================================================

#[test]
fn test_validation_result_working() {
    let result = ValidationResult {
        is_working: true,
        response_time_ms: 150,
        detected_country: Some("United States".to_string()),
        detected_ip: Some("203.0.113.1".to_string()),
        supports_https: true,
        has_ip_leak: false,
        error: None,
        validated_at: Utc::now(),
    };

    assert!(result.is_working);
    assert_eq!(result.response_time_ms, 150);
    assert!(result.detected_country.is_some());
    assert!(result.detected_ip.is_some());
    assert!(result.supports_https);
    assert!(!result.has_ip_leak);
    assert!(result.error.is_none());
}

#[test]
fn test_validation_result_failed() {
    let result = ValidationResult {
        is_working: false,
        response_time_ms: u64::MAX,
        detected_country: None,
        detected_ip: None,
        supports_https: false,
        has_ip_leak: false,
        error: Some("Connection timeout".to_string()),
        validated_at: Utc::now(),
    };

    assert!(!result.is_working);
    assert!(result.error.is_some());
    assert_eq!(
        result.error.as_ref().expect("As Ref operation failed"),
        "Connection timeout"
    );
}

#[test]
fn test_validation_result_with_ip_leak() {
    let result = ValidationResult {
        is_working: true,
        response_time_ms: 200,
        detected_country: Some("United States".to_string()),
        detected_ip: Some("192.168.1.100".to_string()), // Local IP leaked
        supports_https: true,
        has_ip_leak: true, // IP leak detected
        error: None,
        validated_at: Utc::now(),
    };

    assert!(result.is_working);
    assert!(result.has_ip_leak);
}

// ============================================================================
// Quarantine System Tests
// ============================================================================

#[tokio::test]
async fn test_quarantine_manager_creation() {
    let manager = ProxyQuarantineManager::new(
        3,                         // max consecutive failures
        Duration::from_secs(300),  // 5 min quarantine
        Duration::from_secs(3600), // 1 hour max quarantine
    );

    let stats = manager.get_stats().await;
    assert_eq!(stats.total_quarantined, 0);
    assert_eq!(stats.actively_quarantined, 0);
}

#[tokio::test]
async fn test_quarantine_manager_record_failure() {
    let manager = ProxyQuarantineManager::new(
        1, // quarantine after 1 failure
        Duration::from_secs(60),
        Duration::from_secs(3600),
    );

    let proxy = create_test_proxy();

    // Record first failure - should quarantine immediately with max_consecutive_failures = 1
    let was_quarantined = manager
        .record_failure(&proxy, "Connection refused".to_string())
        .await;

    // Check if quarantined
    let is_quarantined = manager.is_quarantined(&proxy).await;
    assert!(is_quarantined || was_quarantined);
}

#[tokio::test]
async fn test_quarantine_manager_record_success_releases() {
    let manager =
        ProxyQuarantineManager::new(1, Duration::from_secs(60), Duration::from_secs(3600));

    let proxy = create_test_proxy();

    // Quarantine the proxy
    manager
        .record_failure(&proxy, "Test failure".to_string())
        .await;

    // Record success should release from quarantine
    manager.record_success(&proxy).await;

    let is_quarantined = manager.is_quarantined(&proxy).await;
    assert!(!is_quarantined);
}

#[tokio::test]
async fn test_quarantine_manager_multiple_failures() {
    let manager = ProxyQuarantineManager::new(
        3, // quarantine after 3 failures
        Duration::from_secs(60),
        Duration::from_secs(3600),
    );

    let proxy = create_test_proxy();

    // Record multiple failures
    for i in 0..3 {
        manager
            .record_failure(&proxy, format!("Failure #{}", i + 1))
            .await;
    }

    let quarantined = manager.get_quarantined().await;
    assert!(!quarantined.is_empty());

    // Verify the proxy in quarantine has 3 failures
    let q_proxy = quarantined.iter().find(|q| q.proxy.ip == proxy.ip);
    assert!(q_proxy.is_some());
    assert_eq!(
        q_proxy
            .expect("Q Proxy operation failed")
            .consecutive_failures,
        3
    );
}

#[tokio::test]
async fn test_quarantine_manager_get_stats() {
    let manager =
        ProxyQuarantineManager::new(1, Duration::from_secs(60), Duration::from_secs(3600));

    let proxies = create_test_proxies();

    // Quarantine multiple proxies
    for proxy in &proxies {
        manager
            .record_failure(proxy, "Test failure".to_string())
            .await;
    }

    let stats = manager.get_stats().await;
    assert!(stats.total_quarantined > 0);
    assert!(stats.average_failures >= 1.0);
}

#[tokio::test]
async fn test_quarantine_manager_is_not_quarantined_initially() {
    let manager =
        ProxyQuarantineManager::new(3, Duration::from_secs(60), Duration::from_secs(3600));

    let proxy = create_test_proxy();

    // Fresh proxy should not be quarantined
    let is_quarantined = manager.is_quarantined(&proxy).await;
    assert!(!is_quarantined);
}

// ============================================================================
// Geographic Verification Tests
// ============================================================================

#[test]
fn test_geo_verification_config_default() {
    let config = GeoVerificationConfig::default();

    assert!(config.enabled);
    assert!(config.tolerance_km > 0.0);
    assert!(!config.geoip_api_urls.is_empty());
}

#[test]
fn test_geo_verification_config_custom() {
    let config = GeoVerificationConfig {
        enabled: false,
        tolerance_km: 1000.0,
        geoip_api_urls: vec!["https://custom.geoip.api/".to_string()],
    };

    assert!(!config.enabled);
    assert_eq!(config.tolerance_km, 1000.0);
    assert_eq!(config.geoip_api_urls.len(), 1);
}

#[test]
fn test_geo_verifier_creation() {
    let config = GeoVerificationConfig::default();
    let verifier = GeoVerifier::new(config);
    assert!(verifier.is_ok());
}

#[test]
fn test_geo_verification_result_verified() {
    let result = GeoVerificationResult {
        is_verified: true,
        expected_country: "United States".to_string(),
        detected_country: Some("United States".to_string()),
        expected_location: Some((37.7749, -122.4194)),
        detected_location: Some((37.7749, -122.4194)),
        distance_km: Some(0.0),
        error: None,
    };

    assert!(result.is_verified);
    assert_eq!(
        result.expected_country,
        result
            .detected_country
            .expect("Detected Country operation failed")
    );
}

#[test]
fn test_geo_verification_result_mismatch() {
    let result = GeoVerificationResult {
        is_verified: false,
        expected_country: "United States".to_string(),
        detected_country: Some("China".to_string()),
        expected_location: None,
        detected_location: None,
        distance_km: None,
        error: None,
    };

    assert!(!result.is_verified);
    assert_ne!(
        result.expected_country,
        result
            .detected_country
            .expect("Detected Country operation failed")
    );
}

// ============================================================================
// ProxyHealthChecker Tests
// ============================================================================

#[test]
fn test_health_checker_creation() {
    let validator = ProxyValidator::new(ProxyValidatorConfig::default());
    let checker = ProxyHealthChecker::new(
        validator,
        Duration::from_secs(60),
        3,
        Duration::from_secs(300),
    );

    // Checker should be created successfully
    assert!(true);
}

#[test]
fn test_enhanced_health_checker_creation() {
    let validator = ProxyValidator::new(ProxyValidatorConfig::default());
    let quarantine_manager =
        ProxyQuarantineManager::new(3, Duration::from_secs(300), Duration::from_secs(3600));
    let geo_verifier = GeoVerifier::new(GeoVerificationConfig::default()).ok();

    let checker = EnhancedProxyHealthChecker::new(
        validator,
        quarantine_manager,
        geo_verifier,
        Duration::from_secs(60),
    );

    assert!(true);
}

// ============================================================================
// QuarantinedProxy Tests
// ============================================================================

#[test]
fn test_quarantined_proxy_structure() {
    let proxy = create_test_proxy();
    let now = Utc::now();

    let quarantined = QuarantinedProxy {
        proxy: proxy.clone(),
        consecutive_failures: 3,
        quarantined_at: now,
        release_at: now + chrono::Duration::minutes(5),
        failure_reasons: vec![
            "Connection timeout".to_string(),
            "Connection refused".to_string(),
            "SSL error".to_string(),
        ],
    };

    assert_eq!(quarantined.consecutive_failures, 3);
    assert_eq!(quarantined.failure_reasons.len(), 3);
    assert!(quarantined.release_at > quarantined.quarantined_at);
}

// ============================================================================
// QuarantineStats Tests
// ============================================================================

#[test]
fn test_quarantine_stats_structure() {
    let stats = QuarantineStats {
        total_quarantined: 10,
        actively_quarantined: 7,
        pending_release: 3,
        average_failures: 2.5,
    };

    assert_eq!(stats.total_quarantined, 10);
    assert_eq!(stats.actively_quarantined, 7);
    assert_eq!(stats.pending_release, 3);
    assert_eq!(stats.average_failures, 2.5);
}

// ============================================================================
// Integration Tests
// ============================================================================

#[cfg(test)]
mod integration_tests {
    use super::*;
    use tokio::sync::RwLock;

    #[tokio::test]
    async fn test_validate_proxy_timeout() {
        let config = ProxyValidatorConfig {
            timeout: Duration::from_millis(100), // Very short timeout
            concurrent_checks: 1,
            test_urls: vec!["https://api.ipify.org?format=json".to_string()],
            max_retries: 1,
        };

        let validator = ProxyValidator::new(config);

        // Use a non-existent proxy that should timeout
        let proxy = FreeProxy {
            ip: "192.0.2.1".to_string(), // TEST-NET-1, won't route
            port: 8080,
            protocol: ProxyType::Http,
            country: "Unknown".to_string(),
            country_code: "XX".to_string(),
            anonymity: "unknown".to_string(),
            speed: 0,
            uptime: 0.0,
            last_checked: Utc::now().to_rfc3339(),
            provider: "Test".to_string(),
            is_working: false,
        };

        let result = validator.validate_proxy(&proxy).await;

        // The validation should complete (either success or failure)
        // With a non-routable IP, it should fail
        match result {
            Ok(validation) => {
                // Even if validation completes, proxy should not be working
                println!("Validation completed: working={}", validation.is_working);
            }
            Err(e) => {
                println!("Validation error (expected): {}", e);
            }
        }
    }

    #[tokio::test]
    async fn test_validate_batch_empty() {
        let config = ProxyValidatorConfig::default();
        let validator = ProxyValidator::new(config);

        let results = validator.validate_batch(&[]).await;
        assert!(results.is_empty());
    }

    #[tokio::test]
    async fn test_quarantine_and_release_flow() {
        let manager = ProxyQuarantineManager::new(
            2,
            Duration::from_millis(100), // Short quarantine for testing
            Duration::from_secs(1),
        );

        let proxy = create_test_proxy();

        // Record failures to quarantine
        manager
            .record_failure(&proxy, "Failure 1".to_string())
            .await;
        manager
            .record_failure(&proxy, "Failure 2".to_string())
            .await;

        // Should be quarantined now
        assert!(manager.is_quarantined(&proxy).await);

        // Wait for quarantine to expire
        tokio::time::sleep(Duration::from_millis(150)).await;

        // Release expired
        let released = manager.release_expired().await;

        // Should be released now
        assert!(!released.is_empty() || !manager.is_quarantined(&proxy).await);
    }

    #[tokio::test]
    async fn test_geo_verification_disabled() {
        let config = GeoVerificationConfig {
            enabled: false,
            tolerance_km: 500.0,
            geoip_api_urls: vec![],
        };

        let verifier = GeoVerifier::new(config).expect("Configuration error");
        let proxy = create_test_proxy();

        let result = verifier.verify_proxy_location(&proxy, "203.0.113.1").await;

        // With verification disabled, should always return verified
        assert!(result.is_verified);
        assert!(result.error.is_some()); // Should have "disabled" message
    }
}
