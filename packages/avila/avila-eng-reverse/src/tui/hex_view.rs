// Hex viewer component
use std::error::Error;

/// Interactive hex viewer
pub struct HexView {
    data: Vec<u8>,
    offset: usize,
    bytes_per_line: usize,
}

impl HexView {
    pub fn new(data: Vec<u8>) -> Self {
        Self {
            data,
            offset: 0,
            bytes_per_line: 16,
        }
    }

    /// Scroll down
    pub fn scroll_down(&mut self, lines: usize) {
        self.offset = (self.offset + lines * self.bytes_per_line).min(self.data.len());
    }

    /// Scroll up
    pub fn scroll_up(&mut self, lines: usize) {
        self.offset = self.offset.saturating_sub(lines * self.bytes_per_line);
    }

    /// Go to offset
    pub fn goto_offset(&mut self, offset: usize) {
        self.offset = offset.min(self.data.len());
    }

    /// Render hex view
    pub fn render(&self, lines: usize) -> Result<String, Box<dyn Error>> {
        let mut output = String::new();

        for line in 0..lines {
            let start = self.offset + line * self.bytes_per_line;
            if start >= self.data.len() {
                break;
            }

            let end = (start + self.bytes_per_line).min(self.data.len());
            let bytes = &self.data[start..end];

            // Address
            output.push_str(&format!("{:08x}  ", start));

            // Hex bytes
            for (i, &byte) in bytes.iter().enumerate() {
                output.push_str(&format!("{:02x} ", byte));
                if i == 7 {
                    output.push(' ');
                }
            }

            // Padding
            for _ in bytes.len()..self.bytes_per_line {
                output.push_str("   ");
            }

            output.push_str(" |");

            // ASCII
            for &byte in bytes {
                let ch = if byte.is_ascii_graphic() {
                    byte as char
                } else {
                    '.'
                };
                output.push(ch);
            }

            output.push_str("|\n");
        }

        Ok(output)
    }
}
