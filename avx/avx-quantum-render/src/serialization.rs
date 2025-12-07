//! Serialization and data export utilities

use std::fs::File;
use std::io::Write;
use std::path::Path;

/// Export format
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExportFormat {
    Json,
    Csv,
    Binary,
    Hdr, // High dynamic range
}

/// Color format for export
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ColorFormat {
    Rgb8,      // 8-bit per channel
    Rgba8,     // With alpha
    Rgb16,     // 16-bit per channel
    Rgb32F,    // 32-bit float
    Rgb64F,    // 64-bit float
}

impl ColorFormat {
    pub fn bytes_per_pixel(&self) -> usize {
        match self {
            Self::Rgb8 => 3,
            Self::Rgba8 => 4,
            Self::Rgb16 => 6,
            Self::Rgb32F => 12,
            Self::Rgb64F => 24,
        }
    }
}

/// Image export buffer
pub struct ImageBuffer {
    width: u32,
    height: u32,
    format: ColorFormat,
    data: Vec<u8>,
}

impl ImageBuffer {
    /// Create new image buffer
    pub fn new(width: u32, height: u32, format: ColorFormat) -> Self {
        let bytes = (width * height) as usize * format.bytes_per_pixel();
        Self {
            width,
            height,
            format,
            data: vec![0u8; bytes],
        }
    }

    /// Get width
    pub fn width(&self) -> u32 {
        self.width
    }

    /// Get height
    pub fn height(&self) -> u32 {
        self.height
    }

    /// Set pixel data (expects values in [0, 1] range)
    pub fn set_pixel(&mut self, x: u32, y: u32, r: f64, g: f64, b: f64) {
        if x >= self.width || y >= self.height {
            return;
        }

        let idx = ((y * self.width + x) as usize) * self.format.bytes_per_pixel();

        match self.format {
            ColorFormat::Rgb8 => {
                self.data[idx] = (r * 255.0) as u8;
                self.data[idx + 1] = (g * 255.0) as u8;
                self.data[idx + 2] = (b * 255.0) as u8;
            }
            ColorFormat::Rgba8 => {
                self.data[idx] = (r * 255.0) as u8;
                self.data[idx + 1] = (g * 255.0) as u8;
                self.data[idx + 2] = (b * 255.0) as u8;
                self.data[idx + 3] = 255;
            }
            ColorFormat::Rgb32F => {
                let bytes_r = r.to_le_bytes();
                let bytes_g = g.to_le_bytes();
                let bytes_b = b.to_le_bytes();
                self.data[idx..idx + 4].copy_from_slice(&bytes_r);
                self.data[idx + 4..idx + 8].copy_from_slice(&bytes_g);
                self.data[idx + 8..idx + 12].copy_from_slice(&bytes_b);
            }
            ColorFormat::Rgb64F => {
                let bytes_r = r.to_le_bytes();
                let bytes_g = g.to_le_bytes();
                let bytes_b = b.to_le_bytes();
                self.data[idx..idx + 8].copy_from_slice(&bytes_r);
                self.data[idx + 8..idx + 16].copy_from_slice(&bytes_g);
                self.data[idx + 16..idx + 24].copy_from_slice(&bytes_b);
            }
            _ => {} // Implement other formats as needed
        }
    }

    /// Export to file
    pub fn save(&self, path: impl AsRef<Path>, format: ExportFormat) -> std::io::Result<()> {
        match format {
            ExportFormat::Binary => self.save_binary(path),
            ExportFormat::Csv => self.save_csv(path),
            ExportFormat::Hdr => self.save_hdr(path),
            _ => Err(std::io::Error::new(
                std::io::ErrorKind::Unsupported,
                "Format not yet implemented",
            )),
        }
    }

    fn save_binary(&self, path: impl AsRef<Path>) -> std::io::Result<()> {
        let mut file = File::create(path)?;
        // Write header
        file.write_all(&self.width.to_le_bytes())?;
        file.write_all(&self.height.to_le_bytes())?;
        file.write_all(&[self.format as u8])?;
        // Write data
        file.write_all(&self.data)?;
        Ok(())
    }

    fn save_csv(&self, path: impl AsRef<Path>) -> std::io::Result<()> {
        let mut file = File::create(path)?;
        writeln!(file, "x,y,r,g,b")?;

        for y in 0..self.height {
            for x in 0..self.width {
                let idx = ((y * self.width + x) as usize) * self.format.bytes_per_pixel();
                let r = if matches!(self.format, ColorFormat::Rgb8 | ColorFormat::Rgba8) {
                    self.data[idx] as f64 / 255.0
                } else {
                    0.0
                };
                let g = if matches!(self.format, ColorFormat::Rgb8 | ColorFormat::Rgba8) {
                    self.data[idx + 1] as f64 / 255.0
                } else {
                    0.0
                };
                let b = if matches!(self.format, ColorFormat::Rgb8 | ColorFormat::Rgba8) {
                    self.data[idx + 2] as f64 / 255.0
                } else {
                    0.0
                };
                writeln!(file, "{},{},{},{},{}", x, y, r, g, b)?;
            }
        }
        Ok(())
    }

    fn save_hdr(&self, path: impl AsRef<Path>) -> std::io::Result<()> {
        let mut file = File::create(path)?;
        writeln!(file, "#?RADIANCE")?;
        writeln!(file, "FORMAT=32-bit_rle_rgbe")?;
        writeln!(file, "EXPOSURE=1.0")?;
        writeln!(file, "RES=X {} Y {}", self.width, self.height)?;

        // Simplified HDR encoding
        for y in 0..self.height {
            for x in 0..self.width {
                let idx = ((y * self.width + x) as usize) * self.format.bytes_per_pixel();
                let r = if self.format as u8 <= 1 {
                    self.data[idx] as f64 / 255.0
                } else {
                    0.0
                };
                let g = if self.format as u8 <= 1 {
                    self.data[idx + 1] as f64 / 255.0
                } else {
                    0.0
                };
                let b = if self.format as u8 <= 1 {
                    self.data[idx + 2] as f64 / 255.0
                } else {
                    0.0
                };

                let max_comp = r.max(g).max(b);
                if max_comp > 0.0 {
                    let exp = max_comp.log2().ceil() as i32;
                    let scale = 2.0_f64.powi(-exp);
                    file.write_all(&[(r * scale * 255.0) as u8])?;
                    file.write_all(&[(g * scale * 255.0) as u8])?;
                    file.write_all(&[(b * scale * 255.0) as u8])?;
                    file.write_all(&[(exp + 128) as u8])?;
                } else {
                    file.write_all(&[0, 0, 0, 0])?;
                }
            }
        }
        Ok(())
    }
}

/// Scene export format
pub struct SceneExporter;

impl SceneExporter {
    /// Export scene as JSON
    pub fn export_json(path: impl AsRef<Path>, scene_name: &str) -> std::io::Result<()> {
        let mut file = File::create(path)?;
        writeln!(file, "{{")?;
        writeln!(file, "  \"scene\": \"{}\"", scene_name)?;
        writeln!(file, "  \"version\": 1")?;
        writeln!(file, "}}")?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_image_buffer_creation() {
        let buf = ImageBuffer::new(512, 512, ColorFormat::Rgb8);
        assert_eq!(buf.width, 512);
        assert_eq!(buf.height, 512);
    }

    #[test]
    fn test_set_pixel() {
        let mut buf = ImageBuffer::new(2, 2, ColorFormat::Rgb8);
        buf.set_pixel(0, 0, 1.0, 0.5, 0.0);
        // Verify data was written
        assert_eq!(buf.data[0], 255); // R channel at max
        assert_eq!(buf.data[1], 127); // G channel at mid
        assert_eq!(buf.data[2], 0);   // B channel at min
    }

    #[test]
    fn test_color_format_sizes() {
        assert_eq!(ColorFormat::Rgb8.bytes_per_pixel(), 3);
        assert_eq!(ColorFormat::Rgba8.bytes_per_pixel(), 4);
        assert_eq!(ColorFormat::Rgb32F.bytes_per_pixel(), 12);
        assert_eq!(ColorFormat::Rgb64F.bytes_per_pixel(), 24);
    }
}
