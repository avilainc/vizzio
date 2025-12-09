//! Octree spatial indexing for efficient culling and queries

#![no_std]

extern crate alloc;
use alloc::vec::Vec;
use alloc::boxed::Box;

/// Octree node for spatial partitioning
#[derive(Debug)]
pub struct Octree<T> {
    /// Bounding box of this node
    pub bounds: BoundingBox,
    /// Maximum items per node before subdivision
    pub capacity: usize,
    /// Maximum depth
    pub max_depth: usize,
    /// Current depth
    pub depth: usize,
    /// Items in this node
    pub items: Vec<OctreeItem<T>>,
    /// Children nodes (8 octants)
    pub children: Option<Box<[Octree<T>; 8]>>,
}

/// Item stored in octree
#[derive(Debug)]
pub struct OctreeItem<T> {
    /// Bounding box of item
    pub bounds: BoundingBox,
    /// User data
    pub data: T,
}

/// Axis-aligned bounding box
#[derive(Debug, Clone, Copy)]
pub struct BoundingBox {
    pub min: [f32; 3],
    pub max: [f32; 3],
}

impl BoundingBox {
    /// Check if point is inside
    pub fn contains_point(&self, point: [f32; 3]) -> bool {
        point[0] >= self.min[0] && point[0] <= self.max[0]
            && point[1] >= self.min[1] && point[1] <= self.max[1]
            && point[2] >= self.min[2] && point[2] <= self.max[2]
    }

    /// Check intersection with another box
    pub fn intersects(&self, other: &BoundingBox) -> bool {
        self.min[0] <= other.max[0] && self.max[0] >= other.min[0]
            && self.min[1] <= other.max[1] && self.max[1] >= other.min[1]
            && self.min[2] <= other.max[2] && self.max[2] >= other.min[2]
    }

    /// Check if completely contains another box
    pub fn contains(&self, other: &BoundingBox) -> bool {
        self.min[0] <= other.min[0] && self.max[0] >= other.max[0]
            && self.min[1] <= other.min[1] && self.max[1] >= other.max[1]
            && self.min[2] <= other.min[2] && self.max[2] >= other.max[2]
    }

    /// Center point
    pub fn center(&self) -> [f32; 3] {
        [
            (self.min[0] + self.max[0]) * 0.5,
            (self.min[1] + self.max[1]) * 0.5,
            (self.min[2] + self.max[2]) * 0.5,
        ]
    }

    /// Size in each dimension
    pub fn size(&self) -> [f32; 3] {
        [
            self.max[0] - self.min[0],
            self.max[1] - self.min[1],
            self.max[2] - self.min[2],
        ]
    }

    /// Distance from point to box (0 if inside)
    pub fn distance_to_point(&self, point: [f32; 3]) -> f32 {
        let mut dist_sq = 0.0f32;

        for i in 0..3 {
            if point[i] < self.min[i] {
                let d = self.min[i] - point[i];
                dist_sq += d * d;
            } else if point[i] > self.max[i] {
                let d = point[i] - self.max[i];
                dist_sq += d * d;
            }
        }

        dist_sq.sqrt()
    }
}

impl<T> Octree<T> {
    /// Create new octree
    pub fn new(bounds: BoundingBox, capacity: usize, max_depth: usize) -> Self {
        Self {
            bounds,
            capacity,
            max_depth,
            depth: 0,
            items: Vec::new(),
            children: None,
        }
    }

    /// Insert item into octree
    pub fn insert(&mut self, item: OctreeItem<T>) -> bool
    where
        T: Clone,
    {
        // Check if item fits in this node
        if !self.bounds.intersects(&item.bounds) {
            return false;
        }

        // If we have children, try to insert in them
        if let Some(ref mut children) = self.children {
            for child in children.iter_mut() {
                if child.bounds.intersects(&item.bounds) {
                    let cloned_item = OctreeItem {
                        bounds: item.bounds,
                        data: item.data.clone(),
                    };
                    if child.insert(cloned_item) {
                        return true;
                    }
                }
            }
        }

        // Add to this node
        self.items.push(item);

        // Subdivide if needed
        if self.items.len() > self.capacity && self.depth < self.max_depth && self.children.is_none() {
            self.subdivide();
        }

        true
    }

    /// Subdivide node into 8 children
    fn subdivide(&mut self)
    where
        T: Clone,
    {
        let center = self.bounds.center();
        let size = self.bounds.size();
        let half_size = [size[0] * 0.5, size[1] * 0.5, size[2] * 0.5];

        let mut octants = Vec::with_capacity(8);

        // Create 8 octants
        for z in 0..2 {
            for y in 0..2 {
                for x in 0..2 {
                    let min = [
                        if x == 0 { self.bounds.min[0] } else { center[0] },
                        if y == 0 { self.bounds.min[1] } else { center[1] },
                        if z == 0 { self.bounds.min[2] } else { center[2] },
                    ];
                    let max = [
                        if x == 0 { center[0] } else { self.bounds.max[0] },
                        if y == 0 { center[1] } else { self.bounds.max[1] },
                        if z == 0 { center[2] } else { self.bounds.max[2] },
                    ];

                    let mut child = Octree::new(
                        BoundingBox { min, max },
                        self.capacity,
                        self.max_depth,
                    );
                    child.depth = self.depth + 1;
                    octants.push(child);
                }
            }
        }

        self.children = Some(Box::new([
            octants.remove(0), octants.remove(0), octants.remove(0), octants.remove(0),
            octants.remove(0), octants.remove(0), octants.remove(0), octants.remove(0),
        ]));

        // Redistribute items to children
        let items = core::mem::take(&mut self.items);
        for item in items {
            self.insert(item);
        }
    }

    /// Query items intersecting with bounds
    pub fn query<'a>(&'a self, bounds: &BoundingBox, results: &mut Vec<&'a T>) {
        if !self.bounds.intersects(bounds) {
            return;
        }

        // Add items from this node
        for item in &self.items {
            if item.bounds.intersects(bounds) {
                results.push(&item.data);
            }
        }

        // Recursively query children
        if let Some(ref children) = self.children {
            for child in children.iter() {
                child.query(bounds, results);
            }
        }
    }

    /// Query items within radius of point
    pub fn query_radius<'a>(&'a self, point: [f32; 3], radius: f32, results: &mut Vec<&'a T>) {
        let radius_sq = radius * radius;

        // Quick rejection test
        if self.bounds.distance_to_point(point) > radius {
            return;
        }

        // Check items in this node
        for item in &self.items {
            if item.bounds.distance_to_point(point) <= radius {
                results.push(&item.data);
            }
        }

        // Recursively query children
        if let Some(ref children) = self.children {
            for child in children.iter() {
                child.query_radius(point, radius, results);
            }
        }
    }

    /// Get all items in tree
    pub fn get_all<'a>(&'a self, results: &mut Vec<&'a T>) {
        for item in &self.items {
            results.push(&item.data);
        }

        if let Some(ref children) = self.children {
            for child in children.iter() {
                child.get_all(results);
            }
        }
    }

    /// Count total items
    pub fn count(&self) -> usize {
        let mut total = self.items.len();
        if let Some(ref children) = self.children {
            for child in children.iter() {
                total += child.count();
            }
        }
        total
    }

    /// Clear all items
    pub fn clear(&mut self) {
        self.items.clear();
        self.children = None;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_octree_insert_and_query() {
        let bounds = BoundingBox {
            min: [0.0, 0.0, 0.0],
            max: [100.0, 100.0, 100.0],
        };

        let mut octree = Octree::new(bounds, 4, 8);

        // Insert items
        for i in 0..10 {
            let x = (i as f32) * 10.0;
            octree.insert(OctreeItem {
                bounds: BoundingBox {
                    min: [x, 0.0, 0.0],
                    max: [x + 5.0, 5.0, 5.0],
                },
                data: i,
            });
        }

        assert_eq!(octree.count(), 10);

        // Query region
        let query_bounds = BoundingBox {
            min: [15.0, 0.0, 0.0],
            max: [35.0, 10.0, 10.0],
        };
        let mut results = Vec::new();
        octree.query(&query_bounds, &mut results);

        assert!(results.len() > 0);
    }

    #[test]
    fn test_radius_query() {
        let bounds = BoundingBox {
            min: [-50.0, -50.0, -50.0],
            max: [50.0, 50.0, 50.0],
        };

        let mut octree = Octree::new(bounds, 4, 8);

        // Insert items in grid
        for x in -2..=2 {
            for y in -2..=2 {
                for z in -2..=2 {
                    let pos = [x as f32 * 10.0, y as f32 * 10.0, z as f32 * 10.0];
                    octree.insert(OctreeItem {
                        bounds: BoundingBox {
                            min: [pos[0] - 1.0, pos[1] - 1.0, pos[2] - 1.0],
                            max: [pos[0] + 1.0, pos[1] + 1.0, pos[2] + 1.0],
                        },
                        data: (x, y, z),
                    });
                }
            }
        }

        let mut results = Vec::new();
        octree.query_radius([0.0, 0.0, 0.0], 15.0, &mut results);

        // Should find items within radius
        assert!(results.len() > 1);
    }
}
