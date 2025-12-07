//! Tracker blocking using filter lists

use std::collections::HashSet;

/// Tracker database (EasyList, EasyPrivacy, etc.)
pub struct TrackerDatabase {
    blocked_domains: HashSet<String>,
    blocked_patterns: Vec<String>,
}

impl TrackerDatabase {
    pub fn new() -> Self {
        Self {
            blocked_domains: HashSet::new(),
            blocked_patterns: Vec::new(),
        }
    }

    /// Load tracker list from string (EasyList format)
    pub fn load_from_string(&mut self, content: &str) {
        for line in content.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('!') || line.starts_with('[') {
                continue;
            }

            if line.starts_with("||") && line.ends_with('^') {
                // Domain blocking rule
                let domain = line.trim_start_matches("||").trim_end_matches('^');
                self.blocked_domains.insert(domain.to_string());
            } else {
                // Pattern blocking rule
                self.blocked_patterns.push(line.to_string());
            }
        }
    }

    /// Check if URL should be blocked
    pub fn should_block(&self, url: &str) -> bool {
        // Check domain blocking
        if let Some(domain) = extract_domain(url) {
            if self.blocked_domains.contains(domain) {
                return true;
            }
        }

        // Check pattern blocking
        for pattern in &self.blocked_patterns {
            if url.contains(pattern) {
                return true;
            }
        }

        false
    }

    pub fn domain_count(&self) -> usize {
        self.blocked_domains.len()
    }

    pub fn pattern_count(&self) -> usize {
        self.blocked_patterns.len()
    }
}

impl Default for TrackerDatabase {
    fn default() -> Self {
        Self::new()
    }
}

/// Tracker blocker
pub struct TrackerBlocker {
    database: TrackerDatabase,
    enabled: bool,
}

impl TrackerBlocker {
    pub fn new(database: TrackerDatabase) -> Self {
        Self {
            database,
            enabled: true,
        }
    }

    /// Check if request should be blocked
    pub fn should_block_request(&self, url: &str) -> bool {
        self.enabled && self.database.should_block(url)
    }

    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }
}

/// Extract domain from URL
fn extract_domain(url: &str) -> Option<&str> {
    // Simple domain extraction (TODO: use proper URL parser)
    if let Some(start) = url.find("://") {
        let rest = &url[start + 3..];
        if let Some(end) = rest.find('/') {
            Some(&rest[..end])
        } else {
            Some(rest)
        }
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tracker_blocking() {
        let mut db = TrackerDatabase::new();
        db.blocked_domains.insert("tracker.com".to_string());

        assert!(db.should_block("https://tracker.com/script.js"));
        assert!(!db.should_block("https://legitimate.com/page.html"));
    }

    #[test]
    fn test_extract_domain() {
        assert_eq!(extract_domain("https://example.com/path"), Some("example.com"));
        assert_eq!(extract_domain("http://sub.example.com"), Some("sub.example.com"));
    }
}
