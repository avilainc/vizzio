//! Parser FBX (Filmbox) - Suporte básico (Rust puro)

use crate::file_parsers::*;
use std::collections::HashMap;

/// Parser FBX básico
pub struct FbxParser;

impl FileParser for FbxParser {
    fn can_parse(&self, format: FileFormat) -> bool {
        matches!(format, FileFormat::FBX)
    }

    fn parse(&self, data: &[u8], filename: &str) -> ParseResult<LoadedModel> {
        // FBX é um formato complexo binário
        // Por ora, apenas detectar e retornar erro de não suportado
        Err(ParseError::UnsupportedVersion("FBX parsing not yet implemented".to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fbx_parser_creation() {
        let parser = FbxParser;
        assert!(parser.can_parse(FileFormat::FBX));
        assert!(!parser.can_parse(FileFormat::IFC));
    }
}
