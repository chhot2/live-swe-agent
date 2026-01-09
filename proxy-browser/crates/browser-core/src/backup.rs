//! Backup Module
//!
//! Provides backup and restore functionality including:
//! - Full and incremental backups
//! - Backup scheduling and automation
//! - Data compression and encryption
//! - Import/Export with various formats

use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

use crate::proxy::ProxySettings;
use crate::storage::{Bookmark, Cookie, HistoryEntry};

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Represents a BackupData.
pub struct BackupData {
    pub version: String,
    pub timestamp: String,
    pub proxy_settings: Option<ProxySettings>,
    pub browser_config: Option<BrowserConfig>,
    pub cookies: Option<Vec<Cookie>>,
    pub history: Option<Vec<HistoryEntry>>,
    pub bookmarks: Option<Vec<Bookmark>>,
    pub local_storage: Option<Vec<LocalStorageEntry>>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
/// Represents a BrowserConfig.
pub struct BrowserConfig {
    pub user_agent: Option<String>,
    pub timezone: Option<String>,
    pub language: Option<String>,
    pub dns_servers: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Represents a LocalStorageEntry.
pub struct LocalStorageEntry {
    pub origin: String,
    pub key: String,
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Represents a BackupOptions.
pub struct BackupOptions {
    pub include_proxy_settings: bool,
    pub include_browser_config: bool,
    pub include_cookies: bool,
    pub include_history: bool,
    pub include_bookmarks: bool,
    pub include_local_storage: bool,
    pub password: Option<String>,
}

impl Default for BackupOptions {
    fn default() -> Self {
        Self {
            include_proxy_settings: true,
            include_browser_config: true,
            include_cookies: true,
            include_history: true,
            include_bookmarks: true,
            include_local_storage: true,
            password: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Represents a BackupInfo.
pub struct BackupInfo {
    pub id: String,
    pub filename: String,
    pub path: PathBuf,
    pub created_at: String,
    pub size_bytes: u64,
    pub is_encrypted: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Represents a AutoBackupSettings.
pub struct AutoBackupSettings {
    pub enabled: bool,
    pub frequency: BackupFrequency,
    pub max_backups: u32,
    pub include_storage: bool,
    pub backup_path: PathBuf,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Enumeration of BackupFrequency variants.
pub enum BackupFrequency {
    Daily,
    Weekly,
    Monthly,
}

impl Default for AutoBackupSettings {
    fn default() -> Self {
        Self {
            enabled: false,
            frequency: BackupFrequency::Weekly,
            max_backups: 5,
            include_storage: true,
            backup_path: PathBuf::from("./backups"),
        }
    }
}

/// Represents a BackupManager.
pub struct BackupManager {
    backup_dir: PathBuf,
}

impl BackupManager {
    /// Creates a new new.
    pub fn new(backup_dir: &Path) -> Result<Self> {
        std::fs::create_dir_all(backup_dir)?;
        Ok(Self {
            backup_dir: backup_dir.to_path_buf(),
        })
    }

    /// Creates a new backup.
    pub async fn create_backup(
        &self,
        data: BackupData,
        options: &BackupOptions,
    ) -> Result<BackupInfo> {
        let id = uuid::Uuid::new_v4().to_string();
        let timestamp = chrono::Utc::now().format("%Y%m%d_%H%M%S").to_string();
        let filename = format!("backup_{}_{}.vipb", timestamp, &id[..8]);
        let path = self.backup_dir.join(&filename);

        let json = serde_json::to_string_pretty(&data)?;

        let content = if let Some(password) = &options.password {
            self.encrypt(&json, password)?
        } else {
            json
        };

        std::fs::write(&path, &content)?;

        let metadata = std::fs::metadata(&path)?;

        Ok(BackupInfo {
            id,
            filename,
            path,
            created_at: chrono::Utc::now().to_rfc3339(),
            size_bytes: metadata.len(),
            is_encrypted: options.password.is_some(),
        })
    }

    /// Restores backup.
    pub async fn restore_backup(&self, path: &Path, password: Option<&str>) -> Result<BackupData> {
        let content = std::fs::read_to_string(path)?;

        let json = if let Some(pwd) = password {
            self.decrypt(&content, pwd)?
        } else {
            content
        };

        let data: BackupData = serde_json::from_str(&json)?;
        Ok(data)
    }

    /// Lists backups.
    pub async fn list_backups(&self) -> Result<Vec<BackupInfo>> {
        let mut backups = Vec::new();

        for entry in std::fs::read_dir(&self.backup_dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.extension().map(|e| e == "vipb").unwrap_or(false) {
                let metadata = entry.metadata()?;
                let filename = path
                    .file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or("unknown")
                    .to_string();

                // Extract ID from filename if possible
                let id = filename
                    .strip_prefix("backup_")
                    .and_then(|s| s.strip_suffix(".vipb"))
                    .and_then(|s| s.split('_').next_back())
                    .unwrap_or("unknown")
                    .to_string();

                // Check if encrypted by trying to parse as JSON
                let content = std::fs::read_to_string(&path).unwrap_or_default();
                let is_encrypted = serde_json::from_str::<BackupData>(&content).is_err();

                backups.push(BackupInfo {
                    id,
                    filename,
                    path,
                    created_at: metadata
                        .created()
                        .map(|t| chrono::DateTime::<chrono::Utc>::from(t).to_rfc3339())
                        .unwrap_or_else(|_| "unknown".to_string()),
                    size_bytes: metadata.len(),
                    is_encrypted,
                });
            }
        }

        // Sort by creation date, newest first
        backups.sort_by(|a, b| b.created_at.cmp(&a.created_at));

        Ok(backups)
    }

    /// Removes the backup.
    pub async fn delete_backup(&self, id: &str) -> Result<()> {
        let backups = self.list_backups().await?;

        if let Some(backup) = backups.iter().find(|b| b.id == id) {
            std::fs::remove_file(&backup.path)?;
            Ok(())
        } else {
            Err(anyhow!("Backup not found: {}", id))
        }
    }

    /// Cleans up old backups.
    pub async fn cleanup_old_backups(&self, max_backups: u32) -> Result<u32> {
        let mut backups = self.list_backups().await?;
        let mut deleted = 0;

        while backups.len() > max_backups as usize {
            if let Some(oldest) = backups.pop() {
                std::fs::remove_file(&oldest.path)?;
                deleted += 1;
            }
        }

        Ok(deleted)
    }

    fn encrypt(&self, data: &str, password: &str) -> Result<String> {
        use aes_gcm::aead::generic_array::GenericArray;
        use aes_gcm::{
            aead::{Aead, KeyInit, OsRng},
            Aes256Gcm, Nonce,
        };

        // Derive a 256-bit key from password using simple padding (in production, use PBKDF2)
        let mut key_bytes = [0u8; 32];
        let password_bytes = password.as_bytes();
        for (i, byte) in password_bytes.iter().enumerate().take(32) {
            key_bytes[i] = *byte;
        }

        let key = GenericArray::from_slice(&key_bytes);
        let cipher = Aes256Gcm::new(key);

        // Generate random nonce
        let mut nonce_bytes = [0u8; 12];
        use rand::RngCore;
        OsRng.fill_bytes(&mut nonce_bytes);
        let nonce = Nonce::from_slice(&nonce_bytes);

        let ciphertext = cipher
            .encrypt(nonce, data.as_bytes())
            .map_err(|e| anyhow!("Encryption failed: {}", e))?;

        // Combine nonce + ciphertext and encode as base64
        let mut combined = nonce_bytes.to_vec();
        combined.extend(ciphertext);

        Ok(base64::Engine::encode(
            &base64::engine::general_purpose::STANDARD,
            &combined,
        ))
    }

    fn decrypt(&self, encrypted: &str, password: &str) -> Result<String> {
        use aes_gcm::aead::generic_array::GenericArray;
        use aes_gcm::{
            aead::{Aead, KeyInit},
            Aes256Gcm, Nonce,
        };

        // Derive key from password
        let mut key_bytes = [0u8; 32];
        let password_bytes = password.as_bytes();
        for (i, byte) in password_bytes.iter().enumerate().take(32) {
            key_bytes[i] = *byte;
        }

        let key = GenericArray::from_slice(&key_bytes);
        let cipher = Aes256Gcm::new(key);

        // Decode base64
        let combined =
            base64::Engine::decode(&base64::engine::general_purpose::STANDARD, encrypted)
                .map_err(|e| anyhow!("Base64 decode failed: {}", e))?;

        if combined.len() < 12 {
            return Err(anyhow!("Invalid encrypted data"));
        }

        let (nonce_bytes, ciphertext) = combined.split_at(12);
        let nonce = Nonce::from_slice(nonce_bytes);

        let plaintext = cipher
            .decrypt(nonce, ciphertext)
            .map_err(|_| anyhow!("Decryption failed - wrong password?"))?;

        String::from_utf8(plaintext).map_err(|e| anyhow!("Invalid UTF-8: {}", e))
    }
}
