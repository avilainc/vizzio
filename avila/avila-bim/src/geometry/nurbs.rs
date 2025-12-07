//! NURBS curves and surfaces

/// NURBS curve (Non-Uniform Rational B-Spline)
#[derive(Debug, Clone)]
pub struct NurbsCurve {
    pub degree: usize,
    pub control_points: Vec<[f64; 3]>,
    pub weights: Vec<f64>,
    pub knots: Vec<f64>,
}

impl NurbsCurve {
    /// Avaliar curva em parâmetro t
    pub fn evaluate(&self, _t: f64) -> [f64; 3] {
        // TODO: Implementar algoritmo de De Boor
        [0.0, 0.0, 0.0]
    }

    /// Tesselar curva em segmentos lineares
    pub fn tessellate(&self, segments: usize) -> Vec<[f64; 3]> {
        (0..=segments)
            .map(|i| {
                let t = i as f64 / segments as f64;
                self.evaluate(t)
            })
            .collect()
    }
}

/// NURBS surface
#[derive(Debug, Clone)]
pub struct NurbsSurface {
    pub degree_u: usize,
    pub degree_v: usize,
    pub control_points: Vec<Vec<[f64; 3]>>,
    pub weights: Vec<Vec<f64>>,
    pub knots_u: Vec<f64>,
    pub knots_v: Vec<f64>,
}

impl NurbsSurface {
    /// Avaliar superfície em (u, v)
    pub fn evaluate(&self, _u: f64, _v: f64) -> [f64; 3] {
        // TODO: Implementar avaliação de superfície
        [0.0, 0.0, 0.0]
    }

    /// Tesselar superfície em mesh triangulada
    pub fn tessellate(&self, _u_segments: usize, _v_segments: usize) -> crate::bim_core::Mesh {
        // TODO: Gerar mesh triangulada
        crate::bim_core::Mesh {
            vertices: vec![],
            normals: vec![],
            indices: vec![],
            uvs: None,
            colors: None,
        }
    }
}
