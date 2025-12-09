use crate::error::{IfcError, Result};
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Read};
use std::path::Path;

#[derive(Debug, Clone, PartialEq)]
pub enum StepValue {
    Null,
    Boolean(bool),
    Integer(i64),
    Real(f64),
    String(String),
    EntityRef(i64),
    Enum(String),
    List(Vec<StepValue>),
    Derived,
}

#[derive(Debug, Clone)]
pub struct StepEntity {
    pub id: i64,
    pub entity_type: String,
    pub attributes: Vec<StepValue>,
}

#[derive(Debug)]
pub struct StepFile {
    pub header: StepHeader,
    pub entities: HashMap<i64, StepEntity>,
}

#[derive(Debug, Default)]
pub struct StepHeader {
    pub file_description: Vec<String>,
    pub file_name: String,
    pub time_stamp: String,
    pub author: Vec<String>,
    pub organization: Vec<String>,
    pub preprocessor_version: String,
    pub originating_system: String,
    pub authorization: String,
    pub schema_identifiers: Vec<String>,
}

pub struct StepParser {
    line_number: usize,
}

impl StepParser {
    pub fn new() -> Self {
        Self { line_number: 0 }
    }

    pub fn parse_file<P: AsRef<Path>>(&mut self, path: P) -> Result<StepFile> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        self.parse_reader(reader)
    }

    pub fn parse_reader<R: Read>(&mut self, reader: BufReader<R>) -> Result<StepFile> {
        let mut lines = reader.lines();

        // Check for ISO-10303-21 signature
        if let Some(Ok(first_line)) = lines.next() {
            self.line_number += 1;
            if !first_line.starts_with("ISO-10303-21") {
                return Err(IfcError::InvalidFormat(
                    "Missing ISO-10303-21 signature".to_string(),
                ));
            }
        } else {
            return Err(IfcError::InvalidFormat("Empty file".to_string()));
        }

        let mut in_header = false;
        let mut in_data = false;
        let mut header = StepHeader::default();
        let mut entities = HashMap::new();
        let mut current_line = String::new();

        for line in lines {
            self.line_number += 1;
            let line = line?;
            let trimmed = line.trim();

            if trimmed.is_empty() {
                continue;
            }

            match trimmed {
                "HEADER;" => {
                    in_header = true;
                    continue;
                }
                "DATA;" => {
                    in_header = false;
                    in_data = true;
                    continue;
                }
                "ENDSEC;" => {
                    in_header = false;
                    if !current_line.is_empty() {
                        if in_data {
                            if let Some(entity) = self.parse_entity_line(&current_line)? {
                                entities.insert(entity.id, entity);
                            }
                        }
                        current_line.clear();
                    }
                    continue;
                }
                "END-ISO-10303-21;" => break,
                _ => {}
            }

            current_line.push_str(trimmed);

            if trimmed.ends_with(';') {
                if in_header {
                    self.parse_header_line(&current_line, &mut header)?;
                } else if in_data {
                    if let Some(entity) = self.parse_entity_line(&current_line)? {
                        entities.insert(entity.id, entity);
                    }
                }
                current_line.clear();
            }
        }

        Ok(StepFile { header, entities })
    }

    fn parse_header_line(&self, line: &str, header: &mut StepHeader) -> Result<()> {
        if line.starts_with("FILE_DESCRIPTION") {
            header.file_description = self.extract_strings_from_list(line)?;
        } else if line.starts_with("FILE_NAME") {
            let parts = self.extract_function_params(line)?;
            if !parts.is_empty() {
                if let StepValue::String(s) = &parts[0] {
                    header.file_name = s.clone();
                }
                if parts.len() > 1 {
                    if let StepValue::String(s) = &parts[1] {
                        header.time_stamp = s.clone();
                    }
                }
            }
        } else if line.starts_with("FILE_SCHEMA") {
            header.schema_identifiers = self.extract_strings_from_list(line)?;
        }
        Ok(())
    }

    fn parse_entity_line(&self, line: &str) -> Result<Option<StepEntity>> {
        if !line.starts_with('#') {
            return Ok(None);
        }

        let line = line.trim_end_matches(';');

        // Parse entity ID
        let eq_pos = line.find('=').ok_or_else(|| {
            IfcError::ParseError {
                line: self.line_number,
                message: "Missing '=' in entity definition".to_string(),
            }
        })?;

        let id_str = &line[1..eq_pos].trim();
        let id: i64 = id_str.parse().map_err(|_| IfcError::ParseError {
            line: self.line_number,
            message: format!("Invalid entity ID: {}", id_str),
        })?;

        let rest = &line[eq_pos + 1..].trim();

        // Parse entity type
        let paren_pos = rest.find('(').ok_or_else(|| {
            IfcError::ParseError {
                line: self.line_number,
                message: "Missing '(' in entity definition".to_string(),
            }
        })?;

        let entity_type = rest[..paren_pos].trim().to_string();

        // Parse attributes
        let params_str = &rest[paren_pos + 1..rest.len() - 1];
        let attributes = self.parse_parameters(params_str)?;

        Ok(Some(StepEntity {
            id,
            entity_type,
            attributes,
        }))
    }

    fn parse_parameters(&self, params: &str) -> Result<Vec<StepValue>> {
        let mut result = Vec::new();
        let mut current = String::new();
        let mut depth = 0;
        let mut in_string = false;

        for ch in params.chars() {
            match ch {
                '\'' if !in_string => in_string = true,
                '\'' if in_string => in_string = false,
                '(' if !in_string => depth += 1,
                ')' if !in_string => depth -= 1,
                ',' if !in_string && depth == 0 => {
                    result.push(self.parse_value(current.trim())?);
                    current.clear();
                    continue;
                }
                _ => {}
            }
            current.push(ch);
        }

        if !current.trim().is_empty() {
            result.push(self.parse_value(current.trim())?);
        }

        Ok(result)
    }

    fn parse_value(&self, value: &str) -> Result<StepValue> {
        let value = value.trim();

        if value == "$" || value.is_empty() {
            return Ok(StepValue::Null);
        }

        if value == "*" {
            return Ok(StepValue::Derived);
        }

        if value.starts_with('#') {
            let id: i64 = value[1..].parse().map_err(|_| IfcError::ParseError {
                line: self.line_number,
                message: format!("Invalid entity reference: {}", value),
            })?;
            return Ok(StepValue::EntityRef(id));
        }

        if value.starts_with('\'') && value.ends_with('\'') {
            let s = value[1..value.len() - 1].to_string();
            return Ok(StepValue::String(self.decode_string(&s)));
        }

        if value.starts_with('.') && value.ends_with('.') {
            let enum_val = value[1..value.len() - 1].to_string();
            if enum_val == "T" {
                return Ok(StepValue::Boolean(true));
            } else if enum_val == "F" {
                return Ok(StepValue::Boolean(false));
            }
            return Ok(StepValue::Enum(enum_val));
        }

        if value.starts_with('(') && value.ends_with(')') {
            let inner = &value[1..value.len() - 1];
            let items = self.parse_parameters(inner)?;
            return Ok(StepValue::List(items));
        }

        // Try to parse as number
        if let Ok(i) = value.parse::<i64>() {
            return Ok(StepValue::Integer(i));
        }

        if let Ok(f) = value.parse::<f64>() {
            return Ok(StepValue::Real(f));
        }

        Ok(StepValue::String(value.to_string()))
    }

    fn decode_string(&self, s: &str) -> String {
        // Decode IFC string encoding (e.g., \X\E3 for ã)
        let mut result = String::new();
        let mut chars = s.chars();

        while let Some(ch) = chars.next() {
            if ch == '\\' {
                if let Some(next) = chars.next() {
                    if next == 'X' {
                        if let Some(backslash) = chars.next() {
                            // Extended encoding
                            result.push(ch);
                            result.push(next);
                            result.push(backslash);
                        }
                    } else {
                        result.push(ch);
                        result.push(next);
                    }
                }
            } else {
                result.push(ch);
            }
        }

        result
    }

    fn extract_strings_from_list(&self, line: &str) -> Result<Vec<String>> {
        let mut result = Vec::new();
        let start = line.find('(').unwrap_or(0);
        let end = line.rfind(')').unwrap_or(line.len());
        let content = &line[start + 1..end];

        let mut current = String::new();
        let mut in_string = false;
        let mut depth = 0;

        for ch in content.chars() {
            match ch {
                '\'' if !in_string => in_string = true,
                '\'' if in_string => {
                    in_string = false;
                    if depth == 0 {
                        result.push(current.clone());
                        current.clear();
                    }
                }
                '(' if !in_string => depth += 1,
                ')' if !in_string => depth -= 1,
                ',' if !in_string && depth == 0 => {
                    if !current.is_empty() {
                        current.clear();
                    }
                }
                _ if in_string => current.push(ch),
                _ => {}
            }
        }

        Ok(result)
    }

    fn extract_function_params(&self, line: &str) -> Result<Vec<StepValue>> {
        let start = line.find('(').unwrap_or(0);
        let end = line.rfind(')').unwrap_or(line.len());
        let params = &line[start + 1..end];
        self.parse_parameters(params)
    }
}

impl Default for StepParser {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_value_null() {
        let parser = StepParser::new();
        assert_eq!(parser.parse_value("$").unwrap(), StepValue::Null);
    }

    #[test]
    fn test_parse_value_integer() {
        let parser = StepParser::new();
        assert_eq!(parser.parse_value("42").unwrap(), StepValue::Integer(42));
    }

    #[test]
    fn test_parse_value_real() {
        let parser = StepParser::new();
        assert_eq!(parser.parse_value("3.14").unwrap(), StepValue::Real(3.14));
    }

    #[test]
    fn test_parse_value_string() {
        let parser = StepParser::new();
        if let StepValue::String(s) = parser.parse_value("'hello'").unwrap() {
            assert_eq!(s, "hello");
        } else {
            panic!("Expected string value");
        }
    }

    #[test]
    fn test_parse_value_entity_ref() {
        let parser = StepParser::new();
        assert_eq!(parser.parse_value("#123").unwrap(), StepValue::EntityRef(123));
    }

    #[test]
    fn test_parse_value_enum() {
        let parser = StepParser::new();
        if let StepValue::Enum(e) = parser.parse_value(".ELEMENT.").unwrap() {
            assert_eq!(e, "ELEMENT");
        } else {
            panic!("Expected enum value");
        }
    }
}
