//! Parser IFC completo (Industry Foundation Classes) (Rust puro)

use crate::file_parsers::*;
use crate::step_tokenizer::{StepTokenizer, Token};
use std::collections::HashMap;

/// Parser IFC
pub struct IfcParser;

impl FileParser for IfcParser {
    fn can_parse(&self, format: FileFormat) -> bool {
        matches!(format, FileFormat::IFC)
    }

    fn parse(&self, data: &[u8], filename: &str) -> ParseResult<LoadedModel> {
        let content = std::str::from_utf8(data)
            .map_err(|_| ParseError::InvalidFormat("Invalid UTF-8 encoding".to_string()))?;

        let mut tokenizer = StepTokenizer::new(content);
        let mut entities = HashMap::new();

        // Parse header
        self.parse_header(&mut tokenizer)?;

        // Parse data section
        self.parse_data_section(&mut tokenizer, &mut entities)?;

        // Convert to LoadedModel
        self.convert_to_model(entities, filename)
    }
}

impl IfcParser {
    fn parse_header(&self, tokenizer: &mut StepTokenizer) -> ParseResult<()> {
        // HEADER section
        let token = tokenizer.next_token()
            .ok_or_else(|| ParseError::InvalidFormat("Missing HEADER".to_string()))?;

        if !matches!(token, Token::Keyword(ref k) if k == "HEADER") {
            return Err(ParseError::InvalidFormat("Expected HEADER".to_string()));
        }

        // Skip header entities for now
        while let Some(token) = tokenizer.next_token() {
            if matches!(token, Token::Keyword(ref k) if k == "DATA") {
                break;
            }
        }

        Ok(())
    }

    fn parse_data_section(
        &self,
        tokenizer: &mut StepTokenizer,
        entities: &mut HashMap<u32, StepEntity>,
    ) -> ParseResult<()> {
        while let Some(token) = tokenizer.next_token() {
            match token {
                Token::EntityRef(id) => {
                    self.parse_entity(tokenizer, id, entities)?;
                }
                Token::Keyword(ref k) if k == "ENDSEC" => break,
                _ => {} // Skip other tokens
            }
        }

        Ok(())
    }

    fn parse_entity(
        &self,
        tokenizer: &mut StepTokenizer,
        id: u32,
        entities: &mut HashMap<u32, StepEntity>,
    ) -> ParseResult<()> {
        // Entity name
        let name_token = tokenizer.next_token()
            .ok_or_else(|| ParseError::InvalidFormat("Missing entity name".to_string()))?;

        let entity_name = match name_token {
            Token::Keyword(name) => name,
            _ => return Err(ParseError::InvalidFormat("Expected entity name".to_string())),
        };

        // Parameters
        let params = self.parse_parameters(tokenizer)?;

        let entity = StepEntity {
            id,
            name: entity_name,
            parameters: params,
        };

        entities.insert(id, entity);
        Ok(())
    }

    fn parse_parameters(&self, tokenizer: &mut StepTokenizer) -> ParseResult<Vec<StepValue>> {
        let mut params = Vec::new();

        // Opening parenthesis
        let token = tokenizer.next_token()
            .ok_or_else(|| ParseError::InvalidFormat("Missing opening parenthesis".to_string()))?;

        if !matches!(token, Token::LeftParen) {
            return Err(ParseError::InvalidFormat("Expected '('".to_string()));
        }

        while let Some(token) = tokenizer.next_token() {
            match token {
                Token::RightParen => break,
                Token::Comma => continue,
                _ => {
                    let value = self.parse_value(token, tokenizer)?;
                    params.push(value);
                }
            }
        }

        Ok(params)
    }

    fn parse_value(&self, token: Token, tokenizer: &mut StepTokenizer) -> ParseResult<StepValue> {
        match token {
            Token::String(s) => Ok(StepValue::String(s)),
            Token::Integer(i) => Ok(StepValue::Integer(i)),
            Token::Float(f) => Ok(StepValue::Float(f)),
            Token::Boolean(b) => Ok(StepValue::Boolean(b)),
            Token::EntityRef(id) => Ok(StepValue::EntityRef(id)),
            Token::Keyword(k) => Ok(StepValue::Keyword(k)),
            Token::LeftParen => {
                let mut values = Vec::new();
                while let Some(token) = tokenizer.next_token() {
                    match token {
                        Token::RightParen => break,
                        Token::Comma => continue,
                        _ => {
                            let value = self.parse_value(token, tokenizer)?;
                            values.push(value);
                        }
                    }
                }
                Ok(StepValue::List(values))
            }
            Token::Null => Ok(StepValue::Null),
            _ => Err(ParseError::InvalidFormat("Unexpected token".to_string())),
        }
    }

    fn convert_to_model(&self, entities: HashMap<u32, StepEntity>, filename: &str) -> ParseResult<LoadedModel> {
        let mut elements = Vec::new();
        let mut materials = Vec::new();
        let mut metadata = HashMap::new();

        // Extract IFC project info
        if let Some(project) = self.find_project(&entities) {
            metadata.insert("project_name".to_string(), project.name.clone());
            metadata.insert("project_id".to_string(), project.id.to_string());
        }

        // Convert IFC entities to ModelElements
        for (id, entity) in entities {
            if let Some(element) = self.convert_entity_to_element(&entity, &entities) {
                elements.push(element);
            }
        }

        Ok(LoadedModel {
            format: FileFormat::IFC,
            elements,
            materials,
            metadata,
        })
    }

    fn find_project(&self, entities: &HashMap<u32, StepEntity>) -> Option<&StepEntity> {
        entities.values().find(|e| e.name == "IFCPROJECT")
    }

    fn convert_entity_to_element(
        &self,
        entity: &StepEntity,
        entities: &HashMap<u32, StepEntity>,
    ) -> Option<ModelElement> {
        match entity.name.as_str() {
            "IFCWALL" | "IFCWALLSTANDARDCASE" => self.convert_wall(entity, entities),
            "IFCSLAB" => self.convert_slab(entity, entities),
            "IFCCOLUMN" => self.convert_column(entity, entities),
            "IFCBEAM" => self.convert_beam(entity, entities),
            "IFCDOOR" => self.convert_door(entity, entities),
            "IFCWINDOW" => self.convert_window(entity, entities),
            _ => None, // Skip unsupported entities
        }
    }

    fn convert_wall(&self, entity: &StepEntity, entities: &HashMap<u32, StepEntity>) -> Option<ModelElement> {
        let geometry = self.extract_geometry(entity, entities)?;

        Some(ModelElement {
            id: format!("wall_{}", entity.id),
            name: self.extract_name(entity),
            element_type: "Wall".to_string(),
            geometry,
            properties: self.extract_properties(entity),
            transform: self.extract_placement(entity, entities),
        })
    }

    fn convert_slab(&self, entity: &StepEntity, entities: &HashMap<u32, StepEntity>) -> Option<ModelElement> {
        let geometry = self.extract_geometry(entity, entities)?;

        Some(ModelElement {
            id: format!("slab_{}", entity.id),
            name: self.extract_name(entity),
            element_type: "Slab".to_string(),
            geometry,
            properties: self.extract_properties(entity),
            transform: self.extract_placement(entity, entities),
        })
    }

    fn convert_column(&self, entity: &StepEntity, entities: &HashMap<u32, StepEntity>) -> Option<ModelElement> {
        let geometry = self.extract_geometry(entity, entities)?;

        Some(ModelElement {
            id: format!("column_{}", entity.id),
            name: self.extract_name(entity),
            element_type: "Column".to_string(),
            geometry,
            properties: self.extract_properties(entity),
            transform: self.extract_placement(entity, entities),
        })
    }

    fn convert_beam(&self, entity: &StepEntity, entities: &HashMap<u32, StepEntity>) -> Option<ModelElement> {
        let geometry = self.extract_geometry(entity, entities)?;

        Some(ModelElement {
            id: format!("beam_{}", entity.id),
            name: self.extract_name(entity),
            element_type: "Beam".to_string(),
            geometry,
            properties: self.extract_properties(entity),
            transform: self.extract_placement(entity, entities),
        })
    }

    fn convert_door(&self, entity: &StepEntity, entities: &HashMap<u32, StepEntity>) -> Option<ModelElement> {
        let geometry = self.extract_geometry(entity, entities)?;

        Some(ModelElement {
            id: format!("door_{}", entity.id),
            name: self.extract_name(entity),
            element_type: "Door".to_string(),
            geometry,
            properties: self.extract_properties(entity),
            transform: self.extract_placement(entity, entities),
        })
    }

    fn convert_window(&self, entity: &StepEntity, entities: &HashMap<u32, StepEntity>) -> Option<ModelElement> {
        let geometry = self.extract_geometry(entity, entities)?;

        Some(ModelElement {
            id: format!("window_{}", entity.id),
            name: self.extract_name(entity),
            element_type: "Window".to_string(),
            geometry,
            properties: self.extract_properties(entity),
            transform: self.extract_placement(entity, entities),
        })
    }

    fn extract_geometry(&self, entity: &StepEntity, entities: &HashMap<u32, StepEntity>) -> Option<ElementGeometry> {
        // Look for representation relationship
        for param in &entity.parameters {
            if let StepValue::EntityRef(rep_id) = param {
                if let Some(rep_entity) = entities.get(rep_id) {
                    if rep_entity.name == "IFCPRODUCTDEFINITIONSHAPE" {
                        return self.extract_from_representation(rep_entity, entities);
                    }
                }
            }
        }

        None
    }

    fn extract_from_representation(
        &self,
        rep_entity: &StepEntity,
        entities: &HashMap<u32, StepEntity>,
    ) -> Option<ElementGeometry> {
        // Find shape representations
        for param in &rep_entity.parameters {
            if let StepValue::List(shapes) = param {
                for shape in shapes {
                    if let StepValue::EntityRef(shape_id) = shape {
                        if let Some(shape_entity) = entities.get(shape_id) {
                            if shape_entity.name == "IFCSHAPEREPRESENTATION" {
                                return self.extract_from_shape_representation(shape_entity, entities);
                            }
                        }
                    }
                }
            }
        }

        None
    }

    fn extract_from_shape_representation(
        &self,
        shape_entity: &StepEntity,
        entities: &HashMap<u32, StepEntity>,
    ) -> Option<ElementGeometry> {
        // Find geometric items
        for param in &shape_entity.parameters {
            if let StepValue::List(items) = param {
                for item in items {
                    if let StepValue::EntityRef(item_id) = item {
                        if let Some(item_entity) = entities.get(item_id) {
                            return self.extract_geometric_item(item_entity, entities);
                        }
                    }
                }
            }
        }

        None
    }

    fn extract_geometric_item(
        &self,
        item_entity: &StepEntity,
        entities: &HashMap<u32, StepEntity>,
    ) -> Option<ElementGeometry> {
        match item_entity.name.as_str() {
            "IFCEXTRUDEDAREASOLID" => self.extract_extruded_area_solid(item_entity, entities),
            "IFCFACETEDBREP" => self.extract_faceted_brep(item_entity, entities),
            _ => None,
        }
    }

    fn extract_extruded_area_solid(
        &self,
        entity: &StepEntity,
        entities: &HashMap<u32, StepEntity>,
    ) -> Option<ElementGeometry> {
        // Parameters: SweptArea, Position, ExtrudedDirection, Depth
        if entity.parameters.len() < 4 {
            return None;
        }

        let swept_area_id = match &entity.parameters[0] {
            StepValue::EntityRef(id) => *id,
            _ => return None,
        };

        let depth = match &entity.parameters[3] {
            StepValue::Float(d) => *d,
            _ => return None,
        };

        // Get profile from swept area
        let profile_points = self.extract_profile_points(swept_area_id, entities)?;

        // Create extruded mesh
        self.create_extruded_mesh(&profile_points, depth)
    }

    fn extract_faceted_brep(
        &self,
        entity: &StepEntity,
        entities: &HashMap<u32, StepEntity>,
    ) -> Option<ElementGeometry> {
        // Parameters: Outer shell
        if entity.parameters.is_empty() {
            return None;
        }

        let shell_id = match &entity.parameters[0] {
            StepValue::EntityRef(id) => *id,
            _ => return None,
        };

        self.extract_closed_shell(shell_id, entities)
    }

    fn extract_profile_points(&self, profile_id: u32, entities: &HashMap<u32, StepEntity>) -> Option<Vec<[f64; 2]>> {
        let profile = entities.get(&profile_id)?;

        match profile.name.as_str() {
            "IFCRECTANGLEPROFILEDEF" => self.extract_rectangle_profile(profile),
            "IFCCIRCLEPROFILEDEF" => self.extract_circle_profile(profile),
            "IFCARBITRARYCLOSEDPROFILEDEF" => self.extract_arbitrary_profile(profile, entities),
            _ => None,
        }
    }

    fn extract_rectangle_profile(&self, profile: &StepEntity) -> Option<Vec<[f64; 2]>> {
        // Parameters: Position, XDim, YDim
        if profile.parameters.len() < 3 {
            return None;
        }

        let x_dim = match &profile.parameters[1] {
            StepValue::Float(x) => *x,
            _ => return None,
        };

        let y_dim = match &profile.parameters[2] {
            StepValue::Float(y) => *y,
            _ => return None,
        };

        Some(vec![
            [-x_dim / 2.0, -y_dim / 2.0],
            [x_dim / 2.0, -y_dim / 2.0],
            [x_dim / 2.0, y_dim / 2.0],
            [-x_dim / 2.0, y_dim / 2.0],
        ])
    }

    fn extract_circle_profile(&self, profile: &StepEntity) -> Option<Vec<[f64; 2]>> {
        // Parameters: Position, Radius
        if profile.parameters.len() < 2 {
            return None;
        }

        let radius = match &profile.parameters[1] {
            StepValue::Float(r) => *r,
            _ => return None,
        };

        // Approximate circle with 16 points
        let mut points = Vec::new();
        for i in 0..16 {
            let angle = (i as f64 / 16.0) * 2.0 * std::f64::consts::PI;
            points.push([radius * angle.cos(), radius * angle.sin()]);
        }

        Some(points)
    }

    fn extract_arbitrary_profile(&self, profile: &StepEntity, entities: &HashMap<u32, StepEntity>) -> Option<Vec<[f64; 2]>> {
        // Parameters: ProfileType, ProfileName, OuterCurve
        if profile.parameters.len() < 3 {
            return None;
        }

        let curve_id = match &profile.parameters[2] {
            StepValue::EntityRef(id) => *id,
            _ => return None,
        };

        self.extract_curve_points(curve_id, entities)
    }

    fn extract_curve_points(&self, curve_id: u32, entities: &HashMap<u32, StepEntity>) -> Option<Vec<[f64; 2]>> {
        let curve = entities.get(&curve_id)?;

        match curve.name.as_str() {
            "IFCPOLYLINE" => self.extract_polyline_points(curve),
            _ => None,
        }
    }

    fn extract_polyline_points(&self, polyline: &StepEntity) -> Option<Vec<[f64; 2]>> {
        // Parameters: Points
        if polyline.parameters.is_empty() {
            return None;
        }

        let mut points = Vec::new();

        if let StepValue::List(point_refs) = &polyline.parameters[0] {
            for point_ref in point_refs {
                if let StepValue::EntityRef(point_id) = point_ref {
                    if let Some(point) = self.extract_cartesian_point(*point_id, &HashMap::new()) {
                        points.push([point[0], point[1]]);
                    }
                }
            }
        }

        Some(points)
    }

    fn extract_cartesian_point(&self, point_id: u32, entities: &HashMap<u32, StepEntity>) -> Option<[f64; 3]> {
        let point = entities.get(&point_id)?;

        if point.name != "IFCCARTESIANPOINT" {
            return None;
        }

        if point.parameters.is_empty() {
            return None;
        }

        if let StepValue::List(coords) = &point.parameters[0] {
            if coords.len() >= 2 {
                let x = match &coords[0] {
                    StepValue::Float(f) => *f,
                    _ => return None,
                };
                let y = match &coords[1] {
                    StepValue::Float(f) => *f,
                    _ => return None,
                };
                let z = if coords.len() > 2 {
                    match &coords[2] {
                        StepValue::Float(f) => *f,
                        _ => 0.0,
                    }
                } else {
                    0.0
                };

                return Some([x, y, z]);
            }
        }

        None
    }

    fn create_extruded_mesh(&self, profile: &[[f64; 2]], depth: f64) -> Option<ElementGeometry> {
        use crate::triangulation::Triangulator;

        let mut vertices = Vec::new();
        let mut indices = Vec::new();

        // Bottom face
        let bottom_start = vertices.len() as u32;
        for point in profile {
            vertices.push([point[0], point[1], 0.0]);
        }

        // Triangulate bottom face
        let triangulator = Triangulator;
        if let Ok(bottom_triangles) = triangulator.triangulate_polygon(profile) {
            for triangle in bottom_triangles {
                indices.push(bottom_start + triangle[0] as u32);
                indices.push(bottom_start + triangle[1] as u32);
                indices.push(bottom_start + triangle[2] as u32);
            }
        }

        // Top face
        let top_start = vertices.len() as u32;
        for point in profile {
            vertices.push([point[0], point[1], depth]);
        }

        // Triangulate top face (reverse order)
        let mut reversed_profile: Vec<[f64; 2]> = profile.iter().rev().cloned().collect();
        if let Ok(top_triangles) = triangulator.triangulate_polygon(&reversed_profile) {
            for triangle in top_triangles {
                indices.push(top_start + triangle[0] as u32);
                indices.push(top_start + triangle[1] as u32);
                indices.push(top_start + triangle[2] as u32);
            }
        }

        // Side faces
        let n = profile.len() as u32;
        for i in 0..n {
            let next = (i + 1) % n;

            // Two triangles per side face
            let v1 = bottom_start + i;
            let v2 = bottom_start + next;
            let v3 = top_start + i;
            let v4 = top_start + next;

            // First triangle
            indices.push(v1);
            indices.push(v2);
            indices.push(v3);

            // Second triangle
            indices.push(v2);
            indices.push(v4);
            indices.push(v3);
        }

        Some(ElementGeometry::Mesh {
            vertices,
            indices,
            normals: None,
            uvs: None,
        })
    }

    fn extract_closed_shell(&self, shell_id: u32, entities: &HashMap<u32, StepEntity>) -> Option<ElementGeometry> {
        let shell = entities.get(&shell_id)?;

        if shell.name != "IFCCLOSEDSHELL" {
            return None;
        }

        let mut all_vertices = Vec::new();
        let mut all_indices = Vec::new();
        let mut vertex_map = HashMap::new();

        // Process faces
        if let Some(StepValue::List(face_refs)) = shell.parameters.get(0) {
            for face_ref in face_refs {
                if let StepValue::EntityRef(face_id) = face_ref {
                    if let Some(face_geom) = self.extract_face(*face_id, entities, &mut all_vertices, &mut vertex_map) {
                        // Offset indices for global vertex list
                        let offset = all_indices.len() as u32;
                        all_indices.extend(face_geom.indices.iter().map(|&i| i + offset));
                    }
                }
            }
        }

        if all_vertices.is_empty() {
            None
        } else {
            Some(ElementGeometry::Mesh {
                vertices: all_vertices,
                indices: all_indices,
                normals: None,
                uvs: None,
            })
        }
    }

    fn extract_face(
        &self,
        face_id: u32,
        entities: &HashMap<u32, StepEntity>,
        all_vertices: &mut Vec<[f64; 3]>,
        vertex_map: &mut HashMap<u32, u32>,
    ) -> Option<ElementGeometry> {
        let face = entities.get(&face_id)?;

        if face.name != "IFCFACE" {
            return None;
        }

        // Get bounds (outer loop)
        if let Some(StepValue::EntityRef(loop_id)) = face.parameters.get(0) {
            return self.extract_face_bound(*loop_id, entities, all_vertices, vertex_map);
        }

        None
    }

    fn extract_face_bound(
        &self,
        loop_id: u32,
        entities: &HashMap<u32, StepEntity>,
        all_vertices: &mut Vec<[f64; 3]>,
        vertex_map: &mut HashMap<u32, u32>,
    ) -> Option<ElementGeometry> {
        let face_loop = entities.get(&loop_id)?;

        if face_loop.name != "IFCFACEOUTERBOUND" {
            return None;
        }

        // Get edge loop
        if let Some(StepValue::EntityRef(edge_loop_id)) = face_loop.parameters.get(0) {
            return self.extract_edge_loop(*edge_loop_id, entities, all_vertices, vertex_map);
        }

        None
    }

    fn extract_edge_loop(
        &self,
        edge_loop_id: u32,
        entities: &HashMap<u32, StepEntity>,
        all_vertices: &mut Vec<[f64; 3]>,
        vertex_map: &mut HashMap<u32, u32>,
    ) -> Option<ElementGeometry> {
        let edge_loop = entities.get(&edge_loop_id)?;

        if edge_loop.name != "IFCPOLYLOOP" {
            return None;
        }

        // Get vertex list
        if let Some(StepValue::List(vertex_refs)) = edge_loop.parameters.get(0) {
            let mut face_vertices = Vec::new();
            let mut face_indices = Vec::new();

            for vertex_ref in vertex_refs {
                if let StepValue::EntityRef(vertex_id) = vertex_ref {
                    let local_index = if let Some(&existing_index) = vertex_map.get(vertex_id) {
                        existing_index
                    } else {
                        if let Some(point) = self.extract_cartesian_point(*vertex_id, entities) {
                            let new_index = all_vertices.len() as u32;
                            all_vertices.push(point);
                            vertex_map.insert(*vertex_id, new_index);
                            new_index
                        } else {
                            continue;
                        }
                    };
                    face_vertices.push(local_index);
                }
            }

            // Triangulate face
            if face_vertices.len() >= 3 {
                // Simple fan triangulation for convex faces
                for i in 1..face_vertices.len() - 1 {
                    face_indices.push(face_vertices[0]);
                    face_indices.push(face_vertices[i]);
                    face_indices.push(face_vertices[i + 1]);
                }
            }

            return Some(ElementGeometry::Mesh {
                vertices: Vec::new(), // Vertices are in global list
                indices: face_indices,
                normals: None,
                uvs: None,
            });
        }

        None
    }

    fn extract_name(&self, entity: &StepEntity) -> Option<String> {
        // Look for Name attribute in IFC entities
        for param in &entity.parameters {
            if let StepValue::String(name) = param {
                return Some(name.clone());
            }
        }
        None
    }

    fn extract_properties(&self, entity: &StepEntity) -> HashMap<String, PropertyValue> {
        let mut properties = HashMap::new();

        // Basic properties
        properties.insert("ifc_type".to_string(), PropertyValue::String(entity.name.clone()));
        properties.insert("ifc_id".to_string(), PropertyValue::Integer(entity.id as i64));

        properties
    }

    fn extract_placement(&self, entity: &StepEntity, entities: &HashMap<u32, StepEntity>) -> Option<TransformMatrix> {
        // Look for ObjectPlacement
        for param in &entity.parameters {
            if let StepValue::EntityRef(placement_id) = param {
                if let Some(placement) = entities.get(placement_id) {
                    if placement.name == "IFCLOCALPLACEMENT" {
                        return self.extract_local_placement(placement, entities);
                    }
                }
            }
        }

        None
    }

    fn extract_local_placement(&self, placement: &StepEntity, entities: &HashMap<u32, StepEntity>) -> Option<TransformMatrix> {
        // Parameters: PlacementRelTo, RelativePlacement
        if placement.parameters.len() < 2 {
            return None;
        }

        let relative_id = match &placement.parameters[1] {
            StepValue::EntityRef(id) => *id,
            _ => return None,
        };

        let relative = entities.get(&relative_id)?;
        if relative.name != "IFCAXIS2PLACEMENT3D" {
            return None;
        }

        self.extract_axis2_placement_3d(relative, entities)
    }

    fn extract_axis2_placement_3d(&self, placement: &StepEntity, entities: &HashMap<u32, StepEntity>) -> Option<TransformMatrix> {
        // Parameters: Location, Axis, RefDirection
        if placement.parameters.is_empty() {
            return None;
        }

        let location_id = match &placement.parameters[0] {
            StepValue::EntityRef(id) => *id,
            _ => return None,
        };

        let location = self.extract_cartesian_point(location_id, entities)?;

        // For now, return identity matrix with translation
        Some(TransformMatrix {
            matrix: [
                1.0, 0.0, 0.0, 0.0,
                0.0, 1.0, 0.0, 0.0,
                0.0, 0.0, 1.0, 0.0,
                location[0], location[1], location[2], 1.0,
            ],
        })
    }
}

/// Entidade STEP parseada
#[derive(Debug, Clone)]
pub struct StepEntity {
    pub id: u32,
    pub name: String,
    pub parameters: Vec<StepValue>,
}

/// Valor STEP
#[derive(Debug, Clone)]
pub enum StepValue {
    String(String),
    Integer(i64),
    Float(f64),
    Boolean(bool),
    EntityRef(u32),
    Keyword(String),
    List(Vec<StepValue>),
    Null,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ifc_parser_creation() {
        let parser = IfcParser;
        assert!(parser.can_parse(FileFormat::IFC));
        assert!(!parser.can_parse(FileFormat::DWG));
    }

    #[test]
    fn test_step_tokenizer_basic() {
        let content = "#1=IFCWALL('Wall1',#2,#3);";
        let mut tokenizer = StepTokenizer::new(content);

        // Skip to entity
        while let Some(token) = tokenizer.next_token() {
            if matches!(token, Token::EntityRef(1)) {
                break;
            }
        }

        let token = tokenizer.next_token().unwrap();
        assert!(matches!(token, Token::Equal));

        let token = tokenizer.next_token().unwrap();
        assert!(matches!(token, Token::Keyword(ref k) if k == "IFCWALL"));
    }
}
