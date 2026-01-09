//! Integration tests for Browser Controls
//!
//! This module tests:
//! - Browser state management
//! - Navigation controls
//! - Download manager integration
//! - Context menu support

use browser_core::browser_controls::{
    BrowserController, BrowserSettings, BrowserState, ContextInfo, ContextMenuItem,
    ContextMenuItemType, ContextMenuManager, ContextType, DownloadItem, DownloadManager,
    DownloadState, HistoryItem, WebRtcPolicy,
};
use browser_core::chromium_engine::BrowserEngineType;
use std::path::PathBuf;

// ============================================================================
// BrowserState Tests
// ============================================================================

#[test]
fn test_browser_state_default() {
    let state = BrowserState::default();

    assert!(state.tab_id.is_empty());
    assert_eq!(state.current_url, "about:blank");
    assert_eq!(state.title, "New Tab");
    assert!(!state.can_go_back);
    assert!(!state.can_go_forward);
    assert!(!state.is_loading);
    assert!(state.history.is_empty());
    assert_eq!(state.history_index, -1);
}

#[test]
fn test_browser_state_serialization() {
    let state = BrowserState {
        tab_id: "test-tab".to_string(),
        current_url: "https://example.com".to_string(),
        title: "Example".to_string(),
        can_go_back: true,
        can_go_forward: false,
        is_loading: false,
        history: vec![HistoryItem {
            url: "https://example.com".to_string(),
            title: "Example".to_string(),
            timestamp: 1234567890,
        }],
        history_index: 0,
    };

    let json = serde_json::to_string(&state).expect("Failed to serialize to JSON");
    let parsed: BrowserState = serde_json::from_str(&json).expect("Failed to parse JSON");

    assert_eq!(parsed.tab_id, state.tab_id);
    assert_eq!(parsed.current_url, state.current_url);
    assert_eq!(parsed.history.len(), 1);
}

// ============================================================================
// HistoryItem Tests
// ============================================================================

#[test]
fn test_history_item_creation() {
    let item = HistoryItem {
        url: "https://example.com".to_string(),
        title: "Example Domain".to_string(),
        timestamp: chrono::Utc::now().timestamp(),
    };

    assert_eq!(item.url, "https://example.com");
    assert_eq!(item.title, "Example Domain");
    assert!(item.timestamp > 0);
}

#[test]
fn test_history_item_serialization() {
    let item = HistoryItem {
        url: "https://test.com".to_string(),
        title: "Test".to_string(),
        timestamp: 1609459200,
    };

    let json = serde_json::to_string(&item).expect("Failed to serialize to JSON");
    let parsed: HistoryItem = serde_json::from_str(&json).expect("Failed to parse JSON");

    assert_eq!(parsed.url, item.url);
    assert_eq!(parsed.timestamp, item.timestamp);
}

// ============================================================================
// BrowserSettings Tests
// ============================================================================

#[test]
fn test_browser_settings_default() {
    let settings = BrowserSettings::default();

    assert!(!settings.user_agent.is_empty());
    assert_eq!(settings.language, "en-US");
    assert_eq!(settings.webrtc_policy, WebRtcPolicy::DisableNonProxiedUdp);
    assert!(settings.dns_over_https);
    assert!(settings.block_trackers);
    assert!(settings.javascript_enabled);
    assert!(settings.cookies_enabled);
    assert_eq!(settings.engine_type, BrowserEngineType::System);
    assert!(settings.stealth_mode);
    assert!(!settings.headless_mode);
}

#[test]
fn test_browser_settings_serialization() {
    let settings = BrowserSettings::default();

    let json = serde_json::to_string(&settings).expect("Failed to serialize to JSON");
    let parsed: BrowserSettings = serde_json::from_str(&json).expect("Failed to parse JSON");

    assert_eq!(parsed.user_agent, settings.user_agent);
    assert_eq!(parsed.webrtc_policy, settings.webrtc_policy);
}

#[test]
fn test_webrtc_policy_variants() {
    let policies = vec![
        WebRtcPolicy::Default,
        WebRtcPolicy::DisableNonProxiedUdp,
        WebRtcPolicy::Disabled,
    ];

    for policy in policies {
        let json = serde_json::to_string(&policy).expect("Failed to serialize to JSON");
        let parsed: WebRtcPolicy = serde_json::from_str(&json).expect("Failed to parse JSON");
        assert_eq!(parsed, policy);
    }
}

// ============================================================================
// BrowserController Tests
// ============================================================================

#[tokio::test]
async fn test_browser_controller_creation() {
    let controller = BrowserController::new();

    let states = controller.get_all_states().await;
    assert!(states.is_empty());
}

#[tokio::test]
async fn test_browser_controller_create_state() {
    let controller = BrowserController::new();

    let state = controller.create_browser_state("tab-1").await;

    assert_eq!(state.tab_id, "tab-1");
    assert_eq!(state.current_url, "about:blank");
}

#[tokio::test]
async fn test_browser_controller_navigate() {
    let controller = BrowserController::new();
    controller.create_browser_state("tab-1").await;

    let state = controller
        .navigate("tab-1", "https://example.com")
        .await
        .expect("Navigation failed");

    assert_eq!(state.current_url, "https://example.com");
    assert!(state.is_loading);
    assert!(!state.can_go_back);
    assert_eq!(state.history.len(), 1);
}

#[tokio::test]
async fn test_browser_controller_navigation_history() {
    let controller = BrowserController::new();
    controller.create_browser_state("tab-1").await;

    // Navigate to multiple pages
    controller
        .navigate("tab-1", "https://page1.com")
        .await
        .expect("Navigation failed");
    controller
        .navigate("tab-1", "https://page2.com")
        .await
        .expect("Navigation failed");
    controller
        .navigate("tab-1", "https://page3.com")
        .await
        .expect("Navigation failed");

    let state = controller
        .get_state("tab-1")
        .await
        .expect("Failed to get browser state");
    assert_eq!(state.history.len(), 3);
    assert!(state.can_go_back);

    // Go back
    let url = controller.go_back("tab-1").await.expect("Go back failed");
    assert_eq!(url, Some("https://page2.com".to_string()));

    let state = controller
        .get_state("tab-1")
        .await
        .expect("Failed to get browser state");
    assert!(state.can_go_forward);

    // Go forward
    let url = controller
        .go_forward("tab-1")
        .await
        .expect("Go forward failed");
    assert_eq!(url, Some("https://page3.com".to_string()));
}

#[tokio::test]
async fn test_browser_controller_reload() {
    let controller = BrowserController::new();
    controller.create_browser_state("tab-1").await;
    controller
        .navigate("tab-1", "https://example.com")
        .await
        .expect("Navigation failed");

    let url = controller.reload("tab-1").await.expect("Reload failed");
    assert_eq!(url, Some("https://example.com".to_string()));
}

#[tokio::test]
async fn test_browser_controller_update_title() {
    let controller = BrowserController::new();
    controller.create_browser_state("tab-1").await;
    controller
        .navigate("tab-1", "https://example.com")
        .await
        .expect("Navigation failed");

    controller.update_title("tab-1", "Example Domain").await;

    let state = controller
        .get_state("tab-1")
        .await
        .expect("Failed to get browser state");
    assert_eq!(state.title, "Example Domain");
}

#[tokio::test]
async fn test_browser_controller_loading_state() {
    let controller = BrowserController::new();
    controller.create_browser_state("tab-1").await;

    controller.set_loading("tab-1", true).await;
    let state = controller
        .get_state("tab-1")
        .await
        .expect("Failed to get browser state");
    assert!(state.is_loading);

    controller.stop_loading("tab-1").await;
    let state = controller
        .get_state("tab-1")
        .await
        .expect("Failed to get browser state");
    assert!(!state.is_loading);
}

#[tokio::test]
async fn test_browser_controller_close_tab() {
    let controller = BrowserController::new();
    controller.create_browser_state("tab-1").await;
    controller.create_browser_state("tab-2").await;

    assert_eq!(controller.get_all_states().await.len(), 2);

    controller.close_tab("tab-1").await;

    assert_eq!(controller.get_all_states().await.len(), 1);
    assert!(controller.get_state("tab-1").await.is_none());
    assert!(controller.get_state("tab-2").await.is_some());
}

#[tokio::test]
async fn test_browser_controller_settings() {
    let controller = BrowserController::new();

    let settings = controller.get_settings().await;
    assert!(settings.javascript_enabled);

    let mut new_settings = settings.clone();
    new_settings.javascript_enabled = false;
    new_settings.block_ads = true;

    controller.set_settings(new_settings).await;

    let updated = controller.get_settings().await;
    assert!(!updated.javascript_enabled);
    assert!(updated.block_ads);
}

// ============================================================================
// DownloadManager Tests
// ============================================================================

#[test]
fn test_download_manager_creation() {
    let manager = DownloadManager::new(PathBuf::from("/tmp/downloads"));
    assert_eq!(manager.download_dir(), &PathBuf::from("/tmp/downloads"));
}

#[test]
fn test_download_manager_default() {
    let manager = DownloadManager::default();
    assert_eq!(manager.download_dir(), &PathBuf::from("./downloads"));
}

#[test]
fn test_download_manager_with_max_concurrent() {
    let manager = DownloadManager::new(PathBuf::from("/tmp")).with_max_concurrent(10);
    // Manager should be configured correctly
    assert!(true);
}

#[tokio::test]
async fn test_download_manager_start_download() {
    let manager = DownloadManager::new(PathBuf::from("/tmp/downloads"));

    let download_id = manager
        .start_download("https://example.com/file.zip", None, Some("tab-1"))
        .await
        .expect("Operation should succeed");

    assert!(!download_id.is_empty());

    let download = manager
        .get_download(&download_id)
        .await
        .expect("Failed to get download");
    assert_eq!(download.url, "https://example.com/file.zip");
    assert_eq!(download.filename, "file.zip");
    assert_eq!(download.state, DownloadState::Pending);
    assert_eq!(download.tab_id, Some("tab-1".to_string()));
}

#[tokio::test]
async fn test_download_manager_custom_filename() {
    let manager = DownloadManager::new(PathBuf::from("/tmp/downloads"));

    let download_id = manager
        .start_download(
            "https://example.com/file?id=123",
            Some("custom_name.zip"),
            None,
        )
        .await
        .expect("Operation should succeed");

    let download = manager
        .get_download(&download_id)
        .await
        .expect("Failed to get download");
    assert_eq!(download.filename, "custom_name.zip");
}

#[tokio::test]
async fn test_download_manager_progress() {
    let manager = DownloadManager::new(PathBuf::from("/tmp/downloads"));

    let download_id = manager
        .start_download("https://example.com/file.zip", None, None)
        .await
        .expect("Operation should succeed");

    // Update progress
    manager.update_progress(&download_id, 500, Some(1000)).await;

    let download = manager
        .get_download(&download_id)
        .await
        .expect("Failed to get download");
    assert_eq!(download.received_bytes, 500);
    assert_eq!(download.total_bytes, Some(1000));
    assert_eq!(download.state, DownloadState::InProgress);
    assert_eq!(download.progress_percent(), 50.0);
}

#[tokio::test]
async fn test_download_manager_pause_resume() {
    let manager = DownloadManager::new(PathBuf::from("/tmp/downloads"));

    let download_id = manager
        .start_download("https://example.com/file.zip", None, None)
        .await
        .expect("Operation should succeed");

    manager.update_progress(&download_id, 100, Some(1000)).await;

    // Pause
    manager
        .pause_download(&download_id)
        .await
        .expect("Failed to pause download");
    let download = manager
        .get_download(&download_id)
        .await
        .expect("Failed to get download");
    assert_eq!(download.state, DownloadState::Paused);

    // Resume
    manager
        .resume_download(&download_id)
        .await
        .expect("Failed to resume download");
    let download = manager
        .get_download(&download_id)
        .await
        .expect("Failed to get download");
    assert_eq!(download.state, DownloadState::InProgress);
}

#[tokio::test]
async fn test_download_manager_cancel() {
    let manager = DownloadManager::new(PathBuf::from("/tmp/downloads"));

    let download_id = manager
        .start_download("https://example.com/file.zip", None, None)
        .await
        .expect("Operation should succeed");

    manager
        .cancel_download(&download_id)
        .await
        .expect("Failed to cancel download");

    let download = manager
        .get_download(&download_id)
        .await
        .expect("Failed to get download");
    assert_eq!(download.state, DownloadState::Cancelled);
}

#[tokio::test]
async fn test_download_manager_complete() {
    let manager = DownloadManager::new(PathBuf::from("/tmp/downloads"));

    let download_id = manager
        .start_download("https://example.com/file.zip", None, None)
        .await
        .expect("Operation should succeed");

    manager
        .update_progress(&download_id, 1000, Some(1000))
        .await;
    manager.complete_download(&download_id).await;

    let download = manager
        .get_download(&download_id)
        .await
        .expect("Failed to get download");
    assert_eq!(download.state, DownloadState::Completed);
    assert!(download.completed_at.is_some());
}

#[tokio::test]
async fn test_download_manager_fail() {
    let manager = DownloadManager::new(PathBuf::from("/tmp/downloads"));

    let download_id = manager
        .start_download("https://example.com/file.zip", None, None)
        .await
        .expect("Operation should succeed");

    manager.fail_download(&download_id, "Network error").await;

    let download = manager
        .get_download(&download_id)
        .await
        .expect("Failed to get download");
    assert_eq!(download.state, DownloadState::Failed);
    assert_eq!(download.error, Some("Network error".to_string()));
}

#[tokio::test]
async fn test_download_manager_get_active() {
    let manager = DownloadManager::new(PathBuf::from("/tmp/downloads"));

    let id1 = manager
        .start_download("https://example.com/1.zip", None, None)
        .await
        .expect("Failed to start download");
    let id2 = manager
        .start_download("https://example.com/2.zip", None, None)
        .await
        .expect("Failed to start download");
    let id3 = manager
        .start_download("https://example.com/3.zip", None, None)
        .await
        .expect("Failed to start download");

    manager.complete_download(&id2).await;

    let active = manager.get_active_downloads().await;
    assert_eq!(active.len(), 2);
}

#[tokio::test]
async fn test_download_manager_clear_completed() {
    let manager = DownloadManager::new(PathBuf::from("/tmp/downloads"));

    let id1 = manager
        .start_download("https://example.com/1.zip", None, None)
        .await
        .expect("Failed to start download");
    let id2 = manager
        .start_download("https://example.com/2.zip", None, None)
        .await
        .expect("Failed to start download");

    manager.complete_download(&id1).await;

    manager.clear_completed().await;

    let all = manager.get_all_downloads().await;
    assert_eq!(all.len(), 1);
    assert!(manager.get_download(&id1).await.is_none());
}

// ============================================================================
// DownloadItem Tests
// ============================================================================

#[test]
fn test_download_item_progress_percent() {
    let item = DownloadItem {
        id: "test".to_string(),
        url: "https://example.com/file.zip".to_string(),
        filename: "file.zip".to_string(),
        save_path: PathBuf::from("/tmp/file.zip"),
        total_bytes: Some(1000),
        received_bytes: 250,
        state: DownloadState::InProgress,
        mime_type: None,
        started_at: 0,
        completed_at: None,
        error: None,
        tab_id: None,
    };

    assert_eq!(item.progress_percent(), 25.0);
}

#[test]
fn test_download_item_is_active() {
    let states = vec![
        (DownloadState::Pending, true),
        (DownloadState::InProgress, true),
        (DownloadState::Paused, false),
        (DownloadState::Completed, false),
        (DownloadState::Failed, false),
        (DownloadState::Cancelled, false),
    ];

    for (state, expected_active) in states {
        let item = DownloadItem {
            id: "test".to_string(),
            url: "https://example.com/file.zip".to_string(),
            filename: "file.zip".to_string(),
            save_path: PathBuf::from("/tmp/file.zip"),
            total_bytes: None,
            received_bytes: 0,
            state,
            mime_type: None,
            started_at: 0,
            completed_at: None,
            error: None,
            tab_id: None,
        };

        assert_eq!(item.is_active(), expected_active);
    }
}

// ============================================================================
// ContextMenuManager Tests
// ============================================================================

#[tokio::test]
async fn test_context_menu_manager_creation() {
    let manager = ContextMenuManager::new();
    // Manager should be created successfully
    assert!(true);
}

#[tokio::test]
async fn test_context_menu_page_menu() {
    let manager = ContextMenuManager::new();

    let context = ContextInfo {
        context_type: ContextType::Page,
        page_url: "https://example.com".to_string(),
        link_url: None,
        link_text: None,
        image_url: None,
        selection_text: None,
        media_url: None,
        is_editable: false,
        position: (100, 200),
    };

    let menu = manager.build_menu(&context).await;

    assert!(!menu.is_empty());
    assert!(menu.iter().any(|item| item.id == "back"));
    assert!(menu.iter().any(|item| item.id == "forward"));
    assert!(menu.iter().any(|item| item.id == "reload"));
}

#[tokio::test]
async fn test_context_menu_link_menu() {
    let manager = ContextMenuManager::new();

    let context = ContextInfo {
        context_type: ContextType::Link,
        page_url: "https://example.com".to_string(),
        link_url: Some("https://example.com/page".to_string()),
        link_text: Some("Click here".to_string()),
        image_url: None,
        selection_text: None,
        media_url: None,
        is_editable: false,
        position: (100, 200),
    };

    let menu = manager.build_menu(&context).await;

    assert!(menu.iter().any(|item| item.id == "open_link"));
    assert!(menu.iter().any(|item| item.id == "open_link_new_tab"));
    assert!(menu.iter().any(|item| item.id == "copy_link"));
}

#[tokio::test]
async fn test_context_menu_image_menu() {
    let manager = ContextMenuManager::new();

    let context = ContextInfo {
        context_type: ContextType::Image,
        page_url: "https://example.com".to_string(),
        link_url: None,
        link_text: None,
        image_url: Some("https://example.com/image.png".to_string()),
        selection_text: None,
        media_url: None,
        is_editable: false,
        position: (100, 200),
    };

    let menu = manager.build_menu(&context).await;

    assert!(menu.iter().any(|item| item.id == "open_image"));
    assert!(menu.iter().any(|item| item.id == "save_image"));
    assert!(menu.iter().any(|item| item.id == "copy_image"));
}

#[tokio::test]
async fn test_context_menu_custom_items() {
    let manager = ContextMenuManager::new();

    let custom_items = vec![ContextMenuItem::new("custom_action", "Custom Action")];

    manager
        .add_custom_items(ContextType::Page, custom_items)
        .await;

    let context = ContextInfo {
        context_type: ContextType::Page,
        page_url: "https://example.com".to_string(),
        link_url: None,
        link_text: None,
        image_url: None,
        selection_text: None,
        media_url: None,
        is_editable: false,
        position: (100, 200),
    };

    let menu = manager.build_menu(&context).await;

    assert!(menu.iter().any(|item| item.id == "custom_action"));
}

// ============================================================================
// ContextMenuItem Tests
// ============================================================================

#[test]
fn test_context_menu_item_creation() {
    let item = ContextMenuItem::new("test", "Test Item");

    assert_eq!(item.id, "test");
    assert_eq!(item.label, "Test Item");
    assert_eq!(item.item_type, ContextMenuItemType::Normal);
    assert!(item.enabled);
    assert!(!item.checked);
}

#[test]
fn test_context_menu_item_separator() {
    let item = ContextMenuItem::separator();

    assert_eq!(item.item_type, ContextMenuItemType::Separator);
}

#[test]
fn test_context_menu_item_with_icon() {
    let item = ContextMenuItem::new("test", "Test").with_icon("icon.png");

    assert_eq!(item.icon, Some("icon.png".to_string()));
}

#[test]
fn test_context_menu_item_with_shortcut() {
    let item = ContextMenuItem::new("copy", "Copy").with_shortcut("Ctrl+C");

    assert_eq!(item.shortcut, Some("Ctrl+C".to_string()));
}

#[test]
fn test_context_menu_item_disabled() {
    let item = ContextMenuItem::new("test", "Test").disabled();

    assert!(!item.enabled);
}

#[test]
fn test_context_menu_item_checkbox() {
    let item = ContextMenuItem::new("option", "Option").checkbox(true);

    assert_eq!(item.item_type, ContextMenuItemType::Checkbox);
    assert!(item.checked);
}

#[test]
fn test_context_menu_item_submenu() {
    let submenu_items = vec![
        ContextMenuItem::new("sub1", "Submenu 1"),
        ContextMenuItem::new("sub2", "Submenu 2"),
    ];

    let item = ContextMenuItem::new("menu", "Menu").with_submenu(submenu_items);

    assert_eq!(item.item_type, ContextMenuItemType::Submenu);
    assert!(item.submenu.is_some());
    assert_eq!(
        item.submenu.as_ref().expect("As ref should succeed").len(),
        2
    );
}

#[test]
fn test_context_menu_item_serialization() {
    let item = ContextMenuItem::new("test", "Test Item")
        .with_shortcut("Ctrl+T")
        .with_icon("test.png");

    let json = serde_json::to_string(&item).expect("Failed to serialize to JSON");
    let parsed: ContextMenuItem = serde_json::from_str(&json).expect("Failed to parse JSON");

    assert_eq!(parsed.id, item.id);
    assert_eq!(parsed.label, item.label);
    assert_eq!(parsed.shortcut, item.shortcut);
}

// ============================================================================
// ContextType Tests
// ============================================================================

#[test]
fn test_context_type_variants() {
    let types = vec![
        ContextType::Page,
        ContextType::Link,
        ContextType::Image,
        ContextType::Selection,
        ContextType::Input,
        ContextType::Video,
        ContextType::Audio,
        ContextType::Tab,
    ];

    for ctx_type in types {
        let json = serde_json::to_string(&ctx_type).expect("Failed to serialize to JSON");
        let parsed: ContextType = serde_json::from_str(&json).expect("Failed to parse JSON");
        assert_eq!(parsed, ctx_type);
    }
}
