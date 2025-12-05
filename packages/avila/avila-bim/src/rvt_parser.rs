//! Parser Revit RVT - Suporte básico (Rust puro)

use crate::file_parsers::*;

/// Parser Revit básico
pub struct RvtParser;

impl FileParser for RvtParser {
    fn can_parse(&self, format: FileFormat) -> bool {
        matches!(format, FileFormat::RVT)
    }

    fn parse(&self, data: &[u8], filename: &str) -> ParseResult<LoadedModel> {
        // RVT é formato proprietário do Revit
        Err(ParseError::UnsupportedVersion("RVT parsing not yet implemented".to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rvt_parser_creation() {
        let parser = RvtParser;
        assert!(parser.can_parse(FileFormat::RVT));
        assert!(!parser.can_parse(FileFormat::IFC));
    }
}
