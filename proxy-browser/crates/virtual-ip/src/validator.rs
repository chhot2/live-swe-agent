use crate::models::VirtualIP;
use anyhow::Result;
use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[allow(dead_code)]
struct IPInfoResponse {
    ip: String,
    country: String,
    city: String,
    region: String,
    timezone: String,
}

/// Represents a IPValidator.
pub struct IPValidator {
    #[allow(dead_code)]
    client: Client,
}

impl Default for IPValidator {
    fn default() -> Self {
        Self::new()
    }
}

impl IPValidator {
    /// Creates a new new.
    pub fn new() -> Self {
        Self {
            client: Client::new(),
        }
    }

    /// Validate IP by checking external service
    pub async fn validate(&self, virtual_ip: &VirtualIP) -> Result<bool> {
        // Hit ipinfo.io to check current public IP
        let response = self.client.get("https://ipinfo.io/json").send().await?;

        let ip_info: IPInfoResponse = response.json().await?;

        // Check if the detected IP matches our virtual IP
        Ok(ip_info.ip == virtual_ip.ip.to_string())
    }

    /// Check for WebRTC leaks (stubbed)
    pub async fn check_webrtc_leak(&self) -> Result<Vec<String>> {
        Ok(vec![])
    }

    /// Check DNS leak (stubbed)
    pub async fn check_dns_leak(&self) -> Result<bool> {
        Ok(true)
    }

    /// Comprehensive validation
    pub async fn validate_comprehensive(&self, virtual_ip: &VirtualIP) -> Result<ValidationReport> {
        let ip_matches = self.validate(virtual_ip).await?;
        let webrtc_leaks = self.check_webrtc_leak().await?;
        let dns_secure = self.check_dns_leak().await?;

        Ok(ValidationReport {
            ip_matches,
            webrtc_leaks: webrtc_leaks.is_empty(),
            dns_secure,
            overall_pass: ip_matches && webrtc_leaks.is_empty() && dns_secure,
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
/// Represents a ValidationReport.
pub struct ValidationReport {
    pub ip_matches: bool,
    pub webrtc_leaks: bool,
    pub dns_secure: bool,
    pub overall_pass: bool,
}
