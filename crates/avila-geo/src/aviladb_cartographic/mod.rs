//! # AvilaDB Cartográfico - Sistema de Banco de Dados Geográfico Pessoal
//!
//! Sistema completo para armazenar, gerenciar e visualizar dados geográficos pessoais
//! com mapas temáticos e simbologia customizável.

pub mod database;
pub mod symbology;
pub mod thematic;
pub mod entities;
pub mod query;
pub mod export;

pub use database::{CartographicDatabase, DatabaseConfig};
pub use symbology::{Symbol, SymbolType, SymbolStyle, ColorRamp, ClassificationMethod};
pub use thematic::{ThematicMap, ThematicLayer, MapTheme};
pub use entities::{Company, Place, Address, PointOfInterest, Entity};
pub use query::{SpatialQuery, AttributeQuery, QueryBuilder};
pub use export::{ExportFormat, MapExporter};

use crate::cartography::{LatLon, BoundingBox};
use std::collections::HashMap;

/// Resultado de operações do database
pub type DbResult<T> = Result<T, DbError>;

/// Erros do database cartográfico
#[derive(Debug, Clone)]
pub enum DbError {
    /// Entidade não encontrada
    NotFound(String),

    /// Erro de validação
    ValidationError(String),

    /// Erro de I/O
    IoError(String),

    /// Erro de serialização
    SerializationError(String),

    /// Erro espacial
    SpatialError(String),
}

impl std::fmt::Display for DbError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DbError::NotFound(msg) => write!(f, "Não encontrado: {}", msg),
            DbError::ValidationError(msg) => write!(f, "Erro de validação: {}", msg),
            DbError::IoError(msg) => write!(f, "Erro de I/O: {}", msg),
            DbError::SerializationError(msg) => write!(f, "Erro de serialização: {}", msg),
            DbError::SpatialError(msg) => write!(f, "Erro espacial: {}", msg),
        }
    }
}

impl std::error::Error for DbError {}

/// Trait para entidades geográficas
pub trait GeoEntity: Send + Sync {
    /// ID único da entidade
    fn id(&self) -> u64;

    /// Nome da entidade
    fn name(&self) -> &str;

    /// Localização geográfica
    fn location(&self) -> LatLon;

    /// Tipo da entidade
    fn entity_type(&self) -> &str;

    /// Atributos customizados
    fn attributes(&self) -> &HashMap<String, String>;

    /// Atributo customizado
    fn get_attribute(&self, key: &str) -> Option<&str> {
        self.attributes().get(key).map(|s| s.as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_database_creation() {
        let config = DatabaseConfig::default();
        let db = CartographicDatabase::new(config);
        assert_eq!(db.count(), 0);
    }
}
