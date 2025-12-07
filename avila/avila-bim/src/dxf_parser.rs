//! Parser DXF (Drawing Exchange Format) (Rust puro)

use crate::file_parsers::*;
use std::collections::HashMap;

/// Parser DXF
pub struct DxfParser;

impl FileParser for DxfParser {
    fn can_parse(&self, format: FileFormat) -> bool {
        matches!(format, FileFormat::DXF)
    }

    fn parse(&self, data: &[u8], filename: &str) -> ParseResult<LoadedModel> {
        let content = std::str::from_utf8(data)
            .map_err(|_| ParseError::InvalidFormat("Invalid UTF-8 encoding".to_string()))?;

        let mut lines: Vec<&str> = content.lines().collect();
        let mut entities = Vec::new();
        let mut materials = Vec::new();
        let mut metadata = HashMap::new();

        // Parse sections
        let mut i = 0;
        while i < lines.len() {
            if lines[i].trim() == "0" && i + 1 < lines.len() {
                let section_name = lines[i + 1].trim();
                i += 2;

                match section_name {
                    "SECTION" => {
                        // Skip to next section
                        continue;
                    }
                    "HEADER" => {
                        i = self.parse_header_section(&lines, i)?;
                    }
                    "TABLES" => {
                        i = self.parse_tables_section(&lines, i)?;
                    }
                    "BLOCKS" => {
                        i = self.parse_blocks_section(&lines, i)?;
                    }
                    "ENTITIES" => {
                        entities = self.parse_entities_section(&lines, i)?;
                        break; // Entities is usually the last section
                    }
                    "ENDSEC" => {
                        continue;
                    }
                    _ => {
                        // Skip unknown sections
                        i = self.skip_section(&lines, i)?;
                    }
                }
            } else {
                i += 1;
            }
        }

        Ok(LoadedModel {
            format: FileFormat::DXF,
            elements: entities,
            materials,
            metadata,
        })
    }
}

impl DxfParser {
    fn parse_header_section(&self, lines: &[&str], mut i: usize) -> Result<usize, ParseError> {
        // Parse header variables
        while i < lines.len() {
            if lines[i].trim() == "0" && i + 1 < lines.len() && lines[i + 1].trim() == "ENDSEC" {
                return Ok(i + 2);
            }

            // Parse header variables (9 = variable name, then value)
            if lines[i].trim() == "9" && i + 2 < lines.len() {
                let var_name = lines[i + 1].trim();
                let var_value = lines[i + 2].trim();
                i += 3;

                // Could store header variables in metadata
                // For now, just skip
            } else {
                i += 1;
            }
        }

        Ok(i)
    }

    fn parse_tables_section(&self, lines: &[&str], mut i: usize) -> Result<usize, ParseError> {
        // Parse tables (layers, linetypes, etc.)
        while i < lines.len() {
            if lines[i].trim() == "0" && i + 1 < lines.len() {
                if lines[i + 1].trim() == "ENDSEC" {
                    return Ok(i + 2);
                }

                if lines[i + 1].trim() == "TABLE" && i + 3 < lines.len() {
                    // Table type
                    let table_type = lines[i + 3].trim();
                    i += 4;

                    // Skip table entries for now
                    i = self.skip_table(&lines, i)?;
                } else {
                    i += 2;
                }
            } else {
                i += 1;
            }
        }

        Ok(i)
    }

    fn parse_blocks_section(&self, lines: &[&str], mut i: usize) -> Result<usize, ParseError> {
        // Parse blocks (reusable geometry)
        while i < lines.len() {
            if lines[i].trim() == "0" && i + 1 < lines.len() {
                if lines[i + 1].trim() == "ENDSEC" {
                    return Ok(i + 2);
                }

                if lines[i + 1].trim() == "BLOCK" {
                    i += 2;
                    // Parse block definition
                    i = self.skip_entity(&lines, i)?;
                } else {
                    i += 2;
                }
            } else {
                i += 1;
            }
        }

        Ok(i)
    }

    fn parse_entities_section(&self, lines: &[&str], mut i: usize) -> Result<Vec<ModelElement>, ParseError> {
        let mut entities = Vec::new();

        while i < lines.len() {
            if lines[i].trim() == "0" && i + 1 < lines.len() {
                if lines[i + 1].trim() == "ENDSEC" {
                    break;
                }

                let entity_type = lines[i + 1].trim();
                i += 2;

                if let Some(element) = self.parse_entity(entity_type, &lines, i)? {
                    entities.push(element);
                    i = self.skip_entity(&lines, i)?;
                } else {
                    i = self.skip_entity(&lines, i)?;
                }
            } else {
                i += 1;
            }
        }

        Ok(entities)
    }

    fn parse_entity(&self, entity_type: &str, lines: &[&str], mut i: usize) -> Result<Option<ModelElement>, ParseError> {
        let mut properties = HashMap::new();

        // Parse entity properties
        while i < lines.len() {
            if lines[i].trim() == "0" {
                break; // Next entity
            }

            if i + 1 < lines.len() {
                let group_code = lines[i].trim();
                let value = lines[i + 1].trim();
                i += 2;

                if let Ok(code) = group_code.parse::<i32>() {
                    properties.insert(code, value.to_string());
                }
            } else {
                i += 1;
            }
        }

        // Convert to ModelElement based on entity type
        let element = match entity_type {
            "LINE" => self.create_line_element(&properties),
            "CIRCLE" => self.create_circle_element(&properties),
            "ARC" => self.create_arc_element(&properties),
            "POLYLINE" => self.parse_polyline(&properties, lines, i),
            "LWPOLYLINE" => self.parse_lwpolyline(&properties, lines, i),
            "INSERT" => self.create_insert_element(&properties),
            "TEXT" => self.create_text_element(&properties),
            "MTEXT" => self.create_mtext_element(&properties),
            _ => None, // Skip unsupported entities
        };

        Ok(element)
    }

    fn create_line_element(&self, properties: &HashMap<i32, String>) -> Option<ModelElement> {
        let x1 = self.get_float_property(properties, 10)?;
        let y1 = self.get_float_property(properties, 20)?;
        let z1 = self.get_float_property(properties, 30).unwrap_or(0.0);

        let x2 = self.get_float_property(properties, 11)?;
        let y2 = self.get_float_property(properties, 21)?;
        let z2 = self.get_float_property(properties, 31).unwrap_or(0.0);

        let layer = self.get_string_property(properties, 8).unwrap_or_else(|| "0".to_string());

        Some(ModelElement {
            id: format!("line_{}_{}", x1, y1),
            name: Some(format!("Line on layer {}", layer)),
            element_type: "Line".to_string(),
            geometry: ElementGeometry::Lines(vec![([x1, y1, z1], [x2, y2, z2])]),
            properties: self.create_properties_from_dxf(properties),
            transform: None,
        })
    }

    fn create_circle_element(&self, properties: &HashMap<i32, String>) -> Option<ModelElement> {
        let center_x = self.get_float_property(properties, 10)?;
        let center_y = self.get_float_property(properties, 20)?;
        let center_z = self.get_float_property(properties, 30).unwrap_or(0.0);
        let radius = self.get_float_property(properties, 40)?;

        let layer = self.get_string_property(properties, 8).unwrap_or_else(|| "0".to_string());

        // Create circle as mesh
        let segments = 32;
        let mut vertices = Vec::new();
        let mut indices = Vec::new();

        // Center vertex
        vertices.push([center_x, center_y, center_z]);

        // Circle vertices
        for i in 0..segments {
            let angle = (i as f64 / segments as f64) * 2.0 * std::f64::consts::PI;
            let x = center_x + radius * angle.cos();
            let y = center_y + radius * angle.sin();
            vertices.push([x, y, center_z]);
        }

        // Triangle fan
        for i in 0..segments {
            indices.push(0);
            indices.push((i + 1) as u32);
            indices.push(((i + 1) % segments + 1) as u32);
        }

        Some(ModelElement {
            id: format!("circle_{}_{}", center_x, center_y),
            name: Some(format!("Circle on layer {}", layer)),
            element_type: "Circle".to_string(),
            geometry: ElementGeometry::Mesh {
                vertices,
                indices,
                normals: None,
                uvs: None,
            },
            properties: self.create_properties_from_dxf(properties),
            transform: None,
        })
    }

    fn create_arc_element(&self, properties: &HashMap<i32, String>) -> Option<ModelElement> {
        let center_x = self.get_float_property(properties, 10)?;
        let center_y = self.get_float_property(properties, 20)?;
        let center_z = self.get_float_property(properties, 30).unwrap_or(0.0);
        let radius = self.get_float_property(properties, 40)?;
        let start_angle = self.get_float_property(properties, 50)?;
        let end_angle = self.get_float_property(properties, 51)?;

        let layer = self.get_string_property(properties, 8).unwrap_or_else(|| "0".to_string());

        // Convert degrees to radians
        let start_rad = start_angle.to_radians();
        let end_rad = end_angle.to_radians();

        // Create arc as line segments
        let segments = 32;
        let mut lines = Vec::new();

        for i in 0..segments {
            let t1 = i as f64 / segments as f64;
            let t2 = (i + 1) as f64 / segments as f64;

            let angle1 = start_rad + t1 * (end_rad - start_rad);
            let angle2 = start_rad + t2 * (end_rad - start_rad);

            let x1 = center_x + radius * angle1.cos();
            let y1 = center_y + radius * angle1.sin();
            let x2 = center_x + radius * angle2.cos();
            let y2 = center_y + radius * angle2.sin();

            lines.push(([x1, y1, center_z], [x2, y2, center_z]));
        }

        Some(ModelElement {
            id: format!("arc_{}_{}", center_x, center_y),
            name: Some(format!("Arc on layer {}", layer)),
            element_type: "Arc".to_string(),
            geometry: ElementGeometry::Lines(lines),
            properties: self.create_properties_from_dxf(properties),
            transform: None,
        })
    }

    fn parse_polyline(&self, properties: &HashMap<i32, String>, lines: &[&str], mut i: usize) -> Option<ModelElement> {
        let layer = self.get_string_property(properties, 8).unwrap_or_else(|| "0".to_string());
        let mut vertices = Vec::new();

        // Parse VERTEX entities that follow
        while i < lines.len() {
            if lines[i].trim() == "0" && i + 1 < lines.len() {
                if lines[i + 1].trim() == "VERTEX" {
                    i += 2;
                    let vertex_props = self.parse_vertex_properties(lines, i)?;
                    i = self.skip_entity(lines, i)?;

                    if let (Some(x), Some(y)) = (vertex_props.get(&10), vertex_props.get(&20)) {
                        let z = vertex_props.get(&30).unwrap_or(&"0".to_string());
                        if let (Ok(x), Ok(y), Ok(z)) = (x.parse::<f64>(), y.parse::<f64>(), z.parse::<f64>()) {
                            vertices.push([x, y, z]);
                        }
                    }
                } else if lines[i + 1].trim() == "SEQEND" {
                    break; // End of polyline
                } else {
                    i += 2;
                }
            } else {
                i += 1;
            }
        }

        if vertices.len() < 2 {
            return None;
        }

        // Create line segments
        let mut lines_vec = Vec::new();
        for j in 0..vertices.len() - 1 {
            lines_vec.push((vertices[j], vertices[j + 1]));
        }

        Some(ModelElement {
            id: format!("polyline_{}", vertices[0][0]),
            name: Some(format!("Polyline on layer {}", layer)),
            element_type: "Polyline".to_string(),
            geometry: ElementGeometry::Lines(lines_vec),
            properties: self.create_properties_from_dxf(properties),
            transform: None,
        })
    }

    fn parse_lwpolyline(&self, properties: &HashMap<i32, String>, lines: &[&str], mut i: usize) -> Option<ModelElement> {
        let layer = self.get_string_property(properties, 8).unwrap_or_else(|| "0".to_string());
        let mut vertices = Vec::new();

        // LWPOLYLINE has vertices inline with group codes 10,20,30,...
        let mut vertex_count = 0;
        if let Some(count) = self.get_int_property(properties, 90) {
            vertex_count = count as usize;
        }

        for j in 0..vertex_count {
            let x_code = 10 + j * 2;
            let y_code = 20 + j * 2;

            if let (Some(x), Some(y)) = (
                self.get_float_property(properties, x_code as i32),
                self.get_float_property(properties, y_code as i32),
            ) {
                let z = 0.0; // LWPOLYLINE is 2D
                vertices.push([x, y, z]);
            }
        }

        if vertices.len() < 2 {
            return None;
        }

        // Create line segments
        let mut lines_vec = Vec::new();
        for j in 0..vertices.len() - 1 {
            lines_vec.push((vertices[j], vertices[j + 1]));
        }

        // Close if necessary
        if self.get_int_property(properties, 70).unwrap_or(0) & 1 != 0 {
            lines_vec.push((vertices[vertices.len() - 1], vertices[0]));
        }

        Some(ModelElement {
            id: format!("lwpolyline_{}", vertices[0][0]),
            name: Some(format!("LWPolyline on layer {}", layer)),
            element_type: "LWPolyline".to_string(),
            geometry: ElementGeometry::Lines(lines_vec),
            properties: self.create_properties_from_dxf(properties),
            transform: None,
        })
    }

    fn create_insert_element(&self, properties: &HashMap<i32, String>) -> Option<ModelElement> {
        let x = self.get_float_property(properties, 10)?;
        let y = self.get_float_property(properties, 20)?;
        let z = self.get_float_property(properties, 30).unwrap_or(0.0);
        let block_name = self.get_string_property(properties, 2)?;
        let layer = self.get_string_property(properties, 8).unwrap_or_else(|| "0".to_string());

        Some(ModelElement {
            id: format!("insert_{}_{}_{}", block_name, x, y),
            name: Some(format!("Block {} on layer {}", block_name, layer)),
            element_type: "BlockInsert".to_string(),
            geometry: ElementGeometry::Points(vec![[x, y, z]]),
            properties: self.create_properties_from_dxf(properties),
            transform: None,
        })
    }

    fn create_text_element(&self, properties: &HashMap<i32, String>) -> Option<ModelElement> {
        let x = self.get_float_property(properties, 10)?;
        let y = self.get_float_property(properties, 20)?;
        let z = self.get_float_property(properties, 30).unwrap_or(0.0);
        let text = self.get_string_property(properties, 1)?;
        let layer = self.get_string_property(properties, 8).unwrap_or_else(|| "0".to_string());

        Some(ModelElement {
            id: format!("text_{}_{}", x, y),
            name: Some(text.clone()),
            element_type: "Text".to_string(),
            geometry: ElementGeometry::Points(vec![[x, y, z]]),
            properties: {
                let mut props = self.create_properties_from_dxf(properties);
                props.insert("text_content".to_string(), PropertyValue::String(text));
                props
            },
            transform: None,
        })
    }

    fn create_mtext_element(&self, properties: &HashMap<i32, String>) -> Option<ModelElement> {
        let x = self.get_float_property(properties, 10)?;
        let y = self.get_float_property(properties, 20)?;
        let z = self.get_float_property(properties, 30).unwrap_or(0.0);
        let text = self.get_string_property(properties, 1)?;
        let layer = self.get_string_property(properties, 8).unwrap_or_else(|| "0".to_string());

        Some(ModelElement {
            id: format!("mtext_{}_{}", x, y),
            name: Some(text.clone()),
            element_type: "MText".to_string(),
            geometry: ElementGeometry::Points(vec![[x, y, z]]),
            properties: {
                let mut props = self.create_properties_from_dxf(properties);
                props.insert("text_content".to_string(), PropertyValue::String(text));
                props
            },
            transform: None,
        })
    }

    fn parse_vertex_properties(&self, lines: &[&str], mut i: usize) -> Option<HashMap<i32, String>> {
        let mut properties = HashMap::new();

        while i < lines.len() {
            if lines[i].trim() == "0" {
                break; // Next entity
            }

            if i + 1 < lines.len() {
                let group_code = lines[i].trim();
                let value = lines[i + 1].trim();
                i += 2;

                if let Ok(code) = group_code.parse::<i32>() {
                    properties.insert(code, value.to_string());
                }
            } else {
                i += 1;
            }
        }

        Some(properties)
    }

    fn get_float_property(&self, properties: &HashMap<i32, String>, code: i32) -> Option<f64> {
        properties.get(&code)?.parse::<f64>().ok()
    }

    fn get_int_property(&self, properties: &HashMap<i32, String>, code: i32) -> Option<i32> {
        properties.get(&code)?.parse::<i32>().ok()
    }

    fn get_string_property(&self, properties: &HashMap<i32, String>, code: i32) -> Option<String> {
        properties.get(&code).cloned()
    }

    fn create_properties_from_dxf(&self, properties: &HashMap<i32, String>) -> HashMap<String, PropertyValue> {
        let mut result = HashMap::new();

        for (code, value) in properties {
            let key = format!("dxf_{}", code);
            result.insert(key, PropertyValue::String(value.clone()));
        }

        result
    }

    fn skip_section(&self, lines: &[&str], mut i: usize) -> Result<usize, ParseError> {
        while i < lines.len() {
            if lines[i].trim() == "0" && i + 1 < lines.len() && lines[i + 1].trim() == "ENDSEC" {
                return Ok(i + 2);
            }
            i += 1;
        }
        Ok(i)
    }

    fn skip_table(&self, lines: &[&str], mut i: usize) -> Result<usize, ParseError> {
        while i < lines.len() {
            if lines[i].trim() == "0" && i + 1 < lines.len() && lines[i + 1].trim() == "ENDTAB" {
                return Ok(i + 2);
            }
            i += 1;
        }
        Ok(i)
    }

    fn skip_entity(&self, lines: &[&str], mut i: usize) -> Result<usize, ParseError> {
        while i < lines.len() {
            if lines[i].trim() == "0" {
                return Ok(i);
            }
            i += 1;
        }
        Ok(i)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dxf_parser_creation() {
        let parser = DxfParser;
        assert!(parser.can_parse(FileFormat::DXF));
        assert!(!parser.can_parse(FileFormat::IFC));
    }

    #[test]
    fn test_dxf_line_parsing() {
        let dxf_content = r#"0
SECTION
2
ENTITIES
0
LINE
8
0
10
0.0
20
0.0
30
0.0
11
10.0
21
0.0
31
0.0
0
ENDSEC
0
EOF
"#;

        let parser = DxfParser;
        let result = parser.parse(dxf_content.as_bytes(), "test.dxf");

        assert!(result.is_ok());
        let model = result.unwrap();
        assert_eq!(model.elements.len(), 1);
        assert_eq!(model.elements[0].element_type, "Line");
    }
}
