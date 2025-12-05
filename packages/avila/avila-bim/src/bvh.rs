//! Bounding Volume Hierarchy (BVH) para aceleração de raycasting (Rust puro)

use crate::bim_core::BoundingBox;

/// Nó da BVH
pub struct BvhNode {
    pub bounds: BoundingBox,
    pub left: Option<Box<BvhNode>>,
    pub right: Option<Box<BvhNode>>,
    pub primitive_indices: Vec<usize>,
}

/// BVH para aceleração de queries espaciais
pub struct Bvh {
    root: Option<Box<BvhNode>>,
    max_primitives_per_leaf: usize,
}

impl Bvh {
    /// Criar BVH vazia
    pub fn new() -> Self {
        Self {
            root: None,
            max_primitives_per_leaf: 4,
        }
    }

    /// Construir BVH a partir de bounding boxes
    pub fn build(bounds: &[BoundingBox], max_leaf_size: usize) -> Self {
        let indices: Vec<usize> = (0..bounds.len()).collect();

        let root = if !bounds.is_empty() {
            Some(Box::new(Self::build_recursive(bounds, &indices, max_leaf_size)))
        } else {
            None
        };

        Self {
            root,
            max_primitives_per_leaf: max_leaf_size,
        }
    }

    fn build_recursive(
        bounds: &[BoundingBox],
        indices: &[usize],
        max_leaf_size: usize,
    ) -> BvhNode {
        // Calcular bounding box que contém todos os primitivos
        let node_bounds = Self::compute_bounds(bounds, indices);

        // Se poucos primitivos, criar folha
        if indices.len() <= max_leaf_size {
            return BvhNode {
                bounds: node_bounds,
                left: None,
                right: None,
                primitive_indices: indices.to_vec(),
            };
        }

        // Encontrar eixo de maior extensão
        let extent = [
            node_bounds.max[0] - node_bounds.min[0],
            node_bounds.max[1] - node_bounds.min[1],
            node_bounds.max[2] - node_bounds.min[2],
        ];

        let split_axis = if extent[0] > extent[1] && extent[0] > extent[2] {
            0
        } else if extent[1] > extent[2] {
            1
        } else {
            2
        };

        // Ordenar primitivos pelo centro no eixo de split
        let mut sorted_indices = indices.to_vec();
        sorted_indices.sort_by(|&a, &b| {
            let center_a = Self::compute_center(&bounds[a]);
            let center_b = Self::compute_center(&bounds[b]);
            center_a[split_axis].partial_cmp(&center_b[split_axis]).unwrap()
        });

        // Dividir no meio
        let mid = sorted_indices.len() / 2;
        let left_indices = &sorted_indices[..mid];
        let right_indices = &sorted_indices[mid..];

        // Recursão
        let left = if !left_indices.is_empty() {
            Some(Box::new(Self::build_recursive(bounds, left_indices, max_leaf_size)))
        } else {
            None
        };

        let right = if !right_indices.is_empty() {
            Some(Box::new(Self::build_recursive(bounds, right_indices, max_leaf_size)))
        } else {
            None
        };

        BvhNode {
            bounds: node_bounds,
            left,
            right,
            primitive_indices: Vec::new(),
        }
    }

    fn compute_bounds(bounds: &[BoundingBox], indices: &[usize]) -> BoundingBox {
        let mut min = [f64::INFINITY, f64::INFINITY, f64::INFINITY];
        let mut max = [f64::NEG_INFINITY, f64::NEG_INFINITY, f64::NEG_INFINITY];

        for &idx in indices {
            let b = &bounds[idx];
            for i in 0..3 {
                min[i] = min[i].min(b.min[i]);
                max[i] = max[i].max(b.max[i]);
            }
        }

        BoundingBox { min, max }
    }

    fn compute_center(bounds: &BoundingBox) -> [f64; 3] {
        [
            (bounds.min[0] + bounds.max[0]) * 0.5,
            (bounds.min[1] + bounds.max[1]) * 0.5,
            (bounds.min[2] + bounds.max[2]) * 0.5,
        ]
    }

    /// Testar intersecção com raio
    pub fn intersect_ray(&self, origin: [f64; 3], direction: [f64; 3]) -> Vec<usize> {
        let mut results = Vec::new();

        if let Some(ref root) = self.root {
            Self::intersect_ray_recursive(root, origin, direction, &mut results);
        }

        results
    }

    fn intersect_ray_recursive(
        node: &BvhNode,
        origin: [f64; 3],
        direction: [f64; 3],
        results: &mut Vec<usize>,
    ) {
        // Testar intersecção com bounding box do nó
        if !Self::ray_box_intersect(origin, direction, &node.bounds) {
            return;
        }

        // Se é folha, adicionar primitivos
        if node.left.is_none() && node.right.is_none() {
            results.extend(&node.primitive_indices);
            return;
        }

        // Recursão nos filhos
        if let Some(ref left) = node.left {
            Self::intersect_ray_recursive(left, origin, direction, results);
        }

        if let Some(ref right) = node.right {
            Self::intersect_ray_recursive(right, origin, direction, results);
        }
    }

    fn ray_box_intersect(origin: [f64; 3], direction: [f64; 3], bounds: &BoundingBox) -> bool {
        let mut tmin = f64::NEG_INFINITY;
        let mut tmax = f64::INFINITY;

        for i in 0..3 {
            if direction[i].abs() > 1e-10 {
                let inv_d = 1.0 / direction[i];
                let mut t0 = (bounds.min[i] - origin[i]) * inv_d;
                let mut t1 = (bounds.max[i] - origin[i]) * inv_d;

                if t0 > t1 {
                    std::mem::swap(&mut t0, &mut t1);
                }

                tmin = tmin.max(t0);
                tmax = tmax.min(t1);

                if tmax < tmin {
                    return false;
                }
            } else {
                // Raio paralelo ao plano
                if origin[i] < bounds.min[i] || origin[i] > bounds.max[i] {
                    return false;
                }
            }
        }

        tmax >= 0.0
    }

    /// Query de objetos dentro de bounding box
    pub fn query_box(&self, query_bounds: &BoundingBox) -> Vec<usize> {
        let mut results = Vec::new();

        if let Some(ref root) = self.root {
            Self::query_box_recursive(root, query_bounds, &mut results);
        }

        results
    }

    fn query_box_recursive(
        node: &BvhNode,
        query_bounds: &BoundingBox,
        results: &mut Vec<usize>,
    ) {
        // Testar overlap
        if !Self::boxes_overlap(&node.bounds, query_bounds) {
            return;
        }

        // Se é folha, adicionar primitivos
        if node.left.is_none() && node.right.is_none() {
            results.extend(&node.primitive_indices);
            return;
        }

        // Recursão
        if let Some(ref left) = node.left {
            Self::query_box_recursive(left, query_bounds, results);
        }

        if let Some(ref right) = node.right {
            Self::query_box_recursive(right, query_bounds, results);
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

    /// Query de objetos dentro de esfera
    pub fn query_sphere(&self, center: [f64; 3], radius: f64) -> Vec<usize> {
        let mut results = Vec::new();

        if let Some(ref root) = self.root {
            Self::query_sphere_recursive(root, center, radius, &mut results);
        }

        results
    }

    fn query_sphere_recursive(
        node: &BvhNode,
        center: [f64; 3],
        radius: f64,
        results: &mut Vec<usize>,
    ) {
        // Testar se esfera intersecta bounding box
        if !Self::sphere_box_intersect(center, radius, &node.bounds) {
            return;
        }

        // Se é folha, adicionar primitivos
        if node.left.is_none() && node.right.is_none() {
            results.extend(&node.primitive_indices);
            return;
        }

        // Recursão
        if let Some(ref left) = node.left {
            Self::query_sphere_recursive(left, center, radius, results);
        }

        if let Some(ref right) = node.right {
            Self::query_sphere_recursive(right, center, radius, results);
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

    /// Contar nós na árvore
    pub fn node_count(&self) -> usize {
        match &self.root {
            Some(root) => Self::count_nodes_recursive(root),
            None => 0,
        }
    }

    fn count_nodes_recursive(node: &BvhNode) -> usize {
        let mut count = 1;

        if let Some(ref left) = node.left {
            count += Self::count_nodes_recursive(left);
        }

        if let Some(ref right) = node.right {
            count += Self::count_nodes_recursive(right);
        }

        count
    }

    /// Calcular profundidade máxima
    pub fn max_depth(&self) -> usize {
        match &self.root {
            Some(root) => Self::compute_depth_recursive(root),
            None => 0,
        }
    }

    fn compute_depth_recursive(node: &BvhNode) -> usize {
        let left_depth = node.left.as_ref().map_or(0, |n| Self::compute_depth_recursive(n));
        let right_depth = node.right.as_ref().map_or(0, |n| Self::compute_depth_recursive(n));

        1 + left_depth.max(right_depth)
    }
}

impl Default for Bvh {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bvh_build() {
        let bounds = vec![
            BoundingBox {
                min: [0.0, 0.0, 0.0],
                max: [1.0, 1.0, 1.0],
            },
            BoundingBox {
                min: [2.0, 0.0, 0.0],
                max: [3.0, 1.0, 1.0],
            },
            BoundingBox {
                min: [0.0, 2.0, 0.0],
                max: [1.0, 3.0, 1.0],
            },
        ];

        let bvh = Bvh::build(&bounds, 1);
        assert!(bvh.node_count() > 0);
    }

    #[test]
    fn test_bvh_query_box() {
        let bounds = vec![
            BoundingBox {
                min: [0.0, 0.0, 0.0],
                max: [1.0, 1.0, 1.0],
            },
            BoundingBox {
                min: [5.0, 5.0, 5.0],
                max: [6.0, 6.0, 6.0],
            },
        ];

        let bvh = Bvh::build(&bounds, 1);

        let query = BoundingBox {
            min: [-0.5, -0.5, -0.5],
            max: [1.5, 1.5, 1.5],
        };

        let results = bvh.query_box(&query);
        assert_eq!(results.len(), 1);
        assert_eq!(results[0], 0);
    }

    #[test]
    fn test_bvh_ray_intersect() {
        let bounds = vec![
            BoundingBox {
                min: [0.0, 0.0, 0.0],
                max: [1.0, 1.0, 1.0],
            },
        ];

        let bvh = Bvh::build(&bounds, 1);

        let origin = [-1.0, 0.5, 0.5];
        let direction = [1.0, 0.0, 0.0];

        let results = bvh.intersect_ray(origin, direction);
        assert_eq!(results.len(), 1);
    }
}
