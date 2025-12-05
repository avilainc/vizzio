//! Parser glTF/GLB (GL Transmission Format) - Suporte básico (Rust puro)

use crate::file_parsers::*;
use std::collections::HashMap;

/// Parser glTF básico
pub struct GltfParser;

impl FileParser for GltfParser {
    fn can_parse(&self, format: FileFormat) -> bool {
        matches!(format, FileFormat::GLTF | FileFormat::GLB)
    }

    fn parse(&self, data: &[u8], filename: &str) -> ParseResult<LoadedModel> {
        // glTF é JSON + binário, complexo
        // Por ora, apenas detectar formato
        if filename.ends_with(".glb") {
            self.parse_glb(data, filename)
        } else {
            Err(ParseError::UnsupportedVersion("glTF JSON parsing not yet implemented".to_string()))
        }
    }
}

impl GltfParser {
    fn parse_glb(&self, data: &[u8], filename: &str) -> ParseResult<LoadedModel> {
        if data.len() < 20 {
            return Err(ParseError::InvalidFormat("GLB file too small".to_string()));
        }

        // GLB header: magic (4 bytes) + version (4 bytes) + length (4 bytes)
        let magic = &data[0..4];
        if magic != b"glTF" {
            return Err(ParseError::InvalidFormat("Invalid GLB magic number".to_string()));
        }

        // Por ora, apenas detectar
        Err(ParseError::UnsupportedVersion("GLB parsing not yet implemented".to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gltf_parser_creation() {
        let parser = GltfParser;
        assert!(parser.can_parse(FileFormat::GLTF));
        assert!(parser.can_parse(FileFormat::GLB));
        assert!(!parser.can_parse(FileFormat::IFC));
    }
}
