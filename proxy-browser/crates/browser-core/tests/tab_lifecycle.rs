//! Tab Lifecycle
//!
//! Integration and unit tests for the module.

use browser_core::TabStatus;
use serde_json::{from_str, to_string};

#[test]
fn test_tab_status_creating() {
    // Test TabStatus variant exists
    let status = TabStatus::Creating;
    let serialized = to_string(&status).expect("Status operation failed");
    assert!(serialized.contains("Creating"));
}

#[test]
fn test_tab_status_active() {
    let status = TabStatus::Active;
    let serialized = to_string(&status).expect("Status operation failed");
    assert!(serialized.contains("Active"));
}

#[test]
fn test_tab_status_closed() {
    let status = TabStatus::Closed;
    let serialized = to_string(&status).expect("Status operation failed");
    assert!(serialized.contains("Closed"));
}

#[test]
fn test_tab_status_serialization_roundtrip() {
    let original = TabStatus::Active;
    let serialized = to_string(&original).expect("Original operation failed");
    let deserialized: TabStatus = from_str(&serialized).expect("Serialized operation failed");

    // Verify by re-serializing
    let reserialized = to_string(&deserialized).expect("Deserialized operation failed");
    assert_eq!(serialized, reserialized);
}
