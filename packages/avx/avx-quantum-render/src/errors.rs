//! Error types and handling for the rendering system

use std::fmt;

/// Result type for rendering operations
pub type RenderResult<T> = Result<T, RenderError>;

/// Comprehensive error types for the rendering pipeline
#[derive(Debug, Clone)]
pub enum RenderError {
    /// Invalid parameter value
    InvalidParameter {
        param: String,
        reason: String,
    },
    /// Geometry error
    GeometryError(String),
    /// Material error
    MaterialError(String),
    /// Texture error
    TextureError(String),
    /// Camera error
    CameraError(String),
    /// Serialization error
    SerializationError(String),
    /// Memory allocation error
    MemoryError(String),
    /// GPU error
    GpuError(String),
    /// Threading error
    ThreadError(String),
}

impl fmt::Display for RenderError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidParameter { param, reason } => {
                write!(f, "Invalid parameter '{}': {}", param, reason)
            }
            Self::GeometryError(msg) => write!(f, "Geometry error: {}", msg),
            Self::MaterialError(msg) => write!(f, "Material error: {}", msg),
            Self::TextureError(msg) => write!(f, "Texture error: {}", msg),
            Self::CameraError(msg) => write!(f, "Camera error: {}", msg),
            Self::SerializationError(msg) => write!(f, "Serialization error: {}", msg),
            Self::MemoryError(msg) => write!(f, "Memory error: {}", msg),
            Self::GpuError(msg) => write!(f, "GPU error: {}", msg),
            Self::ThreadError(msg) => write!(f, "Thread error: {}", msg),
        }
    }
}

impl std::error::Error for RenderError {}

/// Validation utilities
pub struct Validator;

impl Validator {
    /// Validate that a value is within range
    pub fn in_range(value: f64, min: f64, max: f64, param: &str) -> RenderResult<f64> {
        if value >= min && value <= max {
            Ok(value)
        } else {
            Err(RenderError::InvalidParameter {
                param: param.to_string(),
                reason: format!("must be between {} and {}", min, max),
            })
        }
    }

    /// Validate that a value is positive
    pub fn positive(value: f64, param: &str) -> RenderResult<f64> {
        if value > 0.0 {
            Ok(value)
        } else {
            Err(RenderError::InvalidParameter {
                param: param.to_string(),
                reason: "must be positive".to_string(),
            })
        }
    }

    /// Validate that a value is non-negative
    pub fn non_negative(value: f64, param: &str) -> RenderResult<f64> {
        if value >= 0.0 {
            Ok(value)
        } else {
            Err(RenderError::InvalidParameter {
                param: param.to_string(),
                reason: "must be non-negative".to_string(),
            })
        }
    }

    /// Validate vector components
    pub fn vector_valid(v: [f64; 3], param: &str) -> RenderResult<[f64; 3]> {
        for (i, &component) in v.iter().enumerate() {
            if !component.is_finite() {
                return Err(RenderError::InvalidParameter {
                    param: format!("{}[{}]", param, i),
                    reason: "contains non-finite value".to_string(),
                });
            }
        }
        Ok(v)
    }

    /// Validate dimensions
    pub fn dimensions_valid(width: u32, height: u32) -> RenderResult<()> {
        if width == 0 || height == 0 {
            return Err(RenderError::InvalidParameter {
                param: "dimensions".to_string(),
                reason: "width and height must be > 0".to_string(),
            });
        }
        if width > 16384 || height > 16384 {
            return Err(RenderError::InvalidParameter {
                param: "dimensions".to_string(),
                reason: "dimensions too large (max 16384x16384)".to_string(),
            });
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validation_in_range() {
        assert!(Validator::in_range(0.5, 0.0, 1.0, "test").is_ok());
        assert!(Validator::in_range(1.5, 0.0, 1.0, "test").is_err());
    }

    #[test]
    fn test_validation_positive() {
        assert!(Validator::positive(1.0, "test").is_ok());
        assert!(Validator::positive(0.0, "test").is_err());
        assert!(Validator::positive(-1.0, "test").is_err());
    }

    #[test]
    fn test_vector_validation() {
        let valid = [1.0, 2.0, 3.0];
        assert!(Validator::vector_valid(valid, "test").is_ok());

        let invalid = [f64::NAN, 2.0, 3.0];
        assert!(Validator::vector_valid(invalid, "test").is_err());
    }

    #[test]
    fn test_dimensions_validation() {
        assert!(Validator::dimensions_valid(512, 512).is_ok());
        assert!(Validator::dimensions_valid(0, 512).is_err());
        assert!(Validator::dimensions_valid(20000, 512).is_err());
    }
}
