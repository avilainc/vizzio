//! Anti-fingerprinting techniques

/// Fingerprint protection level
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FingerprintLevel {
    /// No protection
    None,
    /// Standard protection (block common techniques)
    Standard,
    /// Strict protection (may break some sites)
    Strict,
}

/// Fingerprint protection manager
pub struct FingerprintProtection {
    level: FingerprintLevel,
}

impl FingerprintProtection {
    pub fn new(level: FingerprintLevel) -> Self {
        Self { level }
    }

    /// Check if canvas fingerprinting should be blocked
    pub fn should_block_canvas(&self) -> bool {
        matches!(self.level, FingerprintLevel::Standard | FingerprintLevel::Strict)
    }

    /// Check if WebGL should be blocked
    pub fn should_block_webgl(&self) -> bool {
        matches!(self.level, FingerprintLevel::Strict)
    }

    /// Check if font enumeration should be limited
    pub fn should_limit_fonts(&self) -> bool {
        matches!(self.level, FingerprintLevel::Standard | FingerprintLevel::Strict)
    }

    /// Get spoofed user agent
    pub fn get_user_agent(&self) -> Option<String> {
        match self.level {
            FingerprintLevel::None => None,
            FingerprintLevel::Standard | FingerprintLevel::Strict => {
                Some("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36".to_string())
            }
        }
    }

    /// Get spoofed timezone
    pub fn get_timezone(&self) -> Option<String> {
        match self.level {
            FingerprintLevel::None => None,
            FingerprintLevel::Standard | FingerprintLevel::Strict => Some("UTC".to_string()),
        }
    }

    /// Get spoofed screen resolution
    pub fn get_screen_resolution(&self) -> Option<(u32, u32)> {
        match self.level {
            FingerprintLevel::None => None,
            FingerprintLevel::Standard | FingerprintLevel::Strict => Some((1920, 1080)),
        }
    }

    /// Check if audio context fingerprinting should be blocked
    pub fn should_block_audio_fingerprinting(&self) -> bool {
        matches!(self.level, FingerprintLevel::Strict)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fingerprint_protection_levels() {
        let none = FingerprintProtection::new(FingerprintLevel::None);
        assert!(!none.should_block_canvas());

        let strict = FingerprintProtection::new(FingerprintLevel::Strict);
        assert!(strict.should_block_canvas());
        assert!(strict.should_block_webgl());
    }
}
