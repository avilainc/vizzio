//! SMTP Authentication mechanisms

use crate::encoding::base64_encode;
use crate::Result;

/// SMTP Authentication mechanism
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AuthMechanism {
    /// PLAIN authentication
    Plain,
    /// LOGIN authentication
    Login,
    /// CRAM-MD5 authentication
    CramMd5,
    /// XOAUTH2 (for Gmail/Outlook)
    XOAuth2,
}

impl AuthMechanism {
    /// Returns the SASL name
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Plain => "PLAIN",
            Self::Login => "LOGIN",
            Self::CramMd5 => "CRAM-MD5",
            Self::XOAuth2 => "XOAUTH2",
        }
    }
}

/// Generates PLAIN authentication string
pub fn auth_plain(username: &str, password: &str) -> String {
    let auth_str = format!("\0{}\0{}", username, password);
    base64_encode(auth_str.as_bytes())
}

/// Generates LOGIN authentication (step 1: username)
pub fn auth_login_username(username: &str) -> String {
    base64_encode(username.as_bytes())
}

/// Generates LOGIN authentication (step 2: password)
pub fn auth_login_password(password: &str) -> String {
    base64_encode(password.as_bytes())
}

/// Generates CRAM-MD5 response
/// TODO: Re-enable when avila_crypto is available
pub fn auth_cram_md5(_username: &str, _password: &str, _challenge: &str) -> Result<String> {
    // TODO: Implement with avila_crypto::mac::hmac
    Err(Box::new(std::io::Error::new(
        std::io::ErrorKind::Unsupported,
        "CRAM-MD5 not yet implemented - needs avila_crypto"
    )))
}

/// Generates XOAUTH2 string for Gmail/Outlook
pub fn auth_xoauth2(username: &str, access_token: &str) -> String {
    let auth_str = format!(
        "user={}\x01auth=Bearer {}\x01\x01",
        username, access_token
    );
    base64_encode(auth_str.as_bytes())
}

/// Supported authentication capabilities from EHLO response
#[derive(Debug, Default)]
pub struct AuthCapabilities {
    /// Supports PLAIN
    pub plain: bool,
    /// Supports LOGIN
    pub login: bool,
    /// Supports CRAM-MD5
    pub cram_md5: bool,
    /// Supports XOAUTH2
    pub xoauth2: bool,
    /// Supports STARTTLS
    pub starttls: bool,
    /// Supports 8BITMIME
    pub eight_bit_mime: bool,
    /// Supports PIPELINING
    pub pipelining: bool,
    /// Maximum message size
    pub size: Option<usize>,
}

impl AuthCapabilities {
    /// Parses EHLO response
    pub fn from_ehlo_response(response: &str) -> Self {
        let mut caps = Self::default();

        for line in response.lines() {
            let line = line.trim();

            if line.contains("AUTH") {
                if line.contains("PLAIN") {
                    caps.plain = true;
                }
                if line.contains("LOGIN") {
                    caps.login = true;
                }
                if line.contains("CRAM-MD5") {
                    caps.cram_md5 = true;
                }
                if line.contains("XOAUTH2") {
                    caps.xoauth2 = true;
                }
            }

            if line.contains("STARTTLS") {
                caps.starttls = true;
            }

            if line.contains("8BITMIME") {
                caps.eight_bit_mime = true;
            }

            if line.contains("PIPELINING") {
                caps.pipelining = true;
            }

            if line.starts_with("250-SIZE") || line.starts_with("250 SIZE") {
                if let Some(size_str) = line.split_whitespace().nth(1) {
                    caps.size = size_str.parse().ok();
                }
            }
        }

        caps
    }

    /// Gets the best authentication mechanism available
    pub fn best_auth_mechanism(&self) -> Option<AuthMechanism> {
        if self.cram_md5 {
            Some(AuthMechanism::CramMd5)
        } else if self.plain {
            Some(AuthMechanism::Plain)
        } else if self.login {
            Some(AuthMechanism::Login)
        } else if self.xoauth2 {
            Some(AuthMechanism::XOAuth2)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_auth_plain() {
        let auth = auth_plain("user", "pass");
        assert!(!auth.is_empty());
    }

    #[test]
    fn test_auth_login() {
        let username = auth_login_username("user");
        let password = auth_login_password("pass");
        assert!(!username.is_empty());
        assert!(!password.is_empty());
    }
    #[test]
    fn test_auth_xoauth2() {
        let auth = auth_xoauth2("user@gmail.com", "ya29.token123");
        let decoded = crate::encoding::base64_decode(&auth).expect("valid base64");
        assert_eq!(
            decoded,
            b"user=user@gmail.com\x01auth=Bearer ya29.token123\x01\x01"
        );
    }

    #[test]
    fn test_auth_cram_md5() {
        // Test with known challenge/response from RFC 2195
        let challenge = base64_encode(b"<1896.697170952@postoffice.example.net>");
        let username = "tim";
        let password = "tanstaaftanstaaf";

        let response = auth_cram_md5(username, password, &challenge).expect("valid cram-md5");
        let decoded = crate::encoding::base64_decode(&response).expect("valid base64");
        let response_str = String::from_utf8_lossy(&decoded);

        // Should be "username hexdigest"
        assert!(response_str.starts_with("tim "));
        assert_eq!(response_str.split_whitespace().count(), 2);

        // Verify hex digest is 32 chars (16 bytes MD5)
        let digest = response_str.split_whitespace().nth(1).unwrap();
        assert_eq!(digest.len(), 32);
    }

    #[test]
    fn test_auth_capabilities() {
        let response = "250-STARTTLS\r\n250-AUTH PLAIN LOGIN\r\n250 8BITMIME\r\n";
        let caps = AuthCapabilities::from_ehlo_response(response);

        assert!(caps.starttls);
        assert!(caps.plain);
        assert!(caps.login);
        assert!(caps.eight_bit_mime);
        assert_eq!(caps.best_auth_mechanism(), Some(AuthMechanism::Plain));
    }
}
