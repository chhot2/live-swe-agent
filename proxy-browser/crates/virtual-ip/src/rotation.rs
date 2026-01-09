use crate::generator::IPGenerator;
use crate::models::VirtualIP;
use anyhow::{anyhow, Result};
use chrono::{DateTime, Duration, Utc};
use rand::Rng;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Manages IP rotation strategies for traffic generation.
pub struct IPRotationManager {
    generator: Arc<IPGenerator>,
    active_ips: Arc<RwLock<HashMap<String, IPSession>>>,
    strategy: RotationStrategy,
}

#[derive(Clone)]
/// Represents a IPSession.
pub struct IPSession {
    pub virtual_ip: VirtualIP,
    pub assigned_at: DateTime<Utc>,
    pub last_used: DateTime<Utc>,
    pub request_count: usize,
    pub session_id: String,
}

#[derive(Debug, Clone)]
/// Enumeration of RotationStrategy variants.
pub enum RotationStrategy {
    /// Rotate IP after N requests
    PerRequest(usize),
    /// Rotate IP after time duration
    PerDuration(Duration),
    /// Rotate IP per session (never during session)
    PerSession,
    /// Random rotation (probabilistic)
    Random { probability: f64 },
    /// Sticky IP (same IP for same target domain)
    Sticky { duration: Duration },
    /// Geographic distribution (rotate within country/region)
    Geographic { country_codes: Vec<String> },
}

impl IPRotationManager {
    /// Creates a new new.
    pub fn new(generator: Arc<IPGenerator>, strategy: RotationStrategy) -> Self {
        Self {
            generator,
            active_ips: Arc::new(RwLock::new(HashMap::new())),
            strategy,
        }
    }

    /// Get or rotate IP for session
    pub async fn get_ip_for_session(&self, session_id: &str) -> Result<VirtualIP> {
        let mut sessions = self.active_ips.write().await;

        if let Some(session) = sessions.get_mut(session_id) {
            if self.should_rotate(session).await {
                let new_ip = self.generate_rotated_ip(&session.virtual_ip).await?;
                session.virtual_ip = new_ip.clone();
                session.last_used = Utc::now();
                session.request_count += 1;
                return Ok(new_ip);
            } else {
                session.last_used = Utc::now();
                session.request_count += 1;
                return Ok(session.virtual_ip.clone());
            }
        }

        // Create new session with new IP
        let virtual_ip = self.generator.generate_random()?;
        let session = IPSession {
            virtual_ip: virtual_ip.clone(),
            assigned_at: Utc::now(),
            last_used: Utc::now(),
            request_count: 1,
            session_id: session_id.to_string(),
        };

        sessions.insert(session_id.to_string(), session);
        Ok(virtual_ip)
    }

    /// Manually rotate IP for session
    pub async fn force_rotate(&self, session_id: &str) -> Result<VirtualIP> {
        let mut sessions = self.active_ips.write().await;
        if let Some(session) = sessions.get_mut(session_id) {
            let new_ip = self.generator.generate_random()?;
            session.virtual_ip = new_ip.clone();
            session.assigned_at = Utc::now();
            session.last_used = Utc::now();
            session.request_count = 0;
            Ok(new_ip)
        } else {
            Err(anyhow!("Session not found"))
        }
    }

    /// Get session statistics
    pub async fn get_session_stats(&self, session_id: &str) -> Option<SessionStats> {
        let sessions = self.active_ips.read().await;
        sessions.get(session_id).map(|s| SessionStats {
            session_id: s.session_id.clone(),
            current_ip: s.virtual_ip.ip.to_string(),
            country: s.virtual_ip.country.clone(),
            assigned_at: s.assigned_at,
            request_count: s.request_count,
            duration_seconds: (Utc::now() - s.assigned_at).num_seconds(),
        })
    }

    /// Clean up expired sessions
    pub async fn cleanup_expired(&self, max_age: Duration) {
        let mut sessions = self.active_ips.write().await;
        let now = Utc::now();
        sessions.retain(|_, session| now - session.last_used < max_age);
    }

    async fn should_rotate(&self, session: &IPSession) -> bool {
        match &self.strategy {
            RotationStrategy::PerRequest(count) => session.request_count >= *count,
            RotationStrategy::PerDuration(duration) => {
                let elapsed = Utc::now() - session.assigned_at;
                elapsed > *duration
            }
            RotationStrategy::PerSession => false,
            RotationStrategy::Random { probability } => {
                rand::thread_rng().gen::<f64>() < *probability
            }
            RotationStrategy::Sticky { duration } => {
                let elapsed = Utc::now() - session.last_used;
                elapsed > *duration
            }
            RotationStrategy::Geographic { .. } => false,
        }
    }

    async fn generate_rotated_ip(&self, _current_ip: &VirtualIP) -> Result<VirtualIP> {
        match &self.strategy {
            RotationStrategy::Geographic { country_codes } => {
                if country_codes.is_empty() {
                    return Err(anyhow!(
                        "No country codes configured for geographic rotation"
                    ));
                }
                let mut rng = rand::thread_rng();
                let country = &country_codes[rng.gen_range(0..country_codes.len())];
                self.generator.generate_for_country(country)
            }
            _ => self.generator.generate_random(),
        }
    }
}

#[derive(Debug, Clone)]
/// Represents a SessionStats.
pub struct SessionStats {
    pub session_id: String,
    pub current_ip: String,
    pub country: String,
    pub assigned_at: DateTime<Utc>,
    pub request_count: usize,
    pub duration_seconds: i64,
}
