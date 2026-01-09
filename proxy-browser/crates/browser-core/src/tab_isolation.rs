use serde::{Deserialize, Serialize};
use std::time::SystemTime;

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Enumeration of TabStatus variants.
pub enum TabStatus {
    Creating,
    Active,
    Closed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Represents a NetworkConfig.
pub struct NetworkConfig {
    pub proxy_url: Option<String>,
    pub dns_servers: Vec<String>,
    pub tls_profile: TLSProfile,
    pub http2_settings: HTTP2Settings,
    pub tcp_fingerprint: TCPFingerprint,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Represents a TLSProfile.
pub struct TLSProfile {
    pub version: String,
    pub cipher_suites: Vec<String>,
    pub extensions: Vec<String>,
    pub ja3_hash: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Represents a HTTP2Settings.
pub struct HTTP2Settings {
    pub settings_frame: Vec<(u32, u32)>,
    pub window_update: u32,
    pub priority: Vec<(u32, u32)>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Represents a TCPFingerprint.
pub struct TCPFingerprint {
    pub ttl: u8,
    pub window_size: u32,
    pub options: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Represents a TabProfile.
pub struct TabProfile {
    pub tab_id: String,
    pub virtual_ip: virtual_ip::VirtualIP,
    pub fingerprint: crate::fingerprint::BrowserFingerprint,
    pub network_config: NetworkConfig,
    pub storage_path: String,
    pub process_id: Option<u32>,
    #[serde(with = "serde_systemtime")]
    pub created_at: SystemTime,
    #[serde(with = "serde_systemtime")]
    pub last_active: SystemTime,
    pub status: TabStatus,
}

mod serde_systemtime {
    use serde::{Deserialize, Deserializer, Serialize, Serializer};
    use std::time::{Duration, SystemTime, UNIX_EPOCH};

    /// Performs serialize operation.
    pub fn serialize<S>(time: &SystemTime, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let duration = time.duration_since(UNIX_EPOCH).unwrap_or(Duration::ZERO);
        duration.as_secs().serialize(serializer)
    }

    /// Performs deserialize operation.
    pub fn deserialize<'de, D>(deserializer: D) -> Result<SystemTime, D::Error>
    where
        D: Deserializer<'de>,
    {
        let secs = u64::deserialize(deserializer)?;
        Ok(UNIX_EPOCH + Duration::from_secs(secs))
    }
}
