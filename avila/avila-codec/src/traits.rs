//! Common traits for encoding and decoding

use crate::Result;
use alloc::vec::Vec;

/// Trait for types that can be encoded to bytes
pub trait Encode {
    /// Encodes self to a byte vector
    fn encode(&self) -> Result<Vec<u8>>;

    /// Encodes self to the provided buffer
    fn encode_to_slice(&self, output: &mut [u8]) -> Result<usize>;
}

/// Trait for types that can be decoded from bytes
pub trait Decode: Sized {
    /// Decodes from a byte slice
    fn decode(data: &[u8]) -> Result<Self>;
}

/// Trait for encoders that produce string output
pub trait StringEncoder {
    /// Encodes bytes to a string
    fn encode_to_string(&self, data: &[u8]) -> Result<alloc::string::String>;
}

/// Trait for streaming encoders
pub trait StreamingEncoder {
    /// Updates the encoder state with more data
    fn update(&mut self, data: &[u8]) -> Result<()>;

    /// Finalizes the encoding and returns the result
    fn finalize(self) -> Result<Vec<u8>>;
}

/// Trait for streaming decoders
pub trait StreamingDecoder {
    /// Updates the decoder state with more data
    fn update(&mut self, data: &[u8]) -> Result<()>;

    /// Finalizes the decoding and returns the result
    fn finalize(self) -> Result<Vec<u8>>;
}

/// Trait for checksum calculators
pub trait Checksum {
    /// Output type of the checksum
    type Output;

    /// Creates a new checksum calculator
    fn new() -> Self;

    /// Updates the checksum with more data
    fn update(&mut self, data: &[u8]);

    /// Finalizes and returns the checksum value
    fn finalize(&self) -> Self::Output;

    /// Convenience method to compute checksum in one call
    fn digest(data: &[u8]) -> Self::Output
    where
        Self: Sized,
    {
        let mut hasher = Self::new();
        hasher.update(data);
        hasher.finalize()
    }
}
