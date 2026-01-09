//! Screenshot Capture Module
//!
//! Provides screenshot functionality including:
//! - Full page screenshots
//! - Viewport screenshots
//! - Element screenshots
//! - Screenshot formats (PNG, JPEG, WebP)

use anyhow::Result;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Screenshot format options
#[derive(Debug, Clone, Copy, Serialize, Deserialize, Default)]
pub enum ScreenshotFormat {
    #[default]
    Png,
    Jpeg,
    WebP,
}

impl ScreenshotFormat {
    /// Get file extension for format
    pub fn extension(&self) -> &'static str {
        match self {
            ScreenshotFormat::Png => "png",
            ScreenshotFormat::Jpeg => "jpg",
            ScreenshotFormat::WebP => "webp",
        }
    }

    /// Get MIME type for format
    pub fn mime_type(&self) -> &'static str {
        match self {
            ScreenshotFormat::Png => "image/png",
            ScreenshotFormat::Jpeg => "image/jpeg",
            ScreenshotFormat::WebP => "image/webp",
        }
    }
}

/// Screenshot options
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScreenshotOptions {
    pub format: ScreenshotFormat,
    pub quality: u8, // 0-100, for JPEG/WebP
    pub full_page: bool,
    pub clip: Option<ScreenshotClip>,
    pub omit_background: bool,
}

impl Default for ScreenshotOptions {
    fn default() -> Self {
        Self {
            format: ScreenshotFormat::Png,
            quality: 90,
            full_page: false,
            clip: None,
            omit_background: false,
        }
    }
}

/// Screenshot clip region
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScreenshotClip {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}

/// Screenshot result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScreenshotResult {
    pub data: Vec<u8>,
    pub format: ScreenshotFormat,
    pub width: u32,
    pub height: u32,
    pub captured_at: DateTime<Utc>,
}

impl ScreenshotResult {
    /// Save screenshot to file
    pub async fn save(&self, path: &PathBuf) -> Result<()> {
        tokio::fs::write(path, &self.data).await?;
        Ok(())
    }

    /// Get base64 encoded data
    pub fn to_base64(&self) -> String {
        use base64::{engine::general_purpose::STANDARD, Engine as _};
        STANDARD.encode(&self.data)
    }

    /// Get data URL
    pub fn to_data_url(&self) -> String {
        format!(
            "data:{};base64,{}",
            self.format.mime_type(),
            self.to_base64()
        )
    }
}

/// Screenshot manager for handling captures
pub struct ScreenshotManager {
    output_dir: PathBuf,
}

impl ScreenshotManager {
    /// Create a new screenshot manager
    pub fn new(output_dir: PathBuf) -> Self {
        Self { output_dir }
    }

    /// Generate filename for screenshot
    pub fn generate_filename(&self, prefix: &str, format: ScreenshotFormat) -> PathBuf {
        let timestamp = Utc::now().format("%Y%m%d_%H%M%S");
        let filename = format!("{}_{}.{}", prefix, timestamp, format.extension());
        self.output_dir.join(filename)
    }

    /// Capture viewport screenshot (placeholder - actual implementation depends on webview)
    pub async fn capture_viewport(
        &self,
        _tab_id: &str,
        options: &ScreenshotOptions,
    ) -> Result<ScreenshotResult> {
        // This would integrate with the actual webview to capture screenshot
        // For now, return a placeholder
        Ok(ScreenshotResult {
            data: Vec::new(),
            format: options.format,
            width: 1920,
            height: 1080,
            captured_at: Utc::now(),
        })
    }

    /// Capture full page screenshot
    pub async fn capture_full_page(
        &self,
        _tab_id: &str,
        options: &ScreenshotOptions,
    ) -> Result<ScreenshotResult> {
        let mut opts = options.clone();
        opts.full_page = true;
        self.capture_viewport(_tab_id, &opts).await
    }

    /// Capture element screenshot
    pub async fn capture_element(
        &self,
        _tab_id: &str,
        _selector: &str,
        options: &ScreenshotOptions,
    ) -> Result<ScreenshotResult> {
        // Would use CDP to capture specific element
        self.capture_viewport(_tab_id, options).await
    }
}

impl Default for ScreenshotManager {
    fn default() -> Self {
        Self::new(PathBuf::from("./screenshots"))
    }
}
