//! Browser Controls Module
//!
//! Provides browser control functionality including:
//! - Navigation controls (back, forward, reload)
//! - Page state management
//! - Download management
//! - Context menu handling

#[cfg(feature = "chromium")]
use crate::chromium_engine::BrowserEngineType;

#[cfg(not(feature = "chromium"))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum BrowserEngineType {
    #[default]
    System,
    IntegratedChromium,
}
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Represents a BrowserState.
pub struct BrowserState {
    pub tab_id: String,
    pub current_url: String,
    pub title: String,
    pub can_go_back: bool,
    pub can_go_forward: bool,
    pub is_loading: bool,
    pub history: Vec<HistoryItem>,
    pub history_index: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Represents a HistoryItem.
pub struct HistoryItem {
    pub url: String,
    pub title: String,
    pub timestamp: i64,
}

impl Default for BrowserState {
    fn default() -> Self {
        Self {
            tab_id: String::new(),
            current_url: "about:blank".to_string(),
            title: "New Tab".to_string(),
            can_go_back: false,
            can_go_forward: false,
            is_loading: false,
            history: Vec::new(),
            history_index: -1,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Represents a BrowserSettings.
pub struct BrowserSettings {
    pub user_agent: String,
    pub language: String,
    pub timezone: String,
    pub webrtc_policy: WebRtcPolicy,
    pub dns_over_https: bool,
    pub block_trackers: bool,
    pub block_ads: bool,
    pub javascript_enabled: bool,
    pub cookies_enabled: bool,
    /// Browser engine type: System (Tauri webview) or IntegratedChromium
    pub engine_type: BrowserEngineType,
    /// Enable stealth mode for integrated Chromium engine
    pub stealth_mode: bool,
    /// Run integrated Chromium in headless mode
    pub headless_mode: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
/// Enumeration of WebRtcPolicy variants.
#[derive(Default)]
pub enum WebRtcPolicy {
    Default,
    #[default]
    DisableNonProxiedUdp,
    Disabled,
}

impl Default for BrowserSettings {
    fn default() -> Self {
        Self {
            user_agent: "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36".to_string(),
            language: "en-US".to_string(),
            timezone: "America/New_York".to_string(),
            webrtc_policy: WebRtcPolicy::DisableNonProxiedUdp,
            dns_over_https: true,
            block_trackers: true,
            block_ads: false,
            javascript_enabled: true,
            cookies_enabled: true,
            engine_type: BrowserEngineType::System,
            stealth_mode: true,
            headless_mode: false,
        }
    }
}

/// Represents a BrowserController.
pub struct BrowserController {
    states: Arc<RwLock<HashMap<String, BrowserState>>>,
    settings: Arc<RwLock<BrowserSettings>>,
}

impl BrowserController {
    /// Creates a new new.
    pub fn new() -> Self {
        Self {
            states: Arc::new(RwLock::new(HashMap::new())),
            settings: Arc::new(RwLock::new(BrowserSettings::default())),
        }
    }

    /// Creates a new browser state.
    pub async fn create_browser_state(&self, tab_id: &str) -> BrowserState {
        let state = BrowserState {
            tab_id: tab_id.to_string(),
            ..Default::default()
        };
        self.states
            .write()
            .await
            .insert(tab_id.to_string(), state.clone());
        state
    }

    /// Gets the state.
    pub async fn get_state(&self, tab_id: &str) -> Option<BrowserState> {
        self.states.read().await.get(tab_id).cloned()
    }

    /// Performs navigate operation.
    pub async fn navigate(&self, tab_id: &str, url: &str) -> Result<BrowserState> {
        let mut states = self.states.write().await;
        let state = states
            .entry(tab_id.to_string())
            .or_insert_with(|| BrowserState {
                tab_id: tab_id.to_string(),
                ..Default::default()
            });

        // Trim history after current index
        if state.history_index >= 0 {
            state.history.truncate((state.history_index + 1) as usize);
        }

        // Add to history
        state.history.push(HistoryItem {
            url: url.to_string(),
            title: String::new(),
            timestamp: chrono::Utc::now().timestamp(),
        });
        state.history_index = (state.history.len() - 1) as i32;

        state.current_url = url.to_string();
        state.can_go_back = state.history_index > 0;
        state.can_go_forward = false;
        state.is_loading = true;

        Ok(state.clone())
    }

    /// Performs go back operation.
    pub async fn go_back(&self, tab_id: &str) -> Result<Option<String>> {
        let mut states = self.states.write().await;
        if let Some(state) = states.get_mut(tab_id) {
            if state.history_index > 0 {
                state.history_index -= 1;
                let url = state.history[state.history_index as usize].url.clone();
                state.current_url = url.clone();
                state.can_go_back = state.history_index > 0;
                state.can_go_forward = true;
                return Ok(Some(url));
            }
        }
        Ok(None)
    }

    /// Performs go forward operation.
    pub async fn go_forward(&self, tab_id: &str) -> Result<Option<String>> {
        let mut states = self.states.write().await;
        if let Some(state) = states.get_mut(tab_id) {
            if (state.history_index as usize) < state.history.len() - 1 {
                state.history_index += 1;
                let url = state.history[state.history_index as usize].url.clone();
                state.current_url = url.clone();
                state.can_go_back = true;
                state.can_go_forward = (state.history_index as usize) < state.history.len() - 1;
                return Ok(Some(url));
            }
        }
        Ok(None)
    }

    /// Performs reload operation.
    pub async fn reload(&self, tab_id: &str) -> Result<Option<String>> {
        let states = self.states.read().await;
        if let Some(state) = states.get(tab_id) {
            return Ok(Some(state.current_url.clone()));
        }
        Ok(None)
    }

    /// Stops the loading.
    pub async fn stop_loading(&self, tab_id: &str) {
        let mut states = self.states.write().await;
        if let Some(state) = states.get_mut(tab_id) {
            state.is_loading = false;
        }
    }

    /// Updates the title.
    pub async fn update_title(&self, tab_id: &str, title: &str) {
        let mut states = self.states.write().await;
        if let Some(state) = states.get_mut(tab_id) {
            state.title = title.to_string();
            // Update history item title
            if state.history_index >= 0 && (state.history_index as usize) < state.history.len() {
                state.history[state.history_index as usize].title = title.to_string();
            }
        }
    }

    /// Sets the loading.
    pub async fn set_loading(&self, tab_id: &str, loading: bool) {
        let mut states = self.states.write().await;
        if let Some(state) = states.get_mut(tab_id) {
            state.is_loading = loading;
        }
    }

    /// Closes tab.
    pub async fn close_tab(&self, tab_id: &str) {
        self.states.write().await.remove(tab_id);
    }

    /// Gets the settings.
    pub async fn get_settings(&self) -> BrowserSettings {
        self.settings.read().await.clone()
    }

    /// Sets the settings.
    pub async fn set_settings(&self, settings: BrowserSettings) {
        *self.settings.write().await = settings;
    }

    /// Gets the all states.
    pub async fn get_all_states(&self) -> Vec<BrowserState> {
        self.states.read().await.values().cloned().collect()
    }
}

impl Default for BrowserController {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// Download Manager Integration
// ============================================================================

use std::path::PathBuf;

/// Download state enumeration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
/// Enumeration of DownloadState variants.
pub enum DownloadState {
    Pending,
    InProgress,
    Paused,
    Completed,
    Failed,
    Cancelled,
}

/// Represents a download item
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Represents a DownloadItem.
pub struct DownloadItem {
    pub id: String,
    pub url: String,
    pub filename: String,
    pub save_path: PathBuf,
    pub total_bytes: Option<u64>,
    pub received_bytes: u64,
    pub state: DownloadState,
    pub mime_type: Option<String>,
    pub started_at: i64,
    pub completed_at: Option<i64>,
    pub error: Option<String>,
    pub tab_id: Option<String>,
}

impl DownloadItem {
    /// Calculate download progress as percentage
    pub fn progress_percent(&self) -> f64 {
        match self.total_bytes {
            Some(total) if total > 0 => (self.received_bytes as f64 / total as f64) * 100.0,
            _ => 0.0,
        }
    }

    /// Check if download is active
    pub fn is_active(&self) -> bool {
        matches!(
            self.state,
            DownloadState::Pending | DownloadState::InProgress
        )
    }
}

/// Download manager for handling file downloads
pub struct DownloadManager {
    downloads: Arc<RwLock<HashMap<String, DownloadItem>>>,
    download_dir: PathBuf,
    max_concurrent_downloads: usize,
}

impl DownloadManager {
    /// Creates a new new.
    pub fn new(download_dir: PathBuf) -> Self {
        Self {
            downloads: Arc::new(RwLock::new(HashMap::new())),
            download_dir,
            max_concurrent_downloads: 5,
        }
    }

    /// Configures with max concurrent.
    pub fn with_max_concurrent(mut self, max: usize) -> Self {
        self.max_concurrent_downloads = max;
        self
    }

    /// Start a new download
    pub async fn start_download(
        &self,
        url: &str,
        filename: Option<&str>,
        tab_id: Option<&str>,
    ) -> Result<String> {
        let download_id = uuid::Uuid::new_v4().to_string();

        let filename = filename
            .map(|f| f.to_string())
            .unwrap_or_else(|| self.extract_filename_from_url(url));

        let save_path = self.download_dir.join(&filename);

        let item = DownloadItem {
            id: download_id.clone(),
            url: url.to_string(),
            filename,
            save_path,
            total_bytes: None,
            received_bytes: 0,
            state: DownloadState::Pending,
            mime_type: None,
            started_at: chrono::Utc::now().timestamp(),
            completed_at: None,
            error: None,
            tab_id: tab_id.map(|s| s.to_string()),
        };

        self.downloads
            .write()
            .await
            .insert(download_id.clone(), item);

        Ok(download_id)
    }

    /// Pause a download
    pub async fn pause_download(&self, download_id: &str) -> Result<()> {
        let mut downloads = self.downloads.write().await;
        if let Some(item) = downloads.get_mut(download_id) {
            if item.state == DownloadState::InProgress {
                item.state = DownloadState::Paused;
            }
        }
        Ok(())
    }

    /// Resume a paused download
    pub async fn resume_download(&self, download_id: &str) -> Result<()> {
        let mut downloads = self.downloads.write().await;
        if let Some(item) = downloads.get_mut(download_id) {
            if item.state == DownloadState::Paused {
                item.state = DownloadState::InProgress;
            }
        }
        Ok(())
    }

    /// Cancel a download
    pub async fn cancel_download(&self, download_id: &str) -> Result<()> {
        let mut downloads = self.downloads.write().await;
        if let Some(item) = downloads.get_mut(download_id) {
            item.state = DownloadState::Cancelled;
        }
        Ok(())
    }

    /// Update download progress
    pub async fn update_progress(
        &self,
        download_id: &str,
        received_bytes: u64,
        total_bytes: Option<u64>,
    ) {
        let mut downloads = self.downloads.write().await;
        if let Some(item) = downloads.get_mut(download_id) {
            item.received_bytes = received_bytes;
            if total_bytes.is_some() {
                item.total_bytes = total_bytes;
            }
            item.state = DownloadState::InProgress;
        }
    }

    /// Mark download as completed
    pub async fn complete_download(&self, download_id: &str) {
        let mut downloads = self.downloads.write().await;
        if let Some(item) = downloads.get_mut(download_id) {
            item.state = DownloadState::Completed;
            item.completed_at = Some(chrono::Utc::now().timestamp());
            if let Some(total) = item.total_bytes {
                item.received_bytes = total;
            }
        }
    }

    /// Mark download as failed
    pub async fn fail_download(&self, download_id: &str, error: &str) {
        let mut downloads = self.downloads.write().await;
        if let Some(item) = downloads.get_mut(download_id) {
            item.state = DownloadState::Failed;
            item.error = Some(error.to_string());
        }
    }

    /// Get a download by ID
    pub async fn get_download(&self, download_id: &str) -> Option<DownloadItem> {
        self.downloads.read().await.get(download_id).cloned()
    }

    /// Get all downloads
    pub async fn get_all_downloads(&self) -> Vec<DownloadItem> {
        self.downloads.read().await.values().cloned().collect()
    }

    /// Get active downloads
    pub async fn get_active_downloads(&self) -> Vec<DownloadItem> {
        self.downloads
            .read()
            .await
            .values()
            .filter(|d| d.is_active())
            .cloned()
            .collect()
    }

    /// Remove a download from the list
    pub async fn remove_download(&self, download_id: &str) -> Option<DownloadItem> {
        self.downloads.write().await.remove(download_id)
    }

    /// Clear completed downloads
    pub async fn clear_completed(&self) {
        let mut downloads = self.downloads.write().await;
        downloads.retain(|_, d| d.state != DownloadState::Completed);
    }

    /// Extract filename from URL
    fn extract_filename_from_url(&self, url: &str) -> String {
        url.split('/')
            .next_back()
            .and_then(|s| s.split('?').next())
            .filter(|s| !s.is_empty())
            .unwrap_or("download")
            .to_string()
    }

    /// Get download directory
    pub fn download_dir(&self) -> &PathBuf {
        &self.download_dir
    }
}

impl Default for DownloadManager {
    fn default() -> Self {
        Self::new(PathBuf::from("./downloads"))
    }
}

// ============================================================================
// Context Menu Support
// ============================================================================

/// Context menu item types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
/// Enumeration of ContextMenuItemType variants.
pub enum ContextMenuItemType {
    Normal,
    Separator,
    Checkbox,
    Radio,
    Submenu,
}

/// A context menu item
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Represents a ContextMenuItem.
pub struct ContextMenuItem {
    pub id: String,
    pub label: String,
    pub item_type: ContextMenuItemType,
    pub enabled: bool,
    pub checked: bool,
    pub icon: Option<String>,
    pub shortcut: Option<String>,
    pub submenu: Option<Vec<ContextMenuItem>>,
}

impl ContextMenuItem {
    /// Creates a new new.
    pub fn new(id: &str, label: &str) -> Self {
        Self {
            id: id.to_string(),
            label: label.to_string(),
            item_type: ContextMenuItemType::Normal,
            enabled: true,
            checked: false,
            icon: None,
            shortcut: None,
            submenu: None,
        }
    }

    /// Performs separator operation.
    pub fn separator() -> Self {
        Self {
            id: String::new(),
            label: String::new(),
            item_type: ContextMenuItemType::Separator,
            enabled: true,
            checked: false,
            icon: None,
            shortcut: None,
            submenu: None,
        }
    }

    /// Configures with icon.
    pub fn with_icon(mut self, icon: &str) -> Self {
        self.icon = Some(icon.to_string());
        self
    }

    /// Configures with shortcut.
    pub fn with_shortcut(mut self, shortcut: &str) -> Self {
        self.shortcut = Some(shortcut.to_string());
        self
    }

    /// Performs disabled operation.
    pub fn disabled(mut self) -> Self {
        self.enabled = false;
        self
    }

    /// Performs checkbox operation.
    pub fn checkbox(mut self, checked: bool) -> Self {
        self.item_type = ContextMenuItemType::Checkbox;
        self.checked = checked;
        self
    }

    /// Configures with submenu.
    pub fn with_submenu(mut self, items: Vec<ContextMenuItem>) -> Self {
        self.item_type = ContextMenuItemType::Submenu;
        self.submenu = Some(items);
        self
    }
}

/// Context types for different areas of the browser
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
/// Enumeration of ContextType variants.
pub enum ContextType {
    Page,
    Link,
    Image,
    Selection,
    Input,
    Video,
    Audio,
    Tab,
}

/// Context information for menu generation
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Represents a ContextInfo.
pub struct ContextInfo {
    pub context_type: ContextType,
    pub page_url: String,
    pub link_url: Option<String>,
    pub link_text: Option<String>,
    pub image_url: Option<String>,
    pub selection_text: Option<String>,
    pub media_url: Option<String>,
    pub is_editable: bool,
    pub position: (i32, i32),
}

/// Context menu manager
pub struct ContextMenuManager {
    custom_items: Arc<RwLock<HashMap<ContextType, Vec<ContextMenuItem>>>>,
}

impl ContextMenuManager {
    /// Creates a new new.
    pub fn new() -> Self {
        Self {
            custom_items: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Build context menu for given context
    pub async fn build_menu(&self, context: &ContextInfo) -> Vec<ContextMenuItem> {
        let mut items = Vec::new();

        match context.context_type {
            ContextType::Page => {
                items.extend(self.build_page_menu(context));
            }
            ContextType::Link => {
                items.extend(self.build_link_menu(context));
            }
            ContextType::Image => {
                items.extend(self.build_image_menu(context));
            }
            ContextType::Selection => {
                items.extend(self.build_selection_menu(context));
            }
            ContextType::Input => {
                items.extend(self.build_input_menu(context));
            }
            ContextType::Video | ContextType::Audio => {
                items.extend(self.build_media_menu(context));
            }
            ContextType::Tab => {
                items.extend(self.build_tab_menu(context));
            }
        }

        // Add custom items for this context type
        let custom = self.custom_items.read().await;
        if let Some(custom_items) = custom.get(&context.context_type) {
            if !items.is_empty() {
                items.push(ContextMenuItem::separator());
            }
            items.extend(custom_items.clone());
        }

        items
    }

    fn build_page_menu(&self, _context: &ContextInfo) -> Vec<ContextMenuItem> {
        vec![
            ContextMenuItem::new("back", "Back").with_shortcut("Alt+Left"),
            ContextMenuItem::new("forward", "Forward").with_shortcut("Alt+Right"),
            ContextMenuItem::new("reload", "Reload").with_shortcut("Ctrl+R"),
            ContextMenuItem::separator(),
            ContextMenuItem::new("save_as", "Save Page As...").with_shortcut("Ctrl+S"),
            ContextMenuItem::new("print", "Print...").with_shortcut("Ctrl+P"),
            ContextMenuItem::separator(),
            ContextMenuItem::new("view_source", "View Page Source").with_shortcut("Ctrl+U"),
            ContextMenuItem::new("inspect", "Inspect").with_shortcut("F12"),
        ]
    }

    fn build_link_menu(&self, context: &ContextInfo) -> Vec<ContextMenuItem> {
        let mut items = vec![
            ContextMenuItem::new("open_link", "Open Link"),
            ContextMenuItem::new("open_link_new_tab", "Open Link in New Tab")
                .with_shortcut("Ctrl+Click"),
            ContextMenuItem::new("open_link_new_window", "Open Link in New Window"),
            ContextMenuItem::separator(),
            ContextMenuItem::new("copy_link", "Copy Link Address"),
            ContextMenuItem::new("download_link", "Save Link As..."),
        ];

        if context.link_text.is_some() {
            items.push(ContextMenuItem::new("copy_link_text", "Copy Link Text"));
        }

        items
    }

    fn build_image_menu(&self, _context: &ContextInfo) -> Vec<ContextMenuItem> {
        vec![
            ContextMenuItem::new("open_image", "Open Image in New Tab"),
            ContextMenuItem::new("save_image", "Save Image As..."),
            ContextMenuItem::new("copy_image", "Copy Image"),
            ContextMenuItem::new("copy_image_url", "Copy Image Address"),
        ]
    }

    fn build_selection_menu(&self, _context: &ContextInfo) -> Vec<ContextMenuItem> {
        vec![
            ContextMenuItem::new("copy", "Copy").with_shortcut("Ctrl+C"),
            ContextMenuItem::separator(),
            ContextMenuItem::new("search", "Search for Selection"),
            ContextMenuItem::new("translate", "Translate Selection"),
        ]
    }

    fn build_input_menu(&self, _context: &ContextInfo) -> Vec<ContextMenuItem> {
        vec![
            ContextMenuItem::new("undo", "Undo").with_shortcut("Ctrl+Z"),
            ContextMenuItem::new("redo", "Redo").with_shortcut("Ctrl+Y"),
            ContextMenuItem::separator(),
            ContextMenuItem::new("cut", "Cut").with_shortcut("Ctrl+X"),
            ContextMenuItem::new("copy", "Copy").with_shortcut("Ctrl+C"),
            ContextMenuItem::new("paste", "Paste").with_shortcut("Ctrl+V"),
            ContextMenuItem::separator(),
            ContextMenuItem::new("select_all", "Select All").with_shortcut("Ctrl+A"),
        ]
    }

    fn build_media_menu(&self, context: &ContextInfo) -> Vec<ContextMenuItem> {
        let media_type = if context.context_type == ContextType::Video {
            "Video"
        } else {
            "Audio"
        };

        vec![
            ContextMenuItem::new("play_pause", "Play/Pause"),
            ContextMenuItem::new("mute", "Mute/Unmute"),
            ContextMenuItem::separator(),
            ContextMenuItem::new("loop", "Loop").checkbox(false),
            ContextMenuItem::new("controls", "Show Controls").checkbox(true),
            ContextMenuItem::separator(),
            ContextMenuItem::new("save_media", &format!("Save {} As...", media_type)),
            ContextMenuItem::new("copy_media_url", &format!("Copy {} Address", media_type)),
        ]
    }

    fn build_tab_menu(&self, _context: &ContextInfo) -> Vec<ContextMenuItem> {
        vec![
            ContextMenuItem::new("new_tab", "New Tab").with_shortcut("Ctrl+T"),
            ContextMenuItem::new("duplicate_tab", "Duplicate Tab"),
            ContextMenuItem::separator(),
            ContextMenuItem::new("reload_tab", "Reload Tab").with_shortcut("Ctrl+R"),
            ContextMenuItem::new("pin_tab", "Pin Tab"),
            ContextMenuItem::new("mute_tab", "Mute Tab"),
            ContextMenuItem::separator(),
            ContextMenuItem::new("close_tab", "Close Tab").with_shortcut("Ctrl+W"),
            ContextMenuItem::new("close_other_tabs", "Close Other Tabs"),
            ContextMenuItem::new("close_tabs_right", "Close Tabs to the Right"),
        ]
    }

    /// Add custom menu items for a context type
    pub async fn add_custom_items(&self, context_type: ContextType, items: Vec<ContextMenuItem>) {
        let mut custom = self.custom_items.write().await;
        custom.entry(context_type).or_default().extend(items);
    }

    /// Remove custom menu items
    pub async fn clear_custom_items(&self, context_type: &ContextType) {
        self.custom_items.write().await.remove(context_type);
    }
}

impl Default for ContextMenuManager {
    fn default() -> Self {
        Self::new()
    }
}
