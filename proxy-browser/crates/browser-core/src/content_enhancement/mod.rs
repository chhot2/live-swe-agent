//! Content Enhancement Module - V1000 Experimental
//!
//! Provides content enhancement features including:
//! - Reader mode with customization
//! - Media player enhancement
//! - Content extraction and transformation
//! - Accessibility enhancements
//! - Language detection and text analysis

mod accessibility;
mod content_transformer;
mod language_detector;
mod manager;
mod media_player;
mod reader_mode;
mod text_analyzer;

pub use accessibility::*;
pub use content_transformer::*;
pub use language_detector::*;
pub use manager::*;
pub use media_player::*;
pub use reader_mode::*;
pub use text_analyzer::*;
