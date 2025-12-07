//! BRep topology structures

use uuid::Uuid;

/// Solid BRep
#[derive(Debug, Clone)]
pub struct BRepSolid {
    pub id: Uuid,
    pub shells: Vec<BRepShell>,
}

/// Shell (conjunto fechado de faces)
#[derive(Debug, Clone)]
pub struct BRepShell {
    pub id: Uuid,
    pub faces: Vec<BRepFace>,
}

/// Face (superfície delimitada por loops)
#[derive(Debug, Clone)]
pub struct BRepFace {
    pub id: Uuid,
    pub surface: BRepSurface,
    pub outer_loop: BRepLoop,
    pub inner_loops: Vec<BRepLoop>,
}

/// Loop (sequência fechada de edges)
#[derive(Debug, Clone)]
pub struct BRepLoop {
    pub edges: Vec<BRepEdge>,
}

/// Edge (curva entre dois vértices)
#[derive(Debug, Clone)]
pub struct BRepEdge {
    pub id: Uuid,
    pub curve: BRepCurve,
    pub start_vertex: BRepVertex,
    pub end_vertex: BRepVertex,
}

/// Vertex (ponto 3D)
#[derive(Debug, Clone)]
pub struct BRepVertex {
    pub id: Uuid,
    pub point: [f64; 3],
}

/// Surface type
#[derive(Debug, Clone)]
pub enum BRepSurface {
    Plane { origin: [f64; 3], normal: [f64; 3] },
    Cylinder { axis: [f64; 3], radius: f64 },
    Sphere { center: [f64; 3], radius: f64 },
    Nurbs(crate::geometry::NurbsSurface),
}

/// Curve type
#[derive(Debug, Clone)]
pub enum BRepCurve {
    Line { start: [f64; 3], end: [f64; 3] },
    Circle { center: [f64; 3], radius: f64, normal: [f64; 3] },
    Nurbs(crate::geometry::NurbsCurve),
}

/// Topology container
pub struct BRepTopology {
    pub solids: Vec<BRepSolid>,
}

impl BRepTopology {
    pub fn new() -> Self {
        Self {
            solids: Vec::new(),
        }
    }

    pub fn add_solid(&mut self, solid: BRepSolid) {
        self.solids.push(solid);
    }
}

impl Default for BRepTopology {
    fn default() -> Self {
        Self::new()
    }
}
