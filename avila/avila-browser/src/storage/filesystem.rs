//! Filesystem storage operations

use std::error::Error;
use std::fmt;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub enum FileStorageError {
    IoError(io::Error),
    InvalidPath,
    PermissionDenied,
}

impl fmt::Display for FileStorageError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::IoError(e) => write!(f, "IO error: {}", e),
            Self::InvalidPath => write!(f, "Invalid file path"),
            Self::PermissionDenied => write!(f, "Permission denied"),
        }
    }
}

impl Error for FileStorageError {}

impl From<io::Error> for FileStorageError {
    fn from(err: io::Error) -> Self {
        Self::IoError(err)
    }
}

/// File storage manager
pub struct FileStorage {
    root_directory: PathBuf,
}

impl FileStorage {
    /// Create new file storage with root directory
    pub fn new<P: AsRef<Path>>(root_directory: P) -> Result<Self, FileStorageError> {
        let root = root_directory.as_ref().to_path_buf();
        fs::create_dir_all(&root)?;

        Ok(Self {
            root_directory: root,
        })
    }

    /// Write data to file
    pub fn write<P: AsRef<Path>>(&self, path: P, data: &[u8]) -> Result<(), FileStorageError> {
        let full_path = self.root_directory.join(path);

        // Create parent directories if needed
        if let Some(parent) = full_path.parent() {
            fs::create_dir_all(parent)?;
        }

        fs::write(full_path, data)?;
        Ok(())
    }

    /// Read data from file
    pub fn read<P: AsRef<Path>>(&self, path: P) -> Result<Vec<u8>, FileStorageError> {
        let full_path = self.root_directory.join(path);
        let data = fs::read(full_path)?;
        Ok(data)
    }

    /// Delete a file
    pub fn delete<P: AsRef<Path>>(&self, path: P) -> Result<(), FileStorageError> {
        let full_path = self.root_directory.join(path);
        fs::remove_file(full_path)?;
        Ok(())
    }

    /// Check if file exists
    pub fn exists<P: AsRef<Path>>(&self, path: P) -> bool {
        let full_path = self.root_directory.join(path);
        full_path.exists()
    }

    /// List files in directory
    pub fn list<P: AsRef<Path>>(&self, path: P) -> Result<Vec<PathBuf>, FileStorageError> {
        let full_path = self.root_directory.join(path);
        let mut files = Vec::new();

        for entry in fs::read_dir(full_path)? {
            let entry = entry?;
            files.push(entry.path());
        }

        Ok(files)
    }

    /// Get file size
    pub fn size<P: AsRef<Path>>(&self, path: P) -> Result<u64, FileStorageError> {
        let full_path = self.root_directory.join(path);
        let metadata = fs::metadata(full_path)?;
        Ok(metadata.len())
    }

    /// Clear all files
    pub fn clear(&self) -> Result<(), FileStorageError> {
        fs::remove_dir_all(&self.root_directory)?;
        fs::create_dir_all(&self.root_directory)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn test_file_storage_operations() {
        let temp_dir = env::temp_dir().join("avila_test");
        let storage = FileStorage::new(&temp_dir).unwrap();

        // Write file
        storage.write("test.txt", b"hello world").unwrap();

        // Read file
        let data = storage.read("test.txt").unwrap();
        assert_eq!(data, b"hello world");

        // Check exists
        assert!(storage.exists("test.txt"));

        // Get size
        let size = storage.size("test.txt").unwrap();
        assert_eq!(size, 11);

        // Delete file
        storage.delete("test.txt").unwrap();
        assert!(!storage.exists("test.txt"));

        // Cleanup
        let _ = storage.clear();
    }
}
