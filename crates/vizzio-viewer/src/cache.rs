//! Sistema de cache simples para modelos IFC

use avila_bim::IfcModel;
use std::collections::HashMap;
use std::path::PathBuf;

/// Cache de modelos IFC
pub struct IfcCache {
    models: HashMap<String, CachedModel>,
}

/// Modelo IFC em cache
pub struct CachedModel {
    /// Caminho do arquivo
    pub path: PathBuf,
    /// Modelo parseado
    pub model: IfcModel,
    /// Timestamp do cache
    pub cached_at: u64,
    /// Tamanho em bytes
    pub size_bytes: usize,
}

impl IfcCache {
    /// Cria novo cache
    pub fn new() -> Self {
        Self {
            models: HashMap::new(),
        }
    }

    /// Busca modelo no cache
    pub fn get(&self, path: &str) -> Option<&CachedModel> {
        self.models.get(&path.to_string())
    }

    /// Insere modelo no cache
    pub fn insert(&mut self, path: String, model: IfcModel, size_bytes: usize) {
        let cached = CachedModel {
            path: PathBuf::from(&path),
            model,
            cached_at: current_timestamp(),
            size_bytes,
        };
        self.models.insert(path, cached);
    }

    /// Limpa cache
    pub fn clear(&mut self) {
        self.models.clear();
        println!("🗑️  Cache limpo");
    }

    /// Estatísticas do cache
    pub fn stats(&self) -> CacheStats {
        let count = self.models.len();
        let total_bytes: usize = self.models.values()
            .map(|m| m.size_bytes)
            .sum();

        CacheStats {
            model_count: count,
            total_bytes,
            total_mb: total_bytes as f64 / (1024.0 * 1024.0),
        }
    }
}

/// Estatísticas do cache
pub struct CacheStats {
    /// Número de modelos em cache
    pub model_count: usize,
    /// Total de bytes
    pub total_bytes: usize,
    /// Total em MB
    pub total_mb: f64,
}

impl CacheStats {
    /// Imprime estatísticas
    pub fn print(&self) {
        println!("💾 Cache: {} modelos, {:.2} MB", self.model_count, self.total_mb);
    }
}

/// Retorna timestamp atual em segundos
fn current_timestamp() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs()
}
