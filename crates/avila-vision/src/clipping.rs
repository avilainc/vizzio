//! Clipping planes for section views

#![no_std]

extern crate alloc;
use alloc::vec::Vec;

/// Clipping plane definition
#[derive(Debug, Clone, Copy)]
pub struct ClippingPlane {
    /// Plane normal (normalized)
    pub normal: [f32; 3],

    /// Distance from origin along normal
    pub distance: f32,

    /// Enabled state
    pub enabled: bool,

    /// Flip direction (clip opposite side)
    pub flip: bool,
}

impl ClippingPlane {
    /// Create plane from point and normal
    pub fn from_point_normal(point: [f32; 3], normal: [f32; 3]) -> Self {
        let len = (normal[0] * normal[0] + normal[1] * normal[1] + normal[2] * normal[2]).sqrt();
        let normalized = [normal[0] / len, normal[1] / len, normal[2] / len];
        let distance = point[0] * normalized[0] + point[1] * normalized[1] + point[2] * normalized[2];

        Self {
            normal: normalized,
            distance,
            enabled: true,
            flip: false,
        }
    }

    /// Create horizontal plane at height
    pub fn horizontal(height: f32) -> Self {
        Self {
            normal: [0.0, 0.0, 1.0],
            distance: height,
            enabled: true,
            flip: false,
        }
    }

    /// Create vertical plane (YZ) at X position
    pub fn vertical_x(x: f32) -> Self {
        Self {
            normal: [1.0, 0.0, 0.0],
            distance: x,
            enabled: true,
            flip: false,
        }
    }

    /// Create vertical plane (XZ) at Y position
    pub fn vertical_y(y: f32) -> Self {
        Self {
            normal: [0.0, 1.0, 0.0],
            distance: y,
            enabled: true,
            flip: false,
        }
    }

    /// Get signed distance from point to plane
    pub fn distance_to_point(&self, point: [f32; 3]) -> f32 {
        let dist = point[0] * self.normal[0]
                 + point[1] * self.normal[1]
                 + point[2] * self.normal[2]
                 - self.distance;

        if self.flip { -dist } else { dist }
    }

    /// Check if point is clipped (should be removed)
    pub fn clips_point(&self, point: [f32; 3]) -> bool {
        if !self.enabled {
            return false;
        }
        self.distance_to_point(point) < 0.0
    }

    /// Check if bounding box is completely clipped
    pub fn clips_box(&self, min: [f32; 3], max: [f32; 3]) -> bool {
        if !self.enabled {
            return false;
        }

        // Test all 8 corners
        let corners = [
            [min[0], min[1], min[2]],
            [max[0], min[1], min[2]],
            [max[0], max[1], min[2]],
            [min[0], max[1], min[2]],
            [min[0], min[1], max[2]],
            [max[0], min[1], max[2]],
            [max[0], max[1], max[2]],
            [min[0], max[1], max[2]],
        ];

        // If all corners are clipped, box is completely clipped
        corners.iter().all(|&corner| self.clips_point(corner))
    }

    /// Check if bounding box intersects plane (partial clipping)
    pub fn intersects_box(&self, min: [f32; 3], max: [f32; 3]) -> bool {
        if !self.enabled {
            return false;
        }

        let corners = [
            [min[0], min[1], min[2]],
            [max[0], min[1], min[2]],
            [max[0], max[1], min[2]],
            [min[0], max[1], min[2]],
            [min[0], min[1], max[2]],
            [max[0], min[1], max[2]],
            [max[0], max[1], max[2]],
            [min[0], max[1], max[2]],
        ];

        let mut has_positive = false;
        let mut has_negative = false;

        for corner in &corners {
            let dist = self.distance_to_point(*corner);
            if dist >= 0.0 {
                has_positive = true;
            }
            if dist <= 0.0 {
                has_negative = true;
            }
        }

        has_positive && has_negative
    }
}

/// Multiple clipping planes manager
#[derive(Debug, Clone)]
pub struct ClippingPlanes {
    planes: Vec<ClippingPlane>,
}

impl Default for ClippingPlanes {
    fn default() -> Self {
        Self {
            planes: Vec::new(),
        }
    }
}

impl ClippingPlanes {
    /// Create new empty set
    pub fn new() -> Self {
        Self::default()
    }

    /// Add clipping plane
    pub fn add(&mut self, plane: ClippingPlane) -> usize {
        let id = self.planes.len();
        self.planes.push(plane);
        id
    }

    /// Remove plane by index
    pub fn remove(&mut self, index: usize) -> Option<ClippingPlane> {
        if index < self.planes.len() {
            Some(self.planes.remove(index))
        } else {
            None
        }
    }

    /// Get plane by index
    pub fn get(&self, index: usize) -> Option<&ClippingPlane> {
        self.planes.get(index)
    }

    /// Get mutable plane by index
    pub fn get_mut(&mut self, index: usize) -> Option<&mut ClippingPlane> {
        self.planes.get_mut(index)
    }

    /// Enable/disable all planes
    pub fn set_all_enabled(&mut self, enabled: bool) {
        for plane in &mut self.planes {
            plane.enabled = enabled;
        }
    }

    /// Clear all planes
    pub fn clear(&mut self) {
        self.planes.clear();
    }

    /// Count planes
    pub fn count(&self) -> usize {
        self.planes.len()
    }

    /// Check if point is clipped by any plane
    pub fn clips_point(&self, point: [f32; 3]) -> bool {
        self.planes.iter().any(|plane| plane.clips_point(point))
    }

    /// Check if box is completely clipped by any plane
    pub fn clips_box(&self, min: [f32; 3], max: [f32; 3]) -> bool {
        self.planes.iter().any(|plane| plane.clips_box(min, max))
    }

    /// Check if box intersects any plane
    pub fn intersects_box(&self, min: [f32; 3], max: [f32; 3]) -> bool {
        self.planes.iter().any(|plane| plane.intersects_box(min, max))
    }

    /// Get enabled planes
    pub fn enabled_planes(&self) -> Vec<&ClippingPlane> {
        self.planes.iter().filter(|p| p.enabled).collect()
    }
}

/// Section box - 6 planes forming a box
#[derive(Debug, Clone)]
pub struct SectionBox {
    pub min: [f32; 3],
    pub max: [f32; 3],
    pub enabled: bool,
}

impl SectionBox {
    /// Create section box
    pub fn new(min: [f32; 3], max: [f32; 3]) -> Self {
        Self {
            min,
            max,
            enabled: true,
        }
    }

    /// Convert to 6 clipping planes
    pub fn to_clipping_planes(&self) -> ClippingPlanes {
        let mut planes = ClippingPlanes::new();

        if self.enabled {
            // -X plane
            planes.add(ClippingPlane {
                normal: [-1.0, 0.0, 0.0],
                distance: -self.min[0],
                enabled: true,
                flip: false,
            });

            // +X plane
            planes.add(ClippingPlane {
                normal: [1.0, 0.0, 0.0],
                distance: self.max[0],
                enabled: true,
                flip: false,
            });

            // -Y plane
            planes.add(ClippingPlane {
                normal: [0.0, -1.0, 0.0],
                distance: -self.min[1],
                enabled: true,
                flip: false,
            });

            // +Y plane
            planes.add(ClippingPlane {
                normal: [0.0, 1.0, 0.0],
                distance: self.max[1],
                enabled: true,
                flip: false,
            });

            // -Z plane
            planes.add(ClippingPlane {
                normal: [0.0, 0.0, -1.0],
                distance: -self.min[2],
                enabled: true,
                flip: false,
            });

            // +Z plane
            planes.add(ClippingPlane {
                normal: [0.0, 0.0, 1.0],
                distance: self.max[2],
                enabled: true,
                flip: false,
            });
        }

        planes
    }

    /// Check if point is inside box
    pub fn contains_point(&self, point: [f32; 3]) -> bool {
        if !self.enabled {
            return true;
        }

        point[0] >= self.min[0] && point[0] <= self.max[0]
            && point[1] >= self.min[1] && point[1] <= self.max[1]
            && point[2] >= self.min[2] && point[2] <= self.max[2]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_horizontal_plane() {
        let plane = ClippingPlane::horizontal(5.0);
        assert!(plane.clips_point([0.0, 0.0, 3.0]));
        assert!(!plane.clips_point([0.0, 0.0, 7.0]));
    }

    #[test]
    fn test_box_clipping() {
        let plane = ClippingPlane::horizontal(5.0);

        // Box completely below plane
        assert!(plane.clips_box([0.0, 0.0, 0.0], [10.0, 10.0, 4.0]));

        // Box completely above plane
        assert!(!plane.clips_box([0.0, 0.0, 6.0], [10.0, 10.0, 10.0]));

        // Box intersects plane
        assert!(!plane.clips_box([0.0, 0.0, 4.0], [10.0, 10.0, 6.0]));
    }

    #[test]
    fn test_section_box() {
        let section = SectionBox::new([0.0, 0.0, 0.0], [10.0, 10.0, 10.0]);

        assert!(section.contains_point([5.0, 5.0, 5.0]));
        assert!(!section.contains_point([15.0, 5.0, 5.0]));
    }

    #[test]
    fn test_multiple_planes() {
        let mut planes = ClippingPlanes::new();
        planes.add(ClippingPlane::horizontal(5.0));
        planes.add(ClippingPlane::vertical_x(10.0));

        assert!(planes.clips_point([5.0, 0.0, 3.0])); // Below horizontal
        assert!(planes.clips_point([8.0, 0.0, 7.0])); // Before vertical
    }
}
