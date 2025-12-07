//! Octree para particionamento espacial (Rust puro)

use crate::bim_core::BoundingBox;

/// Nó da Octree
pub struct OctreeNode {
    pub bounds: BoundingBox,
    pub children: Option<Box<[OctreeNode; 8]>>,
    pub data_indices: Vec<usize>,
}

/// Octree para organização espacial hierárquica
pub struct Octree {
    root: Option<OctreeNode>,
    max_depth: usize,
    max_items_per_node: usize,
}

impl Octree {
    /// Criar Octree vazia
    pub fn new(bounds: BoundingBox, max_depth: usize, max_items: usize) -> Self {
        Self {
            root: Some(OctreeNode {
                bounds,
                children: None,
                data_indices: Vec::new(),
            }),
            max_depth,
            max_items_per_node: max_items,
        }
    }

    /// Inserir ponto na Octree
    pub fn insert(&mut self, point: [f64; 3], data_index: usize) {
        if let Some(ref mut root) = self.root {
            Self::insert_recursive(root, point, data_index, 0, self.max_depth, self.max_items_per_node);
        }
    }

    fn insert_recursive(
        node: &mut OctreeNode,
        point: [f64; 3],
        data_index: usize,
        depth: usize,
        max_depth: usize,
        max_items: usize,
    ) {
        // Se tem filhos, inserir no filho apropriado
        if let Some(ref mut children) = node.children {
            let octant = Self::find_octant(&node.bounds, point);
            Self::insert_recursive(&mut children[octant], point, data_index, depth + 1, max_depth, max_items);
            return;
        }

        // Adicionar ao nó atual
        node.data_indices.push(data_index);

        // Se excedeu limite e não está na profundidade máxima, subdividir
        if node.data_indices.len() > max_items && depth < max_depth {
            Self::subdivide(node);

            // Redistribuir dados existentes
            let indices = node.data_indices.clone();
            node.data_indices.clear();

            // TODO: Precisaríamos dos pontos originais para redistribuir
            // Por ora, manter no nó pai
            node.data_indices = indices;
        }
    }

    fn subdivide(node: &mut OctreeNode) {
        let bounds = &node.bounds;
        let center = [
            (bounds.min[0] + bounds.max[0]) * 0.5,
            (bounds.min[1] + bounds.max[1]) * 0.5,
            (bounds.min[2] + bounds.max[2]) * 0.5,
        ];

        let mut children = Vec::with_capacity(8);

        for i in 0..8 {
            let min = [
                if i & 1 == 0 { bounds.min[0] } else { center[0] },
                if i & 2 == 0 { bounds.min[1] } else { center[1] },
                if i & 4 == 0 { bounds.min[2] } else { center[2] },
            ];

            let max = [
                if i & 1 == 0 { center[0] } else { bounds.max[0] },
                if i & 2 == 0 { center[1] } else { bounds.max[1] },
                if i & 4 == 0 { center[2] } else { bounds.max[2] },
            ];

            children.push(OctreeNode {
                bounds: BoundingBox { min, max },
                children: None,
                data_indices: Vec::new(),
            });
        }

        node.children = Some(Box::new([
            children[0].clone(),
            children[1].clone(),
            children[2].clone(),
            children[3].clone(),
            children[4].clone(),
            children[5].clone(),
            children[6].clone(),
            children[7].clone(),
        ]));
    }

    fn find_octant(bounds: &BoundingBox, point: [f64; 3]) -> usize {
        let center = [
            (bounds.min[0] + bounds.max[0]) * 0.5,
            (bounds.min[1] + bounds.max[1]) * 0.5,
            (bounds.min[2] + bounds.max[2]) * 0.5,
        ];

        let mut octant = 0;

        if point[0] >= center[0] {
            octant |= 1;
        }
        if point[1] >= center[1] {
            octant |= 2;
        }
        if point[2] >= center[2] {
            octant |= 4;
        }

        octant
    }

    /// Query de pontos dentro de bounding box
    pub fn query_box(&self, query_bounds: &BoundingBox) -> Vec<usize> {
        let mut results = Vec::new();

        if let Some(ref root) = self.root {
            Self::query_box_recursive(root, query_bounds, &mut results);
        }

        results
    }

    fn query_box_recursive(
        node: &OctreeNode,
        query_bounds: &BoundingBox,
        results: &mut Vec<usize>,
    ) {
        // Testar overlap
        if !Self::boxes_overlap(&node.bounds, query_bounds) {
            return;
        }

        // Adicionar dados deste nó
        results.extend(&node.data_indices);

        // Recursão nos filhos
        if let Some(ref children) = node.children {
            for child in children.iter() {
                Self::query_box_recursive(child, query_bounds, results);
            }
        }
    }

    fn boxes_overlap(a: &BoundingBox, b: &BoundingBox) -> bool {
        for i in 0..3 {
            if a.max[i] < b.min[i] || a.min[i] > b.max[i] {
                return false;
            }
        }
        true
    }

    /// Query de pontos dentro de esfera
    pub fn query_sphere(&self, center: [f64; 3], radius: f64) -> Vec<usize> {
        let mut results = Vec::new();

        if let Some(ref root) = self.root {
            Self::query_sphere_recursive(root, center, radius, &mut results);
        }

        results
    }

    fn query_sphere_recursive(
        node: &OctreeNode,
        center: [f64; 3],
        radius: f64,
        results: &mut Vec<usize>,
    ) {
        // Testar se esfera intersecta bounding box
        if !Self::sphere_box_intersect(center, radius, &node.bounds) {
            return;
        }

        // Adicionar dados deste nó
        results.extend(&node.data_indices);

        // Recursão nos filhos
        if let Some(ref children) = node.children {
            for child in children.iter() {
                Self::query_sphere_recursive(child, center, radius, results);
            }
        }
    }

    fn sphere_box_intersect(center: [f64; 3], radius: f64, bounds: &BoundingBox) -> bool {
        let mut dist_squared = 0.0;

        for i in 0..3 {
            if center[i] < bounds.min[i] {
                let d = center[i] - bounds.min[i];
                dist_squared += d * d;
            } else if center[i] > bounds.max[i] {
                let d = center[i] - bounds.max[i];
                dist_squared += d * d;
            }
        }

        dist_squared <= radius * radius
    }

    /// Contar total de nós
    pub fn node_count(&self) -> usize {
        match &self.root {
            Some(root) => Self::count_nodes_recursive(root),
            None => 0,
        }
    }

    fn count_nodes_recursive(node: &OctreeNode) -> usize {
        let mut count = 1;

        if let Some(ref children) = node.children {
            for child in children.iter() {
                count += Self::count_nodes_recursive(child);
            }
        }

        count
    }

    /// Calcular profundidade máxima atingida
    pub fn actual_depth(&self) -> usize {
        match &self.root {
            Some(root) => Self::compute_depth_recursive(root),
            None => 0,
        }
    }

    fn compute_depth_recursive(node: &OctreeNode) -> usize {
        if let Some(ref children) = node.children {
            let max_child_depth = children
                .iter()
                .map(|c| Self::compute_depth_recursive(c))
                .max()
                .unwrap_or(0);
            1 + max_child_depth
        } else {
            1
        }
    }
}

impl Clone for OctreeNode {
    fn clone(&self) -> Self {
        Self {
            bounds: self.bounds.clone(),
            children: self.children.as_ref().map(|c| c.clone()),
            data_indices: self.data_indices.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_octree_insert() {
        let bounds = BoundingBox {
            min: [0.0, 0.0, 0.0],
            max: [10.0, 10.0, 10.0],
        };

        let mut octree = Octree::new(bounds, 3, 4);

        octree.insert([1.0, 1.0, 1.0], 0);
        octree.insert([2.0, 2.0, 2.0], 1);
        octree.insert([8.0, 8.0, 8.0], 2);

        assert!(octree.node_count() >= 1);
    }

    #[test]
    fn test_octree_query_box() {
        let bounds = BoundingBox {
            min: [0.0, 0.0, 0.0],
            max: [10.0, 10.0, 10.0],
        };

        let mut octree = Octree::new(bounds, 3, 4);

        octree.insert([1.0, 1.0, 1.0], 0);
        octree.insert([8.0, 8.0, 8.0], 1);

        let query = BoundingBox {
            min: [0.0, 0.0, 0.0],
            max: [2.0, 2.0, 2.0],
        };

        let results = octree.query_box(&query);
        assert!(results.contains(&0));
    }

    #[test]
    fn test_octree_query_sphere() {
        let bounds = BoundingBox {
            min: [0.0, 0.0, 0.0],
            max: [10.0, 10.0, 10.0],
        };

        let mut octree = Octree::new(bounds, 3, 4);

        octree.insert([5.0, 5.0, 5.0], 0);
        octree.insert([9.0, 9.0, 9.0], 1);

        let results = octree.query_sphere([5.0, 5.0, 5.0], 1.0);
        assert!(results.contains(&0));
    }
}
