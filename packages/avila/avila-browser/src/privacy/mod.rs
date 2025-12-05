//! Privacy and anti-tracking features
//!
//! Implements various privacy protection mechanisms:
//! - Anti-fingerprinting (canvas, WebGL, fonts, etc.)
//! - Tracker blocking (EasyList, EasyPrivacy)
//! - Cookie isolation and management
//! - Referrer policy enforcement

pub mod fingerprinting;
pub mod trackers;
pub mod cookies;
pub mod referrer;

pub use fingerprinting::{FingerprintProtection, FingerprintLevel};
pub use trackers::{TrackerBlocker, TrackerDatabase};
pub use cookies::{CookieJar, CookiePolicy};
pub use referrer::{ReferrerPolicy, ReferrerHandler};

/// Privacy configuration
#[derive(Debug, Clone)]
pub struct PrivacyConfig {
    pub block_trackers: bool,
    pub block_third_party_cookies: bool,
    pub fingerprint_protection: FingerprintLevel,
    pub referrer_policy: ReferrerPolicy,
    pub block_webrtc: bool,
    pub block_geolocation: bool,
}

impl Default for PrivacyConfig {
    fn default() -> Self {
        Self {
            block_trackers: true,
            block_third_party_cookies: true,
            fingerprint_protection: FingerprintLevel::Strict,
            referrer_policy: ReferrerPolicy::NoReferrer,
            block_webrtc: true,
            block_geolocation: true,
        }
    }
}
