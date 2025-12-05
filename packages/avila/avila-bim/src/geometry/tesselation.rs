//! Tesselation (conversão de superfícies → meshes trianguladas)

use crate::bim_core::*;
use crate::geometry::brep::*;

/// Tesselator
pub struct Tesselator {
    tolerance: f64,
}

impl Tesselator {
    pub fn new(tolerance: f64) -> Self {
        Self { tolerance }
    }

    /// Tesselar BRep → Mesh
    pub fn tessellate_brep(&self, brep: &BRepTopology) -> Result<Vec<Mesh>> {
        let mut meshes = Vec::new();

        for solid in &brep.solids {
            for shell in &solid.shells {
                for face in &shell.faces {
                    if let Ok(mesh) = self.tessellate_face(face) {
                        meshes.push(mesh);
                    }
                }
            }
        }

        Ok(meshes)
    }

    /// Tesselar face
    fn tessellate_face(&self, face: &BRepFace) -> Result<Mesh> {
        match &face.surface {
            BRepSurface::Plane { origin, normal } => {
                self.tessellate_planar_face(face, *origin, *normal)
            }
            BRepSurface::Cylinder { .. } => {
                self.tessellate_cylindrical_face(face)
            }
            BRepSurface::Sphere { .. } => {
                self.tessellate_spherical_face(face)
            }
            BRepSurface::Nurbs(nurbs) => {
                Ok(nurbs.tessellate(16, 16))
            }
        }
    }

    fn tessellate_planar_face(&self, _face: &BRepFace, _origin: [f64; 3], _normal: [f64; 3]) -> Result<Mesh> {
        // TODO: Triangular face planar
        Ok(Mesh {
            vertices: vec![],
            normals: vec![],
            indices: vec![],
            uvs: None,
            colors: None,
        })
    }

    fn tessellate_cylindrical_face(&self, _face: &BRepFace) -> Result<Mesh> {
        // TODO: Tesselar cilindro
        Ok(Mesh {
            vertices: vec![],
            normals: vec![],
            indices: vec![],
            uvs: None,
            colors: None,
        })
    }

    fn tessellate_spherical_face(&self, _face: &BRepFace) -> Result<Mesh> {
        // TODO: Tesselar esfera
        Ok(Mesh {
            vertices: vec![],
            normals: vec![],
            indices: vec![],
            uvs: None,
            colors: None,
        })
    }
}

impl Default for Tesselator {
    fn default() -> Self {
        Self::new(0.001)
    }
}
