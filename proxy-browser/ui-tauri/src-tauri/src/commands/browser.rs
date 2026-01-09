//! Browser-related Tauri commands
//! 
//! This module contains all commands for browser control and settings.

use browser_core::{BrowserController, BrowserState, BrowserSettings, WebRtcPolicy};
use serde::{Deserialize, Serialize};
use tauri::State;
use tracing::{info, error, debug};
use std::sync::Arc;

/// Response structure for browser operations.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrowserResponse {
    pub success: bool,
    pub message: String,
}

/// Gets the current browser settings.
#[tauri::command]
pub async fn get_browser_settings(
    browser_controller: State<'_, Arc<BrowserController>>,
) -> Result<BrowserSettings, String> {
    debug!("Getting browser settings");
    match browser_controller.get_settings().await {
        Ok(settings) => Ok(settings),
        Err(e) => {
            error!("Failed to get browser settings: {}", e);
            Err(e.to_string())
        }
    }
}

/// Updates the browser settings.
#[tauri::command]
pub async fn update_browser_settings(
    browser_controller: State<'_, Arc<BrowserController>>,
    settings: BrowserSettings,
) -> Result<BrowserResponse, String> {
    info!("Updating browser settings");
    match browser_controller.update_settings(settings).await {
        Ok(_) => Ok(BrowserResponse {
            success: true,
            message: "Settings updated successfully".to_string(),
        }),
        Err(e) => {
            error!("Failed to update browser settings: {}", e);
            Err(e.to_string())
        }
    }
}

/// Gets the current browser state.
#[tauri::command]
pub async fn get_browser_state(
    browser_controller: State<'_, Arc<BrowserController>>,
) -> Result<BrowserState, String> {
    debug!("Getting browser state");
    match browser_controller.get_state().await {
        Ok(state) => Ok(state),
        Err(e) => {
            error!("Failed to get browser state: {}", e);
            Err(e.to_string())
        }
    }
}

/// Sets the WebRTC policy.
#[tauri::command]
pub async fn set_webrtc_policy(
    browser_controller: State<'_, Arc<BrowserController>>,
    policy: WebRtcPolicy,
) -> Result<BrowserResponse, String> {
    info!("Setting WebRTC policy: {:?}", policy);
    match browser_controller.set_webrtc_policy(policy).await {
        Ok(_) => Ok(BrowserResponse {
            success: true,
            message: "WebRTC policy updated".to_string(),
        }),
        Err(e) => {
            error!("Failed to set WebRTC policy: {}", e);
            Err(e.to_string())
        }
    }
}

/// Clears browser data (cache, cookies, etc.).
#[tauri::command]
pub async fn clear_browser_data(
    browser_controller: State<'_, Arc<BrowserController>>,
    clear_cache: bool,
    clear_cookies: bool,
    clear_history: bool,
    clear_local_storage: bool,
) -> Result<BrowserResponse, String> {
    info!(
        "Clearing browser data - cache: {}, cookies: {}, history: {}, local_storage: {}",
        clear_cache, clear_cookies, clear_history, clear_local_storage
    );
    match browser_controller
        .clear_data(clear_cache, clear_cookies, clear_history, clear_local_storage)
        .await
    {
        Ok(_) => Ok(BrowserResponse {
            success: true,
            message: "Browser data cleared".to_string(),
        }),
        Err(e) => {
            error!("Failed to clear browser data: {}", e);
            Err(e.to_string())
        }
    }
}
