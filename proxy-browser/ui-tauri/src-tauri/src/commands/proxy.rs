//! Proxy-related Tauri commands
//! 
//! This module contains all commands for managing proxy settings and operations.

use browser_core::{ProxyManager, ProxySettings, ProxyType, FreeProxy};
use serde::{Deserialize, Serialize};
use tauri::State;
use tracing::{info, error, debug};
use std::sync::Arc;

/// Response structure for proxy operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProxyResponse {
    pub success: bool,
    pub message: String,
}

/// Gets the list of available free proxies.
#[tauri::command]
pub async fn get_free_proxies(
    proxy_manager: State<'_, Arc<ProxyManager>>,
) -> Result<Vec<FreeProxy>, String> {
    debug!("Getting free proxies");
    let proxies = proxy_manager.get_free_proxies().await;
    Ok(proxies)
}

/// Fetches new free proxies from configured providers.
#[tauri::command]
pub async fn fetch_free_proxies(
    proxy_manager: State<'_, Arc<ProxyManager>>,
) -> Result<ProxyResponse, String> {
    info!("Fetching free proxies");
    match proxy_manager.fetch_free_proxies().await {
        Ok(count) => Ok(ProxyResponse {
            success: true,
            message: format!("Fetched {} proxies", count),
        }),
        Err(e) => {
            error!("Failed to fetch proxies: {}", e);
            Err(e.to_string())
        }
    }
}

/// Gets the currently active proxy.
#[tauri::command]
pub async fn get_active_proxy(
    proxy_manager: State<'_, Arc<ProxyManager>>,
) -> Result<Option<FreeProxy>, String> {
    debug!("Getting active proxy");
    Ok(proxy_manager.get_active_proxy().await)
}

/// Sets the active proxy.
#[tauri::command]
pub async fn set_active_proxy(
    proxy_manager: State<'_, Arc<ProxyManager>>,
    proxy: FreeProxy,
) -> Result<ProxyResponse, String> {
    info!("Setting active proxy: {}:{}", proxy.ip, proxy.port);
    match proxy_manager.set_active_proxy(proxy).await {
        Ok(_) => Ok(ProxyResponse {
            success: true,
            message: "Proxy set successfully".to_string(),
        }),
        Err(e) => {
            error!("Failed to set proxy: {}", e);
            Err(e.to_string())
        }
    }
}

/// Rotates to the next proxy based on the configured strategy.
#[tauri::command]
pub async fn rotate_proxy(
    proxy_manager: State<'_, Arc<ProxyManager>>,
    strategy: Option<String>,
) -> Result<Option<FreeProxy>, String> {
    info!("Rotating proxy with strategy: {:?}", strategy);
    match proxy_manager.rotate_proxy(strategy.as_deref()).await {
        Ok(proxy) => Ok(proxy),
        Err(e) => {
            error!("Failed to rotate proxy: {}", e);
            Err(e.to_string())
        }
    }
}

/// Validates a proxy by testing its connectivity.
#[tauri::command]
pub async fn validate_proxy(
    proxy_manager: State<'_, Arc<ProxyManager>>,
    proxy: FreeProxy,
) -> Result<bool, String> {
    info!("Validating proxy: {}:{}", proxy.ip, proxy.port);
    match proxy_manager.validate_proxy(&proxy).await {
        Ok(is_valid) => Ok(is_valid),
        Err(e) => {
            error!("Failed to validate proxy: {}", e);
            Err(e.to_string())
        }
    }
}

/// Gets proxy statistics.
#[tauri::command]
pub async fn get_proxy_stats(
    proxy_manager: State<'_, Arc<ProxyManager>>,
) -> Result<serde_json::Value, String> {
    debug!("Getting proxy stats");
    match proxy_manager.get_stats().await {
        Ok(stats) => Ok(stats),
        Err(e) => {
            error!("Failed to get proxy stats: {}", e);
            Err(e.to_string())
        }
    }
}
