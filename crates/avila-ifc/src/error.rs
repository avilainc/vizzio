use thiserror::Error;

#[derive(Error, Debug)]
pub enum IfcError {
    #[error("Failed to read IFC file: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Invalid IFC format: {0}")]
    InvalidFormat(String),

    #[error("Parse error at line {line}: {message}")]
    ParseError { line: usize, message: String },

    #[error("Invalid STEP entity: {0}")]
    InvalidEntity(String),

    #[error("Entity not found: {0}")]
    EntityNotFound(String),

    #[error("Invalid reference: #{0}")]
    InvalidReference(i64),

    #[error("Unsupported IFC schema: {0}")]
    UnsupportedSchema(String),

    #[error("Geometry error: {0}")]
    GeometryError(String),

    #[error("Invalid attribute: {0}")]
    InvalidAttribute(String),
}

pub type Result<T> = std::result::Result<T, IfcError>;
