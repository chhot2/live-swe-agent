//! Content Transformer Module
//!
//! Provides content transformation and styling.

use serde::{Deserialize, Serialize};

pub struct ContentTransformer {
    transformations: Vec<TransformationType>,
}

/// Types of content transformations
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Enumeration of TransformationType variants.
pub enum TransformationType {
    /// Simplify content for easier reading
    Simplify,
    /// Increase font size
    LargeText,
    /// High contrast mode
    HighContrast,
    /// Reduce motion
    ReduceMotion,
    /// Remove images
    TextOnly,
    /// Add alt text to images
    AutoAltText,
    /// Translate content
    Translate { target_language: String },
    /// Summarize content
    Summarize { max_sentences: usize },
    /// Reading level adjustment
    ReadingLevel { target_grade: u8 },
    /// Custom CSS injection
    CustomCss { css: String },
}

impl ContentTransformer {
    /// Creates a new new.
    pub fn new() -> Self {
        Self {
            transformations: Vec::new(),
        }
    }

    /// Add a transformation
    /// Add a transformation to be applied
    ///
    /// # Arguments
    /// * `transform` - The transformation type to add
    pub fn add_transformation(&mut self, transform: TransformationType) {
        self.transformations.push(transform);
    }

    /// Remove all transformations
    /// Clear all pending transformations
    pub fn clear_transformations(&mut self) {
        self.transformations.clear();
    }

    /// Generate transformation CSS
    /// Generate CSS styles for reader mode based on current configuration
    pub fn generate_css(&self) -> String {
        let mut css = String::new();

        for transform in &self.transformations {
            match transform {
                TransformationType::LargeText => {
                    css.push_str("body { font-size: 150% !important; }\n");
                }
                TransformationType::HighContrast => {
                    css.push_str(
                        r#"
                        body { background: #000 !important; color: #fff !important; }
                        a { color: #ff0 !important; }
                        img { filter: contrast(1.5) !important; }
                    "#,
                    );
                }
                TransformationType::ReduceMotion => {
                    css.push_str(
                        r#"
                        *, *::before, *::after {
                            animation-duration: 0.01ms !important;
                            animation-iteration-count: 1 !important;
                            transition-duration: 0.01ms !important;
                        }
                    "#,
                    );
                }
                TransformationType::TextOnly => {
                    css.push_str(
                        r#"
                        img, video, iframe, svg, canvas {
                            display: none !important;
                        }
                    "#,
                    );
                }
                TransformationType::CustomCss { css: custom } => {
                    css.push_str(custom);
                    css.push('\n');
                }
                _ => {}
            }
        }

        css
    }

    /// Get active transformations
    pub fn get_transformations(&self) -> &[TransformationType] {
        &self.transformations
    }
}

impl Default for ContentTransformer {
    fn default() -> Self {
        Self::new()
    }
}
