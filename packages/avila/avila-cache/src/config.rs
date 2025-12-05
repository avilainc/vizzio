//! Cache configuration
use crate::error::CacheError;

#[derive(Debug, Clone)]
pub struct CacheConfig {
    /// Maximum number of entries (None = unlimited)
    pub max_capacity: Option<usize>,
    /// Enable statistics collection
    pub enable_stats: bool,
}

impl CacheConfig {
    pub fn new() -> Self {
        Self {
            max_capacity: None,
            enable_stats: false,
        }
    }

    pub fn with_capacity(mut self, capacity: usize) -> Self {
        self.max_capacity = Some(capacity);
        self
    }

    pub fn with_stats(mut self, enable: bool) -> Self {
        self.enable_stats = enable;
        self
    }

    pub fn validate(&self) -> Result<(), CacheError> {
        if let Some(cap) = self.max_capacity {
            if cap == 0 {
                return Err(CacheError::InvalidConfig);
            }
        }
        Ok(())
    }
}

impl Default for CacheConfig {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_builder() {
        let config = CacheConfig::new()
            .with_capacity(100)
            .with_stats(true);

        assert_eq!(config.max_capacity, Some(100));
        assert!(config.enable_stats);
    }

    #[test]
    fn test_invalid_config() {
        let config = CacheConfig::new().with_capacity(0);
        assert!(config.validate().is_err());
    }
}
