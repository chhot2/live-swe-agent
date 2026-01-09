//! Tests for WebView Manager functionality
//!
//! This module tests:
//! - WebviewTab structure
//! - Tab configuration
//! - Proxy configuration for tabs

use browser_core::proxy::{ProxySettings, ProxyType};
use browser_core::webview_manager::WebviewTab;
use chrono::Utc;

// ============================================================================
// WebviewTab Tests
// ============================================================================

fn create_test_tab() -> WebviewTab {
    WebviewTab {
        tab_id: "tab-12345".to_string(),
        window_label: "window-1".to_string(),
        url: "https://example.com".to_string(),
        title: "Example Domain".to_string(),
        favicon: Some("https://example.com/favicon.ico".to_string()),
        is_loading: false,
        can_go_back: true,
        can_go_forward: false,
        created_at: Utc::now(),
        last_active: Utc::now(),
        proxy_config: None,
        zoom_level: 1.0,
    }
}

#[test]
fn test_webview_tab_creation() {
    let tab = create_test_tab();

    assert_eq!(tab.tab_id, "tab-12345");
    assert_eq!(tab.window_label, "window-1");
    assert_eq!(tab.url, "https://example.com");
    assert_eq!(tab.title, "Example Domain");
    assert!(tab.favicon.is_some());
    assert!(!tab.is_loading);
    assert!(tab.can_go_back);
    assert!(!tab.can_go_forward);
    assert!(tab.proxy_config.is_none());
    assert_eq!(tab.zoom_level, 1.0);
}

#[test]
fn test_webview_tab_with_proxy() {
    let proxy_config = ProxySettings {
        proxy_type: ProxyType::Http,
        host: Some("192.168.1.1".to_string()),
        port: Some(8080),
        username: None,
        password: None,
        dns_servers: vec!["8.8.8.8".to_string()],
        bypass_list: vec!["localhost".to_string()],
    };

    let tab = WebviewTab {
        tab_id: "tab-proxy".to_string(),
        window_label: "window-2".to_string(),
        url: "https://secure.example.com".to_string(),
        title: "Secure Site".to_string(),
        favicon: None,
        is_loading: true,
        can_go_back: false,
        can_go_forward: false,
        created_at: Utc::now(),
        last_active: Utc::now(),
        proxy_config: Some(proxy_config),
        zoom_level: 1.5,
    };

    assert!(tab.proxy_config.is_some());
    let config = tab.proxy_config.expect("Configuration error");
    assert_eq!(config.proxy_type, ProxyType::Http);
    assert_eq!(config.host, Some("192.168.1.1".to_string()));
    assert_eq!(config.port, Some(8080));
}

#[test]
fn test_webview_tab_clone() {
    let tab = create_test_tab();
    let cloned = tab.clone();

    assert_eq!(cloned.tab_id, tab.tab_id);
    assert_eq!(cloned.url, tab.url);
    assert_eq!(cloned.title, tab.title);
    assert_eq!(cloned.zoom_level, tab.zoom_level);
}

#[test]
fn test_webview_tab_serialization() {
    let tab = create_test_tab();

    let json = serde_json::to_string(&tab).expect("Tab operation failed");
    assert!(json.contains("tab_id"));
    assert!(json.contains("url"));
    assert!(json.contains("zoom_level"));

    let parsed: WebviewTab = serde_json::from_str(&json).expect("Json operation failed");
    assert_eq!(parsed.tab_id, tab.tab_id);
    assert_eq!(parsed.url, tab.url);
}

#[test]
fn test_webview_tab_debug() {
    let tab = create_test_tab();
    let debug_str = format!("{:?}", tab);

    assert!(debug_str.contains("WebviewTab"));
    assert!(debug_str.contains("tab_id"));
}

#[test]
fn test_webview_tab_loading_states() {
    let mut tab = create_test_tab();

    // Initial state - not loading
    assert!(!tab.is_loading);

    // Simulate loading
    tab.is_loading = true;
    assert!(tab.is_loading);

    // Loading complete
    tab.is_loading = false;
    assert!(!tab.is_loading);
}

#[test]
fn test_webview_tab_navigation_state() {
    let mut tab = create_test_tab();

    // Initially can go back but not forward
    assert!(tab.can_go_back);
    assert!(!tab.can_go_forward);

    // After going back
    tab.can_go_back = false;
    tab.can_go_forward = true;
    assert!(!tab.can_go_back);
    assert!(tab.can_go_forward);
}

#[test]
fn test_webview_tab_zoom_levels() {
    let zoom_levels = vec![0.25, 0.5, 0.75, 1.0, 1.25, 1.5, 2.0, 3.0];

    for zoom in zoom_levels {
        let tab = WebviewTab {
            tab_id: format!("tab-zoom-{}", zoom),
            window_label: "window-zoom".to_string(),
            url: "https://example.com".to_string(),
            title: "Zoom Test".to_string(),
            favicon: None,
            is_loading: false,
            can_go_back: false,
            can_go_forward: false,
            created_at: Utc::now(),
            last_active: Utc::now(),
            proxy_config: None,
            zoom_level: zoom,
        };

        assert_eq!(tab.zoom_level, zoom);
    }
}

#[test]
fn test_webview_tab_url_variations() {
    let urls = vec![
        "https://example.com",
        "http://example.com",
        "https://example.com:8080/path",
        "https://subdomain.example.com",
        "https://example.com/path?query=value",
        "about:blank",
        "file:///local/path",
    ];

    for url in urls {
        let tab = WebviewTab {
            tab_id: "tab-url-test".to_string(),
            window_label: "window-url".to_string(),
            url: url.to_string(),
            title: "URL Test".to_string(),
            favicon: None,
            is_loading: false,
            can_go_back: false,
            can_go_forward: false,
            created_at: Utc::now(),
            last_active: Utc::now(),
            proxy_config: None,
            zoom_level: 1.0,
        };

        assert_eq!(tab.url, url);
    }
}

#[test]
fn test_webview_tab_timestamps() {
    let before = Utc::now();
    let tab = create_test_tab();
    let after = Utc::now();

    assert!(tab.created_at >= before);
    assert!(tab.created_at <= after);
    assert!(tab.last_active >= before);
    assert!(tab.last_active <= after);
}

// ============================================================================
// Multiple Tabs Simulation Tests
// ============================================================================

#[test]
fn test_multiple_tabs_unique_ids() {
    let tabs: Vec<WebviewTab> = (0..10)
        .map(|i| WebviewTab {
            tab_id: format!("tab-{}", i),
            window_label: format!("window-{}", i),
            url: format!("https://example{}.com", i),
            title: format!("Tab {}", i),
            favicon: None,
            is_loading: false,
            can_go_back: false,
            can_go_forward: false,
            created_at: Utc::now(),
            last_active: Utc::now(),
            proxy_config: None,
            zoom_level: 1.0,
        })
        .collect();

    // Verify all IDs are unique
    let mut ids: Vec<&str> = tabs.iter().map(|t| t.tab_id.as_str()).collect();
    ids.sort();
    ids.dedup();
    assert_eq!(ids.len(), 10);

    // Verify all URLs are unique
    let mut urls: Vec<&str> = tabs.iter().map(|t| t.url.as_str()).collect();
    urls.sort();
    urls.dedup();
    assert_eq!(urls.len(), 10);
}

#[test]
fn test_multiple_tabs_with_different_proxies() {
    let proxies = vec![
        ("192.168.1.1", 8080, ProxyType::Http),
        ("192.168.1.2", 1080, ProxyType::Socks5),
        ("192.168.1.3", 3128, ProxyType::Http),
    ];

    let tabs: Vec<WebviewTab> = proxies
        .iter()
        .enumerate()
        .map(|(i, (host, port, proxy_type))| WebviewTab {
            tab_id: format!("tab-{}", i),
            window_label: format!("window-{}", i),
            url: format!("https://site{}.com", i),
            title: format!("Proxied Tab {}", i),
            favicon: None,
            is_loading: false,
            can_go_back: false,
            can_go_forward: false,
            created_at: Utc::now(),
            last_active: Utc::now(),
            proxy_config: Some(ProxySettings {
                proxy_type: proxy_type.clone(),
                host: Some(host.to_string()),
                port: Some(*port),
                username: None,
                password: None,
                dns_servers: vec![],
                bypass_list: vec![],
            }),
            zoom_level: 1.0,
        })
        .collect();

    // Verify each tab has a different proxy
    for (i, tab) in tabs.iter().enumerate() {
        let config = tab.proxy_config.as_ref().expect("Configuration error");
        assert_eq!(
            config.host.as_ref().expect("Configuration error"),
            proxies[i].0
        );
        assert_eq!(config.port.expect("Configuration error"), proxies[i].1);
    }
}

#[test]
fn test_tab_collection_operations() {
    use std::collections::HashMap;

    let mut tabs: HashMap<String, WebviewTab> = HashMap::new();

    // Add tabs
    for i in 0..5 {
        let tab = WebviewTab {
            tab_id: format!("tab-{}", i),
            window_label: format!("window-{}", i),
            url: format!("https://example{}.com", i),
            title: format!("Tab {}", i),
            favicon: None,
            is_loading: false,
            can_go_back: false,
            can_go_forward: false,
            created_at: Utc::now(),
            last_active: Utc::now(),
            proxy_config: None,
            zoom_level: 1.0,
        };
        tabs.insert(tab.tab_id.clone(), tab);
    }

    assert_eq!(tabs.len(), 5);

    // Remove a tab
    tabs.remove("tab-2");
    assert_eq!(tabs.len(), 4);
    assert!(tabs.get("tab-2").is_none());

    // Update a tab
    if let Some(tab) = tabs.get_mut("tab-3") {
        tab.url = "https://updated.com".to_string();
        tab.title = "Updated Title".to_string();
    }

    let updated = tabs.get("tab-3").expect("Failed to get value");
    assert_eq!(updated.url, "https://updated.com");
    assert_eq!(updated.title, "Updated Title");
}
