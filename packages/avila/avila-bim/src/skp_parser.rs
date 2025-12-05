//! Parser SketchUp SKP - Suporte básico (Rust puro)

use crate::file_parsers::*;

/// Parser SketchUp básico
pub struct SkpParser;

impl FileParser for SkpParser {
    fn can_parse(&self, format: FileFormat) -> bool {
        matches!(format, FileFormat::SKP)
    }

    fn parse(&self, data: &[u8], filename: &str) -> ParseResult<LoadedModel> {
        // SKP é formato proprietário do SketchUp
        Err(ParseError::UnsupportedVersion("SKP parsing not yet implemented".to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_skp_parser_creation() {
        let parser = SkpParser;
        assert!(parser.can_parse(FileFormat::SKP));
        assert!(!parser.can_parse(FileFormat::IFC));
    }
}
