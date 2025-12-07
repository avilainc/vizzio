//! Collision detection (clash detection)

use crate::bim_core::*;
use std::collections::HashMap;

/// Resultado de detecção de colisão
#[derive(Debug, Clone)]
pub struct Clash {
    pub element_a: IfcGuid,
    pub element_b: IfcGuid,
    pub clash_type: ClashType,
    pub distance: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ClashType {
    Intersection,  // Geometrias se intersectam
    Clearance,     // Distância menor que tolerância
}

/// Detector de colisões
pub struct CollisionDetector {
    tolerance: f64,
}

impl CollisionDetector {
    pub fn new(tolerance: f64) -> Self {
        Self { tolerance }
    }

    /// Detectar colisões em modelo
    pub fn detect_clashes(&self, model: &BimModel) -> Vec<Clash> {
        let mut clashes = Vec::new();

        // Coletar elementos com bounds
        let elements: Vec<_> = model
            .elements
            .values()
            .filter_map(|e| {
                e.geometry.as_ref().map(|g| (e.guid.clone(), g.bounds.clone()))
            })
            .collect();

        // Teste bruto O(n²) - TODO: usar BVH
        for i in 0..elements.len() {
            for j in (i + 1)..elements.len() {
                if let Some(clash) = self.check_collision(&elements[i], &elements[j]) {
                    clashes.push(clash);
                }
            }
        }

        clashes
    }

    /// Verificar colisão entre dois elementos
    fn check_collision(
        &self,
        a: &(IfcGuid, BoundingBox),
        b: &(IfcGuid, BoundingBox),
    ) -> Option<Clash> {
        let (guid_a, bounds_a) = a;
        let (guid_b, bounds_b) = b;

        // Testar intersecção de bounds
        if self.bounds_intersect(bounds_a, bounds_b) {
            return Some(Clash {
                element_a: guid_a.clone(),
                element_b: guid_b.clone(),
                clash_type: ClashType::Intersection,
                distance: 0.0,
            });
        }

        // Testar clearance (distância mínima)
        let distance = self.bounds_distance(bounds_a, bounds_b);
        if distance < self.tolerance {
            return Some(Clash {
                element_a: guid_a.clone(),
                element_b: guid_b.clone(),
                clash_type: ClashType::Clearance,
                distance,
            });
        }

        None
    }

    fn bounds_intersect(&self, a: &BoundingBox, b: &BoundingBox) -> bool {
        for i in 0..3 {
            if a.max[i] < b.min[i] || a.min[i] > b.max[i] {
                return false;
            }
        }
        true
    }

    fn bounds_distance(&self, a: &BoundingBox, b: &BoundingBox) -> f64 {
        let mut dist_sq = 0.0;

        for i in 0..3 {
            let d = (a.min[i] - b.max[i]).max(b.min[i] - a.max[i]).max(0.0);
            dist_sq += d * d;
        }

        dist_sq.sqrt()
    }
}

impl Default for CollisionDetector {
    fn default() -> Self {
        Self::new(0.01) // 1cm tolerance
    }
}
