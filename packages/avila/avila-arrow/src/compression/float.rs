//! Float compression techniques

use crate::error::Result;

/// Compress float array using XOR-based compression
pub fn compress_float(values: &[f32]) -> Result<Vec<u8>> {
    // Placeholder - would implement Gorilla-style float compression
    let bytes = values.iter()
        .flat_map(|f| f.to_le_bytes())
        .collect();
    Ok(bytes)
}

/// Decompress float array
pub fn decompress_float(compressed: &[u8]) -> Result<Vec<f32>> {
    // Placeholder
    let values = compressed.chunks_exact(4)
        .map(|chunk| {
            let bytes: [u8; 4] = chunk.try_into().unwrap();
            f32::from_le_bytes(bytes)
        })
        .collect();
    Ok(values)
}

/// Compress double array
pub fn compress_double(values: &[f64]) -> Result<Vec<u8>> {
    let bytes = values.iter()
        .flat_map(|f| f.to_le_bytes())
        .collect();
    Ok(bytes)
}

/// Decompress double array
pub fn decompress_double(compressed: &[u8]) -> Result<Vec<f64>> {
    let values = compressed.chunks_exact(8)
        .map(|chunk| {
            let bytes: [u8; 8] = chunk.try_into().unwrap();
            f64::from_le_bytes(bytes)
        })
        .collect();
    Ok(values)
}
