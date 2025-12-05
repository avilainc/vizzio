use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

/// Alert levels representing severity of notifications
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum AlertLevel {
    /// Trace level - detailed debugging information
    Trace,
    /// Debug level - debugging information
    Debug,
    /// Info level - informational messages
    Info,
    /// Warning level - warning messages
    Warning,
    /// Error level - error messages
    Error,
    /// Critical level - critical error messages
    Critical,
}

impl AlertLevel {
    /// Returns the string representation of the alert level
    pub fn as_str(&self) -> &'static str {
        match self {
            AlertLevel::Trace => "TRACE",
            AlertLevel::Debug => "DEBUG",
            AlertLevel::Info => "INFO",
            AlertLevel::Warning => "WARNING",
            AlertLevel::Error => "ERROR",
            AlertLevel::Critical => "CRITICAL",
        }
    }

    /// Returns the emoji representation of the alert level
    pub fn emoji(&self) -> &'static str {
        match self {
            AlertLevel::Trace => "🔍",
            AlertLevel::Debug => "🐛",
            AlertLevel::Info => "ℹ️",
            AlertLevel::Warning => "⚠️",
            AlertLevel::Error => "❌",
            AlertLevel::Critical => "🔥",
        }
    }

    /// Returns all alert levels in order
    pub fn all() -> [AlertLevel; 6] {
        [
            AlertLevel::Trace,
            AlertLevel::Debug,
            AlertLevel::Info,
            AlertLevel::Warning,
            AlertLevel::Error,
            AlertLevel::Critical,
        ]
    }
}

impl std::fmt::Display for AlertLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// Simple timestamp structure using system time
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Timestamp {
    seconds: u64,
    nanos: u32,
}

impl Timestamp {
    /// Creates a timestamp from the current system time
    pub fn now() -> Self {
        let duration = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default();
        
        Self {
            seconds: duration.as_secs(),
            nanos: duration.subsec_nanos(),
        }
    }

    /// Returns the timestamp as seconds since UNIX epoch
    pub fn as_secs(&self) -> u64 {
        self.seconds
    }

    /// Returns the nanoseconds component
    pub fn nanos(&self) -> u32 {
        self.nanos
    }

    /// Formats the timestamp as ISO 8601 string (UTC)
    pub fn format_iso8601(&self) -> String {
        // Simple UTC formatting without external dependencies
        let total_secs = self.seconds;
        let days_since_epoch = total_secs / 86400;
        let seconds_today = total_secs % 86400;
        
        let hours = seconds_today / 3600;
        let minutes = (seconds_today % 3600) / 60;
        let seconds = seconds_today % 60;
        
        // Simple algorithm to calculate year, month, day
        let (year, month, day) = Self::days_to_ymd(days_since_epoch);
        
        format!(
            "{:04}-{:02}-{:02}T{:02}:{:02}:{:02}Z",
            year, month, day, hours, minutes, seconds
        )
    }

    /// Simple conversion from days since epoch to year-month-day
    fn days_to_ymd(mut days: u64) -> (u32, u32, u32) {
        // Start from 1970
        let mut year = 1970;
        
        loop {
            let days_in_year = if Self::is_leap_year(year) { 366 } else { 365 };
            if days < days_in_year {
                break;
            }
            days -= days_in_year;
            year += 1;
        }
        
        let days_in_months = if Self::is_leap_year(year) {
            [31, 29, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31]
        } else {
            [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31]
        };
        
        let mut month = 1;
        for &days_in_month in &days_in_months {
            if days < days_in_month as u64 {
                break;
            }
            days -= days_in_month as u64;
            month += 1;
        }
        
        let day = days + 1;
        (year, month, day as u32)
    }

    fn is_leap_year(year: u32) -> bool {
        (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)
    }
}

impl Default for Timestamp {
    fn default() -> Self {
        Self::now()
    }
}

impl std::fmt::Display for Timestamp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.format_iso8601())
    }
}

/// Main Alert structure
#[derive(Debug, Clone)]
pub struct Alert {
    /// Alert level
    pub level: AlertLevel,
    /// Alert message
    pub message: String,
    /// Optional timestamp
    pub timestamp: Option<Timestamp>,
    /// Optional tags for categorization
    pub tags: Vec<String>,
    /// Optional context data
    pub context: HashMap<String, String>,
}

impl Alert {
    /// Creates a new Alert with current timestamp
    pub fn new(level: AlertLevel, message: impl Into<String>) -> Self {
        Self {
            level,
            message: message.into(),
            timestamp: Some(Timestamp::now()),
            tags: Vec::new(),
            context: HashMap::new(),
        }
    }

    /// Creates a new Alert without timestamp
    pub fn new_without_timestamp(level: AlertLevel, message: impl Into<String>) -> Self {
        Self {
            level,
            message: message.into(),
            timestamp: None,
            tags: Vec::new(),
            context: HashMap::new(),
        }
    }

    /// Adds a tag to the alert
    pub fn with_tag(mut self, tag: impl Into<String>) -> Self {
        self.tags.push(tag.into());
        self
    }

    /// Adds multiple tags to the alert
    pub fn with_tags(mut self, tags: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.tags.extend(tags.into_iter().map(|t| t.into()));
        self
    }

    /// Adds context data to the alert
    pub fn with_context(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.context.insert(key.into(), value.into());
        self
    }

    /// Adds multiple context entries to the alert
    pub fn with_context_map(
        mut self,
        context: impl IntoIterator<Item = (impl Into<String>, impl Into<String>)>,
    ) -> Self {
        self.context.extend(
            context
                .into_iter()
                .map(|(k, v)| (k.into(), v.into())),
        );
        self
    }

    /// Sets or removes the timestamp
    pub fn with_timestamp(mut self, timestamp: Option<Timestamp>) -> Self {
        self.timestamp = timestamp;
        self
    }
}

/// Convenience constructors for different alert levels
impl Alert {
    pub fn trace(message: impl Into<String>) -> Self {
        Self::new(AlertLevel::Trace, message)
    }

    pub fn debug(message: impl Into<String>) -> Self {
        Self::new(AlertLevel::Debug, message)
    }

    pub fn info(message: impl Into<String>) -> Self {
        Self::new(AlertLevel::Info, message)
    }

    pub fn warning(message: impl Into<String>) -> Self {
        Self::new(AlertLevel::Warning, message)
    }

    pub fn error(message: impl Into<String>) -> Self {
        Self::new(AlertLevel::Error, message)
    }

    pub fn critical(message: impl Into<String>) -> Self {
        Self::new(AlertLevel::Critical, message)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_alert_level_ordering() {
        assert!(AlertLevel::Trace < AlertLevel::Debug);
        assert!(AlertLevel::Debug < AlertLevel::Info);
        assert!(AlertLevel::Info < AlertLevel::Warning);
        assert!(AlertLevel::Warning < AlertLevel::Error);
        assert!(AlertLevel::Error < AlertLevel::Critical);
    }

    #[test]
    fn test_timestamp_creation() {
        let ts = Timestamp::now();
        assert!(ts.as_secs() > 0);
    }

    #[test]
    fn test_timestamp_formatting() {
        let ts = Timestamp::now();
        let formatted = ts.format_iso8601();
        assert!(formatted.contains("T"));
        assert!(formatted.ends_with("Z"));
    }

    #[test]
    fn test_alert_creation() {
        let alert = Alert::info("Test");
        assert_eq!(alert.level, AlertLevel::Info);
        assert_eq!(alert.message, "Test");
        assert!(alert.timestamp.is_some());
    }
}
