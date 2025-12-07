//! Hash utilities (Rust puro - implementação simples)

/// Simples hash FNV-1a de 64 bits
pub struct SimpleHash;

impl SimpleHash {
    const FNV_OFFSET: u64 = 14695981039346656037;
    const FNV_PRIME: u64 = 1099511628211;

    /// Hash de bytes usando FNV-1a
    pub fn hash_bytes(data: &[u8]) -> u64 {
        let mut hash = Self::FNV_OFFSET;

        for &byte in data {
            hash ^= byte as u64;
            hash = hash.wrapping_mul(Self::FNV_PRIME);
        }

        hash
    }

    /// Hash de string
    pub fn hash_string(s: &str) -> u64 {
        Self::hash_bytes(s.as_bytes())
    }

    /// Hash de floats (para geometria)
    pub fn hash_floats(values: &[f32]) -> String {
        let mut hash = Self::FNV_OFFSET;

        for &value in values {
            let bytes = value.to_ne_bytes();
            for &byte in &bytes {
                hash ^= byte as u64;
                hash = hash.wrapping_mul(Self::FNV_PRIME);
            }
        }

        format!("{:016x}", hash)
    }

    /// Hash de mesh (para cache)
    pub fn hash_mesh_data(vertices: &[f32], indices: &[u32]) -> String {
        let mut hash = Self::FNV_OFFSET;

        // Hash vertices
        for &v in vertices {
            let bytes = v.to_ne_bytes();
            for &byte in &bytes {
                hash ^= byte as u64;
                hash = hash.wrapping_mul(Self::FNV_PRIME);
            }
        }

        // Hash indices
        for &idx in indices {
            let bytes = idx.to_ne_bytes();
            for &byte in &bytes {
                hash ^= byte as u64;
                hash = hash.wrapping_mul(Self::FNV_PRIME);
            }
        }

        format!("{:016x}", hash)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_string() {
        let hash1 = SimpleHash::hash_string("hello");
        let hash2 = SimpleHash::hash_string("hello");
        let hash3 = SimpleHash::hash_string("world");

        assert_eq!(hash1, hash2);
        assert_ne!(hash1, hash3);
    }

    #[test]
    fn test_hash_floats() {
        let data1 = vec![1.0, 2.0, 3.0];
        let data2 = vec![1.0, 2.0, 3.0];
        let data3 = vec![1.0, 2.0, 3.1];

        let hash1 = SimpleHash::hash_floats(&data1);
        let hash2 = SimpleHash::hash_floats(&data2);
        let hash3 = SimpleHash::hash_floats(&data3);

        assert_eq!(hash1, hash2);
        assert_ne!(hash1, hash3);
    }
}
