// Memory management for emulation
use std::collections::HashMap;

/// Memory manager for emulation
pub struct EmulationMemory {
    regions: HashMap<u64, MemoryRegion>,
}

#[derive(Debug, Clone)]
pub struct MemoryRegion {
    pub base: u64,
    pub size: usize,
    pub permissions: Permissions,
    pub data: Vec<u8>,
}

#[derive(Debug, Clone)]
pub struct Permissions {
    pub read: bool,
    pub write: bool,
    pub execute: bool,
}

impl EmulationMemory {
    pub fn new() -> Self {
        Self {
            regions: HashMap::new(),
        }
    }

    /// Allocate memory region
    pub fn allocate(&mut self, base: u64, size: usize, perms: Permissions) -> Result<(), String> {
        let region = MemoryRegion {
            base,
            size,
            permissions: perms,
            data: vec![0; size],
        };
        self.regions.insert(base, region);
        Ok(())
    }

    /// Free memory region
    pub fn free(&mut self, base: u64) -> Result<(), String> {
        self.regions.remove(&base);
        Ok(())
    }

    /// Write to memory
    pub fn write(&mut self, address: u64, data: &[u8]) -> Result<(), String> {
        for (base, region) in &mut self.regions {
            if address >= *base && address + data.len() as u64 <= *base + region.size as u64 {
                if !region.permissions.write {
                    return Err("Write permission denied".to_string());
                }
                let offset = (address - *base) as usize;
                region.data[offset..offset + data.len()].copy_from_slice(data);
                return Ok(());
            }
        }
        Err("Invalid memory address".to_string())
    }

    /// Read from memory
    pub fn read(&self, address: u64, size: usize) -> Result<Vec<u8>, String> {
        for (base, region) in &self.regions {
            if address >= *base && address + size as u64 <= *base + region.size as u64 {
                if !region.permissions.read {
                    return Err("Read permission denied".to_string());
                }
                let offset = (address - *base) as usize;
                return Ok(region.data[offset..offset + size].to_vec());
            }
        }
        Err("Invalid memory address".to_string())
    }
}
