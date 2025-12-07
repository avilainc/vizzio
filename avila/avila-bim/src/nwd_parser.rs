//! Parser Navisworks NWD - Suporte básico (Rust puro)

use crate::file_parsers::*;

/// Parser Navisworks básico
pub struct NwdParser;

impl FileParser for NwdParser {
    fn can_parse(&self, format: FileFormat) -> bool {
        matches!(format, FileFormat::NWD)
    }

    fn parse(&self, data: &[u8], filename: &str) -> ParseResult<LoadedModel> {
        // NWD é formato proprietário do Navisworks
        Err(ParseError::UnsupportedVersion("NWD parsing not yet implemented".to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nwd_parser_creation() {
        let parser = NwdParser;
        assert!(parser.can_parse(FileFormat::NWD));
        assert!(!parser.can_parse(FileFormat::IFC));
    }
}
