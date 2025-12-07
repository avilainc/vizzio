use thiserror::Error;

/// Unified error type for Avila Analytics
#[derive(Error, Debug)]
pub enum AvilaError {
    /// Storage-related errors
    #[error("Storage error: {0}")]
    Storage(String),

    /// Event tracking errors
    #[error("Tracker error: {0}")]
    Tracker(String),

    /// Analysis errors (funnel, cohort, segmentation)
    #[error("Analysis error: {0}")]
    Analysis(String),

    /// Prediction/ML errors
    #[error("Prediction error: {0}")]
    Prediction(String),

    /// Industry 4.0 module errors
    #[error("Industry 4.0 error: {0}")]
    Industry40(String),

    /// Configuration errors
    #[error("Configuration error: {0}")]
    Config(String),

    /// API/Server errors
    #[error("API error: {0}")]
    Api(String),

    /// WebSocket errors
    #[error("WebSocket error: {0}")]
    WebSocket(String),

    /// Export errors
    #[error("Export error: {0}")]
    Export(String),

    /// Streaming/processing errors
    #[error("Streaming error: {0}")]
    Streaming(String),

    /// Validation errors
    #[error("Validation error: {0}")]
    Validation(String),

    /// Not found errors
    #[error("Not found: {0}")]
    NotFound(String),

    /// IO errors
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    /// Serialization errors
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    /// Generic errors with context
    #[error("{context}: {source}")]
    WithContext {
        context: String,
        source: Box<AvilaError>,
    },
}

impl AvilaError {
    /// Add context to an error
    pub fn context<S: Into<String>>(self, context: S) -> Self {
        AvilaError::WithContext {
            context: context.into(),
            source: Box::new(self),
        }
    }

    /// Create a storage error
    pub fn storage<S: Into<String>>(msg: S) -> Self {
        AvilaError::Storage(msg.into())
    }

    /// Create a tracker error
    pub fn tracker<S: Into<String>>(msg: S) -> Self {
        AvilaError::Tracker(msg.into())
    }

    /// Create an analysis error
    pub fn analysis<S: Into<String>>(msg: S) -> Self {
        AvilaError::Analysis(msg.into())
    }

    /// Create a validation error
    pub fn validation<S: Into<String>>(msg: S) -> Self {
        AvilaError::Validation(msg.into())
    }

    /// Create a not found error
    pub fn not_found<S: Into<String>>(msg: S) -> Self {
        AvilaError::NotFound(msg.into())
    }
}

/// Result type for Avila Analytics operations
pub type AvilaResult<T> = Result<T, AvilaError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_creation() {
        let err = AvilaError::tracker("Invalid event");
        assert_eq!(err.to_string(), "Tracker error: Invalid event");
    }

    #[test]
    fn test_error_context() {
        let err = AvilaError::storage("Connection failed")
            .context("Failed to connect to database");
        assert!(err.to_string().contains("Failed to connect to database"));
    }

    #[test]
    fn test_error_types() {
        assert!(matches!(
            AvilaError::validation("test"),
            AvilaError::Validation(_)
        ));
        assert!(matches!(
            AvilaError::not_found("user"),
            AvilaError::NotFound(_)
        ));
    }
}
