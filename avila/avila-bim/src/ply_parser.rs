//! Parser PLY (Polygon File Format) (Rust puro)

use crate::file_parsers::*;
use std::collections::HashMap;

/// Parser PLY
pub struct PlyParser;

impl FileParser for PlyParser {
    fn can_parse(&self, format: FileFormat) -> bool {
        matches!(format, FileFormat::PLY)
    }

    fn parse(&self, data: &[u8], filename: &str) -> ParseResult<LoadedModel> {
        let content = std::str::from_utf8(data)
            .map_err(|_| ParseError::InvalidFormat("Invalid UTF-8 encoding".to_string()))?;

        let mut lines = content.lines();
        let mut header_ended = false;
        let mut vertex_count = 0;
        let mut face_count = 0;
        let mut vertex_properties = Vec::new();
        let mut is_binary = false;
        let mut is_big_endian = false;

        // Parse header
        while let Some(line) = lines.next() {
            let line = line.trim();

            if line == "ply" {
                continue;
            } else if line == "end_header" {
                header_ended = true;
                break;
            } else if line.starts_with("format ") {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 2 {
                    is_binary = parts[1] != "ascii";
                    if parts.len() >= 3 {
                        is_big_endian = parts[2] == "binary_big_endian";
                    }
                }
            } else if line.starts_with("element vertex ") {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 3 {
                    vertex_count = parts[2].parse().unwrap_or(0);
                }
            } else if line.starts_with("element face ") {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 3 {
                    face_count = parts[2].parse().unwrap_or(0);
                }
            } else if line.starts_with("property ") {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 3 {
                    vertex_properties.push(PlyProperty {
                        name: parts[2].to_string(),
                        data_type: parts[1].to_string(),
                    });
                }
            }
        }

        if !header_ended {
            return Err(ParseError::InvalidFormat("Invalid PLY header".to_string()));
        }

        if is_binary {
            return Err(ParseError::UnsupportedVersion("Binary PLY not yet supported".to_string()));
        }

        // Parse ASCII data
        self.parse_ascii_data(lines, vertex_count, face_count, &vertex_properties, filename)
    }
}

impl PlyParser {
    fn parse_ascii_data(
        &self,
        mut lines: std::str::Lines,
        vertex_count: usize,
        face_count: usize,
        properties: &[PlyProperty],
        filename: &str,
    ) -> ParseResult<LoadedModel> {
        let mut vertices = Vec::with_capacity(vertex_count);
        let mut faces = Vec::with_capacity(face_count);

        // Parse vertices
        for _ in 0..vertex_count {
            if let Some(line) = lines.next() {
                let parts: Vec<f64> = line.split_whitespace()
                    .filter_map(|s| s.parse().ok())
                    .collect();

                if parts.len() >= 3 {
                    vertices.push([parts[0], parts[1], parts[2]]);
                }
            }
        }

        // Parse faces
        for _ in 0..face_count {
            if let Some(line) = lines.next() {
                let parts: Vec<usize> = line.split_whitespace()
                    .filter_map(|s| s.parse().ok())
                    .collect();

                if parts.len() >= 4 {
                    // First number is vertex count, rest are indices
                    let face_indices: Vec<u32> = parts[1..].iter().map(|&i| i as u32).collect();
                    faces.push(face_indices);
                }
            }
        }

        // Convert faces to triangles
        let mut indices = Vec::new();
        for face in faces {
            if face.len() >= 3 {
                // Triangulate (simple fan)
                for i in 1..face.len() - 1 {
                    indices.push(face[0]);
                    indices.push(face[i]);
                    indices.push(face[i + 1]);
                }
            }
        }

        let geometry = ElementGeometry::Mesh {
            vertices,
            indices,
            normals: None,
            uvs: None,
        };

        let properties_map = HashMap::from([
            ("vertex_count".to_string(), PropertyValue::Integer(vertex_count as i64)),
            ("face_count".to_string(), PropertyValue::Integer(face_count as i64)),
        ]);

        let element = ModelElement {
            id: "ply_mesh".to_string(),
            name: Some(filename.to_string()),
            element_type: "Mesh".to_string(),
            geometry,
            properties: properties_map,
            transform: None,
        };

        Ok(LoadedModel {
            format: FileFormat::PLY,
            elements: vec![element],
            materials: Vec::new(),
            metadata: HashMap::new(),
        })
    }
}

/// Propriedade PLY
#[derive(Debug, Clone)]
struct PlyProperty {
    name: String,
    data_type: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ply_parser_creation() {
        let parser = PlyParser;
        assert!(parser.can_parse(FileFormat::PLY));
        assert!(!parser.can_parse(FileFormat::IFC));
    }

    #[test]
    fn test_ply_ascii_parsing() {
        let ply_content = r#"ply
format ascii 1.0
element vertex 4
property float x
property float y
property float z
element face 2
property list uchar int vertex_indices
end_header
0.0 0.0 0.0
1.0 0.0 0.0
1.0 1.0 0.0
0.0 1.0 0.0
3 0 1 2
3 0 2 3
"#;

        let parser = PlyParser;
        let result = parser.parse(ply_content.as_bytes(), "quad.ply");

        assert!(result.is_ok());
        let model = result.unwrap();
        assert_eq!(model.elements.len(), 1);

        if let ElementGeometry::Mesh { vertices, indices, .. } = &model.elements[0].geometry {
            assert_eq!(vertices.len(), 4);
            assert_eq!(indices.len(), 6); // 2 triangles
        } else {
            panic!("Expected mesh geometry");
        }
    }
}
