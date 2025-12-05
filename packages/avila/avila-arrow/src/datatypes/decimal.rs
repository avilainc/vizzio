//! Decimal data types: Decimal128, Decimal256

/// Decimal128 type with precision and scale
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Decimal128Type {
    pub precision: u8,
    pub scale: i8,
}

/// Decimal256 type with precision and scale
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Decimal256Type {
    pub precision: u8,
    pub scale: i8,
}
