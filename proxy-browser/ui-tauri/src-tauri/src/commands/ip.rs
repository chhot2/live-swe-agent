//! IP-related Tauri commands
//! 
//! This module contains all commands for IP detection and virtual IP generation.

use browser_core::{PublicIpDetector, PublicIpInfo, FreeIpProviderManager};
use virtual_ip::{IPGenerator, VirtualIP};
use serde::{Deserialize, Serialize};
use tauri::State;
use tracing::{info, error, debug};
use std::sync::Arc;

/// Response structure for tab creation with virtual IP.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TabResponse {
    pub tab_id: String,
    pub ip: String,
    pub country_code: String,
    pub country_name: String,
    pub city: String,
    pub timezone: String,
    pub isp: String,
}

/// Detects the current public IP address.
#[tauri::command]
pub async fn detect_public_ip() -> Result<PublicIpInfo, String> {
    info!("Detecting public IP");
    match PublicIpDetector::detect().await {
        Ok(ip_info) => {
            info!("Detected public IP: {}", ip_info.ip);
            Ok(ip_info)
        }
        Err(e) => {
            error!("Failed to detect public IP: {}", e);
            Err(e.to_string())
        }
    }
}

/// Generates a random virtual IP.
#[tauri::command]
pub async fn generate_virtual_ip(
    ip_generator: State<'_, Arc<IPGenerator>>,
) -> Result<VirtualIP, String> {
    debug!("Generating random virtual IP");
    match ip_generator.generate_random() {
        Ok(virtual_ip) => {
            info!("Generated virtual IP: {}", virtual_ip.ip);
            Ok(virtual_ip)
        }
        Err(e) => {
            error!("Failed to generate virtual IP: {}", e);
            Err(e.to_string())
        }
    }
}

/// Generates a virtual IP for a specific country.
#[tauri::command]
pub async fn generate_virtual_ip_for_country(
    ip_generator: State<'_, Arc<IPGenerator>>,
    country_code: String,
) -> Result<VirtualIP, String> {
    info!("Generating virtual IP for country: {}", country_code);
    match ip_generator.generate_for_country(&country_code) {
        Ok(virtual_ip) => {
            info!("Generated virtual IP: {} for {}", virtual_ip.ip, country_code);
            Ok(virtual_ip)
        }
        Err(e) => {
            error!("Failed to generate virtual IP for {}: {}", country_code, e);
            Err(e.to_string())
        }
    }
}

/// Gets the list of available countries for IP generation.
#[tauri::command]
pub async fn get_available_countries(
    ip_generator: State<'_, Arc<IPGenerator>>,
) -> Result<Vec<String>, String> {
    debug!("Getting available countries");
    Ok(ip_generator.get_available_countries())
}

/// Gets IP providers status.
#[tauri::command]
pub async fn get_ip_providers_status(
    provider_manager: State<'_, Arc<FreeIpProviderManager>>,
) -> Result<serde_json::Value, String> {
    debug!("Getting IP providers status");
    match provider_manager.get_status().await {
        Ok(status) => Ok(status),
        Err(e) => {
            error!("Failed to get IP providers status: {}", e);
            Err(e.to_string())
        }
    }
}

/// Refreshes IP providers.
#[tauri::command]
pub async fn refresh_ip_providers(
    provider_manager: State<'_, Arc<FreeIpProviderManager>>,
) -> Result<serde_json::Value, String> {
    info!("Refreshing IP providers");
    match provider_manager.refresh_all().await {
        Ok(result) => Ok(result),
        Err(e) => {
            error!("Failed to refresh IP providers: {}", e);
            Err(e.to_string())
        }
    }
}
