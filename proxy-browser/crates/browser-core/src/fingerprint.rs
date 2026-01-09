//! Fingerprint Module
//!
//! Provides browser fingerprint management including:
//! - Canvas fingerprint protection
//! - WebGL fingerprint randomization
//! - Audio context fingerprint spoofing
//! - User-Agent rotation

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Represents a BrowserFingerprint.
pub struct BrowserFingerprint {
    pub user_agent: String,
    pub accept_language: String,
    pub timezone: String,
    pub screen_resolution: (u32, u32),
    pub color_depth: u8,
    pub hardware_concurrency: u8,
    pub device_memory: u8,
    pub platform: String,
    pub webgl_vendor: String,
    pub webgl_renderer: String,
    pub canvas_hash: String,
    pub audio_hash: String,
}
