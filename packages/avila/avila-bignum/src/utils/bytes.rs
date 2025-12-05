//! Byte serialization/deserialization utilities

/// Convert limbs to big-endian bytes
pub fn to_bytes_be(limbs: &[u64]) -> Vec<u8> {
    let mut bytes = Vec::with_capacity(limbs.len() * 8);
    for &limb in limbs.iter().rev() {
        bytes.extend_from_slice(&limb.to_be_bytes());
    }
    bytes
}

/// Convert limbs to little-endian bytes
pub fn to_bytes_le(limbs: &[u64]) -> Vec<u8> {
    let mut bytes = Vec::with_capacity(limbs.len() * 8);
    for &limb in limbs.iter() {
        bytes.extend_from_slice(&limb.to_le_bytes());
    }
    bytes
}

/// Convert big-endian bytes to limbs
pub fn from_bytes_be(bytes: &[u8], limbs: &mut [u64]) {
    limbs.fill(0);
    let mut chunks = bytes.rchunks_exact(8);
    for (i, chunk) in chunks.by_ref().enumerate() {
        if i < limbs.len() {
            limbs[i] = u64::from_be_bytes(chunk.try_into().unwrap());
        }
    }
    let remainder = chunks.remainder();
    if !remainder.is_empty() && limbs.len() > bytes.len() / 8 {
        let mut last = [0u8; 8];
        last[8 - remainder.len()..].copy_from_slice(remainder);
        limbs[bytes.len() / 8] = u64::from_be_bytes(last);
    }
}

/// Convert little-endian bytes to limbs
pub fn from_bytes_le(bytes: &[u8], limbs: &mut [u64]) {
    limbs.fill(0);
    for (i, chunk) in bytes.chunks(8).enumerate() {
        if i < limbs.len() {
            let mut limb_bytes = [0u8; 8];
            limb_bytes[..chunk.len()].copy_from_slice(chunk);
            limbs[i] = u64::from_le_bytes(limb_bytes);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_bytes_le() {
        let limbs = [0x0123456789abcdefu64, 0xfedcba9876543210u64];
        let bytes = to_bytes_le(&limbs);
        assert_eq!(bytes.len(), 16);
    }

    #[test]
    fn test_roundtrip_le() {
        let original = [0x0123456789abcdefu64, 0xfedcba9876543210u64];
        let bytes = to_bytes_le(&original);
        let mut result = [0u64; 2];
        from_bytes_le(&bytes, &mut result);
        assert_eq!(original, result);
    }
}
