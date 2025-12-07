//! Parser OBJ (Wavefront OBJ) (Rust puro)

use crate::file_parsers::*;
use std::collections::HashMap;

/// Parser OBJ
pub struct ObjParser;

impl FileParser for ObjParser {
    fn can_parse(&self, format: FileFormat) -> bool {
        matches!(format, FileFormat::OBJ)
    }

    fn parse(&self, data: &[u8], filename: &str) -> ParseResult<LoadedModel> {
        let content = std::str::from_utf8(data)
            .map_err(|_| ParseError::InvalidFormat("Invalid UTF-8 encoding".to_string()))?;

        let mut vertices = Vec::new();
        let mut normals = Vec::new();
        let mut uvs = Vec::new();
        let mut faces = Vec::new();
        let mut materials = Vec::new();
        let mut metadata = HashMap::new();

        // Parse lines
        for (line_num, line) in content.lines().enumerate() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }

            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.is_empty() {
                continue;
            }

            match parts[0] {
                "v" => {
                    if parts.len() >= 4 {
                        let x = parts[1].parse::<f64>().map_err(|_| ParseError::InvalidFormat(
                            format!("Invalid vertex X at line {}", line_num)
                        ))?;
                        let y = parts[2].parse::<f64>().map_err(|_| ParseError::InvalidFormat(
                            format!("Invalid vertex Y at line {}", line_num)
                        ))?;
                        let z = parts[3].parse::<f64>().map_err(|_| ParseError::InvalidFormat(
                            format!("Invalid vertex Z at line {}", line_num)
                        ))?;
                        vertices.push([x, y, z]);
                    }
                }
                "vn" => {
                    if parts.len() >= 4 {
                        let x = parts[1].parse::<f64>().map_err(|_| ParseError::InvalidFormat(
                            format!("Invalid normal X at line {}", line_num)
                        ))?;
                        let y = parts[2].parse::<f64>().map_err(|_| ParseError::InvalidFormat(
                            format!("Invalid normal Y at line {}", line_num)
                        ))?;
                        let z = parts[3].parse::<f64>().map_err(|_| ParseError::InvalidFormat(
                            format!("Invalid normal Z at line {}", line_num)
                        ))?;
                        normals.push([x, y, z]);
                    }
                }
                "vt" => {
                    if parts.len() >= 3 {
                        let u = parts[1].parse::<f64>().map_err(|_| ParseError::InvalidFormat(
                            format!("Invalid UV U at line {}", line_num)
                        ))?;
                        let v = parts[2].parse::<f64>().map_err(|_| ParseError::InvalidFormat(
                            format!("Invalid UV V at line {}", line_num)
                        ))?;
                        uvs.push([u, v]);
                    }
                }
                "f" => {
                    if parts.len() >= 4 {
                        let face_indices = self.parse_face_indices(&parts[1..], line_num)?;
                        faces.push(face_indices);
                    }
                }
                "mtllib" => {
                    if parts.len() >= 2 {
                        metadata.insert("mtllib".to_string(), parts[1..].join(" "));
                    }
                }
                "usemtl" => {
                    if parts.len() >= 2 {
                        metadata.insert("usemtl".to_string(), parts[1..].join(" "));
                    }
                }
                "o" => {
                    if parts.len() >= 2 {
                        metadata.insert("object_name".to_string(), parts[1..].join(" "));
                    }
                }
                "g" => {
                    if parts.len() >= 2 {
                        metadata.insert("group_name".to_string(), parts[1..].join(" "));
                    }
                }
                _ => {
                    // Skip unknown lines
                }
            }
        }

        // Convert to ModelElement
        let element = self.create_mesh_element(vertices, normals, uvs, faces)?;

        Ok(LoadedModel {
            format: FileFormat::OBJ,
            elements: vec![element],
            materials,
            metadata,
        })
    }
}

impl ObjParser {
    fn parse_face_indices(&self, face_parts: &[&str], line_num: usize) -> ParseResult<Vec<ObjFaceIndex>> {
        let mut indices = Vec::new();

        for part in face_parts {
            let sub_parts: Vec<&str> = part.split('/').collect();

            let vertex_idx = sub_parts.get(0)
                .and_then(|s| s.parse::<i32>().ok())
                .ok_or_else(|| ParseError::InvalidFormat(
                    format!("Invalid vertex index in face at line {}", line_num)
                ))?;

            let uv_idx = sub_parts.get(1)
                .and_then(|s| if s.is_empty() { None } else { s.parse::<i32>().ok() });

            let normal_idx = sub_parts.get(2)
                .and_then(|s| if s.is_empty() { None } else { s.parse::<i32>().ok() });

            indices.push(ObjFaceIndex {
                vertex: vertex_idx,
                uv: uv_idx,
                normal: normal_idx,
            });
        }

        Ok(indices)
    }

    fn create_mesh_element(
        &self,
        vertices: Vec<[f64; 3]>,
        normals: Vec<[f64; 3]>,
        uvs: Vec<[f64; 2]>,
        faces: Vec<Vec<ObjFaceIndex>>,
    ) -> ParseResult<ModelElement> {
        if vertices.is_empty() {
            return Err(ParseError::InvalidFormat("No vertices found".to_string()));
        }

        let mut mesh_vertices = Vec::new();
        let mut mesh_normals = Vec::new();
        let mut mesh_uvs = Vec::new();
        let mut indices = Vec::new();
        let mut vertex_map = HashMap::new();

        // Process each face
        for face in faces {
            if face.len() < 3 {
                continue; // Skip degenerate faces
            }

            let mut face_indices = Vec::new();

            // Convert face indices to mesh indices
            for face_idx in &face {
                let key = (face_idx.vertex, face_idx.uv, face_idx.normal);
                let mesh_idx = if let Some(&idx) = vertex_map.get(&key) {
                    idx
                } else {
                    let idx = mesh_vertices.len();

                    // Add vertex (OBJ uses 1-based indexing, convert to 0-based)
                    let vertex_idx = self.obj_to_zero_based(face_idx.vertex, vertices.len());
                    mesh_vertices.push(vertices[vertex_idx]);

                    // Add UV if present
                    if let Some(uv_idx) = face_idx.uv {
                        let uv_idx = self.obj_to_zero_based(uv_idx, uvs.len());
                        mesh_uvs.push(uvs[uv_idx]);
                    }

                    // Add normal if present
                    if let Some(normal_idx) = face_idx.normal {
                        let normal_idx = self.obj_to_zero_based(normal_idx, normals.len());
                        mesh_normals.push(normals[normal_idx]);
                    }

                    vertex_map.insert(key, idx);
                    idx
                };

                face_indices.push(mesh_idx as u32);
            }

            // Triangulate face (simple fan triangulation)
            for i in 1..face_indices.len() - 1 {
                indices.push(face_indices[0]);
                indices.push(face_indices[i]);
                indices.push(face_indices[i + 1]);
            }
        }

        let geometry = ElementGeometry::Mesh {
            vertices: mesh_vertices,
            indices,
            normals: if mesh_normals.is_empty() { None } else { Some(mesh_normals) },
            uvs: if mesh_uvs.is_empty() { None } else { Some(mesh_uvs) },
        };

        let properties = HashMap::from([
            ("vertex_count".to_string(), PropertyValue::Integer(vertices.len() as i64)),
            ("face_count".to_string(), PropertyValue::Integer(faces.len() as i64)),
            ("normal_count".to_string(), PropertyValue::Integer(normals.len() as i64)),
            ("uv_count".to_string(), PropertyValue::Integer(uvs.len() as i64)),
        ]);

        Ok(ModelElement {
            id: "obj_mesh".to_string(),
            name: Some("OBJ Mesh".to_string()),
            element_type: "Mesh".to_string(),
            geometry,
            properties,
            transform: None,
        })
    }

    fn obj_to_zero_based(&self, obj_idx: i32, array_len: usize) -> usize {
        if obj_idx > 0 {
            (obj_idx - 1) as usize
        } else {
            // Negative indices count from end
            (array_len as i32 + obj_idx) as usize
        }
    }
}

/// Índice de face OBJ (v/vt/vn)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct ObjFaceIndex {
    vertex: i32,
    uv: Option<i32>,
    normal: Option<i32>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_obj_parser_creation() {
        let parser = ObjParser;
        assert!(parser.can_parse(FileFormat::OBJ));
        assert!(!parser.can_parse(FileFormat::IFC));
    }

    #[test]
    fn test_obj_simple_cube() {
        let obj_content = r#"# Simple cube
v 0.0 0.0 0.0
v 1.0 0.0 0.0
v 1.0 1.0 0.0
v 0.0 1.0 0.0
v 0.0 0.0 1.0
v 1.0 0.0 1.0
v 1.0 1.0 1.0
v 0.0 1.0 1.0

f 1 2 3 4
f 5 8 7 6
f 1 5 6 2
f 2 6 7 3
f 3 7 8 4
f 4 8 5 1
"#;

        let parser = ObjParser;
        let result = parser.parse(obj_content.as_bytes(), "cube.obj");

        assert!(result.is_ok());
        let model = result.unwrap();
        assert_eq!(model.elements.len(), 1);

        if let ElementGeometry::Mesh { vertices, indices, .. } = &model.elements[0].geometry {
            assert_eq!(vertices.len(), 8); // 8 vertices
            assert_eq!(indices.len() % 3, 0); // Triangles
        } else {
            panic!("Expected mesh geometry");
        }
    }

    #[test]
    fn test_obj_face_parsing() {
        let parser = ObjParser;
        let face_parts = ["1/2/3", "4/5/6", "7/8/9"];
        let indices = parser.parse_face_indices(&face_parts, 1).unwrap();

        assert_eq!(indices.len(), 3);
        assert_eq!(indices[0].vertex, 1);
        assert_eq!(indices[0].uv, Some(2));
        assert_eq!(indices[0].normal, Some(3));
    }
}
