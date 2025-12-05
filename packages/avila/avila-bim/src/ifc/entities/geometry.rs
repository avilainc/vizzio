//! Geometry entities (IfcExtrudedAreaSolid, IfcFacetedBrep, etc.)

use crate::bim_core::*;

/// Converter para geometria IFC
pub struct GeometryConverter;

impl GeometryConverter {
    /// Converter IfcExtrudedAreaSolid → Mesh
    pub fn convert_extruded_solid(
        profile: ExtrusionProfile,
        depth: f64,
    ) -> Result<Mesh> {
        match profile {
            ExtrusionProfile::Rectangle { width, height } => {
                Self::extrude_rectangle(width, height, depth)
            }
            ExtrusionProfile::Circle { radius } => {
                Self::extrude_circle(radius, depth)
            }
            ExtrusionProfile::IShape { width, height, web_thickness, flange_thickness } => {
                Self::extrude_i_shape(width, height, web_thickness, flange_thickness, depth)
            }
        }
    }

    /// Extrudar retângulo → Caixa
    fn extrude_rectangle(width: f64, height: f64, depth: f64) -> Result<Mesh> {
        Ok(crate::mesh_gen::MeshGenerator::box_mesh(width, height, depth))
    }

    /// Extrudar círculo → Cilindro
    fn extrude_circle(radius: f64, depth: f64) -> Result<Mesh> {
        Ok(crate::mesh_gen::MeshGenerator::cylinder_mesh(radius, depth, 32))
    }

    /// Extrudar perfil I
    fn extrude_i_shape(
        width: f64,
        height: f64,
        web_thickness: f64,
        flange_thickness: f64,
        depth: f64,
    ) -> Result<Mesh> {
        // Definir pontos do perfil I em 2D
        let w2 = width / 2.0;
        let h2 = height / 2.0;
        let wt2 = web_thickness / 2.0;
        let ft = flange_thickness;

        #[rustfmt::skip]
        let profile_points = vec![
            // Flange superior (esquerda para direita)
            [-w2, h2],
            [w2, h2],
            [w2, h2 - ft],
            [wt2, h2 - ft],
            // Web (direita)
            [wt2, -h2 + ft],
            // Flange inferior (direita para esquerda)
            [w2, -h2 + ft],
            [w2, -h2],
            [-w2, -h2],
            [-w2, -h2 + ft],
            // Web (esquerda)
            [-wt2, -h2 + ft],
            [-wt2, h2 - ft],
            [-w2, h2 - ft],
        ];

        Self::extrude_2d_profile(&profile_points, depth)
    }

    /// Extrudar perfil 2D genérico
    fn extrude_2d_profile(points: &[[f64; 2]], depth: f64) -> Result<Mesh> {
        let n = points.len();
        if n < 3 {
            return Err(BimError::InvalidGeometry("Profile needs at least 3 points".into()));
        }

        let half_depth = (depth / 2.0) as f32;
        let mut vertices = Vec::new();
        let mut normals = Vec::new();
        let mut indices = Vec::new();

        // Vértices das duas faces (frente e trás)
        for point in points {
            let x = point[0] as f32;
            let y = point[1] as f32;

            // Face frontal (z = +depth/2)
            vertices.extend_from_slice(&[x, y, half_depth]);
            normals.extend_from_slice(&[0.0, 0.0, 1.0]);
        }

        for point in points {
            let x = point[0] as f32;
            let y = point[1] as f32;

            // Face traseira (z = -depth/2)
            vertices.extend_from_slice(&[x, y, -half_depth]);
            normals.extend_from_slice(&[0.0, 0.0, -1.0]);
        }

        // Triangular faces frontal e traseira
        let front_indices = crate::triangulation::Triangulator::triangulate_polygon(points);

        // Face frontal
        for &idx in &front_indices {
            indices.push(idx);
        }

        // Face traseira (ordem invertida)
        for i in (0..front_indices.len()).step_by(3).rev() {
            if i + 2 < front_indices.len() {
                indices.push(n as u32 + front_indices[i + 2]);
                indices.push(n as u32 + front_indices[i + 1]);
                indices.push(n as u32 + front_indices[i]);
            }
        }

        // Faces laterais
        let base_vertex_count = vertices.len() / 3;
        for i in 0..n {
            let next = (i + 1) % n;

            let v0_front = i;
            let v1_front = next;
            let v0_back = n + i;
            let v1_back = n + next;

            // Calcular normal da face lateral
            let p0 = points[i];
            let p1 = points[next];
            let edge = [p1[0] - p0[0], p1[1] - p0[1]];
            let normal = [-edge[1], edge[0]];
            let len = (normal[0] * normal[0] + normal[1] * normal[1]).sqrt();
            let normal = [normal[0] as f32 / len as f32, normal[1] as f32 / len as f32, 0.0];

            // Adicionar 4 vértices para face lateral (com normais corretas)
            let base_idx = (base_vertex_count + i * 4) as u32;

            vertices.extend_from_slice(&[
                points[i][0] as f32, points[i][1] as f32, half_depth,
                points[next][0] as f32, points[next][1] as f32, half_depth,
                points[next][0] as f32, points[next][1] as f32, -half_depth,
                points[i][0] as f32, points[i][1] as f32, -half_depth,
            ]);

            normals.extend_from_slice(&[
                normal[0], normal[1], normal[2],
                normal[0], normal[1], normal[2],
                normal[0], normal[1], normal[2],
                normal[0], normal[1], normal[2],
            ]);

            // Dois triângulos para face lateral
            indices.extend_from_slice(&[
                base_idx, base_idx + 1, base_idx + 2,
                base_idx, base_idx + 2, base_idx + 3,
            ]);
        }

        Ok(Mesh {
            vertices,
            normals,
            indices,
            uvs: None,
            colors: None,
        })
    }

    /// Converter IfcFacetedBrep → Mesh
    pub fn convert_faceted_brep(faces: Vec<Face>) -> Result<Mesh> {
        let mut vertices = Vec::new();
        let mut normals = Vec::new();
        let mut indices = Vec::new();

        for face in faces {
            let base_idx = (vertices.len() / 3) as u32;

            // Adicionar vértices da face
            for vertex in &face.vertices {
                vertices.extend_from_slice(&[
                    vertex[0] as f32,
                    vertex[1] as f32,
                    vertex[2] as f32,
                ]);

                // Normal da face
                normals.extend_from_slice(&[
                    face.normal[0] as f32,
                    face.normal[1] as f32,
                    face.normal[2] as f32,
                ]);
            }

            // Triangular face (assumindo polígono convexo simples)
            let n = face.vertices.len() as u32;
            for i in 1..(n - 1) {
                indices.extend_from_slice(&[base_idx, base_idx + i, base_idx + i + 1]);
            }
        }

        Ok(Mesh {
            vertices,
            normals,
            indices,
            uvs: None,
            colors: None,
        })
    }
}
