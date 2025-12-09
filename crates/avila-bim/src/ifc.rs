//! Parser IFC STEP format

use alloc::vec::Vec;
use alloc::string::{String, ToString};
use avila_error::{Result, Error};
use crate::{IfcModel, IfcEntity, IfcHeader};

/// Parse arquivo IFC STEP
pub fn parse_step(content: &str) -> Result<IfcModel> {
    let mut entities = Vec::new();
    let mut header = IfcHeader::default();

    let mut in_header = false;
    let mut in_data = false;

    for line in content.lines() {
        let trimmed = line.trim();

        // Detecta seções
        if trimmed.starts_with("HEADER;") {
            in_header = true;
            continue;
        }
        if trimmed.starts_with("ENDSEC;") {
            in_header = false;
            in_data = false;
            continue;
        }
        if trimmed.starts_with("DATA;") {
            in_data = true;
            continue;
        }

        // Parse header
        if in_header {
            if let Some((key, value)) = parse_header_line(trimmed) {
                match key.as_str() {
                    "FILE_NAME" => header.file_name = value,
                    "FILE_DESCRIPTION" => header.description = value,
                    "FILE_SCHEMA" => header.schema = value,
                    _ => {}
                }
            }
        }

        // Parse entidades
        if in_data && trimmed.starts_with('#') {
            if let Some(entity) = parse_entity_line(trimmed) {
                entities.push(entity);
            }
        }
    }

    Ok(IfcModel { entities, header })
}

fn parse_header_line(line: &str) -> Option<(String, String)> {
    // Simplificado: FILE_NAME('nome','timestamp','autor'...)
    if let Some(start) = line.find('(') {
        if let Some(end) = line.find(')') {
            let key = line[..start].to_string();
            let value = line[start+1..end]
                .split(',')
                .next()?
                .trim_matches('\'')
                .trim_matches(' ')
                .to_string();
            return Some((key, value));
        }
    }
    None
}

fn parse_entity_line(line: &str) -> Option<IfcEntity> {
    // Formato: #123 = IFCWALL('guid', #owner, 'name', ...)
    if !line.starts_with('#') {
        return None;
    }

    let parts: Vec<&str> = line.splitn(2, '=').collect();
    if parts.len() != 2 {
        return None;
    }

    let id = parts[0].trim().trim_start_matches('#').parse::<u32>().ok()?;
    let rest = parts[1].trim();

    let paren_pos = rest.find('(')?;
    let entity_type = rest[..paren_pos].trim().to_string();

    let end_paren = rest.rfind(')')?;
    let params_str = &rest[paren_pos+1..end_paren];

    let params: Vec<String> = params_str
        .split(',')
        .map(|s| s.trim().to_string())
        .collect();

    Some(IfcEntity {
        id,
        entity_type,
        params,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_entity_line() {
        let line = "#123 = IFCWALL('guid', #1, 'My Wall', 'Description');";
        let entity = parse_entity_line(line).unwrap();

        assert_eq!(entity.id, 123);
        assert_eq!(entity.entity_type, "IFCWALL");
        assert_eq!(entity.params.len(), 4);
    }

    #[test]
    fn test_parse_simple_ifc() {
        let ifc_content = r#"
ISO-10303-21;
HEADER;
FILE_NAME('test.ifc','2025-01-01T12:00:00',('Avila'),('Vizzio'),'IFC exporter','IfcOpenShell','');
FILE_SCHEMA(('IFC4'));
ENDSEC;
DATA;
#1 = IFCPROJECT('guid', #2, 'Test Project', 'Description', $, $, $, $);
#2 = IFCWALL('wall-guid', #1, 'Wall 01', 'A wall', $, $, $, $);
ENDSEC;
END-ISO-10303-21;
        "#;

        let model = parse_step(ifc_content).unwrap();
        assert_eq!(model.entities.len(), 2);
        assert_eq!(model.header.schema, "IFC4");
    }
}
