//! Browser Profile Management Module
//!
//! Provides browser profile management including:
//! - Multiple profile support
//! - Profile switching
//! - Profile import/export
//! - Isolated storage per profile

use anyhow::Result;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use tokio::sync::RwLock;
use uuid::Uuid;

/// Browser profile configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrowserProfile {
    pub id: String,
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub last_used: DateTime<Utc>,
    pub data_dir: PathBuf,
    pub settings: ProfileSettings,
    pub is_default: bool,
}

/// Profile-specific settings
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ProfileSettings {
    pub proxy_enabled: bool,
    pub proxy_config: Option<ProfileProxyConfig>,
    pub user_agent: Option<String>,
    pub language: String,
    pub timezone: Option<String>,
    pub geolocation: Option<GeoLocation>,
    pub fingerprint_protection: bool,
}

/// Profile proxy configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileProxyConfig {
    pub host: String,
    pub port: u16,
    pub protocol: String,
    pub username: Option<String>,
    pub password: Option<String>,
}

/// Geolocation settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeoLocation {
    pub latitude: f64,
    pub longitude: f64,
    pub accuracy: f64,
}

/// Browser profile manager
pub struct BrowserProfileManager {
    profiles: RwLock<HashMap<String, BrowserProfile>>,
    active_profile_id: RwLock<Option<String>>,
    base_data_dir: PathBuf,
}

impl BrowserProfileManager {
    /// Create a new profile manager
    pub fn new(base_data_dir: PathBuf) -> Self {
        Self {
            profiles: RwLock::new(HashMap::new()),
            active_profile_id: RwLock::new(None),
            base_data_dir,
        }
    }

    /// Create a new profile
    pub async fn create_profile(&self, name: &str, is_default: bool) -> Result<BrowserProfile> {
        let id = Uuid::new_v4().to_string();
        let now = Utc::now();
        let data_dir = self.base_data_dir.join(&id);

        // Create profile directory
        tokio::fs::create_dir_all(&data_dir).await?;

        let profile = BrowserProfile {
            id: id.clone(),
            name: name.to_string(),
            created_at: now,
            last_used: now,
            data_dir,
            settings: ProfileSettings::default(),
            is_default,
        };

        self.profiles.write().await.insert(id, profile.clone());
        Ok(profile)
    }

    /// Get a profile by ID
    pub async fn get_profile(&self, profile_id: &str) -> Option<BrowserProfile> {
        self.profiles.read().await.get(profile_id).cloned()
    }

    /// Get all profiles
    pub async fn list_profiles(&self) -> Vec<BrowserProfile> {
        self.profiles.read().await.values().cloned().collect()
    }

    /// Switch to a profile
    pub async fn switch_profile(&self, profile_id: &str) -> Result<()> {
        let mut profiles = self.profiles.write().await;
        if let Some(profile) = profiles.get_mut(profile_id) {
            profile.last_used = Utc::now();
            *self.active_profile_id.write().await = Some(profile_id.to_string());
            Ok(())
        } else {
            Err(anyhow::anyhow!("Profile not found"))
        }
    }

    /// Delete a profile
    pub async fn delete_profile(&self, profile_id: &str) -> Result<()> {
        let mut profiles = self.profiles.write().await;
        if let Some(profile) = profiles.remove(profile_id) {
            // Remove profile directory
            if profile.data_dir.exists() {
                tokio::fs::remove_dir_all(&profile.data_dir).await?;
            }
        }
        Ok(())
    }

    /// Update profile settings
    pub async fn update_settings(&self, profile_id: &str, settings: ProfileSettings) -> Result<()> {
        let mut profiles = self.profiles.write().await;
        if let Some(profile) = profiles.get_mut(profile_id) {
            profile.settings = settings;
            Ok(())
        } else {
            Err(anyhow::anyhow!("Profile not found"))
        }
    }

    /// Export profile to JSON
    pub async fn export_profile(&self, profile_id: &str) -> Result<String> {
        let profiles = self.profiles.read().await;
        if let Some(profile) = profiles.get(profile_id) {
            Ok(serde_json::to_string_pretty(profile)?)
        } else {
            Err(anyhow::anyhow!("Profile not found"))
        }
    }

    /// Import profile from JSON
    pub async fn import_profile(&self, json: &str) -> Result<BrowserProfile> {
        let mut profile: BrowserProfile = serde_json::from_str(json)?;
        profile.id = Uuid::new_v4().to_string();
        profile.data_dir = self.base_data_dir.join(&profile.id);

        tokio::fs::create_dir_all(&profile.data_dir).await?;
        self.profiles
            .write()
            .await
            .insert(profile.id.clone(), profile.clone());

        Ok(profile)
    }

    /// Get active profile
    pub async fn get_active_profile(&self) -> Option<BrowserProfile> {
        let active_id = self.active_profile_id.read().await;
        if let Some(id) = active_id.as_ref() {
            self.profiles.read().await.get(id).cloned()
        } else {
            None
        }
    }
}

impl Default for BrowserProfileManager {
    fn default() -> Self {
        Self::new(PathBuf::from("./profiles"))
    }
}
