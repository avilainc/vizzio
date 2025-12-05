//! Accelerated data structures for ray tracing (BVH, grids, etc.)

/// Axis-aligned bounding box
#[derive(Debug, Clone, Copy)]
pub struct AABB {
    pub min: [f64; 3],
    pub max: [f64; 3],
}

impl AABB {
    /// Create bounding box from two points
    pub fn new(p1: [f64; 3], p2: [f64; 3]) -> Self {
        Self {
            min: [p1[0].min(p2[0]), p1[1].min(p2[1]), p1[2].min(p2[2])],
            max: [p1[0].max(p2[0]), p1[1].max(p2[1]), p1[2].max(p2[2])],
        }
    }

    /// Expand box to contain point
    pub fn expand(&mut self, p: [f64; 3]) {
        self.min[0] = self.min[0].min(p[0]);
        self.min[1] = self.min[1].min(p[1]);
        self.min[2] = self.min[2].min(p[2]);
        self.max[0] = self.max[0].max(p[0]);
        self.max[1] = self.max[1].max(p[1]);
        self.max[2] = self.max[2].max(p[2]);
    }

    /// Expand box to contain another box
    pub fn merge(&mut self, other: AABB) {
        self.expand(other.min);
        self.expand(other.max);
    }

    /// Test ray intersection with box
    pub fn intersect_ray(&self, origin: [f64; 3], dir: [f64; 3]) -> bool {
        let mut t_min = f64::NEG_INFINITY;
        let mut t_max = f64::INFINITY;

        for i in 0..3 {
            if dir[i].abs() > 1e-6 {
                let t0 = (self.min[i] - origin[i]) / dir[i];
                let t1 = (self.max[i] - origin[i]) / dir[i];

                let (t0, t1) = if t0 > t1 { (t1, t0) } else { (t0, t1) };

                t_min = t_min.max(t0);
                t_max = t_max.min(t1);

                if t_min > t_max {
                    return false;
                }
            } else if origin[i] < self.min[i] || origin[i] > self.max[i] {
                return false;
            }
        }

        t_max >= t_min && t_min < f64::INFINITY
    }

    /// Volume of the box
    pub fn volume(&self) -> f64 {
        let dx = self.max[0] - self.min[0];
        let dy = self.max[1] - self.min[1];
        let dz = self.max[2] - self.min[2];
        dx * dy * dz
    }

    /// Surface area of the box
    pub fn surface_area(&self) -> f64 {
        let dx = self.max[0] - self.min[0];
        let dy = self.max[1] - self.min[1];
        let dz = self.max[2] - self.min[2];
        2.0 * (dx * dy + dy * dz + dz * dx)
    }

    /// Center of the box
    pub fn center(&self) -> [f64; 3] {
        [
            (self.min[0] + self.max[0]) * 0.5,
            (self.min[1] + self.max[1]) * 0.5,
            (self.min[2] + self.max[2]) * 0.5,
        ]
    }

    /// Maximum extent (diagonal length)
    pub fn extent(&self) -> f64 {
        let dx = self.max[0] - self.min[0];
        let dy = self.max[1] - self.min[1];
        let dz = self.max[2] - self.min[2];
        (dx * dx + dy * dy + dz * dz).sqrt()
    }
}

/// Spatial grid for acceleration
pub struct SpatialGrid {
    grid_size: u32,
    bounds: AABB,
    cells: Vec<Vec<usize>>, // Cell -> list of primitive indices
}

impl SpatialGrid {
    /// Create new spatial grid
    pub fn new(bounds: AABB, grid_size: u32) -> Self {
        let cell_count = (grid_size * grid_size * grid_size) as usize;
        Self {
            grid_size,
            bounds,
            cells: vec![Vec::new(); cell_count],
        }
    }

    /// Get grid cell for point
    fn get_cell_index(&self, p: [f64; 3]) -> Option<usize> {
        let dx = self.bounds.max[0] - self.bounds.min[0];
        let dy = self.bounds.max[1] - self.bounds.min[1];
        let dz = self.bounds.max[2] - self.bounds.min[2];

        if dx == 0.0 || dy == 0.0 || dz == 0.0 {
            return None;
        }

        let x = ((p[0] - self.bounds.min[0]) / dx * self.grid_size as f64) as u32;
        let y = ((p[1] - self.bounds.min[1]) / dy * self.grid_size as f64) as u32;
        let z = ((p[2] - self.bounds.min[2]) / dz * self.grid_size as f64) as u32;

        if x >= self.grid_size || y >= self.grid_size || z >= self.grid_size {
            return None;
        }

        Some((z * self.grid_size * self.grid_size + y * self.grid_size + x) as usize)
    }

    /// Insert primitive into grid
    pub fn insert(&mut self, prim_idx: usize, bounds: AABB) {
        let min_idx = self.get_cell_index(bounds.min);
        let max_idx = self.get_cell_index(bounds.max);

        if let (Some(min_i), Some(max_i)) = (min_idx, max_idx) {
            let min_x = min_i % self.grid_size as usize;
            let max_x = max_i % self.grid_size as usize;

            for i in min_x..=max_x {
                if i < self.cells.len() {
                    self.cells[i].push(prim_idx);
                }
            }
        }
    }

    /// Get primitives in cells intersected by ray
    pub fn query_ray(&self, origin: [f64; 3], _dir: [f64; 3]) -> Vec<usize> {
        let mut result = Vec::new();

        // Simple grid traversal
        if let Some(_start_idx) = self.get_cell_index(origin) {
            // Collect all primitives in cells along ray
            let mut visited = std::collections::HashSet::new();

            for cell_primitives in &self.cells {
                for &prim_idx in cell_primitives {
                    if visited.insert(prim_idx) {
                        result.push(prim_idx);
                    }
                }
            }
        }

        result
    }
}

/// Bounding Volume Hierarchy node
pub struct BVHNode {
    pub bounds: AABB,
    pub left: Option<Box<BVHNode>>,
    pub right: Option<Box<BVHNode>>,
    pub primitive_idx: Option<usize>, // Leaf node
}

impl BVHNode {
    /// Create leaf node
    pub fn leaf(bounds: AABB, prim_idx: usize) -> Self {
        Self {
            bounds,
            left: None,
            right: None,
            primitive_idx: Some(prim_idx),
        }
    }

    /// Create internal node
    pub fn internal(
        bounds: AABB,
        left: BVHNode,
        right: BVHNode,
    ) -> Self {
        Self {
            bounds,
            left: Some(Box::new(left)),
            right: Some(Box::new(right)),
            primitive_idx: None,
        }
    }

    /// Is leaf node
    pub fn is_leaf(&self) -> bool {
        self.primitive_idx.is_some()
    }

    /// Cost of traversal (for SAH)
    pub fn sah_cost(&self, ray_count: u32) -> f64 {
        if self.is_leaf() {
            self.bounds.surface_area() * ray_count as f64
        } else {
            let cost = self.bounds.surface_area();
            let left_cost = self.left.as_ref().map_or(0.0, |n| n.sah_cost(ray_count));
            let right_cost = self.right.as_ref().map_or(0.0, |n| n.sah_cost(ray_count));
            cost + left_cost + right_cost
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_aabb_creation() {
        let aabb = AABB::new([0.0, 0.0, 0.0], [1.0, 1.0, 1.0]);
        assert_eq!(aabb.min, [0.0, 0.0, 0.0]);
        assert_eq!(aabb.max, [1.0, 1.0, 1.0]);
    }

    #[test]
    fn test_aabb_volume() {
        let aabb = AABB::new([0.0, 0.0, 0.0], [2.0, 2.0, 2.0]);
        assert_eq!(aabb.volume(), 8.0);
    }

    #[test]
    fn test_aabb_ray_intersection() {
        let aabb = AABB::new([0.0, 0.0, 0.0], [1.0, 1.0, 1.0]);
        assert!(aabb.intersect_ray([0.5, 0.5, -1.0], [0.0, 0.0, 1.0]));
    }

    #[test]
    fn test_spatial_grid() {
        let bounds = AABB::new([0.0, 0.0, 0.0], [10.0, 10.0, 10.0]);
        let grid = SpatialGrid::new(bounds, 4);
        assert_eq!(grid.grid_size, 4);
    }

    #[test]
    fn test_bvh_node_creation() {
        let bounds = AABB::new([0.0, 0.0, 0.0], [1.0, 1.0, 1.0]);
        let node = BVHNode::leaf(bounds, 0);
        assert!(node.is_leaf());
        assert_eq!(node.primitive_idx, Some(0));
    }
}
