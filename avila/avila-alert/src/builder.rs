use crate::types::{Alert, AlertLevel};

/// Builder pattern for creating Alerts
#[derive(Debug, Default)]
pub struct AlertBuilder {
    level: Option<AlertLevel>,
    message: Option<String>,
    tags: Vec<String>,
    context: std::collections::HashMap<String, String>,
}

impl AlertBuilder {
    /// Creates a new AlertBuilder
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the alert level
    pub fn level(mut self, level: AlertLevel) -> Self {
        self.level = Some(level);
        self
    }

    /// Sets the message
    pub fn message(mut self, message: impl Into<String>) -> Self {
        self.message = Some(message.into());
        self
    }

    /// Adds a tag
    pub fn tag(mut self, tag: impl Into<String>) -> Self {
        self.tags.push(tag.into());
        self
    }

    /// Adds multiple tags
    pub fn tags(mut self, tags: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.tags.extend(tags.into_iter().map(|t| t.into()));
        self
    }

    /// Adds a context entry
    pub fn context(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.context.insert(key.into(), value.into());
        self
    }

    /// Adds multiple context entries
    pub fn context_map(
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

    /// Builds the Alert
    ///
    /// # Errors
    ///
    /// Returns an error if level or message is not set
    pub fn build(self) -> Result<Alert, &'static str> {
        let level = self.level.ok_or("Alert level is required")?;
        let message = self.message.ok_or("Alert message is required")?;

        let mut alert = Alert::new(level, message);
        alert.tags = self.tags;
        alert.context = self.context;

        Ok(alert)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_builder_basic() {
        let alert = AlertBuilder::new()
            .level(AlertLevel::Info)
            .message("Test message")
            .build()
            .unwrap();

        assert_eq!(alert.level, AlertLevel::Info);
        assert_eq!(alert.message, "Test message");
    }

    #[test]
    fn test_builder_with_tags() {
        let alert = AlertBuilder::new()
            .level(AlertLevel::Warning)
            .message("Test")
            .tag("auth")
            .tag("security")
            .build()
            .unwrap();

        assert_eq!(alert.tags.len(), 2);
        assert!(alert.tags.contains(&"auth".to_string()));
    }

    #[test]
    fn test_builder_missing_level() {
        let result = AlertBuilder::new()
            .message("Test")
            .build();

        assert!(result.is_err());
    }
}
