//! Parser STL (STereoLithography) (Rust puro)

use crate::file_parsers::*;

/// Parser STL
pub struct StlParser;

impl FileParser for StlParser {
    fn can_parse(&self, format: FileFormat) -> bool {
        matches!(format, FileFormat::STL)
    }

    fn parse(&self, data: &[u8], filename: &str) -> ParseResult<LoadedModel> {
        // Detect if binary or ASCII
        if self.is_binary_stl(data) {
            self.parse_binary_stl(data, filename)
        } else {
            self.parse_ascii_stl(data, filename)
        }
    }
}

impl StlParser {
    fn is_binary_stl(&self, data: &[u8]) -> bool {
        if data.len() < 84 {
            return false;
        }

        // Binary STL starts with 80-byte header + 4-byte triangle count
        // ASCII STL starts with "solid "
        !data.starts_with(b"solid ")
    }

    fn parse_ascii_stl(&self, data: &[u8], filename: &str) -> ParseResult<LoadedModel> {
        let content = std::str::from_utf8(data)
            .map_err(|_| ParseError::InvalidFormat("Invalid UTF-8 encoding".to_string()))?;

        let mut vertices = Vec::new();
        let mut normals = Vec::new();
        let mut indices = Vec::new();
        let mut vertex_count = 0;

        let mut lines = content.lines();
        let mut current_normal = None;

        while let Some(line) = lines.next() {
            let line = line.trim();

            if line.starts_with("solid ") {
                // Header line, skip
                continue;
            } else if line.starts_with("endsolid") {
                // End of file
                break;
            } else if line.starts_with("facet normal ") {
                // Normal vector
                current_normal = self.parse_vector3(&line[13..]);
            } else if line.starts_with("outer loop") {
                // Start of triangle vertices
                let mut triangle_vertices = Vec::new();

                // Read 3 vertices
                for _ in 0..3 {
                    if let Some(vertex_line) = lines.next() {
                        let vertex_line = vertex_line.trim();
                        if vertex_line.starts_with("vertex ") {
                            if let Some(vertex) = self.parse_vector3(&vertex_line[7..]) {
                                triangle_vertices.push(vertex);
                            }
                        }
                    }
                }

                if triangle_vertices.len() == 3 {
                    // Add vertices
                    for vertex in &triangle_vertices {
                        vertices.push(*vertex);
                        vertex_count += 1;
                    }

                    // Add normal for each vertex if available
                    if let Some(normal) = current_normal {
                        for _ in 0..3 {
                            normals.push(normal);
                        }
                    }

                    // Add triangle indices
                    indices.push(vertex_count - 3);
                    indices.push(vertex_count - 2);
                    indices.push(vertex_count - 1);
                }

                // Skip "endloop" and "endfacet"
                while let Some(skip_line) = lines.next() {
                    let skip_line = skip_line.trim();
                    if skip_line == "endfacet" {
                        break;
                    }
                }
            }
        }

        self.create_stl_model(vertices, normals, indices, filename)
    }

    fn parse_binary_stl(&self, data: &[u8], filename: &str) -> ParseResult<LoadedModel> {
        if data.len() < 84 {
            return Err(ParseError::InvalidFormat("Binary STL file too small".to_string()));
        }

        // Skip 80-byte header
        let mut offset = 80;

        // Read triangle count (4 bytes, little endian)
        let triangle_count = u32::from_le_bytes([
            data[offset], data[offset + 1], data[offset + 2], data[offset + 3]
        ]) as usize;
        offset += 4;

        let mut vertices = Vec::with_capacity(triangle_count * 3);
        let mut normals = Vec::with_capacity(triangle_count * 3);
        let mut indices = Vec::with_capacity(triangle_count * 3);

        for i in 0..triangle_count {
            if offset + 50 > data.len() {
                return Err(ParseError::CorruptedFile("Unexpected end of binary STL data".to_string()));
            }

            // Read normal (12 bytes: 3 floats)
            let normal = self.read_vector3_le(&data[offset..offset + 12]);
            offset += 12;

            // Read 3 vertices (36 bytes: 9 floats)
            for _ in 0..3 {
                let vertex = self.read_vector3_le(&data[offset..offset + 12]);
                vertices.push(vertex);
                normals.push(normal);
                indices.push(i * 3 + (vertices.len() - 1));
                offset += 12;
            }

            // Skip attribute byte count (2 bytes)
            offset += 2;
        }

        self.create_stl_model(vertices, normals, indices, filename)
    }

    fn parse_vector3(&self, s: &str) -> Option<[f64; 3]> {
        let parts: Vec<&str> = s.split_whitespace().collect();
        if parts.len() == 3 {
            let x = parts[0].parse::<f64>().ok()?;
            let y = parts[1].parse::<f64>().ok()?;
            let z = parts[2].parse::<f64>().ok()?;
            Some([x, y, z])
        } else {
            None
        }
    }

    fn read_vector3_le(&self, data: &[u8]) -> [f64; 3] {
        let x = f32::from_le_bytes([data[0], data[1], data[2], data[3]]) as f64;
        let y = f32::from_le_bytes([data[4], data[5], data[6], data[7]]) as f64;
        let z = f32::from_le_bytes([data[8], data[9], data[10], data[11]]) as f64;
        [x, y, z]
    }

    fn create_stl_model(
        &self,
        vertices: Vec<[f64; 3]>,
        normals: Vec<[f64; 3]>,
        indices: Vec<usize>,
        filename: &str,
    ) -> ParseResult<LoadedModel> {
        if vertices.is_empty() {
            return Err(ParseError::InvalidFormat("No vertices found in STL".to_string()));
        }

        let geometry = ElementGeometry::Mesh {
            vertices,
            indices: indices.into_iter().map(|i| i as u32).collect(),
            normals: if normals.is_empty() { None } else { Some(normals) },
            uvs: None,
        };

        let triangle_count = indices.len() / 3;
        let properties = std::collections::HashMap::from([
            ("triangle_count".to_string(), PropertyValue::Integer(triangle_count as i64)),
            ("vertex_count".to_string(), PropertyValue::Integer(geometry.vertices().len() as i64)),
        ]);

        let element = ModelElement {
            id: "stl_mesh".to_string(),
            name: Some(filename.to_string()),
            element_type: "Mesh".to_string(),
            geometry,
            properties,
            transform: None,
        };

        Ok(LoadedModel {
            format: FileFormat::STL,
            elements: vec![element],
            materials: Vec::new(),
            metadata: std::collections::HashMap::new(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stl_parser_creation() {
        let parser = StlParser;
        assert!(parser.can_parse(FileFormat::STL));
        assert!(!parser.can_parse(FileFormat::IFC));
    }

    #[test]
    fn test_stl_ascii_parsing() {
        let stl_content = r#"solid cube
  facet normal 0.0 0.0 1.0
    outer loop
      vertex 0.0 0.0 0.0
      vertex 1.0 0.0 0.0
      vertex 1.0 1.0 0.0
    endloop
  endfacet
  facet normal 0.0 0.0 1.0
    outer loop
      vertex 0.0 0.0 0.0
      vertex 1.0 1.0 0.0
      vertex 0.0 1.0 0.0
    endloop
  endfacet
endsolid cube
"#;

        let parser = StlParser;
        let result = parser.parse(stl_content.as_bytes(), "cube.stl");

        assert!(result.is_ok());
        let model = result.unwrap();
        assert_eq!(model.elements.len(), 1);

        if let ElementGeometry::Mesh { vertices, indices, .. } = &model.elements[0].geometry {
            assert_eq!(vertices.len(), 6); // 2 triangles * 3 vertices each
            assert_eq!(indices.len(), 6); // 2 triangles
        } else {
            panic!("Expected mesh geometry");
        }
    }

    #[test]
    fn test_stl_detect_binary() {
        let parser = StlParser;

        // ASCII STL
        let ascii_data = b"solid cube\nendsolid cube";
        assert!(!parser.is_binary_stl(ascii_data));

        // Binary STL (small header)
        let mut binary_data = [0u8; 84];
        binary_data[80..84].copy_from_slice(&[1, 0, 0, 0]); // 1 triangle
        assert!(parser.is_binary_stl(&binary_data));
    }
}
