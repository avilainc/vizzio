//! Storage and persistence layer
//!
//! Provides secure storage mechanisms:
//! - Embedded database (SQLite)
//! - Filesystem operations
//! - Secure keychain for credentials

pub mod database;
pub mod filesystem;
pub mod keychain;

pub use database::{Database, DatabaseError};
pub use filesystem::{FileStorage, FileStorageError};
pub use keychain::{Keychain, KeychainError};

/// Storage configuration
#[derive(Debug, Clone)]
pub struct StorageConfig {
    pub data_directory: String,
    pub enable_encryption: bool,
    pub max_cache_size_mb: usize,
}

impl Default for StorageConfig {
    fn default() -> Self {
        Self {
            data_directory: "./data".to_string(),
            enable_encryption: true,
            max_cache_size_mb: 500,
        }
    }
}
