//! Clash detection rules

use crate::bim_core::*;
use crate::spatial::CollisionDetector;

/// Regras de detecção de colisões
pub struct ClashDetectionRules;

impl ClashDetectionRules {
    /// Detectar colisões hard (geometrias se intersectam)
    pub fn detect_hard_clashes(model: &BimModel) -> Vec<String> {
        let detector = CollisionDetector::new(0.0);
        let clashes = detector.detect_clashes(model);

        clashes
            .iter()
            .filter(|c| matches!(c.clash_type, crate::spatial::collision::ClashType::Intersection))
            .map(|c| {
                format!(
                    "Hard clash between {} and {}",
                    c.element_a.as_str(),
                    c.element_b.as_str()
                )
            })
            .collect()
    }

    /// Detectar colisões soft (clearance insuficiente)
    pub fn detect_soft_clashes(model: &BimModel, tolerance: f64) -> Vec<String> {
        let detector = CollisionDetector::new(tolerance);
        let clashes = detector.detect_clashes(model);

        clashes
            .iter()
            .filter(|c| matches!(c.clash_type, crate::spatial::collision::ClashType::Clearance))
            .map(|c| {
                format!(
                    "Soft clash between {} and {} (distance: {:.3}m)",
                    c.element_a.as_str(),
                    c.element_b.as_str(),
                    c.distance
                )
            })
            .collect()
    }

    /// Detectar colisões específicas (ex: MEP × estrutura)
    pub fn detect_mep_structural_clashes(model: &BimModel) -> Vec<String> {
        let mut clashes = Vec::new();

        // Coletar elementos MEP
        let mep_types = ["IfcPipeSegment", "IfcDuctSegment", "IfcCableSegment"];
        let structural_types = ["IfcWall", "IfcSlab", "IfcColumn", "IfcBeam"];

        // TODO: Implementar detecção específica MEP × estrutura

        clashes
    }
}
