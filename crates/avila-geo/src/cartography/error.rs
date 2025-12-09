//! Tipos de erro para o módulo de cartografia

use std::fmt;
use std::error::Error as StdError;

/// Erros do sistema de cartografia
#[derive(Debug, Clone)]
pub enum GeoError {
    /// Continente inválido
    InvalidContinent(String),

    /// Coordenada inválida
    InvalidCoordinate { lat: f64, lon: f64 },

    /// Erro de projeção
    ProjectionError(String),

    /// Erro de renderização
    RenderError(String),

    /// Operação não suportada
    UnsupportedOperation(String),

    /// Erro de I/O
    IoError(String),

    /// Dados não encontrados
    DataNotFound(String),
}

impl fmt::Display for GeoError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GeoError::InvalidContinent(name) => {
                write!(f, "Continente inválido: '{}'. Use 'europe', 'africa' ou 'asia'.", name)
            }
            GeoError::InvalidCoordinate { lat, lon } => {
                write!(f, "Coordenada inválida: lat={}, lon={}", lat, lon)
            }
            GeoError::ProjectionError(msg) => {
                write!(f, "Erro de projeção: {}", msg)
            }
            GeoError::RenderError(msg) => {
                write!(f, "Erro de renderização: {}", msg)
            }
            GeoError::UnsupportedOperation(msg) => {
                write!(f, "Operação não suportada: {}", msg)
            }
            GeoError::IoError(msg) => {
                write!(f, "Erro de I/O: {}", msg)
            }
            GeoError::DataNotFound(msg) => {
                write!(f, "Dados não encontrados: {}", msg)
            }
        }
    }
}

impl StdError for GeoError {}

/// Result type para operações de cartografia
pub type GeoResult<T> = Result<T, GeoError>;

impl From<std::io::Error> for GeoError {
    fn from(err: std::io::Error) -> Self {
        GeoError::IoError(err.to_string())
    }
}
