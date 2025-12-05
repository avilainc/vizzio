//! BRep builder utilities

use super::topology::*;
use uuid::Uuid;

/// BRep builder
pub struct BRepBuilder {
    topology: BRepTopology,
}

impl BRepBuilder {
    pub fn new() -> Self {
        Self {
            topology: BRepTopology::new(),
        }
    }

    /// Criar box (paralelepípedo)
    pub fn create_box(&mut self, width: f64, height: f64, depth: f64) -> Uuid {
        // TODO: Criar topologia de box com 6 faces planares
        let solid_id = Uuid::new_v4();

        let solid = BRepSolid {
            id: solid_id,
            shells: vec![],
        };

        self.topology.add_solid(solid);
        solid_id
    }

    /// Criar cilindro
    pub fn create_cylinder(&mut self, radius: f64, height: f64) -> Uuid {
        // TODO: Criar topologia de cilindro
        let solid_id = Uuid::new_v4();

        let solid = BRepSolid {
            id: solid_id,
            shells: vec![],
        };

        self.topology.add_solid(solid);
        solid_id
    }

    /// Criar esfera
    pub fn create_sphere(&mut self, radius: f64) -> Uuid {
        // TODO: Criar topologia de esfera
        let solid_id = Uuid::new_v4();

        let solid = BRepSolid {
            id: solid_id,
            shells: vec![],
        };

        self.topology.add_solid(solid);
        solid_id
    }

    pub fn build(self) -> BRepTopology {
        self.topology
    }
}

impl Default for BRepBuilder {
    fn default() -> Self {
        Self::new()
    }
}
