//! Parsers adicionais para tipos IFC complexos

use alloc::vec::Vec;
use alloc::collections::BTreeMap;
use crate::IfcGeometry;

/// Parser para IFCPOLYLINE - linhas conectadas
pub fn parse_polyline(
    entity_id: u32,
    params: &[String],
    entities: &BTreeMap<u32, (String, Vec<String>)>,
) -> Option<IfcGeometry> {
    // IFCPOLYLINE(Points) - lista de pontos separados por vírgula
    if params.is_empty() {
        return None;
    }

    // Reconstrói lista completa (IFC parser divide por vírgula)
    let mut point_ids = Vec::new();
    for param in params {
        if let Some(id) = extract_ref_id(param) {
            point_ids.push(id);
        }
    }

    if point_ids.is_empty() {
        return None;
    }

    // Coleta pontos 3D
    let mut vertices = Vec::new();
    for point_id in &point_ids {
        if let Some((point_type, point_params)) = entities.get(point_id) {
            if point_type == "IFCCARTESIANPOINT" && !point_params.is_empty() {
                // Reconstrói coordenadas (IFC parser separa por vírgula)
                let coords_str = point_params.join(",");
                if let Some(coords) = parse_cartesian_point(&coords_str) {
                    vertices.push([coords.0, coords.1, coords.2]);
                }
            }
        }
    }

    if vertices.len() < 2 {
        return None;
    }    // Cria geometria de linha (como cilindro fino entre pontos)
    let mut all_vertices = Vec::new();
    let mut indices = Vec::new();
    let mut normals = Vec::new();
    let radius = 0.05; // Raio da linha

    for i in 0..vertices.len() - 1 {
        let p1 = vertices[i];
        let p2 = vertices[i + 1];

        // Adiciona segmento como cilindro simples (4 vértices por segmento)
        let base_idx = all_vertices.len() as u32;

        all_vertices.push(p1);
        all_vertices.push([p1[0] + radius, p1[1], p1[2]]);
        all_vertices.push(p2);
        all_vertices.push([p2[0] + radius, p2[1], p2[2]]);

        // Triângulos
        indices.push(base_idx);
        indices.push(base_idx + 1);
        indices.push(base_idx + 2);

        indices.push(base_idx + 1);
        indices.push(base_idx + 3);
        indices.push(base_idx + 2);

        // Normais simples
        normals.push([0.0, 0.0, 1.0]);
        normals.push([0.0, 0.0, 1.0]);
        normals.push([0.0, 0.0, 1.0]);
        normals.push([0.0, 0.0, 1.0]);
    }

    // Calcula bounding box
    let bbox_min = [
        vertices.iter().map(|v| v[0]).min_by(|a, b| a.partial_cmp(b).unwrap())?,
        vertices.iter().map(|v| v[1]).min_by(|a, b| a.partial_cmp(b).unwrap())?,
        vertices.iter().map(|v| v[2]).min_by(|a, b| a.partial_cmp(b).unwrap())?,
    ];
    let bbox_max = [
        vertices.iter().map(|v| v[0]).max_by(|a, b| a.partial_cmp(b).unwrap())?,
        vertices.iter().map(|v| v[1]).max_by(|a, b| a.partial_cmp(b).unwrap())?,
        vertices.iter().map(|v| v[2]).max_by(|a, b| a.partial_cmp(b).unwrap())?,
    ];

    Some(IfcGeometry {
        entity_id,
        entity_type: "IFCPOLYLINE".to_string(),
        vertices: all_vertices,
        indices,
        normals,
        color: [0.9, 0.3, 0.3, 1.0], // Vermelho (linhas elétricas)
        bbox_min,
        bbox_max,
    })
}

/// Parser para IFCFACEOUTERBOUND - face com contorno externo
pub fn parse_face_outer_bound(
    entity_id: u32,
    params: &[String],
    entities: &BTreeMap<u32, (String, Vec<String>)>,
) -> Option<IfcGeometry> {
    // IFCFACEOUTERBOUND(Bound, Orientation)
    if params.is_empty() {
        return None;
    }

    let bound_id = extract_ref_id(&params[0])?;
    let (bound_type, bound_params) = entities.get(&bound_id)?;

    // Se é um polyloop
    if bound_type == "IFCPOLYLOOP" && !bound_params.is_empty() {
        // Reconstrói lista de IDs (IFC parser separa por vírgula)
        let ids_str = bound_params.join(",");
        let point_ids = extract_id_list(&ids_str)?;

        let mut vertices = Vec::new();
        for point_id in &point_ids {
            if let Some((pt_type, pt_params)) = entities.get(point_id) {
                if pt_type == "IFCCARTESIANPOINT" && !pt_params.is_empty() {
                    // Reconstrói coordenadas (IFC parser separa por vírgula)
                    let coords_str = pt_params.join(",");
                    if let Some(coords) = parse_cartesian_point(&coords_str) {
                        vertices.push([coords.0, coords.1, coords.2]);
                    }
                }
            }
        }

        if vertices.len() < 3 {
            return None;
        }

        // Triangula face (fan triangulation)
        let mut indices = Vec::new();
        for i in 1..vertices.len() - 1 {
            indices.push(0);
            indices.push(i as u32);
            indices.push((i + 1) as u32);
        }

        // Calcula normal da face
        let v1 = [
            vertices[1][0] - vertices[0][0],
            vertices[1][1] - vertices[0][1],
            vertices[1][2] - vertices[0][2],
        ];
        let v2 = [
            vertices[2][0] - vertices[0][0],
            vertices[2][1] - vertices[0][1],
            vertices[2][2] - vertices[0][2],
        ];
        let normal = [
            v1[1] * v2[2] - v1[2] * v2[1],
            v1[2] * v2[0] - v1[0] * v2[2],
            v1[0] * v2[1] - v1[1] * v2[0],
        ];
        let len = (normal[0] * normal[0] + normal[1] * normal[1] + normal[2] * normal[2]).sqrt();
        let normal = if len > 0.0001 {
            [normal[0] / len, normal[1] / len, normal[2] / len]
        } else {
            [0.0, 0.0, 1.0]
        };

        let normals = vec![normal; vertices.len()];

        // Bounding box
        let bbox_min = [
            vertices.iter().map(|v| v[0]).min_by(|a, b| a.partial_cmp(b).unwrap())?,
            vertices.iter().map(|v| v[1]).min_by(|a, b| a.partial_cmp(b).unwrap())?,
            vertices.iter().map(|v| v[2]).min_by(|a, b| a.partial_cmp(b).unwrap())?,
        ];
        let bbox_max = [
            vertices.iter().map(|v| v[0]).max_by(|a, b| a.partial_cmp(b).unwrap())?,
            vertices.iter().map(|v| v[1]).max_by(|a, b| a.partial_cmp(b).unwrap())?,
            vertices.iter().map(|v| v[2]).max_by(|a, b| a.partial_cmp(b).unwrap())?,
        ];

        return Some(IfcGeometry {
            entity_id,
            entity_type: "IFCFACEOUTERBOUND".to_string(),
            vertices,
            indices,
            normals,
            color: [0.3, 0.8, 0.3, 1.0], // Verde (faces)
            bbox_min,
            bbox_max,
        });
    }

    None
}

/// Parser para IFCTRIANGULATEDFACESET
pub fn parse_triangulated_faceset(
    entity_id: u32,
    params: &[String],
    entities: &BTreeMap<u32, (String, Vec<String>)>,
) -> Option<IfcGeometry> {
    // IFCTRIANGULATEDFACESET(Coordinates, Normals, Closed, CoordIndex, NormalIndex)
    if params.len() < 3 {
        return None;
    }

    // Extrai coordenadas
    let coord_id = extract_ref_id(&params[0])?;
    let (coord_type, coord_params) = entities.get(&coord_id)?;

    if coord_type != "IFCCARTESIANPOINTLIST3D" || coord_params.is_empty() {
        return None;
    }

    // Parse lista de pontos
    let points_str = &coord_params[0];
    let vertices = parse_point_list_3d(points_str)?;

    // Extrai índices dos triângulos
    let coord_index_str = if params.len() >= 4 {
        &params[3]
    } else {
        return None;
    };

    let indices = parse_index_list(coord_index_str)?;

    // Calcula normais se não fornecidas
    let normals = calculate_normals(&vertices, &indices);

    // Bounding box
    let bbox_min = [
        vertices.iter().map(|v| v[0]).min_by(|a, b| a.partial_cmp(b).unwrap())?,
        vertices.iter().map(|v| v[1]).min_by(|a, b| a.partial_cmp(b).unwrap())?,
        vertices.iter().map(|v| v[2]).min_by(|a, b| a.partial_cmp(b).unwrap())?,
    ];
    let bbox_max = [
        vertices.iter().map(|v| v[0]).max_by(|a, b| a.partial_cmp(b).unwrap())?,
        vertices.iter().map(|v| v[1]).max_by(|a, b| a.partial_cmp(b).unwrap())?,
        vertices.iter().map(|v| v[2]).max_by(|a, b| a.partial_cmp(b).unwrap())?,
    ];

    Some(IfcGeometry {
        entity_id,
        entity_type: "IFCTRIANGULATEDFACESET".to_string(),
        vertices,
        indices,
        normals,
        color: [0.3, 0.8, 0.8, 1.0], // Ciano (facesets)
        bbox_min,
        bbox_max,
    })
}

// ===== Funções auxiliares =====

/// Extrai lista de IDs do formato (#123,#456,#789)
fn extract_id_list(s: &str) -> Option<Vec<u32>> {
    let cleaned = s.trim().trim_start_matches('(').trim_end_matches(')');
    let mut ids = Vec::new();

    for part in cleaned.split(',') {
        if let Some(id) = extract_ref_id(part.trim()) {
            ids.push(id);
        }
    }

    if ids.is_empty() {
        None
    } else {
        Some(ids)
    }
}

/// Extrai ID de referência do formato #123
fn extract_ref_id(s: &str) -> Option<u32> {
    s.trim()
        .trim_start_matches('(')  // Remove parêntese de abertura
        .trim_end_matches(')')    // Remove parêntese de fechamento
        .trim_start_matches('#')
        .trim_end_matches(';')
        .parse::<u32>()
        .ok()
}

/// Parse cartesian point do formato (x,y) ou (x,y,z)
fn parse_cartesian_point(s: &str) -> Option<(f32, f32, f32)> {
    let cleaned = s.trim().trim_start_matches('(').trim_end_matches(')');
    let parts: Vec<&str> = cleaned.split(',').collect();

    if parts.len() >= 2 {
        let x = parts[0].trim().parse::<f32>().ok()?;
        let y = parts[1].trim().parse::<f32>().ok()?;
        let z = if parts.len() >= 3 {
            parts[2].trim().parse::<f32>().ok()?
        } else {
            0.0  // Pontos 2D têm Z=0
        };
        Some((x, y, z))
    } else {
        None
    }
}

/// Parse lista de pontos 3D do formato ((x1,y1,z1),(x2,y2,z2),...)
fn parse_point_list_3d(s: &str) -> Option<Vec<[f32; 3]>> {
    let mut vertices = Vec::new();
    let cleaned = s.trim().trim_start_matches('(').trim_end_matches(')');

    let mut depth = 0;
    let mut current = String::new();

    for ch in cleaned.chars() {
        match ch {
            '(' => {
                depth += 1;
                current.push(ch);
            }
            ')' => {
                depth -= 1;
                current.push(ch);
                if depth == 0 && !current.is_empty() {
                    if let Some((x, y, z)) = parse_cartesian_point(&current) {
                        vertices.push([x, y, z]);
                    }
                    current.clear();
                }
            }
            ',' if depth == 0 => {
                // Separador entre pontos
            }
            _ => {
                current.push(ch);
            }
        }
    }

    if vertices.is_empty() {
        None
    } else {
        Some(vertices)
    }
}

/// Parse lista de índices do formato ((1,2,3),(4,5,6),...)
fn parse_index_list(s: &str) -> Option<Vec<u32>> {
    let mut indices = Vec::new();
    let cleaned = s.trim().trim_start_matches('(').trim_end_matches(')');

    let mut depth = 0;
    let mut current = String::new();

    for ch in cleaned.chars() {
        match ch {
            '(' => {
                depth += 1;
                if depth > 1 {
                    current.push(ch);
                }
            }
            ')' => {
                depth -= 1;
                if depth == 0 && !current.is_empty() {
                    // Parse tripla (i1,i2,i3)
                    for num_str in current.split(',') {
                        if let Ok(idx) = num_str.trim().parse::<u32>() {
                            indices.push(idx - 1); // IFC usa índices base-1
                        }
                    }
                    current.clear();
                }
            }
            ',' if depth == 0 => {
                // Separador entre triplas
            }
            _ if depth > 0 => {
                current.push(ch);
            }
            _ => {}
        }
    }

    if indices.is_empty() {
        None
    } else {
        Some(indices)
    }
}

/// Calcula normais por vértice
fn calculate_normals(vertices: &[[f32; 3]], indices: &[u32]) -> Vec<[f32; 3]> {
    let mut normals = vec![[0.0f32, 0.0, 0.0]; vertices.len()];

    // Acumula normais de cada face
    for tri in indices.chunks(3) {
        if tri.len() == 3 {
            let i0 = tri[0] as usize;
            let i1 = tri[1] as usize;
            let i2 = tri[2] as usize;

            if i0 < vertices.len() && i1 < vertices.len() && i2 < vertices.len() {
                let v0 = vertices[i0];
                let v1 = vertices[i1];
                let v2 = vertices[i2];

                // Vetores da face
                let e1 = [v1[0] - v0[0], v1[1] - v0[1], v1[2] - v0[2]];
                let e2 = [v2[0] - v0[0], v2[1] - v0[1], v2[2] - v0[2]];

                // Cross product
                let n = [
                    e1[1] * e2[2] - e1[2] * e2[1],
                    e1[2] * e2[0] - e1[0] * e2[2],
                    e1[0] * e2[1] - e1[1] * e2[0],
                ];

                // Acumula em cada vértice
                normals[i0][0] += n[0];
                normals[i0][1] += n[1];
                normals[i0][2] += n[2];

                normals[i1][0] += n[0];
                normals[i1][1] += n[1];
                normals[i1][2] += n[2];

                normals[i2][0] += n[0];
                normals[i2][1] += n[1];
                normals[i2][2] += n[2];
            }
        }
    }

    // Normaliza
    for normal in &mut normals {
        let len = (normal[0] * normal[0] + normal[1] * normal[1] + normal[2] * normal[2]).sqrt();
        if len > 0.0001 {
            normal[0] /= len;
            normal[1] /= len;
            normal[2] /= len;
        } else {
            *normal = [0.0, 0.0, 1.0];
        }
    }

    normals
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_id_list() {
        let result = extract_id_list("(#123,#456,#789)");
        assert_eq!(result, Some(vec![123, 456, 789]));
    }

    #[test]
    fn test_parse_cartesian_point() {
        let result = parse_cartesian_point("(1.5,2.5,3.5)");
        assert_eq!(result, Some((1.5, 2.5, 3.5)));
    }
}
