//! Webview Manager Module for Browser Core
//!
//! Provides comprehensive webview tab management including:
//! - Tab lifecycle management (create, close, focus)
//! - Navigation control (navigate, back, forward, reload, stop)
//! - Proxy integration with per-tab proxy settings
//! - Proxy rotation and session management
//! - PAC (Proxy Auto-Config) server integration
//! - Free proxy provider management

use crate::free_ip_providers::FreeIpProviderManager;
use crate::local_proxy::LocalProxyManager;
use crate::pac_server::PacManager;
use crate::proxy::{FreeProxy, ProxySettings};
use crate::proxy_rotation::{ProxyRotationManager, ProxyRotationStrategy, ProxySessionStats};
use anyhow::{anyhow, Result};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tauri::{AppHandle, Manager, WebviewUrl, WebviewWindow, WebviewWindowBuilder};
use tokio::sync::RwLock;
use tracing::{debug, info};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Represents a WebviewTab.
pub struct WebviewTab {
    pub tab_id: String,
    pub window_label: String,
    pub url: String,
    pub title: String,
    pub favicon: Option<String>,
    pub is_loading: bool,
    pub can_go_back: bool,
    pub can_go_forward: bool,
    pub created_at: DateTime<Utc>,
    pub last_active: DateTime<Utc>,
    pub proxy_config: Option<ProxySettings>,
    pub zoom_level: f64,
}

/// Represents a WebviewManager.
pub struct WebviewManager {
    app_handle: AppHandle,
    tabs: RwLock<HashMap<String, WebviewTab>>,
    window_counter: RwLock<u32>,
    active_tab: RwLock<Option<String>>,
    local_proxy_manager: Arc<LocalProxyManager>,
    pub(crate) pac_manager: Arc<PacManager>,
    proxy_provider_manager: Arc<RwLock<FreeIpProviderManager>>,
    proxy_rotation_manager: Arc<RwLock<ProxyRotationManager>>,
}

impl WebviewManager {
    /// Create a new WebviewManager instance
    ///
    /// Initializes all proxy infrastructure including local proxy manager,
    /// PAC server, proxy provider manager, and rotation manager.
    ///
    /// # Arguments
    /// * `app_handle` - The Tauri application handle
    pub fn new(app_handle: AppHandle) -> Self {
        // Initialize local proxy manager with port range 9000-9999
        let local_proxy_manager = Arc::new(LocalProxyManager::new(9000..10000));

        // Initialize PAC server on port 8080
        let pac_manager = Arc::new(PacManager::new(8080).expect("Failed to create PAC manager"));

        // Initialize proxy provider manager
        let proxy_provider_manager = Arc::new(RwLock::new(
            FreeIpProviderManager::new().expect("Failed to create proxy provider manager"),
        ));

        // Initialize proxy rotation manager with round-robin strategy
        let proxy_rotation_manager = Arc::new(RwLock::new(ProxyRotationManager::new(
            proxy_provider_manager.clone(),
            ProxyRotationStrategy::RoundRobin,
        )));

        Self {
            app_handle,
            tabs: RwLock::new(HashMap::new()),
            window_counter: RwLock::new(0),
            active_tab: RwLock::new(None),
            local_proxy_manager,
            pac_manager,
            proxy_provider_manager,
            proxy_rotation_manager,
        }
    }

    /// Start the proxy infrastructure
    /// Start the proxy infrastructure including PAC server
    ///
    /// This should be called during application initialization to enable
    /// proxy functionality for all tabs.
    pub async fn start_proxy_infrastructure(&self) -> Result<()> {
        // Start PAC server
        self.pac_manager.start().await?;

        // Create a default local proxy for all tabs
        self.local_proxy_manager
            .create_proxy_for_tab("default", None)
            .await?;

        // Update proxy pool from all providers
        {
            let mut provider_manager = self.proxy_provider_manager.write().await;
            provider_manager.update_proxy_pool().await?;
        }

        info!("Proxy infrastructure started");
        Ok(())
    }

    /// Get a proxy for a tab with rotation
    /// Get the proxy assigned to a specific tab
    ///
    /// # Arguments
    /// * `tab_id` - The ID of the tab
    /// * `domain` - Optional domain for domain-specific proxy selection
    pub async fn get_proxy_for_tab(
        &self,
        tab_id: &str,
        domain: Option<&str>,
    ) -> Result<Option<FreeProxy>> {
        let rotation_manager = self.proxy_rotation_manager.read().await;
        match rotation_manager.get_proxy_for_tab(tab_id, domain).await {
            Ok(proxy) => Ok(Some(proxy)),
            Err(_) => Ok(None), // No proxies available
        }
    }

    /// Force rotate proxy for a tab
    /// Rotate to a new proxy for a specific tab
    ///
    /// # Arguments
    /// * `tab_id` - The ID of the tab to rotate proxy for
    pub async fn rotate_proxy_for_tab(&self, tab_id: &str) -> Result<Option<FreeProxy>> {
        let rotation_manager = self.proxy_rotation_manager.read().await;
        match rotation_manager.force_rotate(tab_id).await {
            Ok(proxy) => Ok(Some(proxy)),
            Err(_) => Ok(None),
        }
    }

    /// Get proxy session statistics
    /// Get proxy session statistics for a tab
    ///
    /// # Arguments
    /// * `tab_id` - The ID of the tab
    pub async fn get_proxy_session_stats(&self, tab_id: &str) -> Result<Option<ProxySessionStats>> {
        let rotation_manager = self.proxy_rotation_manager.read().await;
        Ok(rotation_manager.get_session_stats(tab_id).await)
    }

    /// Update proxy rotation strategy
    /// Update the proxy rotation strategy
    ///
    /// # Arguments
    /// * `strategy` - The new rotation strategy to use
    pub async fn update_rotation_strategy(&self, strategy: ProxyRotationStrategy) -> Result<()> {
        let mut rotation_manager = self.proxy_rotation_manager.write().await;
        rotation_manager.update_strategy(strategy).await;
        Ok(())
    }

    /// Record proxy performance
    /// Record proxy performance metrics
    ///
    /// # Arguments
    /// * `proxy_id` - The ID of the proxy
    /// * `success` - Whether the request was successful
    /// * `response_time_ms` - Optional response time in milliseconds
    pub async fn record_proxy_performance(
        &self,
        proxy_id: &str,
        success: bool,
        response_time_ms: Option<f64>,
    ) -> Result<()> {
        let rotation_manager = self.proxy_rotation_manager.read().await;
        rotation_manager
            .record_performance(proxy_id, success, response_time_ms)
            .await;
        Ok(())
    }

    /// Create a new webview tab with native window
    /// Create a new tab without proxy settings
    ///
    /// # Arguments
    /// * `initial_url` - Optional initial URL to navigate to
    pub async fn create_tab(&self, initial_url: Option<String>) -> Result<WebviewTab> {
        self.create_tab_with_proxy(initial_url, None).await
    }

    /// Create a new webview tab with proxy configuration
    pub async fn create_tab_with_proxy(
        &self,
        initial_url: Option<String>,
        proxy_config: Option<ProxySettings>,
    ) -> Result<WebviewTab> {
        let tab_id = Uuid::new_v4().to_string();
        let mut counter = self.window_counter.write().await;
        *counter += 1;
        let window_label = format!("tab_{}", counter);

        let url = initial_url.unwrap_or_else(|| "https://www.google.com".to_string());

        // Store proxy configuration for this tab
        // Note: Using single proxy approach due to WebView2 limitations
        let proxy_port = if proxy_config.is_some() {
            // Use default proxy server on port 9000
            9000
        } else {
            0 // No proxy
        };

        // Register PAC file for this tab
        let _pac_url = self
            .pac_manager
            .register_proxy_for_tab(&tab_id, proxy_port)
            .await
            .map_err(|e| anyhow!("Failed to register PAC: {}", e))?;

        // Parse URL for Tauri 2.0
        let url_str = url.clone();
        let webview_url = if url.starts_with("http://") || url.starts_with("https://") {
            WebviewUrl::External(url.parse()?)
        } else {
            WebviewUrl::App(url.into())
        };

        // Create new webview window using Tauri 2.0 API
        let window = WebviewWindowBuilder::new(&self.app_handle, window_label.clone(), webview_url)
            .title("New Tab")
            .inner_size(1200.0, 800.0)
            .min_inner_size(400.0, 300.0)
            .center()
            .decorations(true)
            .resizable(true)
            .build()
            .map_err(|e| anyhow!("Failed to create WebView window: {}", e))?;

        let tab = WebviewTab {
            tab_id: tab_id.clone(),
            window_label: window_label.clone(),
            url: url_str,
            title: "New Tab".to_string(),
            favicon: None,
            is_loading: false,
            can_go_back: false,
            can_go_forward: false,
            created_at: Utc::now(),
            last_active: Utc::now(),
            proxy_config,
            zoom_level: 1.0,
        };

        // Store tab reference
        self.tabs.write().await.insert(tab_id.clone(), tab.clone());

        // Set as active tab
        *self.active_tab.write().await = Some(tab_id.clone());

        // Setup event listeners
        self.setup_window_events(&window, &tab_id).await?;

        // Inject tab identifier for proxy routing
        self.configure_tab_identifier(&window, &tab_id).await?;

        Ok(tab)
    }

    /// Navigate a tab to a new URL
    pub async fn navigate(&self, tab_id: &str, url: &str) -> Result<()> {
        // Validate URL
        if !url.starts_with("http://")
            && !url.starts_with("https://")
            && !url.starts_with("about:")
            && !url.starts_with("data:")
        {
            return Err(anyhow!("Invalid URL scheme"));
        }

        let tabs = self.tabs.read().await;
        let tab = tabs.get(tab_id).ok_or_else(|| anyhow!("Tab not found"))?;

        if let Some(window) = self.app_handle.get_webview_window(&tab.window_label) {
            window.eval(format!("window.location.href = '{}';", url))?;

            // Update tab info
            drop(tabs);
            let mut tabs = self.tabs.write().await;
            if let Some(tab) = tabs.get_mut(tab_id) {
                tab.url = url.to_string();
                tab.is_loading = true;
                tab.last_active = Utc::now();
            }
        }

        Ok(())
    }

    /// Get all tabs
    pub async fn list_tabs(&self) -> Vec<WebviewTab> {
        self.tabs.read().await.values().cloned().collect()
    }

    /// Close a tab and clean up associated proxy resources
    pub async fn close_tab(&self, tab_id: &str) -> Result<()> {
        let tabs = self.tabs.read().await;
        let tab = tabs.get(tab_id).ok_or_else(|| anyhow!("Tab not found"))?;

        if let Some(window) = self.app_handle.get_webview_window(&tab.window_label) {
            window.close()?;
        }

        drop(tabs);

        // Clean up proxy resources
        self.local_proxy_manager
            .remove_proxy_for_tab(tab_id)
            .await?;
        self.pac_manager.remove_proxy_for_tab(tab_id).await?;

        self.tabs.write().await.remove(tab_id);

        // Update active tab if necessary
        let mut active_tab = self.active_tab.write().await;
        if *active_tab == Some(tab_id.to_string()) {
            *active_tab = self.tabs.read().await.keys().next().cloned();
        }

        Ok(())
    }

    /// Configure proxy settings in a WebView using PAC URL
    #[allow(dead_code)]
    async fn configure_proxy_in_webview(
        &self,
        window: &WebviewWindow,
        pac_url: &str,
    ) -> Result<()> {
        // JavaScript to configure proxy settings
        // Note: WebView2 doesn't support direct PAC configuration via JavaScript
        // This is a placeholder for the actual implementation
        let js_code = format!(
            r#"
            // Store PAC URL for potential use by extensions
            window.__PAC_URL = '{}';
            
            // Log that proxy configuration is set
            console.log('Proxy PAC URL configured:', window.__PAC_URL);
            "#,
            pac_url
        );

        // Inject JavaScript into the WebView
        window
            .eval(&js_code)
            .map_err(|e| anyhow!("Failed to inject proxy configuration: {}", e))?;

        debug!("Injected proxy configuration with PAC URL: {}", pac_url);
        Ok(())
    }

    /// Configure tab identifier for proxy routing
    async fn configure_tab_identifier(&self, window: &WebviewWindow, tab_id: &str) -> Result<()> {
        // JavaScript to inject tab identifier
        let js_code = format!(
            r#"
            // Store tab ID for proxy routing
            window.__TAB_ID = '{}';
            
            // Override fetch to add tab ID header
            const originalFetch = window.fetch;
            window.fetch = function(...args) {{
                if (args[1] && typeof args[1] === 'object') {{
                    args[1].headers = {{
                        ...args[1].headers,
                        'X-Tab-ID': '{}'
                    }};
                }}
                return originalFetch.apply(this, args);
            }};
            
            console.log('Tab identifier configured:', window.__TAB_ID);
            "#,
            tab_id, tab_id
        );

        // Inject JavaScript into the WebView
        window
            .eval(&js_code)
            .map_err(|e| anyhow!("Failed to inject tab identifier: {}", e))?;

        debug!("Injected tab identifier: {}", tab_id);
        Ok(())
    }

    /// Focus a tab's window
    pub async fn focus_tab(&self, tab_id: &str) -> Result<()> {
        let tabs = self.tabs.read().await;
        let tab = tabs.get(tab_id).ok_or_else(|| anyhow!("Tab not found"))?;

        if let Some(window) = self.app_handle.get_webview_window(&tab.window_label) {
            window.set_focus()?;
            window.unminimize()?;

            // Update active tab
            *self.active_tab.write().await = Some(tab_id.to_string());

            // Update last active time
            drop(tabs);
            let mut tabs_mut = self.tabs.write().await;
            if let Some(tab_mut) = tabs_mut.get_mut(tab_id) {
                tab_mut.last_active = Utc::now();
            }
        }

        Ok(())
    }

    /// Get tab by ID
    pub async fn get_tab(&self, tab_id: &str) -> Option<WebviewTab> {
        self.tabs.read().await.get(tab_id).cloned()
    }

    /// Update tab title
    pub async fn update_tab_title(&self, tab_id: &str, title: String) -> Result<()> {
        let mut tabs = self.tabs.write().await;
        if let Some(tab) = tabs.get_mut(tab_id) {
            tab.title = title;
        }
        Ok(())
    }

    /// Go back in navigation history
    /// Navigate back in tab history
    ///
    /// # Arguments
    /// * `tab_id` - The ID of the tab
    pub async fn go_back(&self, tab_id: &str) -> Result<()> {
        let tabs = self.tabs.read().await;
        let tab = tabs.get(tab_id).ok_or_else(|| anyhow!("Tab not found"))?;

        if let Some(window) = self.app_handle.get_webview_window(&tab.window_label) {
            window.eval("window.history.back();")?;
        }

        Ok(())
    }

    /// Go forward in navigation history
    /// Navigate forward in tab history
    ///
    /// # Arguments
    /// * `tab_id` - The ID of the tab
    pub async fn go_forward(&self, tab_id: &str) -> Result<()> {
        let tabs = self.tabs.read().await;
        let tab = tabs.get(tab_id).ok_or_else(|| anyhow!("Tab not found"))?;

        if let Some(window) = self.app_handle.get_webview_window(&tab.window_label) {
            window.eval("window.history.forward();")?;
        }

        Ok(())
    }

    /// Reload the current page
    /// Reload the current page in a tab
    ///
    /// # Arguments
    /// * `tab_id` - The ID of the tab
    pub async fn reload(&self, tab_id: &str) -> Result<()> {
        let tabs = self.tabs.read().await;
        let tab = tabs.get(tab_id).ok_or_else(|| anyhow!("Tab not found"))?;

        if let Some(window) = self.app_handle.get_webview_window(&tab.window_label) {
            window.eval("window.location.reload();")?;
        }

        Ok(())
    }

    /// Stop loading the current page
    /// Stop loading in a tab
    ///
    /// # Arguments
    /// * `tab_id` - The ID of the tab
    pub async fn stop(&self, tab_id: &str) -> Result<()> {
        let tabs = self.tabs.read().await;
        let tab = tabs.get(tab_id).ok_or_else(|| anyhow!("Tab not found"))?;

        if let Some(window) = self.app_handle.get_webview_window(&tab.window_label) {
            window.eval("window.stop();")?;

            // Update loading state
            drop(tabs);
            let mut tabs_mut = self.tabs.write().await;
            if let Some(tab_mut) = tabs_mut.get_mut(tab_id) {
                tab_mut.is_loading = false;
            }
        }

        Ok(())
    }

    /// Set zoom level for a tab
    /// Set the zoom level for a tab
    ///
    /// # Arguments
    /// * `tab_id` - The ID of the tab
    /// * `level` - The zoom level (1.0 = 100%)
    pub async fn set_zoom(&self, tab_id: &str, level: f64) -> Result<()> {
        if !(0.25..=5.0).contains(&level) {
            return Err(anyhow!("Zoom level must be between 0.25 and 5.0"));
        }

        let tabs = self.tabs.read().await;
        let tab = tabs.get(tab_id).ok_or_else(|| anyhow!("Tab not found"))?;

        if let Some(window) = self.app_handle.get_webview_window(&tab.window_label) {
            window.eval(format!("document.body.style.zoom = '{}';", level))?;

            // Update zoom level in tab info
            drop(tabs);
            let mut tabs_mut = self.tabs.write().await;
            if let Some(tab_mut) = tabs_mut.get_mut(tab_id) {
                tab_mut.zoom_level = level;
            }
        }

        Ok(())
    }

    /// Get the currently active tab
    /// Get the ID of the currently active tab
    pub async fn get_active_tab_id(&self) -> Option<String> {
        self.active_tab.read().await.clone()
    }

    /// Execute JavaScript in a tab
    /// Execute JavaScript in a tab
    ///
    /// # Arguments
    /// * `tab_id` - The ID of the tab
    /// * `script` - The JavaScript code to execute
    pub async fn execute_script(&self, tab_id: &str, script: &str) -> Result<()> {
        let tabs = self.tabs.read().await;
        let tab = tabs.get(tab_id).ok_or_else(|| anyhow!("Tab not found"))?;

        if let Some(window) = self.app_handle.get_webview_window(&tab.window_label) {
            window.eval(script)?;
        }

        Ok(())
    }

    /// Update tab navigation state (public method for Tauri commands)
    /// Update the navigation state for a tab
    ///
    /// # Arguments
    /// * `tab_id` - The ID of the tab
    /// * `url` - The current URL
    /// * `title` - The page title
    /// * `can_go_back` - Whether back navigation is possible
    /// * `can_go_forward` - Whether forward navigation is possible
    pub async fn update_navigation_state(
        &self,
        tab_id: &str,
        url: String,
        title: String,
        can_go_back: bool,
        can_go_forward: bool,
        is_loading: bool,
    ) -> Result<()> {
        let mut tabs = self.tabs.write().await;
        if let Some(tab) = tabs.get_mut(tab_id) {
            tab.url = url;
            tab.title = title;
            tab.can_go_back = can_go_back;
            tab.can_go_forward = can_go_forward;
            tab.is_loading = is_loading;
        }
        Ok(())
    }

    /// Setup window event listeners
    async fn setup_window_events(&self, _window: &WebviewWindow, tab_id: &str) -> Result<()> {
        // Note: Tauri 2.0 event listeners are handled differently
        // For now, we rely on frontend-initiated events
        debug!("Window events setup for tab: {}", tab_id);
        Ok(())
    }
}

// Tauri command handlers
#[tauri::command]
/// Creates a new webview tab.
pub async fn create_webview_tab(
    app_handle: tauri::AppHandle,
    url: Option<String>,
) -> Result<WebviewTab, String> {
    let manager = app_handle.state::<WebviewManager>();
    manager.create_tab(url).await.map_err(|e| e.to_string())
}

#[tauri::command]
/// Creates a new tab with proxy.
pub async fn create_tab_with_proxy(
    app_handle: tauri::AppHandle,
    url: Option<String>,
    proxy_config: Option<ProxySettings>,
) -> Result<WebviewTab, String> {
    let manager = app_handle.state::<WebviewManager>();
    manager
        .create_tab_with_proxy(url, proxy_config)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
/// Starts the proxy infrastructure.
pub async fn start_proxy_infrastructure(app_handle: tauri::AppHandle) -> Result<(), String> {
    let manager = app_handle.state::<WebviewManager>();
    manager
        .start_proxy_infrastructure()
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
/// Gets the proxy status.
pub async fn get_proxy_status(
    app_handle: tauri::AppHandle,
) -> Result<HashMap<String, String>, String> {
    let manager = app_handle.state::<WebviewManager>();
    let active_proxies = manager.local_proxy_manager.get_active_proxies().await;
    let registered_pacs = manager.pac_manager.get_registered_proxies().await;

    let mut status = HashMap::new();
    status.insert(
        "active_proxies".to_string(),
        format!("{}", active_proxies.len()),
    );
    status.insert(
        "registered_pacs".to_string(),
        format!("{}", registered_pacs.len()),
    );

    Ok(status)
}

#[tauri::command]
/// Gets the tab proxy info.
pub async fn get_tab_proxy_info(
    app_handle: tauri::AppHandle,
    tab_id: String,
) -> Result<Option<String>, String> {
    let manager = app_handle.state::<WebviewManager>();
    Ok(manager.pac_manager.get_pac_url_for_tab(&tab_id).await)
}

#[tauri::command]
/// Fetches proxies from provider.
pub async fn fetch_proxies_from_provider(
    _app_handle: tauri::AppHandle,
    provider_name: String,
) -> Result<Vec<FreeProxy>, String> {
    use crate::free_ip_providers::{FreeIpProvider, FreeIpProviderManager};

    let provider = match provider_name.as_str() {
        "ProxyScrape" => FreeIpProvider::ProxyScrape,
        "GeoNode" => FreeIpProvider::GeoNode,
        "PubProxy" => FreeIpProvider::PubProxy,
        "FreeProxyList" => FreeIpProvider::FreeProxyList,
        "ProxyNova" => FreeIpProvider::ProxyNova,
        "SpysOne" => FreeIpProvider::SpysOne,
        _ => return Err("Invalid provider name".to_string()),
    };

    let mut manager = FreeIpProviderManager::new().map_err(|e| e.to_string())?;

    manager
        .fetch_from_provider(&provider)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
/// Tests proxy.
pub async fn test_proxy(
    _app_handle: tauri::AppHandle,
    proxy: FreeProxy,
) -> Result<crate::proxy::ProxyTestResult, String> {
    use crate::free_ip_providers::FreeIpProviderManager;

    let manager = FreeIpProviderManager::new().map_err(|e| e.to_string())?;

    Ok(manager.test_proxy(&proxy).await)
}

#[tauri::command]
/// Rotates proxy for tab.
pub async fn rotate_proxy_for_tab(
    app_handle: tauri::AppHandle,
    tab_id: String,
) -> Result<Option<FreeProxy>, String> {
    let manager = app_handle.state::<WebviewManager>();
    manager
        .rotate_proxy_for_tab(&tab_id)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
/// Gets the proxy session stats.
pub async fn get_proxy_session_stats(
    app_handle: tauri::AppHandle,
    tab_id: String,
) -> Result<Option<ProxySessionStats>, String> {
    let manager = app_handle.state::<WebviewManager>();
    manager
        .get_proxy_session_stats(&tab_id)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
/// Updates the rotation strategy.
pub async fn update_rotation_strategy(
    app_handle: tauri::AppHandle,
    strategy: String,
    params: Option<serde_json::Value>,
) -> Result<(), String> {
    let manager = app_handle.state::<WebviewManager>();

    let strategy = match strategy.as_str() {
        "per_request" => {
            let count = params
                .as_ref()
                .and_then(|p| p.get("count"))
                .and_then(|v| v.as_u64())
                .unwrap_or(100) as usize;
            ProxyRotationStrategy::PerRequest(count)
        }
        "per_duration" => {
            let minutes = params
                .as_ref()
                .and_then(|p| p.get("minutes"))
                .and_then(|v| v.as_u64())
                .unwrap_or(5) as i64;
            ProxyRotationStrategy::PerDuration(chrono::Duration::minutes(minutes))
        }
        "per_session" => ProxyRotationStrategy::PerSession,
        "random" => {
            let probability = params
                .as_ref()
                .and_then(|p| p.get("probability"))
                .and_then(|v| v.as_f64())
                .unwrap_or(0.1);
            ProxyRotationStrategy::Random { probability }
        }
        "sticky" => {
            let minutes = params
                .as_ref()
                .and_then(|p| p.get("minutes"))
                .and_then(|v| v.as_u64())
                .unwrap_or(10) as i64;
            ProxyRotationStrategy::Sticky {
                duration: chrono::Duration::minutes(minutes),
            }
        }
        "geographic" => {
            let countries = params
                .as_ref()
                .and_then(|p| p.get("countries"))
                .and_then(|v| v.as_array())
                .map(|arr| {
                    arr.iter()
                        .filter_map(|v| v.as_str())
                        .map(|s| s.to_string())
                        .collect()
                })
                .unwrap_or_default();
            ProxyRotationStrategy::Geographic {
                country_codes: countries,
            }
        }
        "performance_based" => ProxyRotationStrategy::PerformanceBased,
        "round_robin" => ProxyRotationStrategy::RoundRobin,
        "domain_based" => ProxyRotationStrategy::DomainBased,
        "manual" => ProxyRotationStrategy::Manual,
        _ => return Err("Invalid rotation strategy".to_string()),
    };

    manager
        .update_rotation_strategy(strategy)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
/// Performs navigate webview tab operation.
pub async fn navigate_webview_tab(
    app_handle: tauri::AppHandle,
    tab_id: String,
    url: String,
) -> Result<(), String> {
    let manager = app_handle.state::<WebviewManager>();
    manager
        .navigate(&tab_id, &url)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
/// Closes webview tab.
pub async fn close_webview_tab(app_handle: tauri::AppHandle, tab_id: String) -> Result<(), String> {
    let manager = app_handle.state::<WebviewManager>();
    manager.close_tab(&tab_id).await.map_err(|e| e.to_string())
}

#[tauri::command]
/// Performs focus webview tab operation.
pub async fn focus_webview_tab(app_handle: tauri::AppHandle, tab_id: String) -> Result<(), String> {
    let manager = app_handle.state::<WebviewManager>();
    manager.focus_tab(&tab_id).await.map_err(|e| e.to_string())
}

#[tauri::command]
/// Gets the webview tabs.
pub async fn get_webview_tabs(app_handle: tauri::AppHandle) -> Result<Vec<WebviewTab>, String> {
    let manager = app_handle.state::<WebviewManager>();
    Ok(manager.list_tabs().await)
}

#[tauri::command]
/// Performs navigation changed operation.
pub async fn navigation_changed(
    app_handle: tauri::AppHandle,
    tab_id: String,
    url: String,
    title: String,
    can_go_back: bool,
    can_go_forward: bool,
) -> Result<(), String> {
    let manager = app_handle.state::<WebviewManager>();
    manager
        .update_navigation_state(&tab_id, url, title, can_go_back, can_go_forward, false)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
/// Performs title changed operation.
pub async fn title_changed(
    app_handle: tauri::AppHandle,
    tab_id: String,
    title: String,
) -> Result<(), String> {
    let manager = app_handle.state::<WebviewManager>();
    manager
        .update_tab_title(&tab_id, title)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
/// Performs go back tab operation.
pub async fn go_back_tab(app_handle: tauri::AppHandle, tab_id: String) -> Result<(), String> {
    let manager = app_handle.state::<WebviewManager>();
    manager.go_back(&tab_id).await.map_err(|e| e.to_string())
}

#[tauri::command]
/// Performs go forward tab operation.
pub async fn go_forward_tab(app_handle: tauri::AppHandle, tab_id: String) -> Result<(), String> {
    let manager = app_handle.state::<WebviewManager>();
    manager.go_forward(&tab_id).await.map_err(|e| e.to_string())
}

#[tauri::command]
/// Reloads tab.
pub async fn reload_tab(app_handle: tauri::AppHandle, tab_id: String) -> Result<(), String> {
    let manager = app_handle.state::<WebviewManager>();
    manager.reload(&tab_id).await.map_err(|e| e.to_string())
}

#[tauri::command]
/// Stops the tab.
pub async fn stop_tab(app_handle: tauri::AppHandle, tab_id: String) -> Result<(), String> {
    let manager = app_handle.state::<WebviewManager>();
    manager.stop(&tab_id).await.map_err(|e| e.to_string())
}

#[tauri::command]
/// Sets the tab zoom.
pub async fn set_tab_zoom(
    app_handle: tauri::AppHandle,
    tab_id: String,
    level: f64,
) -> Result<(), String> {
    let manager = app_handle.state::<WebviewManager>();
    manager
        .set_zoom(&tab_id, level)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
/// Gets the active tab.
pub async fn get_active_tab(app_handle: tauri::AppHandle) -> Result<Option<String>, String> {
    let manager = app_handle.state::<WebviewManager>();
    Ok(manager.get_active_tab_id().await)
}

#[tauri::command]
/// Executes script in tab.
pub async fn execute_script_in_tab(
    app_handle: tauri::AppHandle,
    tab_id: String,
    script: String,
) -> Result<(), String> {
    let manager = app_handle.state::<WebviewManager>();
    manager
        .execute_script(&tab_id, &script)
        .await
        .map_err(|e| e.to_string())
}
