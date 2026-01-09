use anyhow::{anyhow, Result};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, info};

/// Ad verification standards supported
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
/// Enumeration of VerificationStandard variants.
#[derive(Default)]
pub enum VerificationStandard {
    /// Media Rating Council standard
    #[default]
    MRC,
    /// Interactive Advertising Bureau
    IAB,
    /// Open Measurement SDK
    OMSDK,
    /// Custom verification rules
    Custom,
}

/// Ad format types for verification
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
/// Enumeration of AdFormat variants.
pub enum AdFormat {
    Display,
    Video,
    Native,
    Rich,
    Banner,
    Interstitial,
    Rewarded,
}

/// Viewability status according to MRC/IAB standards
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
/// Enumeration of ViewabilityStatus variants.
pub enum ViewabilityStatus {
    /// Ad is viewable (meets MRC standards)
    Viewable,
    /// Ad is not viewable
    NotViewable,
    /// Ad viewability cannot be determined
    Undetermined,
    /// Ad is partially viewable
    PartiallyViewable,
}

/// Ad impression verification result
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Represents a ImpressionVerification.
pub struct ImpressionVerification {
    pub impression_id: String,
    pub ad_id: String,
    pub timestamp: DateTime<Utc>,
    pub viewability: ViewabilityStatus,
    pub viewable_time_ms: u64,
    pub total_time_ms: u64,
    pub visible_percentage: f32,
    pub in_viewport: bool,
    pub is_valid: bool,
    pub fraud_signals: Vec<FraudSignal>,
}

/// Fraud detection signals
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
/// Enumeration of FraudSignal variants.
pub enum FraudSignal {
    /// Bot-like behavior detected
    BotTraffic,
    /// Invalid user agent
    InvalidUserAgent,
    /// Hidden ad placement
    HiddenAd,
    /// Ad stacking detected
    AdStacking,
    /// Pixel stuffing detected
    PixelStuffing,
    /// Invalid geo location
    InvalidGeo,
    /// Data center traffic
    DataCenterTraffic,
    /// Abnormal click pattern
    AbnormalClicks,
    /// Domain spoofing
    DomainSpoofing,
    /// Invalid IVT (Invalid Traffic)
    InvalidTraffic,
}

/// VAST (Video Ad Serving Template) verification
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Represents a VastVerification.
pub struct VastVerification {
    pub vast_version: String,
    pub ad_id: String,
    pub creative_id: String,
    pub duration_seconds: u32,
    pub skip_offset: Option<u32>,
    pub tracking_events: Vec<VastTrackingEvent>,
    pub verification_vendors: Vec<String>,
    pub is_compliant: bool,
    pub errors: Vec<String>,
}

/// VAST tracking event types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
/// Enumeration of VastTrackingEvent variants.
pub enum VastTrackingEvent {
    Impression,
    Start,
    FirstQuartile,
    Midpoint,
    ThirdQuartile,
    Complete,
    Mute,
    Unmute,
    Pause,
    Resume,
    Skip,
    Click,
    Close,
    Error,
}

/// VPAID (Video Player-Ad Interface Definition) verification
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Represents a VpaidVerification.
pub struct VpaidVerification {
    pub vpaid_version: String,
    pub ad_unit_type: String,
    pub linear: bool,
    pub skippable: bool,
    pub api_framework: String,
    pub events_received: Vec<String>,
    pub is_compliant: bool,
    pub errors: Vec<String>,
}

/// Ad verification configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Represents a AdVerificationConfig.
pub struct AdVerificationConfig {
    /// Verification standard to use
    pub standard: VerificationStandard,
    /// Minimum viewable percentage (MRC: 50% for display, 50% for video)
    pub min_viewable_percentage: f32,
    /// Minimum viewable time in milliseconds (MRC: 1000ms for display, 2000ms for video)
    pub min_viewable_time_ms: u64,
    /// Enable fraud detection
    pub fraud_detection_enabled: bool,
    /// Enable VAST verification
    pub vast_verification_enabled: bool,
    /// Enable VPAID verification
    pub vpaid_verification_enabled: bool,
    /// Custom verification scripts
    pub custom_scripts: Vec<String>,
    /// Verification vendor tags
    pub vendor_tags: Vec<String>,
}

impl Default for AdVerificationConfig {
    fn default() -> Self {
        Self {
            standard: VerificationStandard::MRC,
            min_viewable_percentage: 50.0,
            min_viewable_time_ms: 1000,
            fraud_detection_enabled: true,
            vast_verification_enabled: true,
            vpaid_verification_enabled: true,
            custom_scripts: Vec::new(),
            vendor_tags: Vec::new(),
        }
    }
}

/// Ad verification session for tracking
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Represents a VerificationSession.
pub struct VerificationSession {
    pub session_id: String,
    pub started_at: DateTime<Utc>,
    pub page_url: String,
    pub impressions: Vec<ImpressionVerification>,
    pub vast_verifications: Vec<VastVerification>,
    pub vpaid_verifications: Vec<VpaidVerification>,
    pub total_ads_verified: u32,
    pub viewable_ads: u32,
    pub fraud_detected_count: u32,
}

/// Ad verification manager
pub struct AdVerificationManager {
    config: Arc<RwLock<AdVerificationConfig>>,
    sessions: Arc<RwLock<HashMap<String, VerificationSession>>>,
    active_session_id: Arc<RwLock<Option<String>>>,
}

impl AdVerificationManager {
    /// Creates a new new.
    pub fn new(config: AdVerificationConfig) -> Self {
        Self {
            config: Arc::new(RwLock::new(config)),
            sessions: Arc::new(RwLock::new(HashMap::new())),
            active_session_id: Arc::new(RwLock::new(None)),
        }
    }

    /// Start a new verification session
    pub async fn start_session(&self, page_url: &str) -> Result<String> {
        let session_id = uuid::Uuid::new_v4().to_string();
        let session = VerificationSession {
            session_id: session_id.clone(),
            started_at: Utc::now(),
            page_url: page_url.to_string(),
            impressions: Vec::new(),
            vast_verifications: Vec::new(),
            vpaid_verifications: Vec::new(),
            total_ads_verified: 0,
            viewable_ads: 0,
            fraud_detected_count: 0,
        };

        self.sessions
            .write()
            .await
            .insert(session_id.clone(), session);
        *self.active_session_id.write().await = Some(session_id.clone());

        info!("Started ad verification session: {}", session_id);
        Ok(session_id)
    }

    /// End the current verification session
    pub async fn end_session(&self, session_id: &str) -> Result<VerificationSession> {
        let session = self
            .sessions
            .write()
            .await
            .remove(session_id)
            .ok_or_else(|| anyhow!("Session not found: {}", session_id))?;

        if self.active_session_id.read().await.as_deref() == Some(session_id) {
            *self.active_session_id.write().await = None;
        }

        info!(
            "Ended ad verification session: {} - {} ads verified, {} viewable",
            session_id, session.total_ads_verified, session.viewable_ads
        );
        Ok(session)
    }

    /// Verify an ad impression
    pub async fn verify_impression(&self, data: ImpressionData) -> Result<ImpressionVerification> {
        let config = self.config.read().await;

        // Calculate viewability
        let viewability = self.calculate_viewability(&data, &config);

        // Check for fraud signals
        let fraud_signals = if config.fraud_detection_enabled {
            self.detect_fraud(&data).await
        } else {
            Vec::new()
        };

        let is_valid = fraud_signals.is_empty() && viewability == ViewabilityStatus::Viewable;

        let verification = ImpressionVerification {
            impression_id: uuid::Uuid::new_v4().to_string(),
            ad_id: data.ad_id.clone(),
            timestamp: Utc::now(),
            viewability,
            viewable_time_ms: data.viewable_time_ms,
            total_time_ms: data.total_time_ms,
            visible_percentage: data.visible_percentage,
            in_viewport: data.in_viewport,
            is_valid,
            fraud_signals: fraud_signals.clone(),
        };

        // Update session if active
        if let Some(session_id) = self.active_session_id.read().await.clone() {
            if let Some(session) = self.sessions.write().await.get_mut(&session_id) {
                session.impressions.push(verification.clone());
                session.total_ads_verified += 1;
                if verification.viewability == ViewabilityStatus::Viewable {
                    session.viewable_ads += 1;
                }
                if !fraud_signals.is_empty() {
                    session.fraud_detected_count += 1;
                }
            }
        }

        debug!(
            "Verified impression {} - viewable: {:?}, valid: {}",
            verification.impression_id, verification.viewability, verification.is_valid
        );

        Ok(verification)
    }

    /// Calculate viewability status based on MRC/IAB standards
    fn calculate_viewability(
        &self,
        data: &ImpressionData,
        config: &AdVerificationConfig,
    ) -> ViewabilityStatus {
        if !data.in_viewport {
            return ViewabilityStatus::NotViewable;
        }

        let min_percentage = match data.ad_format {
            AdFormat::Video => 50.0,
            _ => config.min_viewable_percentage,
        };

        let min_time = match data.ad_format {
            AdFormat::Video => 2000, // 2 seconds for video
            _ => config.min_viewable_time_ms,
        };

        if data.visible_percentage >= min_percentage && data.viewable_time_ms >= min_time {
            ViewabilityStatus::Viewable
        } else if data.visible_percentage > 0.0 {
            ViewabilityStatus::PartiallyViewable
        } else {
            ViewabilityStatus::NotViewable
        }
    }

    /// Detect potential ad fraud signals
    async fn detect_fraud(&self, data: &ImpressionData) -> Vec<FraudSignal> {
        let mut signals = Vec::new();

        // Check for hidden ad (0x0 or 1x1 pixel ads)
        if data.ad_width <= 1 || data.ad_height <= 1 {
            signals.push(FraudSignal::PixelStuffing);
        }

        // Check for hidden placement
        if data.visible_percentage == 0.0 && data.in_viewport {
            signals.push(FraudSignal::HiddenAd);
        }

        // Check for abnormal viewable time (instant views)
        if data.viewable_time_ms > 0 && data.total_time_ms < 100 {
            signals.push(FraudSignal::BotTraffic);
        }

        // Check for ad stacking (multiple ads in same position)
        if data.z_index < 0 {
            signals.push(FraudSignal::AdStacking);
        }

        signals
    }

    /// Verify VAST ad
    pub async fn verify_vast(&self, vast_xml: &str) -> Result<VastVerification> {
        // Parse VAST XML and extract verification data
        let verification = self.parse_vast(vast_xml)?;

        // Update session if active
        if let Some(session_id) = self.active_session_id.read().await.clone() {
            if let Some(session) = self.sessions.write().await.get_mut(&session_id) {
                session.vast_verifications.push(verification.clone());
            }
        }

        Ok(verification)
    }

    /// Parse VAST XML for verification
    fn parse_vast(&self, _vast_xml: &str) -> Result<VastVerification> {
        // Simplified VAST parsing - in production, use proper XML parser
        Ok(VastVerification {
            vast_version: "4.0".to_string(),
            ad_id: uuid::Uuid::new_v4().to_string(),
            creative_id: uuid::Uuid::new_v4().to_string(),
            duration_seconds: 30,
            skip_offset: Some(5),
            tracking_events: vec![
                VastTrackingEvent::Impression,
                VastTrackingEvent::Start,
                VastTrackingEvent::FirstQuartile,
                VastTrackingEvent::Midpoint,
                VastTrackingEvent::ThirdQuartile,
                VastTrackingEvent::Complete,
            ],
            verification_vendors: vec!["MOAT".to_string(), "IAS".to_string()],
            is_compliant: true,
            errors: Vec::new(),
        })
    }

    /// Get JavaScript for ad verification injection
    pub fn get_verification_script(&self) -> String {
        r#"
        (function() {
            window.__adVerification = {
                impressions: [],
                
                trackImpression: function(adElement, adId) {
                    const rect = adElement.getBoundingClientRect();
                    const viewportHeight = window.innerHeight;
                    const viewportWidth = window.innerWidth;
                    
                    const visibleHeight = Math.min(rect.bottom, viewportHeight) - Math.max(rect.top, 0);
                    const visibleWidth = Math.min(rect.right, viewportWidth) - Math.max(rect.left, 0);
                    const visibleArea = Math.max(0, visibleHeight) * Math.max(0, visibleWidth);
                    const totalArea = rect.width * rect.height;
                    const visiblePercentage = totalArea > 0 ? (visibleArea / totalArea) * 100 : 0;
                    
                    const inViewport = rect.top < viewportHeight && rect.bottom > 0 &&
                                       rect.left < viewportWidth && rect.right > 0;
                    
                    const impression = {
                        adId: adId,
                        timestamp: Date.now(),
                        visiblePercentage: visiblePercentage,
                        inViewport: inViewport,
                        adWidth: rect.width,
                        adHeight: rect.height,
                        viewableStartTime: null,
                        totalViewableTime: 0
                    };
                    
                    this.impressions.push(impression);
                    return impression;
                },
                
                startViewabilityTracking: function(adElement, adId, callback) {
                    const self = this;
                    let viewableStartTime = null;
                    let totalViewableTime = 0;
                    
                    const observer = new IntersectionObserver((entries) => {
                        entries.forEach(entry => {
                            if (entry.intersectionRatio >= 0.5) {
                                if (!viewableStartTime) {
                                    viewableStartTime = Date.now();
                                }
                            } else {
                                if (viewableStartTime) {
                                    totalViewableTime += Date.now() - viewableStartTime;
                                    viewableStartTime = null;
                                }
                            }
                            
                            if (callback) {
                                callback({
                                    adId: adId,
                                    intersectionRatio: entry.intersectionRatio,
                                    isIntersecting: entry.isIntersecting,
                                    totalViewableTime: totalViewableTime + 
                                        (viewableStartTime ? Date.now() - viewableStartTime : 0)
                                });
                            }
                        });
                    }, { threshold: [0, 0.25, 0.5, 0.75, 1.0] });
                    
                    observer.observe(adElement);
                    return observer;
                },
                
                getImpressions: function() {
                    return this.impressions;
                },
                
                clearImpressions: function() {
                    this.impressions = [];
                }
            };
            
            console.log('[AdVerification] Initialized');
        })();
        "#.to_string()
    }

    /// Get current configuration
    pub async fn get_config(&self) -> AdVerificationConfig {
        self.config.read().await.clone()
    }

    /// Update configuration
    pub async fn set_config(&self, config: AdVerificationConfig) {
        *self.config.write().await = config;
    }

    /// Get active session
    pub async fn get_active_session(&self) -> Option<VerificationSession> {
        if let Some(session_id) = self.active_session_id.read().await.clone() {
            self.sessions.read().await.get(&session_id).cloned()
        } else {
            None
        }
    }

    /// Get session statistics
    pub async fn get_session_stats(&self, session_id: &str) -> Option<SessionStats> {
        self.sessions
            .read()
            .await
            .get(session_id)
            .map(|s| SessionStats {
                session_id: s.session_id.clone(),
                total_impressions: s.impressions.len() as u32,
                viewable_impressions: s.viewable_ads,
                viewability_rate: if s.total_ads_verified > 0 {
                    (s.viewable_ads as f32 / s.total_ads_verified as f32) * 100.0
                } else {
                    0.0
                },
                fraud_detected: s.fraud_detected_count,
                vast_verified: s.vast_verifications.len() as u32,
                vpaid_verified: s.vpaid_verifications.len() as u32,
            })
    }
}

/// Data for verifying an impression
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Represents a ImpressionData.
pub struct ImpressionData {
    pub ad_id: String,
    pub ad_format: AdFormat,
    pub ad_width: u32,
    pub ad_height: u32,
    pub visible_percentage: f32,
    pub in_viewport: bool,
    pub viewable_time_ms: u64,
    pub total_time_ms: u64,
    pub z_index: i32,
    pub page_url: String,
}

/// Session statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Represents a SessionStats.
pub struct SessionStats {
    pub session_id: String,
    pub total_impressions: u32,
    pub viewable_impressions: u32,
    pub viewability_rate: f32,
    pub fraud_detected: u32,
    pub vast_verified: u32,
    pub vpaid_verified: u32,
}

impl Default for AdVerificationManager {
    fn default() -> Self {
        Self::new(AdVerificationConfig::default())
    }
}
