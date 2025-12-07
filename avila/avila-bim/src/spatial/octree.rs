//! Octree para indexação espacial 3D

use crate::bim_core::*;

const MAX_DEPTH: usize = 8;
const MAX_ELEMENTS_PER_NODE: usize = 8;

/// Nó da Octree
#[derive(Debug, Clone)]
pub struct OctreeNode {
    pub bounds: BoundingBox,
    pub elements: Vec<IfcGuid>,
    pub children: Option<[Box<OctreeNode>; 8]>,
}

/// Octree 3D
pub struct Octree {
    root: Option<OctreeNode>,
}

impl Octree {
    pub fn new(bounds: BoundingBox) -> Self {
        Self {
            root: Some(OctreeNode {
                bounds,
                elements: Vec::new(),
                children: None,
            }),
        }
    }

    /// Inserir elemento
    pub fn insert(&mut self, guid: IfcGuid, bounds: &BoundingBox) {
        if let Some(ref mut root) = self.root {
            Self::insert_recursive(root, guid, bounds, 0);
        }
    }

    fn insert_recursive(node: &mut OctreeNode, guid: IfcGuid, bounds: &BoundingBox, depth: usize) {
        // Se alcançou profundidade máxima ou poucos elementos, adicionar aqui
        if depth >= MAX_DEPTH || node.elements.len() < MAX_ELEMENTS_PER_NODE {
            node.elements.push(guid);
            return;
        }

        // Se não tem filhos, subdividir
        if node.children.is_none() {
            node.children = Some(Self::subdivide(&node.bounds));
        }

        // Inserir no filho apropriado
        if let Some(ref mut children) = node.children {
            let octant = Self::find_octant(&node.bounds, bounds);
            Self::insert_recursive(&mut children[octant], guid, bounds, depth + 1);
        }
    }

    /// Subdividir nó em 8 octantes
    fn subdivide(bounds: &BoundingBox) -> [Box<OctreeNode>; 8] {
        let center = bounds.center();
        let size = bounds.size();
        let half_size = [size[0] / 2.0, size[1] / 2.0, size[2] / 2.0];

        [
            // 8 octantes (combinações de +/- em x, y, z)
            Box::new(OctreeNode {
                bounds: BoundingBox {
                    min: bounds.min,
                    max: center,
                },
                elements: Vec::new(),
                children: None,
            }),
            // TODO: Implementar outros 7 octantes
            Box::new(OctreeNode {
                bounds: bounds.clone(),
                elements: Vec::new(),
                children: None,
            }),
            Box::new(OctreeNode {
                bounds: bounds.clone(),
                elements: Vec::new(),
                children: None,
            }),
            Box::new(OctreeNode {
                bounds: bounds.clone(),
                elements: Vec::new(),
                children: None,
            }),
            Box::new(OctreeNode {
                bounds: bounds.clone(),
                elements: Vec::new(),
                children: None,
            }),
            Box::new(OctreeNode {
                bounds: bounds.clone(),
                elements: Vec::new(),
                children: None,
            }),
            Box::new(OctreeNode {
                bounds: bounds.clone(),
                elements: Vec::new(),
                children: None,
            }),
            Box::new(OctreeNode {
                bounds: bounds.clone(),
                elements: Vec::new(),
                children: None,
            }),
        ]
    }

    fn find_octant(_parent_bounds: &BoundingBox, _element_bounds: &BoundingBox) -> usize {
        // TODO: Determinar octante baseado em centro
        0
    }

    /// Query elementos em região
    pub fn query(&self, query_bounds: &BoundingBox) -> Vec<IfcGuid> {
        let mut results = Vec::new();

        if let Some(ref root) = self.root {
            Self::query_recursive(root, query_bounds, &mut results);
        }

        results
    }

    fn query_recursive(node: &OctreeNode, query: &BoundingBox, results: &mut Vec<IfcGuid>) {
        // Adicionar elementos deste nó
        results.extend(node.elements.iter().cloned());

        // Recursão nos filhos
        if let Some(ref children) = node.children {
            for child in children.iter() {
                if Self::bounds_intersect(&child.bounds, query) {
                    Self::query_recursive(child, query, results);
                }
            }
        }
    }

    fn bounds_intersect(a: &BoundingBox, b: &BoundingBox) -> bool {
        for i in 0..3 {
            if a.max[i] < b.min[i] || a.min[i] > b.max[i] {
                return false;
            }
        }
        true
    }
}
