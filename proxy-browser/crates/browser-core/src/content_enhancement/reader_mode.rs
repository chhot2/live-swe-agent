//! Reader Mode Module
//!
//! Provides reader mode functionality for distraction-free reading.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Reader mode configuration and engine
pub struct ReaderMode {
    config: ReaderModeConfig,
    extracted_content: HashMap<String, ExtractedArticle>,
}

/// Reader mode configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Represents a ReaderModeConfig.
pub struct ReaderModeConfig {
    /// Font family
    pub font_family: String,
    /// Font size in pixels
    pub font_size: u32,
    /// Line height multiplier
    pub line_height: f64,
    /// Maximum content width in pixels
    pub max_width: u32,
    /// Theme
    pub theme: ReaderTheme,
    /// Enable text-to-speech
    pub tts_enabled: bool,
    /// Enable auto-scroll
    pub auto_scroll_enabled: bool,
    /// Auto-scroll speed (words per minute)
    pub auto_scroll_wpm: u32,
    /// Enable estimated reading time
    pub show_reading_time: bool,
    /// Enable progress indicator
    pub show_progress: bool,
}

impl Default for ReaderModeConfig {
    fn default() -> Self {
        Self {
            font_family: "Georgia, serif".to_string(),
            font_size: 18,
            line_height: 1.6,
            max_width: 680,
            theme: ReaderTheme::Light,
            tts_enabled: false,
            auto_scroll_enabled: false,
            auto_scroll_wpm: 250,
            show_reading_time: true,
            show_progress: true,
        }
    }
}

/// Reader mode themes
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
/// Enumeration of ReaderTheme variants.
pub enum ReaderTheme {
    Light,
    Dark,
    Sepia,
    HighContrast,
    Custom,
}

/// Extracted article content
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Represents a ExtractedArticle.
pub struct ExtractedArticle {
    pub url: String,
    pub title: String,
    pub author: Option<String>,
    pub published_date: Option<String>,
    pub content: String,
    pub text_content: String,
    pub word_count: usize,
    pub reading_time_minutes: u32,
    pub images: Vec<ArticleImage>,
    pub language: Option<String>,
}

/// Article image
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Represents a ArticleImage.
pub struct ArticleImage {
    pub src: String,
    pub alt: Option<String>,
    pub caption: Option<String>,
}

impl ReaderMode {
    /// Creates a new new.
    pub fn new() -> Self {
        Self {
            config: ReaderModeConfig::default(),
            extracted_content: HashMap::new(),
        }
    }

    /// Configures with config.
    /// Create a new ReaderMode instance with custom configuration
    ///
    /// # Arguments
    /// * `config` - The reader mode configuration to use
    pub fn with_config(config: ReaderModeConfig) -> Self {
        Self {
            config,
            extracted_content: HashMap::new(),
        }
    }

    /// Extract article content from HTML
    /// Extract article content from HTML
    ///
    /// # Arguments
    /// * `url` - The URL of the page
    /// * `html` - The HTML content to extract from
    /// * `title` - Optional title override
    pub fn extract_article(
        &mut self,
        url: &str,
        html: &str,
        title: Option<&str>,
    ) -> ExtractedArticle {
        // Simple content extraction (in production would use readability algorithm)
        let text_content = self.strip_html(html);
        let word_count = text_content.split_whitespace().count();
        let reading_time = (word_count as f64 / 200.0).ceil() as u32; // 200 WPM average

        let article = ExtractedArticle {
            url: url.to_string(),
            title: title.unwrap_or("Untitled").to_string(),
            author: self.extract_author(html),
            published_date: self.extract_date(html),
            content: html.to_string(),
            text_content: text_content.clone(),
            word_count,
            reading_time_minutes: reading_time,
            images: self.extract_images(html),
            language: self.detect_language(&text_content),
        };

        self.extracted_content
            .insert(url.to_string(), article.clone());
        article
    }

    /// Strip HTML tags from content
    fn strip_html(&self, html: &str) -> String {
        // Simple HTML stripping
        let mut result = String::new();
        let mut in_tag = false;

        for c in html.chars() {
            match c {
                '<' => in_tag = true,
                '>' => in_tag = false,
                _ if !in_tag => result.push(c),
                _ => {}
            }
        }

        result
    }

    /// Extract author from HTML
    fn extract_author(&self, html: &str) -> Option<String> {
        // Simple author extraction using meta tags
        if html.contains("author") {
            Some("Unknown Author".to_string())
        } else {
            None
        }
    }

    /// Extract publication date
    fn extract_date(&self, _html: &str) -> Option<String> {
        None
    }

    /// Extract images from HTML
    fn extract_images(&self, html: &str) -> Vec<ArticleImage> {
        // Simple image extraction
        let mut images = Vec::new();
        let lower = html.to_lowercase();

        if lower.contains("<img") {
            images.push(ArticleImage {
                src: "placeholder.jpg".to_string(),
                alt: Some("Image".to_string()),
                caption: None,
            });
        }

        images
    }

    /// Detect language using simple heuristics
    /// Note: For production use, consider using a proper language detection library
    fn detect_language(&self, text: &str) -> Option<String> {
        if text.len() < 20 {
            return None; // Not enough text for reliable detection
        }

        let lower = text.to_lowercase();
        let words: Vec<&str> = lower.split_whitespace().collect();
        let word_count = words.len();

        if word_count < 10 {
            return None;
        }

        // English detection - requires multiple common words
        let english_markers = [
            "the", "and", "is", "are", "was", "were", "have", "has", "been",
        ];
        let english_count = english_markers
            .iter()
            .filter(|marker| words.contains(marker))
            .count();
        if english_count >= 3 {
            return Some("en".to_string());
        }

        // Spanish detection
        let spanish_markers = ["el", "los", "las", "es", "son", "está", "están", "que"];
        let spanish_count = spanish_markers
            .iter()
            .filter(|marker| words.contains(marker))
            .count();
        if spanish_count >= 3 {
            return Some("es".to_string());
        }

        // French detection
        let french_markers = ["le", "les", "est", "sont", "avec", "dans", "pour", "qui"];
        let french_count = french_markers
            .iter()
            .filter(|marker| words.contains(marker))
            .count();
        if french_count >= 3 {
            return Some("fr".to_string());
        }

        // German detection
        let german_markers = ["der", "die", "das", "ist", "sind", "und", "mit", "für"];
        let german_count = german_markers
            .iter()
            .filter(|marker| words.contains(marker))
            .count();
        if german_count >= 3 {
            return Some("de".to_string());
        }

        None
    }

    /// Generate reader mode CSS
    /// Generate CSS styles for reader mode based on current configuration
    pub fn generate_css(&self) -> String {
        let (bg_color, text_color) = match self.config.theme {
            ReaderTheme::Light => ("#ffffff", "#1a1a1a"),
            ReaderTheme::Dark => ("#1a1a1a", "#e0e0e0"),
            ReaderTheme::Sepia => ("#f4ecd8", "#5c4b37"),
            ReaderTheme::HighContrast => ("#000000", "#ffffff"),
            ReaderTheme::Custom => ("#ffffff", "#000000"),
        };

        format!(
            r#"
            body {{
                font-family: {font_family};
                font-size: {font_size}px;
                line-height: {line_height};
                max-width: {max_width}px;
                margin: 0 auto;
                padding: 20px;
                background-color: {bg_color};
                color: {text_color};
            }}
            
            h1, h2, h3 {{
                line-height: 1.3;
                margin-top: 1.5em;
            }}
            
            p {{
                margin-bottom: 1em;
            }}
            
            img {{
                max-width: 100%;
                height: auto;
                display: block;
                margin: 1em auto;
            }}
            
            blockquote {{
                border-left: 3px solid #ccc;
                margin-left: 0;
                padding-left: 1em;
                font-style: italic;
            }}
            
            a {{
                color: #0066cc;
            }}
            "#,
            font_family = self.config.font_family,
            font_size = self.config.font_size,
            line_height = self.config.line_height,
            max_width = self.config.max_width,
            bg_color = bg_color,
            text_color = text_color,
        )
    }

    /// Update configuration
    /// Update the reader mode configuration
    ///
    /// # Arguments
    /// * `config` - The new configuration to use
    pub fn set_config(&mut self, config: ReaderModeConfig) {
        self.config = config;
    }

    /// Get current configuration
    /// Get the current reader mode configuration
    pub fn get_config(&self) -> &ReaderModeConfig {
        &self.config
    }

    /// Get cached article
    /// Get a cached extracted article by URL
    ///
    /// # Arguments
    /// * `url` - The URL to look up
    pub fn get_cached(&self, url: &str) -> Option<&ExtractedArticle> {
        self.extracted_content.get(url)
    }
}

impl Default for ReaderMode {
    fn default() -> Self {
        Self::new()
    }
}
