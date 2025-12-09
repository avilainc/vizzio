//! Extração de geometria 3D de entidades IFC

use alloc::vec::Vec;
use alloc::collections::BTreeMap;
use avila_error::Result;
use crate::{IfcModel, IfcGeometry};

/// Extrai geometria de todas as entidades
pub fn extract_all(model: &IfcModel) -> Result<Vec<IfcGeometry>> {
    let mut geometries = Vec::new();

    // Cria mapa de entidades para lookup rápido
    let mut entity_map: BTreeMap<u32, (String, Vec<String>)> = BTreeMap::new();
    for entity in &model.entities {
        entity_map.insert(
            entity.id,
            (entity.entity_type.clone(), entity.params.clone())
        );
    }

    // Processa cada entidade
    for entity in &model.entities {
        let geom = match entity.entity_type.as_str() {
            "IFCWALL" => create_wall_geometry(entity.id, &entity.params, &entity_map),
            "IFCSLAB" => create_slab_geometry(entity.id, &entity.params, &entity_map),
            "IFCBEAM" => create_beam_geometry(entity.id, &entity.params, &entity_map),
            "IFCCOLUMN" => create_column_geometry(entity.id, &entity.params, &entity_map),
            "IFCEXTRUDEDAREASOLID" => {
                if entity.params.len() >= 4 {
                    let params_str = entity.params.join(",");
                    parse_extruded_area_solid(&params_str, &entity_map)
                } else {
                    None
                }
            },
            // Novos tipos IFC
            "IFCPOLYLINE" => crate::geometry_extra::parse_polyline(
                entity.id,
                &entity.params,
                &entity_map,
            ),
            "IFCFACEOUTERBOUND" => crate::geometry_extra::parse_face_outer_bound(
                entity.id,
                &entity.params,
                &entity_map,
            ),
            "IFCTRIANGULATEDFACESET" => crate::geometry_extra::parse_triangulated_faceset(
                entity.id,
                &entity.params,
                &entity_map,
            ),
            _ => None,
        };

        if let Some(g) = geom {
            geometries.push(g);
        }
    }

    Ok(geometries)
}

/// Retorna cor baseada no tipo IFC
fn get_color_for_type(ifc_type: &str) -> [f32; 4] {
    match ifc_type {
        "IFCWALL" => [0.85, 0.85, 0.85, 1.0],           // Cinza claro (paredes)
        "IFCSLAB" => [0.75, 0.70, 0.65, 1.0],           // Bege (lajes)
        "IFCBEAM" => [0.4, 0.6, 0.8, 1.0],              // Azul claro (vigas)
        "IFCCOLUMN" => [0.3, 0.5, 0.7, 1.0],            // Azul escuro (pilares)
        "IFCPOLYLINE" => [0.9, 0.3, 0.3, 1.0],          // Vermelho (linhas elétricas)
        "IFCFACEOUTERBOUND" => [0.3, 0.8, 0.3, 1.0],    // Verde (faces)
        "IFCEXTRUDEDAREASOLID" => [0.7, 0.7, 0.7, 1.0], // Cinza médio
        _ => [0.6, 0.6, 0.6, 1.0],                      // Cinza padrão
    }
}

/// Parseia float de string IFC
fn parse_float(s: &str) -> Option<f32> {
    s.trim().trim_end_matches(';').parse::<f32>().ok()
}

/// Parseia IFCCARTESIANPOINT((x,y,z)) ou ((x,y))
fn parse_cartesian_point(params: &str) -> Option<(f32, f32, f32)> {
    let inner = params.trim_start_matches('(').trim_end_matches(')');
    let inner = inner.trim_start_matches('(').trim_end_matches(')');
    let coords: Vec<&str> = inner.split(',').collect();

    match coords.len() {
        2 => {
            let x = parse_float(coords[0])?;
            let y = parse_float(coords[1])?;
            Some((x, y, 0.0))
        }
        3 => {
            let x = parse_float(coords[0])?;
            let y = parse_float(coords[1])?;
            let z = parse_float(coords[2])?;
            Some((x, y, z))
        }
        _ => None,
    }
}

/// Extrai ID de referência do formato #123
fn extract_ref_id(s: &str) -> Option<u32> {
    let trimmed = s.trim().trim_end_matches(';');
    if trimmed.starts_with('#') {
        trimmed[1..].parse::<u32>().ok()
    } else {
        None
    }
}

/// Parseia IFCEXTRUDEDAREASOLID para criar geometria de caixa
/// Formato: IFCEXTRUDEDAREASOLID(#profile,#position,#direction,depth)
fn parse_extruded_area_solid(
    params: &str,
    entities: &BTreeMap<u32, (String, Vec<String>)>,
) -> Option<IfcGeometry> {
    // Parseia parâmetros: (#53,#54,#9,2.7196400244049581)
    let inner = params.trim_start_matches('(').trim_end_matches(')').trim_end_matches(';');
    let parts: Vec<&str> = inner.split(',').collect();

    if parts.len() < 4 {
        return None;
    }

    // Extrai profundidade de extrusão
    let depth = parse_float(parts[3])?;

    // Obtém referência do perfil
    let profile_id = extract_ref_id(parts[0])?;
    let (profile_type, profile_params) = entities.get(&profile_id)?;

    // Parseia perfil retangular: IFCRECTANGLEPROFILEDEF(.AREA.,$,#52,2.78,4.74)
    if profile_type == "IFCRECTANGLEPROFILEDEF" {
        if profile_params.len() >= 5 {
            let width = parse_float(&profile_params[3])?;
            let height = parse_float(&profile_params[4])?;

            // Obtém posição para translação
            let position_id = extract_ref_id(parts[1])?;
            let position = entities.get(&position_id)
                .and_then(|(ptype, pparams)| {
                    if ptype == "IFCAXIS2PLACEMENT3D" && !pparams.is_empty() {
                        let point_id = extract_ref_id(&pparams[0])?;
                        entities.get(&point_id)
                            .and_then(|(pt_type, pt_params)| {
                                if pt_type == "IFCCARTESIANPOINT" && !pt_params.is_empty() {
                                    // pt_params[0] é algo como "(0.,0.,0.)"
                                    parse_cartesian_point(&pt_params[0])
                                } else {
                                    None
                                }
                            })
                    } else {
                        None
                    }
                })
                .unwrap_or((0.0, 0.0, 0.0));

            return Some(create_box_geometry(0, "IFCEXTRUDEDAREASOLID", width, height, depth, position));
        }
    }

    None
}

/// Cria geometria de caixa a partir de dimensões e posição
fn create_box_geometry(
    entity_id: u32,
    entity_type: &str,
    width: f32,
    height: f32,
    depth: f32,
    pos: (f32, f32, f32),
) -> IfcGeometry {
    let (x, y, z) = pos;
    let w = width / 2.0;
    let h = height / 2.0;

    IfcGeometry {
        entity_id,
        entity_type: entity_type.to_string(),
        vertices: vec![
            [x - w, y - h, z],          // v0
            [x + w, y - h, z],          // v1
            [x + w, y + h, z],          // v2
            [x - w, y + h, z],          // v3
            [x - w, y - h, z + depth],  // v4
            [x + w, y - h, z + depth],  // v5
            [x + w, y + h, z + depth],  // v6
            [x - w, y + h, z + depth],  // v7
        ],
        indices: vec![
            0, 1, 2, 2, 3, 0, // front
            4, 5, 6, 6, 7, 4, // back
            0, 4, 7, 7, 3, 0, // left
            1, 5, 6, 6, 2, 1, // right
            3, 2, 6, 6, 7, 3, // top
            0, 1, 5, 5, 4, 0, // bottom
        ],
        normals: vec![
            [0.0, 0.0, 1.0], [0.0, 0.0, 1.0], [0.0, 0.0, 1.0], [0.0, 0.0, 1.0],
            [0.0, 0.0, -1.0], [0.0, 0.0, -1.0], [0.0, 0.0, -1.0], [0.0, 0.0, -1.0],
        ],
        color: get_color_for_type(entity_type),
        bbox_min: [x - w, y - h, z],
        bbox_max: [x + w, y + h, z + depth],
    }
}

/// Cria geometria para elemento de parede
fn create_wall_geometry(
    entity_id: u32,
    _params: &[String],
    _entities: &BTreeMap<u32, (String, Vec<String>)>,
) -> Option<IfcGeometry> {
    // Por enquanto retorna placeholder - TODO: vincular com IFCEXTRUDEDAREASOLID
    Some(create_box_geometry(entity_id, "IFCWALL", 1.0, 3.0, 0.3, (0.0, 0.0, 0.0)))
}

/// Cria geometria para elemento de laje
fn create_slab_geometry(
    entity_id: u32,
    _params: &[String],
    _entities: &BTreeMap<u32, (String, Vec<String>)>,
) -> Option<IfcGeometry> {
    Some(create_box_geometry(entity_id, "IFCSLAB", 5.0, 5.0, 0.2, (0.0, 0.0, 0.0)))
}

/// Cria geometria para elemento de viga
fn create_beam_geometry(
    entity_id: u32,
    _params: &[String],
    _entities: &BTreeMap<u32, (String, Vec<String>)>,
) -> Option<IfcGeometry> {
    Some(create_box_geometry(entity_id, "IFCBEAM", 5.0, 0.3, 0.4, (0.0, 0.0, 0.0)))
}

/// Cria geometria para elemento de coluna
fn create_column_geometry(
    entity_id: u32,
    _params: &[String],
    _entities: &BTreeMap<u32, (String, Vec<String>)>,
) -> Option<IfcGeometry> {
    Some(create_box_geometry(entity_id, "IFCCOLUMN", 0.3, 0.3, 3.0, (0.0, 0.0, 0.0)))
}

