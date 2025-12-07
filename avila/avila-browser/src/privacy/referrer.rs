//! Referrer policy enforcement

/// Referrer policy options
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReferrerPolicy {
    /// Never send referrer
    NoReferrer,
    /// Send only origin (no path)
    Origin,
    /// Send full URL only for same-origin requests
    SameOrigin,
    /// Send origin when crossing origins
    StrictOriginWhenCrossOrigin,
    /// Always send full URL
    UnsafeUrl,
}

impl ReferrerPolicy {
    /// Parse policy from string
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "no-referrer" => Some(Self::NoReferrer),
            "origin" => Some(Self::Origin),
            "same-origin" => Some(Self::SameOrigin),
            "strict-origin-when-cross-origin" => Some(Self::StrictOriginWhenCrossOrigin),
            "unsafe-url" => Some(Self::UnsafeUrl),
            _ => None,
        }
    }

    /// Convert to string representation
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::NoReferrer => "no-referrer",
            Self::Origin => "origin",
            Self::SameOrigin => "same-origin",
            Self::StrictOriginWhenCrossOrigin => "strict-origin-when-cross-origin",
            Self::UnsafeUrl => "unsafe-url",
        }
    }
}

/// Referrer handler
pub struct ReferrerHandler {
    policy: ReferrerPolicy,
}

impl ReferrerHandler {
    pub fn new(policy: ReferrerPolicy) -> Self {
        Self { policy }
    }

    /// Calculate referrer header value
    pub fn calculate_referrer(
        &self,
        source_url: &str,
        target_url: &str,
        is_secure: bool,
    ) -> Option<String> {
        match self.policy {
            ReferrerPolicy::NoReferrer => None,
            ReferrerPolicy::Origin => Some(extract_origin(source_url)),
            ReferrerPolicy::SameOrigin => {
                if is_same_origin(source_url, target_url) {
                    Some(source_url.to_string())
                } else {
                    None
                }
            }
            ReferrerPolicy::StrictOriginWhenCrossOrigin => {
                if is_same_origin(source_url, target_url) {
                    Some(source_url.to_string())
                } else {
                    Some(extract_origin(source_url))
                }
            }
            ReferrerPolicy::UnsafeUrl => Some(source_url.to_string()),
        }
    }

    pub fn set_policy(&mut self, policy: ReferrerPolicy) {
        self.policy = policy;
    }
}

/// Extract origin from URL
fn extract_origin(url: &str) -> String {
    // Simple origin extraction (TODO: use proper URL parser)
    if let Some(pos) = url.find("://") {
        if let Some(end) = url[pos + 3..].find('/') {
            url[..pos + 3 + end].to_string()
        } else {
            url.to_string()
        }
    } else {
        url.to_string()
    }
}

/// Check if two URLs are same-origin
fn is_same_origin(url1: &str, url2: &str) -> bool {
    extract_origin(url1) == extract_origin(url2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_referrer_policy_parsing() {
        assert_eq!(
            ReferrerPolicy::from_str("no-referrer"),
            Some(ReferrerPolicy::NoReferrer)
        );
        assert_eq!(
            ReferrerPolicy::from_str("origin"),
            Some(ReferrerPolicy::Origin)
        );
    }

    #[test]
    fn test_referrer_calculation() {
        let handler = ReferrerHandler::new(ReferrerPolicy::NoReferrer);
        let referrer = handler.calculate_referrer(
            "https://example.com/page",
            "https://other.com/",
            true,
        );
        assert_eq!(referrer, None);

        let handler = ReferrerHandler::new(ReferrerPolicy::Origin);
        let referrer = handler.calculate_referrer(
            "https://example.com/page",
            "https://other.com/",
            true,
        );
        assert_eq!(referrer, Some("https://example.com".to_string()));
    }

    #[test]
    fn test_same_origin() {
        assert!(is_same_origin(
            "https://example.com/page1",
            "https://example.com/page2"
        ));
        assert!(!is_same_origin(
            "https://example.com/page",
            "https://other.com/page"
        ));
    }
}
