//! Hex dump and formatting utilities

use crate::ByteBuffer;

/// Formats bytes as hex dump
pub struct HexDump<'a> {
    data: &'a [u8],
    bytes_per_line: usize,
    show_ascii: bool,
    show_offset: bool,
}

impl<'a> HexDump<'a> {
    /// Creates a new hex dump formatter
    pub fn new(data: &'a [u8]) -> Self {
        Self {
            data,
            bytes_per_line: 16,
            show_ascii: true,
            show_offset: true,
        }
    }

    /// Sets bytes per line
    pub fn bytes_per_line(mut self, n: usize) -> Self {
        self.bytes_per_line = n;
        self
    }

    /// Sets whether to show ASCII representation
    pub fn show_ascii(mut self, show: bool) -> Self {
        self.show_ascii = show;
        self
    }

    /// Sets whether to show offset
    pub fn show_offset(mut self, show: bool) -> Self {
        self.show_offset = show;
        self
    }

    /// Formats to string
    pub fn format(&self) -> alloc::string::String {
        use alloc::string::String;
        use alloc::vec::Vec;

        let mut result = String::new();

        for (line_num, chunk) in self.data.chunks(self.bytes_per_line).enumerate() {
            // Offset
            if self.show_offset {
                result.push_str(&alloc::format!("{:08X}  ", line_num * self.bytes_per_line));
            }

            // Hex bytes
            for (i, byte) in chunk.iter().enumerate() {
                result.push_str(&alloc::format!("{:02X} ", byte));

                // Extra space in the middle
                if i == self.bytes_per_line / 2 - 1 {
                    result.push(' ');
                }
            }

            // Padding if last line is incomplete
            if chunk.len() < self.bytes_per_line {
                let missing = self.bytes_per_line - chunk.len();
                for i in 0..missing {
                    result.push_str("   ");
                    if chunk.len() + i == self.bytes_per_line / 2 - 1 {
                        result.push(' ');
                    }
                }
            }

            // ASCII representation
            if self.show_ascii {
                result.push_str(" |");
                for &byte in chunk {
                    let ch = if byte.is_ascii_graphic() || byte == b' ' {
                        byte as char
                    } else {
                        '.'
                    };
                    result.push(ch);
                }
                result.push('|');
            }

            result.push('\n');
        }

        result
    }
}

impl<'a> core::fmt::Display for HexDump<'a> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.format())
    }
}

/// Extension trait for hex dumping
pub trait HexDumpExt {
    /// Creates a hex dump
    fn hex_dump(&self) -> HexDump<'_>;

    /// Formats as compact hex string
    fn to_hex(&self) -> alloc::string::String;

    /// Formats as hex with spaces
    fn to_hex_spaced(&self) -> alloc::string::String;
}

impl HexDumpExt for [u8] {
    fn hex_dump(&self) -> HexDump<'_> {
        HexDump::new(self)
    }

    fn to_hex(&self) -> alloc::string::String {
        use alloc::string::String;
        let mut result = String::with_capacity(self.len() * 2);
        for byte in self {
            result.push_str(&alloc::format!("{:02X}", byte));
        }
        result
    }

    fn to_hex_spaced(&self) -> alloc::string::String {
        use alloc::string::String;
        let mut result = String::new();
        for (i, byte) in self.iter().enumerate() {
            if i > 0 {
                result.push(' ');
            }
            result.push_str(&alloc::format!("{:02X}", byte));
        }
        result
    }
}

impl HexDumpExt for ByteBuffer {
    fn hex_dump(&self) -> HexDump<'_> {
        HexDump::new(self.as_slice())
    }

    fn to_hex(&self) -> alloc::string::String {
        self.as_slice().to_hex()
    }

    fn to_hex_spaced(&self) -> alloc::string::String {
        self.as_slice().to_hex_spaced()
    }
}

/// Parses hex string to bytes
pub fn parse_hex(s: &str) -> Result<alloc::vec::Vec<u8>, &'static str> {
    let s = s.trim().replace([' ', '\n', '\t', ':'], "");

    if s.len() % 2 != 0 {
        return Err("Hex string must have even length");
    }

    let mut result = alloc::vec::Vec::with_capacity(s.len() / 2);

    for i in (0..s.len()).step_by(2) {
        let byte_str = &s[i..i + 2];
        let byte = u8::from_str_radix(byte_str, 16)
            .map_err(|_| "Invalid hex digit")?;
        result.push(byte);
    }

    Ok(result)
}

/// Binary dump formatter
pub struct BinaryDump<'a> {
    data: &'a [u8],
    bytes_per_line: usize,
}

impl<'a> BinaryDump<'a> {
    /// Creates new binary dump
    pub fn new(data: &'a [u8]) -> Self {
        Self {
            data,
            bytes_per_line: 4,
        }
    }

    /// Sets bytes per line
    pub fn bytes_per_line(mut self, n: usize) -> Self {
        self.bytes_per_line = n;
        self
    }

    /// Formats to string
    pub fn format(&self) -> alloc::string::String {
        use alloc::string::String;

        let mut result = String::new();

        for chunk in self.data.chunks(self.bytes_per_line) {
            for (i, byte) in chunk.iter().enumerate() {
                if i > 0 {
                    result.push(' ');
                }
                result.push_str(&alloc::format!("{:08b}", byte));
            }
            result.push('\n');
        }

        result
    }
}

impl<'a> core::fmt::Display for BinaryDump<'a> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.format())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hex_dump() {
        let data = b"Hello, World!";
        let dump = HexDump::new(data).format();

        assert!(dump.contains("48 65 6C 6C 6F"));
        assert!(dump.contains("|Hello, World!|"));
    }

    #[test]
    fn test_to_hex() {
        let data = b"ABC";
        assert_eq!(data.to_hex(), "414243");
        assert_eq!(data.to_hex_spaced(), "41 42 43");
    }

    #[test]
    fn test_parse_hex() {
        let bytes = parse_hex("48 65 6C 6C 6F").unwrap();
        assert_eq!(&bytes, b"Hello");

        let bytes = parse_hex("DEADBEEF").unwrap();
        assert_eq!(bytes, vec![0xDE, 0xAD, 0xBE, 0xEF]);
    }

    #[test]
    fn test_binary_dump() {
        let data = b"A";
        let dump = BinaryDump::new(data).format();
        assert!(dump.contains("01000001"));
    }

    #[test]
    fn test_hex_dump_buffer() {
        let buffer = ByteBuffer::from(b"Test" as &[u8]);
        let hex = buffer.to_hex();
        assert_eq!(hex, "54657374");
    }
}
