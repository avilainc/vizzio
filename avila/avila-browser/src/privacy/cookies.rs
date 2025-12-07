//! Cookie management and isolation

use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

/// Cookie policy
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CookiePolicy {
    /// Accept all cookies
    AcceptAll,
    /// Accept only first-party cookies
    AcceptFirstParty,
    /// Block all cookies
    BlockAll,
}

/// HTTP cookie
#[derive(Debug, Clone)]
pub struct Cookie {
    pub name: String,
    pub value: String,
    pub domain: String,
    pub path: String,
    pub expires: Option<u64>,
    pub secure: bool,
    pub http_only: bool,
    pub same_site: Option<SameSite>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SameSite {
    Strict,
    Lax,
    None,
}

impl Cookie {
    pub fn new(name: String, value: String, domain: String) -> Self {
        Self {
            name,
            value,
            domain,
            path: "/".to_string(),
            expires: None,
            secure: false,
            http_only: false,
            same_site: Some(SameSite::Lax),
        }
    }

    /// Check if cookie is expired
    pub fn is_expired(&self) -> bool {
        if let Some(expires) = self.expires {
            let now = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs();
            now > expires
        } else {
            false
        }
    }

    /// Check if cookie matches domain
    pub fn matches_domain(&self, domain: &str) -> bool {
        domain == self.domain || domain.ends_with(&format!(".{}", self.domain))
    }

    /// Check if cookie matches path
    pub fn matches_path(&self, path: &str) -> bool {
        path.starts_with(&self.path)
    }
}

/// Cookie jar with isolation
pub struct CookieJar {
    cookies: HashMap<String, Vec<Cookie>>,
    policy: CookiePolicy,
}

impl CookieJar {
    pub fn new(policy: CookiePolicy) -> Self {
        Self {
            cookies: HashMap::new(),
            policy,
        }
    }

    /// Add a cookie to the jar
    pub fn add(&mut self, cookie: Cookie, is_first_party: bool) {
        match self.policy {
            CookiePolicy::BlockAll => return,
            CookiePolicy::AcceptFirstParty if !is_first_party => return,
            _ => {}
        }

        let domain_cookies = self.cookies.entry(cookie.domain.clone()).or_insert_with(Vec::new);

        // Remove existing cookie with same name
        domain_cookies.retain(|c| c.name != cookie.name);

        // Add new cookie
        domain_cookies.push(cookie);
    }

    /// Get cookies for a domain and path
    pub fn get(&self, domain: &str, path: &str, secure: bool) -> Vec<&Cookie> {
        let mut result = Vec::new();

        for cookies in self.cookies.values() {
            for cookie in cookies {
                if !cookie.is_expired()
                    && cookie.matches_domain(domain)
                    && cookie.matches_path(path)
                    && (!cookie.secure || secure)
                {
                    result.push(cookie);
                }
            }
        }

        result
    }

    /// Clear all cookies
    pub fn clear(&mut self) {
        self.cookies.clear();
    }

    /// Clear cookies for a specific domain
    pub fn clear_domain(&mut self, domain: &str) {
        self.cookies.remove(domain);
    }

    /// Remove expired cookies
    pub fn remove_expired(&mut self) {
        for cookies in self.cookies.values_mut() {
            cookies.retain(|c| !c.is_expired());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cookie_jar() {
        let mut jar = CookieJar::new(CookiePolicy::AcceptAll);
        let cookie = Cookie::new("session".to_string(), "abc123".to_string(), "example.com".to_string());

        jar.add(cookie, true);
        let cookies = jar.get("example.com", "/", false);
        assert_eq!(cookies.len(), 1);
    }

    #[test]
    fn test_cookie_policy() {
        let mut jar = CookieJar::new(CookiePolicy::AcceptFirstParty);
        let cookie = Cookie::new("tracking".to_string(), "xyz".to_string(), "tracker.com".to_string());

        jar.add(cookie, false);
        let cookies = jar.get("tracker.com", "/", false);
        assert_eq!(cookies.len(), 0);
    }
}
