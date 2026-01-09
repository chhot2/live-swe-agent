//! Browser Core Library
//!
//! This crate provides core browser functionality with proxy support and privacy features.
//!
//! ## Features
//!
//! - `mvp` (default): Minimal viable product with core features
//! - `full`: All features enabled
//! - `chromium`: Chromium browser engine integration
//! - `automation`: Browser automation capabilities
//! - `advanced-privacy`: Advanced privacy features
//! - `content-enhancement`: Reader mode, content transformation
//! - `efficiency`: Performance optimizations
//! - `experimental`: Experimental features
//! - `ad-verification`: Ad verification system
//! - `network-intelligence`: Network analysis
//! - `memory-profiling`: Memory profiling tools

pub mod prelude;

// Re-export prelude utilities for easier access
pub use prelude::{
    retry_async, string_utils, unix_timestamp, unix_timestamp_ms, validators, CircuitBreaker,
    CircuitState, HistogramStats, MetricsCollector, MetricsSnapshot, OptionExt, RateLimiter,
    ResultExt, RetryConfig,
};

// ============================================================================
// MVP Core Modules (Always included)
// ============================================================================
pub mod browser_profile;
pub mod config_manager;
pub mod http_client;
pub mod proxy;
pub mod request;
pub mod screenshot;
pub mod security;
pub mod storage;
pub mod tab_manager;

// Core exports
pub use browser_profile::{BrowserProfile, BrowserProfileManager, ProfileSettings};
pub use config_manager::{
    AppConfig, ConfigManager, FeatureFlags, GeneralConfig, LoggingConfig,
    NetworkConfig as AppNetworkConfig, PerformanceConfig, PrivacyConfig, ProxyConfig,
    StorageConfig,
};
pub use http_client::{HttpClient, PublicIpDetector, PublicIpInfo};
pub use proxy::{FreeProxy, ProxyManager, ProxySettings, ProxyTestResult, ProxyType};
pub use request::{
    HttpMethod, RequestBody, RequestBuilder, RequestConfig, RequestError, RequestErrorKind,
    RequestManager, RequestResponse,
};
pub use screenshot::{ScreenshotFormat, ScreenshotManager, ScreenshotOptions, ScreenshotResult};
pub use security::{BookmarkInput, ProxyInput, SecurityManager};
pub use storage::{
    Bookmark, BrowserSession, Cookie, ExportOptions, HistoryEntry, ImportExportStats,
    ImportOptions, ScrollPosition, SessionManager, SessionProxyConfig, SessionSettings,
    SessionStatistics, SessionTab, StorageEngine, StorageExport, TabHistoryEntry, WindowState,
};
pub use tab_manager::TabIPManager;

// ============================================================================
// MVP Extended Modules (Part of default MVP)
// ============================================================================
pub mod backup;
pub mod browser_controls;
pub mod browser_tab_manager;
pub mod error_recovery;
pub mod fingerprint;
pub mod free_ip_providers;
pub mod local_proxy;
pub mod pac_server;
pub mod proxy_rotation;
pub mod proxy_validator;
pub mod scraper_util;
pub mod tab_isolation;
pub mod webview_manager;

// MVP Extended exports
pub use backup::{AutoBackupSettings, BackupData, BackupInfo, BackupManager, BackupOptions};
pub use browser_controls::{
    BrowserController, BrowserSettings, BrowserState, ContextInfo, ContextMenuItem,
    ContextMenuItemType, ContextMenuManager, ContextType, DownloadItem, DownloadManager,
    DownloadState, HistoryItem, WebRtcPolicy,
};
pub use browser_tab_manager::{BrowserTab, BrowserTabManager, CreateTabConfig, TabStats};
pub use error_recovery::{
    CrashPrediction, ErrorCategory, ErrorRecoveryConfig, ErrorRecoveryManager, ErrorSeverity,
    ErrorStats, OperationMetrics, RecoveryResult, RecoveryStrategy,
};
pub use fingerprint::BrowserFingerprint;
pub use free_ip_providers::{FreeIpProvider, FreeIpProviderManager, ProxyFilter};
pub use local_proxy::{
    InterceptedRequest, LocalProxyManager, LocalProxyServer, ModificationRule, NetworkInterceptor,
    ProxyConnection, RequestModifications, WebSocketInterception, WebSocketProxyHandler,
};
pub use pac_server::{PacManager, PacServer};
pub use proxy_rotation::{
    BandwidthStats, GeoDiversityManager, ProxyHealthMonitor, ProxyHealthStatus, ProxyMetrics,
    ProxyRotationManager, ProxyRotationStrategy, ProxySessionStats, SmartProxySelector,
};
pub use proxy_validator::{
    EnhancedProxyHealthChecker, GeoVerificationConfig, GeoVerificationResult, GeoVerifier,
    ProxyHealthChecker, ProxyQuarantineManager, ProxyValidator, ProxyValidatorConfig,
    QuarantineStats, QuarantinedProxy, ValidationResult,
};
pub use scraper_util::ProxyScraper;
pub use tab_isolation::{
    HTTP2Settings, NetworkConfig, TCPFingerprint, TLSProfile, TabProfile, TabStatus,
};
pub use webview_manager::{WebviewManager, WebviewTab};

// ============================================================================
// Performance Optimizer (Always included for basic optimization)
// ============================================================================
pub mod performance_optimizer;

pub use performance_optimizer::{
    CachePriority, CacheStats, CoreWebVitals, PerformanceConfig as PerfOptimizerConfig,
    PerformanceOptimizer, PerformanceReport,
};

// ============================================================================
// Chromium Engine (Optional)
// ============================================================================
#[cfg(feature = "chromium")]
pub mod chromium_engine;

#[cfg(feature = "chromium")]
pub use chromium_engine::{
    BrowserEngineManager, BrowserEngineType, ChromiumEngine, ChromiumEngineConfig, ChromiumTab,
    EngineCapabilities,
};

// Stub for when chromium is not enabled
#[cfg(not(feature = "chromium"))]
pub mod chromium_engine {
    //! Chromium engine stub - enable the `chromium` feature for full functionality
    pub struct ChromiumEngine;
    pub struct ChromiumEngineConfig;
    pub struct ChromiumTab;
}

// Export BrowserEngineType - either from chromium_engine (full) or browser_controls (stub)
#[cfg(not(feature = "chromium"))]
pub use browser_controls::BrowserEngineType;

// ============================================================================
// Advanced Privacy Features (Optional)
// ============================================================================
#[cfg(feature = "advanced-privacy")]
pub mod privacy_fortress;

#[cfg(feature = "advanced-privacy")]
pub use privacy_fortress::{
    CookieIsolationLevel, PrivacyConfig as PrivacyFortressConfig, PrivacyFortress, PrivacyGrade,
    PrivacyReport, TrackerStats,
};

// ============================================================================
// Ad Verification (Optional)
// ============================================================================
#[cfg(feature = "ad-verification")]
pub mod ad_verification;

#[cfg(feature = "ad-verification")]
pub use ad_verification::{
    AdFormat, AdVerificationConfig, AdVerificationManager, FraudSignal, ImpressionData,
    ImpressionVerification, SessionStats, VastVerification, VerificationSession,
    VerificationStandard, ViewabilityStatus, VpaidVerification,
};

// ============================================================================
// Automation Features (Optional)
// ============================================================================
#[cfg(feature = "automation")]
pub mod automation;

#[cfg(feature = "automation")]
pub use automation::{
    ActionRecorder, ActionType, AutomationManager, AutomationNode, AutomationStats, AutomationStep,
    DistributedAutomation, DistributedTask, NaturalLanguageAutomation, NodeStatus, RecordedAction,
    StepType, TaskStatus, VisualAutomationBuilder, Workflow,
};

// ============================================================================
// Content Enhancement (Optional)
// ============================================================================
#[cfg(feature = "content-enhancement")]
pub mod content_enhancement;

#[cfg(feature = "content-enhancement")]
pub use content_enhancement::{
    AccessibilityManager, AccessibilityReport, AccessibilitySettings, ColorBlindnessType,
    ContentEnhancementManager, ContentTransformer, EnhancedMediaPlayer, FontSettings,
    LanguageDetector, MediaPlayerConfig, MediaType, ReaderMode, ReaderModeConfig, ReaderTheme,
    ScriptType, TextAnalyzer, TextStatistics, TransformationType, VideoQuality,
};

// ============================================================================
// Efficiency Optimizations (Optional)
// ============================================================================
#[cfg(feature = "efficiency")]
pub mod efficiency;

#[cfg(feature = "efficiency")]
pub use efficiency::{
    unrolled_loop, AsyncExecutor, BatchProcessor, BufferPool, ConnectionPool, ConnectionPoolStats,
    CpuOptimizer, EfficiencyConfig, EfficiencyManager, EfficiencyMetrics, EfficiencyStats,
    ExecutorStats, MemoryOptimizer, MemoryPressure, MemoryStats as EfficiencyMemoryStats,
    MemoryThresholds as EfficiencyMemoryThresholds, OptimizedCacheManager, ParallelProcessor,
    PooledBuffer, RateLimiter as EfficiencyRateLimiter, ScheduledTask, TaskPriority,
};

// ============================================================================
// Experimental Features (Optional)
// ============================================================================
#[cfg(feature = "experimental")]
pub mod experimental;

#[cfg(feature = "experimental")]
pub use experimental::{
    AdaptiveRenderer, AntiCorrelationSystem, BehavioralAuth, BlockchainDns, DecentralizedIdentity,
    DecoyTrafficGenerator, DifferentialPrivacy, DnsResolver, DnsStrategy, EnclaveType, EngineType,
    ExperimentalFeatureInfo, ExperimentalFeaturesManager, GpuRenderConfig, IntelligentFormFiller,
    IpfsBrowser, LocalLlm, MemorySafeSandbox, MeshNode, MeshProxyNetwork, MultiEngineSystem,
    OnionRouter, PostQuantumCrypto, PqcAlgorithm, ProcessIsolationConfig, QuantumRng, SandboxLevel,
    SecureEnclaveManager, Spatial3DConfig, VisualEngine, WasiBrowser, WasiCapabilities,
    ZkAuthSystem,
};

// ============================================================================
// Network Intelligence (Optional)
// ============================================================================
#[cfg(feature = "network-intelligence")]
pub mod network_intelligence;

#[cfg(feature = "network-intelligence")]
pub use network_intelligence::{
    NetworkIntelligence, NetworkIntelligenceConfig, NetworkIntelligenceReport, QosPriority,
    TrafficReport,
};

// ============================================================================
// Memory Profiling (Optional - typically debug only)
// ============================================================================
#[cfg(feature = "memory-profiling")]
pub mod memory_profiler;

#[cfg(feature = "memory-profiling")]
pub use memory_profiler::{
    GcRecommendation, LeakReport, MemoryAlert, MemoryProfiler, MemorySnapshot, MemoryStats,
    MemoryThresholds,
};
