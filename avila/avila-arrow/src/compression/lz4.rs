//! LZ4_FRAME compression codec

use crate::error::Result;

/// LZ4 compression
pub fn compress_lz4(data: &[u8]) -> Result<Vec<u8>> {
    // Placeholder - would use lz4_flex or similar crate
    Ok(data.to_vec())
}

/// LZ4 decompression
pub fn decompress_lz4(compressed: &[u8]) -> Result<Vec<u8>> {
    // Placeholder - would use lz4_flex or similar crate
    Ok(compressed.to_vec())
}
