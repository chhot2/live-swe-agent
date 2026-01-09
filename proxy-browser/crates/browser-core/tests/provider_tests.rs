//! Tests for the Free IP Provider abstraction layer
//!
//! This module tests the proxy provider functionality including:
//! - Provider trait implementation
//! - Filter functionality
//! - Rate limiting
//! - Provider manager operations

use browser_core::free_ip_providers::{FreeIpProvider, FreeIpProviderManager, ProxyFilter};
use browser_core::proxy::{FreeProxy, ProxyType};
use std::time::Duration;

// ============================================================================
// FreeIpProvider Enum Tests
// ============================================================================

#[test]
fn test_free_ip_provider_all() {
    let providers = FreeIpProvider::all();
    assert_eq!(providers.len(), 6);

    // Verify all providers are present
    let names: Vec<&str> = providers.iter().map(|p| p.name()).collect();
    assert!(names.contains(&"ProxyScrape"));
    assert!(names.contains(&"GeoNode"));
    assert!(names.contains(&"PubProxy"));
    assert!(names.contains(&"FreeProxyList"));
    assert!(names.contains(&"ProxyNova"));
    assert!(names.contains(&"SpysOne"));
}

#[test]
fn test_free_ip_provider_names() {
    assert_eq!(FreeIpProvider::ProxyScrape.name(), "ProxyScrape");
    assert_eq!(FreeIpProvider::GeoNode.name(), "GeoNode");
    assert_eq!(FreeIpProvider::PubProxy.name(), "PubProxy");
    assert_eq!(FreeIpProvider::FreeProxyList.name(), "FreeProxyList");
    assert_eq!(FreeIpProvider::ProxyNova.name(), "ProxyNova");
    assert_eq!(FreeIpProvider::SpysOne.name(), "SpysOne");
}

#[test]
fn test_free_ip_provider_rate_limits() {
    // API-based providers should have lower rate limits
    assert_eq!(
        FreeIpProvider::ProxyScrape.rate_limit(),
        Duration::from_secs(1)
    );
    assert_eq!(FreeIpProvider::GeoNode.rate_limit(), Duration::from_secs(2));
    assert_eq!(
        FreeIpProvider::PubProxy.rate_limit(),
        Duration::from_secs(1)
    );

    // Web scraping providers should have higher rate limits
    assert_eq!(
        FreeIpProvider::FreeProxyList.rate_limit(),
        Duration::from_secs(5)
    );
    assert_eq!(
        FreeIpProvider::ProxyNova.rate_limit(),
        Duration::from_secs(5)
    );
    assert_eq!(FreeIpProvider::SpysOne.rate_limit(), Duration::from_secs(5));
}

#[test]
fn test_free_ip_provider_is_api_based() {
    // API-based providers
    assert!(FreeIpProvider::ProxyScrape.is_api_based());
    assert!(FreeIpProvider::GeoNode.is_api_based());
    assert!(FreeIpProvider::PubProxy.is_api_based());

    // Web scraping providers
    assert!(!FreeIpProvider::FreeProxyList.is_api_based());
    assert!(!FreeIpProvider::ProxyNova.is_api_based());
    assert!(!FreeIpProvider::SpysOne.is_api_based());
}

// ============================================================================
// ProxyFilter Tests
// ============================================================================

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
            last_checked: "2024-01-01T00:00:00Z".to_string(),
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
            last_checked: "2024-01-01T00:00:00Z".to_string(),
            provider: "TestProvider".to_string(),
            is_working: false,
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
            last_checked: "2024-01-01T00:00:00Z".to_string(),
            provider: "TestProvider".to_string(),
            is_working: true,
        },
        FreeProxy {
            ip: "192.168.1.4".to_string(),
            port: 1081,
            protocol: ProxyType::Socks4,
            country: "United States".to_string(),
            country_code: "US".to_string(),
            anonymity: "elite".to_string(),
            speed: 120,
            uptime: 98.0,
            last_checked: "2024-01-01T00:00:00Z".to_string(),
            provider: "TestProvider".to_string(),
            is_working: true,
        },
    ]
}

#[test]
fn test_proxy_filter_all() {
    let proxies = create_test_proxies();
    let filtered = apply_filter(proxies.clone(), ProxyFilter::All);
    assert_eq!(filtered.len(), 4);
}

#[test]
fn test_proxy_filter_by_country() {
    let proxies = create_test_proxies();

    // Filter by country name
    let filtered = apply_filter(
        proxies.clone(),
        ProxyFilter::ByCountry(vec!["United States".to_string()]),
    );
    assert_eq!(filtered.len(), 2);
    assert!(filtered.iter().all(|p| p.country == "United States"));

    // Filter by country code
    let filtered = apply_filter(
        proxies.clone(),
        ProxyFilter::ByCountry(vec!["DE".to_string()]),
    );
    assert_eq!(filtered.len(), 1);
    assert_eq!(filtered[0].country_code, "DE");

    // Filter by multiple countries
    let filtered = apply_filter(
        proxies.clone(),
        ProxyFilter::ByCountry(vec!["US".to_string(), "FR".to_string()]),
    );
    assert_eq!(filtered.len(), 3);
}

#[test]
fn test_proxy_filter_by_type() {
    let proxies = create_test_proxies();

    // Filter by HTTP
    let filtered = apply_filter(proxies.clone(), ProxyFilter::ByType(vec![ProxyType::Http]));
    assert_eq!(filtered.len(), 2);
    assert!(filtered.iter().all(|p| p.protocol == ProxyType::Http));

    // Filter by SOCKS5
    let filtered = apply_filter(
        proxies.clone(),
        ProxyFilter::ByType(vec![ProxyType::Socks5]),
    );
    assert_eq!(filtered.len(), 1);
    assert_eq!(filtered[0].protocol, ProxyType::Socks5);

    // Filter by multiple types
    let filtered = apply_filter(
        proxies.clone(),
        ProxyFilter::ByType(vec![ProxyType::Socks4, ProxyType::Socks5]),
    );
    assert_eq!(filtered.len(), 2);
}

#[test]
fn test_proxy_filter_working_only() {
    let proxies = create_test_proxies();
    let filtered = apply_filter(proxies.clone(), ProxyFilter::WorkingOnly);
    assert_eq!(filtered.len(), 3);
    assert!(filtered.iter().all(|p| p.is_working));
}

// Helper function to apply filter (mirrors the trait implementation)
fn apply_filter(proxies: Vec<FreeProxy>, filter: ProxyFilter) -> Vec<FreeProxy> {
    match filter {
        ProxyFilter::All => proxies,
        ProxyFilter::ByCountry(countries) => proxies
            .into_iter()
            .filter(|p| countries.contains(&p.country) || countries.contains(&p.country_code))
            .collect(),
        ProxyFilter::ByType(types) => proxies
            .into_iter()
            .filter(|p| types.contains(&p.protocol))
            .collect(),
        ProxyFilter::WorkingOnly => proxies.into_iter().filter(|p| p.is_working).collect(),
    }
}

// ============================================================================
// FreeIpProviderManager Tests
// ============================================================================

#[test]
fn test_provider_manager_creation() {
    let manager = FreeIpProviderManager::new();
    assert!(manager.is_ok());
}

#[test]
fn test_provider_manager_default() {
    let manager = FreeIpProviderManager::default();
    // Default should create successfully
    assert!(manager.get_proxy_pool().is_empty());
}

#[test]
fn test_provider_manager_with_update_interval() {
    let manager = FreeIpProviderManager::new()
        .expect("Operation should succeed")
        .with_update_interval(Duration::from_secs(600));

    // Manager should be configured (we can't directly check interval, but creation should succeed)
    assert!(manager.get_proxy_pool().is_empty());
}

#[test]
fn test_provider_manager_get_working_proxies_empty() {
    let manager = FreeIpProviderManager::new().expect("Operation should succeed");
    let working = manager.get_working_proxies();
    assert!(working.is_empty());
}

#[test]
fn test_provider_manager_get_random_working_proxy_empty() {
    let manager = FreeIpProviderManager::new().expect("Operation should succeed");
    let proxy = manager.get_random_working_proxy();
    assert!(proxy.is_none());
}

#[test]
fn test_provider_manager_needs_update_initially() {
    let manager = FreeIpProviderManager::new().expect("Operation should succeed");
    // A fresh manager should need an update
    assert!(manager.needs_update());
}

// ============================================================================
// FreeProxy Tests
// ============================================================================

#[test]
fn test_free_proxy_to_proxy_settings() {
    let proxy = FreeProxy {
        ip: "192.168.1.1".to_string(),
        port: 8080,
        protocol: ProxyType::Http,
        country: "United States".to_string(),
        country_code: "US".to_string(),
        anonymity: "elite".to_string(),
        speed: 100,
        uptime: 99.5,
        last_checked: "2024-01-01T00:00:00Z".to_string(),
        provider: "TestProvider".to_string(),
        is_working: true,
    };

    let settings = proxy.to_proxy_settings();
    assert_eq!(settings.proxy_type, ProxyType::Http);
    assert_eq!(settings.host, Some("192.168.1.1".to_string()));
    assert_eq!(settings.port, Some(8080));
    assert!(settings.username.is_none());
    assert!(settings.password.is_none());
}

#[test]
fn test_free_proxy_socks5_to_settings() {
    let proxy = FreeProxy {
        ip: "10.0.0.1".to_string(),
        port: 1080,
        protocol: ProxyType::Socks5,
        country: "Germany".to_string(),
        country_code: "DE".to_string(),
        anonymity: "anonymous".to_string(),
        speed: 50,
        uptime: 95.0,
        last_checked: "2024-01-01T00:00:00Z".to_string(),
        provider: "TestProvider".to_string(),
        is_working: true,
    };

    let settings = proxy.to_proxy_settings();
    assert_eq!(settings.proxy_type, ProxyType::Socks5);
    assert_eq!(settings.host, Some("10.0.0.1".to_string()));
    assert_eq!(settings.port, Some(1080));
}

// ============================================================================
// ProxySettings Tests
// ============================================================================

#[test]
fn test_proxy_settings_to_url_http() {
    use browser_core::proxy::ProxySettings;

    let settings = ProxySettings {
        proxy_type: ProxyType::Http,
        host: Some("192.168.1.1".to_string()),
        port: Some(8080),
        username: None,
        password: None,
        dns_servers: vec![],
        bypass_list: vec![],
    };

    let url = settings.to_url();
    assert_eq!(url, Some("http://192.168.1.1:8080".to_string()));
}

#[test]
fn test_proxy_settings_to_url_with_auth() {
    use browser_core::proxy::ProxySettings;

    let settings = ProxySettings {
        proxy_type: ProxyType::Socks5,
        host: Some("192.168.1.1".to_string()),
        port: Some(1080),
        username: Some("user".to_string()),
        password: Some("pass".to_string()),
        dns_servers: vec![],
        bypass_list: vec![],
    };

    let url = settings.to_url();
    assert_eq!(url, Some("socks5://user:pass@192.168.1.1:1080".to_string()));
}

#[test]
fn test_proxy_settings_to_url_direct() {
    use browser_core::proxy::ProxySettings;

    let settings = ProxySettings {
        proxy_type: ProxyType::Direct,
        host: Some("192.168.1.1".to_string()),
        port: Some(8080),
        username: None,
        password: None,
        dns_servers: vec![],
        bypass_list: vec![],
    };

    let url = settings.to_url();
    assert!(url.is_none());
}

#[test]
fn test_proxy_settings_is_configured() {
    use browser_core::proxy::ProxySettings;

    // Fully configured
    let settings = ProxySettings {
        proxy_type: ProxyType::Http,
        host: Some("192.168.1.1".to_string()),
        port: Some(8080),
        username: None,
        password: None,
        dns_servers: vec![],
        bypass_list: vec![],
    };
    assert!(settings.is_configured());

    // Direct type - not configured
    let settings = ProxySettings::default();
    assert!(!settings.is_configured());

    // Missing host
    let settings = ProxySettings {
        proxy_type: ProxyType::Http,
        host: None,
        port: Some(8080),
        username: None,
        password: None,
        dns_servers: vec![],
        bypass_list: vec![],
    };
    assert!(!settings.is_configured());

    // Missing port
    let settings = ProxySettings {
        proxy_type: ProxyType::Http,
        host: Some("192.168.1.1".to_string()),
        port: None,
        username: None,
        password: None,
        dns_servers: vec![],
        bypass_list: vec![],
    };
    assert!(!settings.is_configured());
}

// ============================================================================
// ProxyType Tests
// ============================================================================

#[test]
fn test_proxy_type_default() {
    let default_type = ProxyType::default();
    assert_eq!(default_type, ProxyType::Direct);
}

#[test]
fn test_proxy_type_equality() {
    assert_eq!(ProxyType::Http, ProxyType::Http);
    assert_eq!(ProxyType::Socks5, ProxyType::Socks5);
    assert_ne!(ProxyType::Http, ProxyType::Socks5);
    assert_ne!(ProxyType::Direct, ProxyType::Http);
}

// ============================================================================
// Async Integration Tests (require tokio runtime)
// ============================================================================

#[cfg(test)]
mod async_tests {
    use super::*;

    #[tokio::test]
    async fn test_provider_manager_fetch_from_provider() {
        let mut manager = FreeIpProviderManager::new().expect("Operation should succeed");

        // Test fetching from a single provider (may fail due to network, but shouldn't panic)
        let provider = FreeIpProvider::ProxyScrape;
        let result = manager.fetch_from_provider(&provider).await;

        match result {
            Ok(proxies) => {
                println!(
                    "Provider {} returned {} proxies",
                    provider.name(),
                    proxies.len()
                );
                // If we get proxies, verify they have valid structure
                for proxy in &proxies {
                    assert!(!proxy.ip.is_empty());
                    assert!(proxy.port > 0);
                }
            }
            Err(e) => {
                println!(
                    "Provider {} failed (expected in test environment): {}",
                    provider.name(),
                    e
                );
            }
        }
    }

    #[tokio::test]
    async fn test_provider_manager_update_proxy_pool() {
        let mut manager = FreeIpProviderManager::new().expect("Operation should succeed");

        // Update pool should not panic even if providers fail
        let result = manager.update_proxy_pool().await;
        match result {
            Ok(count) => {
                println!("Updated pool with {} proxies", count);
                assert_eq!(manager.get_proxy_pool().len(), count);
            }
            Err(e) => {
                println!("Update failed (expected in test environment): {}", e);
            }
        }
    }

    #[tokio::test]
    async fn test_provider_manager_fetch_all() {
        let mut manager = FreeIpProviderManager::new().expect("Operation should succeed");

        // Fetch all should return proxies from multiple providers
        let proxies = manager.fetch_all().await;
        println!("Fetched {} proxies from all providers", proxies.len());

        // Verify proxy structure if any were returned
        for proxy in &proxies {
            assert!(!proxy.ip.is_empty());
            assert!(proxy.port > 0);
            assert!(!proxy.provider.is_empty());
        }
    }
}
