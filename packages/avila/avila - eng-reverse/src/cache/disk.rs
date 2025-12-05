// Disk-based cache implementation
use super::storage::CacheStorage;
use std::error::Error;
use std::fs;
use std::path::PathBuf;
use std::time::Duration;

/// Disk-based persistent cache
pub struct DiskCache {
    cache_dir: PathBuf,
    max_size: usize,
}

impl DiskCache {
    pub fn new(cache_dir: PathBuf, max_size_mb: usize) -> Result<Self, Box<dyn Error>> {
        fs::create_dir_all(&cache_dir)?;
        Ok(Self {
            cache_dir,
            max_size: max_size_mb * 1024 * 1024,
        })
    }

    fn key_to_path(&self, key: &str) -> PathBuf {
        let hash = format!("{:x}", md5::compute(key));
        self.cache_dir.join(hash)
    }
}

impl CacheStorage for DiskCache {
    fn get(&self, key: &str) -> Result<Option<Vec<u8>>, Box<dyn Error>> {
        let path = self.key_to_path(key);
        if path.exists() {
            let data = fs::read(path)?;
            Ok(Some(data))
        } else {
            Ok(None)
        }
    }

    fn set(&mut self, key: &str, value: Vec<u8>, _ttl: Duration) -> Result<(), Box<dyn Error>> {
        let path = self.key_to_path(key);
        fs::write(path, value)?;
        Ok(())
    }

    fn delete(&mut self, key: &str) -> Result<(), Box<dyn Error>> {
        let path = self.key_to_path(key);
        if path.exists() {
            fs::remove_file(path)?;
        }
        Ok(())
    }

    fn clear(&mut self) -> Result<(), Box<dyn Error>> {
        for entry in fs::read_dir(&self.cache_dir)? {
            let entry = entry?;
            fs::remove_file(entry.path())?;
        }
        Ok(())
    }

    fn exists(&self, key: &str) -> Result<bool, Box<dyn Error>> {
        Ok(self.key_to_path(key).exists())
    }
}
