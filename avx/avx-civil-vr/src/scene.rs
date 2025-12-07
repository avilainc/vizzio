use avila_bim_core::{BimElement, Geometry, BimGeometry, Mesh as BimMesh, BoundingBox};
use avila_mesh::Mesh;
use avila_tesselation::Tesselator;
use std::collections::HashMap;

pub struct Scene {
    pub meshes: Vec<RenderMesh>,
    pub bounds: Option<BoundingBox>,
}

pub struct RenderMesh {
    pub vertices: Vec<f32>,
    pub indices: Vec<u32>,
    pub transform: [f32; 16],
    pub color: [f32; 4],
}

impl Scene {
    pub fn new() -> Self {
        Self {
            meshes: Vec::new(),
            bounds: None,
        }
    }

    pub fn clear(&mut self) {
        self.meshes.clear();
        self.bounds = None;
    }

    pub fn add_element(&mut self, element: &BimElement, geometry: &Geometry) -> Result<(), String> {
        if let Some(mesh) = &geometry.mesh {
            // Handle mesh
            self.add_mesh(mesh, &element.placement.matrix);
        } else if let Some(bim_geom) = &geometry.brep {
            // Handle BRep
            self.add_brep(bim_geom, &element.placement.matrix);
        } else {
            // Fallback
            self.add_fallback_box(&element.placement.matrix);
        }
        Ok(())
    }

    fn add_bim_geometry(&mut self, geom: &BimGeometry, transform: &[f64; 16]) -> Result<(), String> {
        match geom {
            BimGeometry::ExtrudedSolid { profile, depth, transform: geom_transform } => {
                let mesh = self.extrude_profile_to_mesh(profile, *depth, geom_transform)?;
                self.add_render_mesh(mesh, transform);
            }
            BimGeometry::TriangulatedMesh { vertices, normals: _, indices } => {
                let mesh = RenderMesh {
                    vertices: vertices.iter().flat_map(|&v| v.map(|x| x as f32)).collect(),
                    indices: indices.clone(),
                    transform: transform.map(|x| x as f32),
                    color: [0.8, 0.8, 0.8, 1.0], // Default gray
                };
                self.meshes.push(mesh);
            }
            _ => {
                // Unsupported geometry type, add fallback
                self.add_fallback_box(transform);
            }
        }
        Ok(())
    }

    fn extrude_profile_to_mesh(&self, profile: &avila_bim_core::ExtrusionProfile, depth: f64, transform: &[f64; 16]) -> Result<RenderMesh, String> {
        // Simple extrusion to mesh
        // For now, create a basic box approximation
        let (width, height) = match profile {
            avila_bim_core::ExtrusionProfile::Rectangle { width, height } => (*width as f32, *height as f32),
            avila_bim_core::ExtrusionProfile::Circle { radius } => (*radius as f32 * 2.0, *radius as f32 * 2.0),
            _ => (1.0, 1.0), // Default
        };

        let depth = depth as f32;

        // Create box vertices
        let vertices = vec![
            // Front face
            -width/2.0, -height/2.0, depth/2.0,
             width/2.0, -height/2.0, depth/2.0,
             width/2.0,  height/2.0, depth/2.0,
            -width/2.0,  height/2.0, depth/2.0,
            // Back face
            -width/2.0, -height/2.0, -depth/2.0,
             width/2.0, -height/2.0, -depth/2.0,
             width/2.0,  height/2.0, -depth/2.0,
            -width/2.0,  height/2.0, -depth/2.0,
        ];

        let indices = vec![
            0, 1, 2, 2, 3, 0, // Front
            4, 5, 6, 6, 7, 4, // Back
            0, 1, 5, 5, 4, 0, // Bottom
            2, 3, 7, 7, 6, 2, // Top
            0, 3, 7, 7, 4, 0, // Left
            1, 2, 6, 6, 5, 1, // Right
        ];

        Ok(RenderMesh {
            vertices,
            indices,
            transform: transform.map(|x| x as f32),
            color: [0.7, 0.7, 0.9, 1.0], // Light blue for structural elements
        })
    }

    fn add_fallback_box(&mut self, transform: &[f64; 16]) {
        let mesh = RenderMesh {
            vertices: vec![
                -0.5, -0.5, -0.5,
                 0.5, -0.5, -0.5,
                 0.5,  0.5, -0.5,
                -0.5,  0.5, -0.5,
                -0.5, -0.5,  0.5,
                 0.5, -0.5,  0.5,
                 0.5,  0.5,  0.5,
                -0.5,  0.5,  0.5,
            ],
            indices: vec![
                0,1,2, 2,3,0,
                4,5,6, 6,7,4,
                0,1,5, 5,4,0,
                2,3,7, 7,6,2,
                0,3,7, 7,4,0,
                1,2,6, 6,5,1,
            ],
            transform: transform.map(|x| x as f32),
            color: [1.0, 0.5, 0.5, 1.0], // Red for fallback
        };
        self.meshes.push(mesh);
    }

    fn add_render_mesh(&mut self, mesh: RenderMesh, global_transform: &[f64; 16]) {
        // Combine transforms if needed
        let combined_transform = global_transform.map(|x| x as f32); // For now, just use global
        let mut mesh = mesh;
        mesh.transform = combined_transform;
        self.meshes.push(mesh);
    }

    fn add_mesh(&mut self, mesh: &avila_bim_core::Mesh, transform: &[f64; 16]) {
        let render_mesh = RenderMesh {
            vertices: mesh.vertices.clone(),
            indices: mesh.indices.clone(),
            transform: transform.map(|x| x as f32),
            color: [0.8, 0.8, 0.8, 1.0],
        };
        self.meshes.push(render_mesh);
    }

    fn add_brep(&mut self, brep: &avila_bim_core::BRep, transform: &[f64; 16]) {
        // For now, add fallback
        self.add_fallback_box(transform);
    }

    pub fn get_bounds(&self) -> Option<BoundingBox> {
        if self.meshes.is_empty() {
            return None;
        }

        let mut min = [f64::INFINITY; 3];
        let mut max = [f64::NEG_INFINITY; 3];

        for mesh in &self.meshes {
            for chunk in mesh.vertices.chunks(3) {
                for i in 0..3 {
                    min[i] = min[i].min(chunk[i] as f64);
                    max[i] = max[i].max(chunk[i] as f64);
                }
            }
        }

        Some(BoundingBox { min, max })
    }
}