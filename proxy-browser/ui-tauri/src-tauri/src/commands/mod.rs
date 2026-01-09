//! Tauri command modules
//! 
//! This module organizes all Tauri commands into logical groups.

pub mod proxy;
pub mod browser;
pub mod storage;
pub mod backup;
pub mod ip;

// Re-export all commands for easy access
pub use proxy::*;
pub use browser::*;
pub use storage::*;
pub use backup::*;
pub use ip::*;
