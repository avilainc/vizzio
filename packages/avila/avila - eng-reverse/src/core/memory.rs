#[derive(Debug, Clone)]
pub struct MemoryRegion {
    pub start: u64,
    pub end: u64,
    pub size: u64,
    pub permissions: String,
    pub name: Option<String>,
    pub data: Vec<u8>,
}

impl MemoryRegion {
    pub fn new(start: u64, size: u64) -> Self {
        Self {
            start,
            end: start + size,
            size,
            permissions: String::from("rwx"),
            name: None,
            data: vec![0; size as usize],
        }
    }

    pub fn contains(&self, address: u64) -> bool {
        address >= self.start && address < self.end
    }

    pub fn read_bytes(&self, offset: u64, size: usize) -> Option<&[u8]> {
        if offset + size as u64 > self.size {
            return None;
        }
        Some(&self.data[offset as usize..(offset as usize + size)])
    }

    pub fn write_bytes(&mut self, offset: u64, data: &[u8]) -> Result<(), String> {
        if offset + data.len() as u64 > self.size {
            return Err("Write exceeds region size".to_string());
        }
        self.data[offset as usize..offset as usize + data.len()].copy_from_slice(data);
        Ok(())
    }
}
