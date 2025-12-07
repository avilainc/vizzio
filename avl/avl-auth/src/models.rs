//! Data models for AVL Auth
//!
//! Comprehensive data structures for:
//! - User accounts and credentials
//! - Sessions and tokens
//! - Roles, permissions, and policies
//! - API keys and secrets
//! - MFA configurations
//! - OAuth2 providers
//! - Audit logs and risk assessments

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::net::IpAddr;
use uuid::Uuid;

/// User credentials for authentication
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Credentials {
    pub email: String,
    pub password: String,
    pub device_id: Option<String>,
    pub ip_address: Option<IpAddr>,
}

/// User account
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub email_verified: bool,
    pub password_hash: String,
    pub display_name: Option<String>,
    pub avatar_url: Option<String>,
    pub roles: Vec<String>,
    pub permissions: Vec<String>,
    pub metadata: HashMap<String, serde_json::Value>,
    pub mfa_enabled: bool,
    pub mfa_secret: Option<String>,
    pub webauthn_credentials: Vec<WebAuthnCredential>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub last_login_at: Option<DateTime<Utc>>,
    pub login_count: u64,
    pub failed_login_attempts: u32,
    pub locked_until: Option<DateTime<Utc>>,
    pub password_changed_at: DateTime<Utc>,
    pub status: UserStatus,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum UserStatus {
    Active,
    Suspended,
    Locked,
    Deleted,
}

/// Authentication session
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Session {
    pub id: Uuid,
    pub user_id: Uuid,
    pub access_token: String,
    pub refresh_token: String,
    pub token_type: String,
    pub expires_at: DateTime<Utc>,
    pub refresh_expires_at: DateTime<Utc>,
    pub device_id: Option<String>,
    pub ip_address: Option<IpAddr>,
    pub user_agent: Option<String>,
    pub created_at: DateTime<Utc>,
    pub last_active_at: DateTime<Utc>,
    pub scopes: Vec<String>,
}

/// JWT claims
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    pub sub: Uuid,
    pub email: String,
    pub roles: Vec<String>,
    pub permissions: Vec<String>,
    pub session_id: Uuid,
    pub iat: i64,
    pub exp: i64,
    pub nbf: i64,
    pub iss: String,
    pub aud: String,
    pub jti: String,
    pub scopes: Vec<String>,
    pub device_id: Option<String>,
}

/// Role definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Role {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub permissions: Vec<String>,
    pub inherits: Vec<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Permission definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Permission {
    pub id: String,
    pub resource: String,
    pub action: String,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
}

/// Access policy (ABAC)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Policy {
    pub id: Uuid,
    pub name: String,
    pub effect: PolicyEffect,
    pub conditions: Vec<PolicyCondition>,
    pub actions: Vec<String>,
    pub resources: Vec<String>,
    pub priority: i32,
    pub enabled: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PolicyEffect {
    Allow,
    Deny,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum PolicyCondition {
    IpRange { cidrs: Vec<String> },
    TimeWindow { start: String, end: String },
    UserAttribute { key: String, value: serde_json::Value },
    RiskScore { max: u8 },
}

/// API Key
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiKey {
    pub id: Uuid,
    pub key_hash: String,
    pub prefix: String,
    pub user_id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub scopes: Vec<String>,
    pub rate_limit: Option<u32>,
    pub expires_at: Option<DateTime<Utc>>,
    pub last_used_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub revoked: bool,
}

/// MFA TOTP configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TotpConfig {
    pub secret: String,
    pub algorithm: TotpAlgorithm,
    pub digits: u32,
    pub period: u32,
    pub issuer: String,
    pub account_name: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TotpAlgorithm {
    SHA1,
    SHA256,
    SHA512,
}

/// WebAuthn credential
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebAuthnCredential {
    pub id: String,
    pub public_key: Vec<u8>,
    pub counter: u32,
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub last_used_at: Option<DateTime<Utc>>,
}

/// OAuth2 provider configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OAuth2Provider {
    pub name: String,
    pub client_id: String,
    pub client_secret: String,
    pub auth_url: String,
    pub token_url: String,
    pub redirect_url: String,
    pub scopes: Vec<String>,
}

/// Audit log entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditLog {
    pub id: Uuid,
    pub user_id: Option<Uuid>,
    pub session_id: Option<Uuid>,
    pub action: String,
    pub resource: String,
    pub result: AuditResult,
    pub ip_address: Option<IpAddr>,
    pub user_agent: Option<String>,
    pub metadata: HashMap<String, serde_json::Value>,
    pub risk_score: u8,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum AuditResult {
    Success,
    Failure,
    Blocked,
}

/// Risk assessment
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskAssessment {
    pub score: u8,
    pub level: RiskLevel,
    pub factors: Vec<RiskFactor>,
    pub recommended_action: RiskAction,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum RiskLevel {
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskFactor {
    pub name: String,
    pub score: u8,
    pub reason: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum RiskAction {
    Allow,
    Challenge,
    Deny,
    RequireMfa,
}

/// Device trust information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceInfo {
    pub device_id: String,
    pub device_name: String,
    pub device_type: DeviceType,
    pub os: String,
    pub os_version: String,
    pub browser: Option<String>,
    pub browser_version: Option<String>,
    pub fingerprint: String,
    pub trusted: bool,
    pub first_seen: DateTime<Utc>,
    pub last_seen: DateTime<Utc>,
    pub location: Option<GeoLocation>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum DeviceType {
    Desktop,
    Mobile,
    Tablet,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeoLocation {
    pub country: String,
    pub region: Option<String>,
    pub city: Option<String>,
    pub latitude: f64,
    pub longitude: f64,
}

/// Login attempt tracking
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginAttempt {
    pub id: Uuid,
    pub user_id: Option<Uuid>,
    pub email: String,
    pub ip_address: IpAddr,
    pub user_agent: Option<String>,
    pub success: bool,
    pub failure_reason: Option<String>,
    pub device_id: Option<String>,
    pub location: Option<GeoLocation>,
    pub timestamp: DateTime<Utc>,
}

/// Password reset request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PasswordResetRequest {
    pub id: Uuid,
    pub user_id: Uuid,
    pub token_hash: String,
    pub expires_at: DateTime<Utc>,
    pub used: bool,
    pub used_at: Option<DateTime<Utc>>,
    pub ip_address: IpAddr,
    pub created_at: DateTime<Utc>,
}

/// Email verification token
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmailVerificationToken {
    pub id: Uuid,
    pub user_id: Uuid,
    pub email: String,
    pub token_hash: String,
    pub expires_at: DateTime<Utc>,
    pub verified: bool,
    pub verified_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

/// Rate limit configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateLimitConfig {
    pub max_requests: u32,
    pub window_seconds: u32,
    pub scope: RateLimitScope,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum RateLimitScope {
    Global,
    PerUser,
    PerIp,
    PerApiKey,
}

/// Security event for monitoring
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityEvent {
    pub id: Uuid,
    pub event_type: SecurityEventType,
    pub severity: SecuritySeverity,
    pub user_id: Option<Uuid>,
    pub ip_address: Option<IpAddr>,
    pub description: String,
    pub metadata: HashMap<String, serde_json::Value>,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SecurityEventType {
    FailedLogin,
    AccountLockout,
    PasswordReset,
    MfaEnabled,
    MfaDisabled,
    SuspiciousActivity,
    ApiKeyCreated,
    ApiKeyRevoked,
    PrivilegeEscalation,
    UnauthorizedAccess,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SecuritySeverity {
    Info,
    Warning,
    Critical,
}

/// User profile for external display
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserProfile {
    pub id: Uuid,
    pub email: String,
    pub display_name: Option<String>,
    pub avatar_url: Option<String>,
    pub roles: Vec<String>,
    pub created_at: DateTime<Utc>,
    pub last_login_at: Option<DateTime<Utc>>,
}

impl From<User> for UserProfile {
    fn from(user: User) -> Self {
        Self {
            id: user.id,
            email: user.email,
            display_name: user.display_name,
            avatar_url: user.avatar_url,
            roles: user.roles,
            created_at: user.created_at,
            last_login_at: user.last_login_at,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user_status_serialization() {
        let status = UserStatus::Active;
        let json = serde_json::to_string(&status).unwrap();
        assert_eq!(json, "\"active\"");
    }

    #[test]
    fn test_policy_effect_serialization() {
        let effect = PolicyEffect::Allow;
        let json = serde_json::to_string(&effect).unwrap();
        assert_eq!(json, "\"allow\"");
    }

    #[test]
    fn test_risk_level_ordering() {
        assert!(RiskLevel::Low < RiskLevel::Medium);
        assert!(RiskLevel::Medium < RiskLevel::High);
        assert!(RiskLevel::High < RiskLevel::Critical);
    }

    #[test]
    fn test_user_profile_conversion() {
        let user = User {
            id: Uuid::new_v4(),
            email: "test@example.com".to_string(),
            email_verified: true,
            password_hash: "hash".to_string(),
            display_name: Some("Test User".to_string()),
            avatar_url: None,
            roles: vec!["user".to_string()],
            permissions: vec![],
            metadata: HashMap::new(),
            mfa_enabled: false,
            mfa_secret: None,
            webauthn_credentials: vec![],
            created_at: Utc::now(),
            updated_at: Utc::now(),
            last_login_at: None,
            login_count: 0,
            failed_login_attempts: 0,
            locked_until: None,
            password_changed_at: Utc::now(),
            status: UserStatus::Active,
        };

        let profile: UserProfile = user.into();
        assert_eq!(profile.email, "test@example.com");
    }
}
