//! Bounding Volume Hierarchy (BVH) para aceleração espacial

use crate::bim_core::*;

/// Nó da BVH
#[derive(Debug, Clone)]
pub struct BvhNode {
    pub bounds: BoundingBox,
    pub children: BvhChildren,
}

#[derive(Debug, Clone)]
pub enum BvhChildren {
    Leaf { element_guids: Vec<IfcGuid> },
    Internal { left: Box<BvhNode>, right: Box<BvhNode> },
}

/// Bounding Volume Hierarchy
pub struct BoundingVolumeHierarchy {
    root: Option<BvhNode>,
}

impl BoundingVolumeHierarchy {
    pub fn new() -> Self {
        Self { root: None }
    }

    /// Construir BVH a partir de elementos
    pub fn build(&mut self, elements: &[(&IfcGuid, &BoundingBox)]) {
        if elements.is_empty() {
            return;
        }

        self.root = Some(self.build_recursive(elements));
    }

    /// Construir recursivamente
    fn build_recursive(&self, elements: &[(&IfcGuid, &BoundingBox)]) -> BvhNode {
        // Calcular bounds total
        let bounds = self.compute_bounds(elements);

        // Se poucos elementos, criar folha
        if elements.len() <= 4 {
            return BvhNode {
                bounds,
                children: BvhChildren::Leaf {
                    element_guids: elements.iter().map(|(g, _)| (*g).clone()).collect(),
                },
            };
        }

        // Dividir no maior eixo
        let mut sorted = elements.to_vec();
        sorted.sort_by(|a, b| {
            let ca = a.1.center();
            let cb = b.1.center();
            ca[0].partial_cmp(&cb[0]).unwrap()
        });

        let mid = sorted.len() / 2;
        let left = self.build_recursive(&sorted[..mid]);
        let right = self.build_recursive(&sorted[mid..]);

        BvhNode {
            bounds,
            children: BvhChildren::Internal {
                left: Box::new(left),
                right: Box::new(right),
            },
        }
    }

    /// Computar bounds de múltiplos elementos
    fn compute_bounds(&self, elements: &[(&IfcGuid, &BoundingBox)]) -> BoundingBox {
        let mut min = [f64::INFINITY; 3];
        let mut max = [f64::NEG_INFINITY; 3];

        for (_, bbox) in elements {
            for i in 0..3 {
                min[i] = min[i].min(bbox.min[i]);
                max[i] = max[i].max(bbox.max[i]);
            }
        }

        BoundingBox { min, max }
    }

    /// Query por intersecção com bounds
    pub fn query(&self, query_bounds: &BoundingBox) -> Vec<IfcGuid> {
        let mut results = Vec::new();

        if let Some(ref root) = self.root {
            self.query_recursive(root, query_bounds, &mut results);
        }

        results
    }

    fn query_recursive(&self, node: &BvhNode, query: &BoundingBox, results: &mut Vec<IfcGuid>) {
        // Testar intersecção
        if !self.bounds_intersect(&node.bounds, query) {
            return;
        }

        match &node.children {
            BvhChildren::Leaf { element_guids } => {
                results.extend(element_guids.iter().cloned());
            }
            BvhChildren::Internal { left, right } => {
                self.query_recursive(left, query, results);
                self.query_recursive(right, query, results);
            }
        }
    }

    fn bounds_intersect(&self, a: &BoundingBox, b: &BoundingBox) -> bool {
        for i in 0..3 {
            if a.max[i] < b.min[i] || a.min[i] > b.max[i] {
                return false;
            }
        }
        true
    }
}

impl Default for BoundingVolumeHierarchy {
    fn default() -> Self {
        Self::new()
    }
}
