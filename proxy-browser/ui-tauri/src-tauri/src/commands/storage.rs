//! Storage-related Tauri commands
//! 
//! This module contains all commands for managing stored data like cookies, history, and bookmarks.

use browser_core::{StorageEngine, Cookie, HistoryEntry, Bookmark};
use serde::{Deserialize, Serialize};
use tauri::State;
use tracing::{info, error, debug};
use std::sync::Arc;

/// Response structure for storage operations.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageResponse {
    pub success: bool,
    pub message: String,
}

/// Gets all cookies.
#[tauri::command]
pub async fn get_cookies(
    storage_engine: State<'_, Arc<StorageEngine>>,
) -> Result<Vec<Cookie>, String> {
    debug!("Getting cookies");
    match storage_engine.get_cookies().await {
        Ok(cookies) => Ok(cookies),
        Err(e) => {
            error!("Failed to get cookies: {}", e);
            Err(e.to_string())
        }
    }
}

/// Deletes a cookie.
#[tauri::command]
pub async fn delete_cookie(
    storage_engine: State<'_, Arc<StorageEngine>>,
    domain: String,
    name: String,
) -> Result<StorageResponse, String> {
    info!("Deleting cookie: {} from {}", name, domain);
    match storage_engine.delete_cookie(&domain, &name).await {
        Ok(_) => Ok(StorageResponse {
            success: true,
            message: "Cookie deleted".to_string(),
        }),
        Err(e) => {
            error!("Failed to delete cookie: {}", e);
            Err(e.to_string())
        }
    }
}

/// Gets browsing history.
#[tauri::command]
pub async fn get_history(
    storage_engine: State<'_, Arc<StorageEngine>>,
    limit: Option<usize>,
) -> Result<Vec<HistoryEntry>, String> {
    debug!("Getting history with limit: {:?}", limit);
    match storage_engine.get_history(limit.unwrap_or(100)).await {
        Ok(history) => Ok(history),
        Err(e) => {
            error!("Failed to get history: {}", e);
            Err(e.to_string())
        }
    }
}

/// Adds a history entry.
#[tauri::command]
pub async fn add_history_entry(
    storage_engine: State<'_, Arc<StorageEngine>>,
    url: String,
    title: Option<String>,
) -> Result<StorageResponse, String> {
    debug!("Adding history entry: {}", url);
    match storage_engine.add_history_entry(&url, title.as_deref()).await {
        Ok(_) => Ok(StorageResponse {
            success: true,
            message: "History entry added".to_string(),
        }),
        Err(e) => {
            error!("Failed to add history entry: {}", e);
            Err(e.to_string())
        }
    }
}

/// Deletes a history entry.
#[tauri::command]
pub async fn delete_history_entry(
    storage_engine: State<'_, Arc<StorageEngine>>,
    id: String,
) -> Result<StorageResponse, String> {
    info!("Deleting history entry: {}", id);
    match storage_engine.delete_history_entry(&id).await {
        Ok(_) => Ok(StorageResponse {
            success: true,
            message: "History entry deleted".to_string(),
        }),
        Err(e) => {
            error!("Failed to delete history entry: {}", e);
            Err(e.to_string())
        }
    }
}

/// Clears all history.
#[tauri::command]
pub async fn clear_history(
    storage_engine: State<'_, Arc<StorageEngine>>,
) -> Result<StorageResponse, String> {
    info!("Clearing all history");
    match storage_engine.clear_history().await {
        Ok(_) => Ok(StorageResponse {
            success: true,
            message: "History cleared".to_string(),
        }),
        Err(e) => {
            error!("Failed to clear history: {}", e);
            Err(e.to_string())
        }
    }
}

/// Gets all bookmarks.
#[tauri::command]
pub async fn get_bookmarks(
    storage_engine: State<'_, Arc<StorageEngine>>,
) -> Result<Vec<Bookmark>, String> {
    debug!("Getting bookmarks");
    match storage_engine.get_bookmarks().await {
        Ok(bookmarks) => Ok(bookmarks),
        Err(e) => {
            error!("Failed to get bookmarks: {}", e);
            Err(e.to_string())
        }
    }
}

/// Adds a bookmark.
#[tauri::command]
pub async fn add_bookmark(
    storage_engine: State<'_, Arc<StorageEngine>>,
    url: String,
    title: String,
    folder: Option<String>,
) -> Result<StorageResponse, String> {
    info!("Adding bookmark: {} - {}", title, url);
    match storage_engine.add_bookmark(&url, &title, folder.as_deref()).await {
        Ok(_) => Ok(StorageResponse {
            success: true,
            message: "Bookmark added".to_string(),
        }),
        Err(e) => {
            error!("Failed to add bookmark: {}", e);
            Err(e.to_string())
        }
    }
}

/// Updates a bookmark.
#[tauri::command]
pub async fn update_bookmark(
    storage_engine: State<'_, Arc<StorageEngine>>,
    id: String,
    url: Option<String>,
    title: Option<String>,
    folder: Option<String>,
) -> Result<StorageResponse, String> {
    info!("Updating bookmark: {}", id);
    match storage_engine.update_bookmark(&id, url.as_deref(), title.as_deref(), folder.as_deref()).await {
        Ok(_) => Ok(StorageResponse {
            success: true,
            message: "Bookmark updated".to_string(),
        }),
        Err(e) => {
            error!("Failed to update bookmark: {}", e);
            Err(e.to_string())
        }
    }
}

/// Deletes a bookmark.
#[tauri::command]
pub async fn delete_bookmark(
    storage_engine: State<'_, Arc<StorageEngine>>,
    id: String,
) -> Result<StorageResponse, String> {
    info!("Deleting bookmark: {}", id);
    match storage_engine.delete_bookmark(&id).await {
        Ok(_) => Ok(StorageResponse {
            success: true,
            message: "Bookmark deleted".to_string(),
        }),
        Err(e) => {
            error!("Failed to delete bookmark: {}", e);
            Err(e.to_string())
        }
    }
}
