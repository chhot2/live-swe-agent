//! Accessibility Module
//!
//! Provides accessibility enhancements.

use serde::{Deserialize, Serialize};
use tracing::info;

pub struct AccessibilityManager {
    config: AccessibilityConfig,
    announcements: Vec<String>,
}

/// Accessibility configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Represents a AccessibilityConfig.
pub struct AccessibilityConfig {
    /// Enable screen reader support
    pub screen_reader_support: bool,
    /// Enable keyboard navigation
    pub keyboard_navigation: bool,
    /// Focus indicator style
    pub focus_indicator: FocusIndicatorStyle,
    /// Color blindness mode
    pub color_blindness_mode: ColorBlindnessMode,
    /// Enable captions/subtitles
    pub captions_enabled: bool,
    /// Enable audio descriptions
    pub audio_descriptions: bool,
    /// Minimum contrast ratio
    pub min_contrast_ratio: f64,
    /// Enable dyslexia-friendly font
    pub dyslexia_font: bool,
    /// Enable text spacing
    pub enhanced_text_spacing: bool,
}

impl Default for AccessibilityConfig {
    fn default() -> Self {
        Self {
            screen_reader_support: true,
            keyboard_navigation: true,
            focus_indicator: FocusIndicatorStyle::Default,
            color_blindness_mode: ColorBlindnessMode::None,
            captions_enabled: false,
            audio_descriptions: false,
            min_contrast_ratio: 4.5,
            dyslexia_font: false,
            enhanced_text_spacing: false,
        }
    }
}

/// Focus indicator styles
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
/// Enumeration of FocusIndicatorStyle variants.
pub enum FocusIndicatorStyle {
    Default,
    HighVisibility,
    Custom,
    None,
}

/// Color blindness modes
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
/// Enumeration of ColorBlindnessMode variants.
pub enum ColorBlindnessMode {
    None,
    Protanopia,    // Red-blind
    Deuteranopia,  // Green-blind
    Tritanopia,    // Blue-blind
    Achromatopsia, // Total color blindness
}

impl AccessibilityManager {
    /// Creates a new new.
    pub fn new() -> Self {
        Self {
            config: AccessibilityConfig::default(),
            announcements: Vec::new(),
        }
    }

    /// Generate accessibility CSS
    /// Generate CSS styles for reader mode based on current configuration
    pub fn generate_css(&self) -> String {
        let mut css = String::new();

        // Focus indicator
        if self.config.focus_indicator == FocusIndicatorStyle::HighVisibility {
            css.push_str(
                r#"
                *:focus {
                    outline: 3px solid #ff0 !important;
                    outline-offset: 2px !important;
                }
            "#,
            );
        }

        // Dyslexia font
        if self.config.dyslexia_font {
            css.push_str(
                r#"
                body {
                    font-family: "OpenDyslexic", "Comic Sans MS", sans-serif !important;
                    letter-spacing: 0.05em !important;
                    word-spacing: 0.1em !important;
                }
            "#,
            );
        }

        // Enhanced text spacing
        if self.config.enhanced_text_spacing {
            css.push_str(
                r#"
                body {
                    line-height: 1.8 !important;
                    letter-spacing: 0.12em !important;
                    word-spacing: 0.16em !important;
                }
                p { margin-bottom: 2em !important; }
            "#,
            );
        }

        // Color blindness filters
        let filter = match self.config.color_blindness_mode {
            ColorBlindnessMode::None => "",
            ColorBlindnessMode::Protanopia => "filter: url('#protanopia') !important;",
            ColorBlindnessMode::Deuteranopia => "filter: url('#deuteranopia') !important;",
            ColorBlindnessMode::Tritanopia => "filter: url('#tritanopia') !important;",
            ColorBlindnessMode::Achromatopsia => "filter: grayscale(100%) !important;",
        };

        if !filter.is_empty() {
            css.push_str(&format!("html {{ {} }}", filter));
        }

        css
    }

    /// Announce message for screen readers
    /// Add a screen reader announcement
    ///
    /// # Arguments
    /// * `message` - The message to announce
    pub fn announce(&mut self, message: &str) {
        self.announcements.push(message.to_string());
        info!("Accessibility announcement: {}", message);
    }

    /// Get pending announcements
    /// Get and clear pending announcements
    pub fn get_announcements(&mut self) -> Vec<String> {
        std::mem::take(&mut self.announcements)
    }

    /// Update configuration
    pub fn set_config(&mut self, config: AccessibilityConfig) {
        self.config = config;
    }

    /// Get configuration
    pub fn get_config(&self) -> &AccessibilityConfig {
        &self.config
    }
}

impl Default for AccessibilityManager {
    fn default() -> Self {
        Self::new()
    }
}
