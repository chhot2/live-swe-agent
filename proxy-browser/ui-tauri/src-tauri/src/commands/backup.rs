//! Backup-related Tauri commands
//! 
//! This module contains all commands for backup and restore operations.

use browser_core::{BackupManager, BackupData, BackupOptions, BackupInfo};
use serde::{Deserialize, Serialize};
use tauri::State;
use tracing::{info, error, debug};
use std::sync::Arc;

/// Response structure for backup operations.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackupResponse {
    pub success: bool,
    pub message: String,
    pub backup_id: Option<String>,
}

/// Creates a new backup.
#[tauri::command]
pub async fn create_backup(
    backup_manager: State<'_, Arc<BackupManager>>,
    options: Option<BackupOptions>,
) -> Result<BackupResponse, String> {
    info!("Creating backup with options: {:?}", options);
    match backup_manager.create_backup(options.unwrap_or_default()).await {
        Ok(backup_info) => Ok(BackupResponse {
            success: true,
            message: format!("Backup created: {}", backup_info.filename),
            backup_id: Some(backup_info.id),
        }),
        Err(e) => {
            error!("Failed to create backup: {}", e);
            Err(e.to_string())
        }
    }
}

/// Lists all available backups.
#[tauri::command]
pub async fn list_backups(
    backup_manager: State<'_, Arc<BackupManager>>,
) -> Result<Vec<BackupInfo>, String> {
    debug!("Listing backups");
    match backup_manager.list_backups().await {
        Ok(backups) => Ok(backups),
        Err(e) => {
            error!("Failed to list backups: {}", e);
            Err(e.to_string())
        }
    }
}

/// Restores from a backup.
#[tauri::command]
pub async fn restore_backup(
    backup_manager: State<'_, Arc<BackupManager>>,
    backup_id: String,
    password: Option<String>,
) -> Result<BackupResponse, String> {
    info!("Restoring backup: {}", backup_id);
    match backup_manager.restore_backup(&backup_id, password.as_deref()).await {
        Ok(_) => Ok(BackupResponse {
            success: true,
            message: "Backup restored successfully".to_string(),
            backup_id: Some(backup_id),
        }),
        Err(e) => {
            error!("Failed to restore backup: {}", e);
            Err(e.to_string())
        }
    }
}

/// Deletes a backup.
#[tauri::command]
pub async fn delete_backup(
    backup_manager: State<'_, Arc<BackupManager>>,
    backup_id: String,
) -> Result<BackupResponse, String> {
    info!("Deleting backup: {}", backup_id);
    match backup_manager.delete_backup(&backup_id).await {
        Ok(_) => Ok(BackupResponse {
            success: true,
            message: "Backup deleted".to_string(),
            backup_id: Some(backup_id),
        }),
        Err(e) => {
            error!("Failed to delete backup: {}", e);
            Err(e.to_string())
        }
    }
}

/// Exports backup to a file.
#[tauri::command]
pub async fn export_backup(
    backup_manager: State<'_, Arc<BackupManager>>,
    backup_id: String,
    path: String,
) -> Result<BackupResponse, String> {
    info!("Exporting backup {} to {}", backup_id, path);
    match backup_manager.export_backup(&backup_id, &path).await {
        Ok(_) => Ok(BackupResponse {
            success: true,
            message: format!("Backup exported to {}", path),
            backup_id: Some(backup_id),
        }),
        Err(e) => {
            error!("Failed to export backup: {}", e);
            Err(e.to_string())
        }
    }
}

/// Imports backup from a file.
#[tauri::command]
pub async fn import_backup(
    backup_manager: State<'_, Arc<BackupManager>>,
    path: String,
) -> Result<BackupResponse, String> {
    info!("Importing backup from {}", path);
    match backup_manager.import_backup(&path).await {
        Ok(backup_info) => Ok(BackupResponse {
            success: true,
            message: format!("Backup imported: {}", backup_info.filename),
            backup_id: Some(backup_info.id),
        }),
        Err(e) => {
            error!("Failed to import backup: {}", e);
            Err(e.to_string())
        }
    }
}
