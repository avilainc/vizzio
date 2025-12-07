//! Raycasting em modelos BIM

use crate::bim_core::*;

/// Ray 3D
#[derive(Debug, Clone)]
pub struct Ray {
    pub origin: [f64; 3],
    pub direction: [f64; 3],
}

/// Resultado de raycast
#[derive(Debug, Clone)]
pub struct RayHit {
    pub element_guid: IfcGuid,
    pub distance: f64,
    pub point: [f64; 3],
    pub normal: [f64; 3],
}

/// Raycast engine
pub struct Raycast;

impl Raycast {
    /// Raycast em bounding box
    pub fn ray_intersects_bounds(ray: &Ray, bounds: &BoundingBox) -> Option<f64> {
        let mut tmin = f64::NEG_INFINITY;
        let mut tmax = f64::INFINITY;

        for i in 0..3 {
            let inv_d = 1.0 / ray.direction[i];
            let mut t0 = (bounds.min[i] - ray.origin[i]) * inv_d;
            let mut t1 = (bounds.max[i] - ray.origin[i]) * inv_d;

            if inv_d < 0.0 {
                std::mem::swap(&mut t0, &mut t1);
            }

            tmin = tmin.max(t0);
            tmax = tmax.min(t1);

            if tmax <= tmin {
                return None;
            }
        }

        Some(tmin)
    }

    /// Raycast em triângulo (Möller–Trumbore algorithm)
    pub fn ray_intersects_triangle(
        ray: &Ray,
        v0: [f64; 3],
        v1: [f64; 3],
        v2: [f64; 3],
    ) -> Option<f64> {
        const EPSILON: f64 = 0.0000001;

        let edge1 = [v1[0] - v0[0], v1[1] - v0[1], v1[2] - v0[2]];
        let edge2 = [v2[0] - v0[0], v2[1] - v0[1], v2[2] - v0[2]];

        let h = Self::cross(&ray.direction, &edge2);
        let a = Self::dot(&edge1, &h);

        if a > -EPSILON && a < EPSILON {
            return None; // Ray paralelo ao triângulo
        }

        let f = 1.0 / a;
        let s = [ray.origin[0] - v0[0], ray.origin[1] - v0[1], ray.origin[2] - v0[2]];
        let u = f * Self::dot(&s, &h);

        if u < 0.0 || u > 1.0 {
            return None;
        }

        let q = Self::cross(&s, &edge1);
        let v = f * Self::dot(&ray.direction, &q);

        if v < 0.0 || u + v > 1.0 {
            return None;
        }

        let t = f * Self::dot(&edge2, &q);

        if t > EPSILON {
            Some(t)
        } else {
            None
        }
    }

    fn cross(a: &[f64; 3], b: &[f64; 3]) -> [f64; 3] {
        [
            a[1] * b[2] - a[2] * b[1],
            a[2] * b[0] - a[0] * b[2],
            a[0] * b[1] - a[1] * b[0],
        ]
    }

    fn dot(a: &[f64; 3], b: &[f64; 3]) -> f64 {
        a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ray_box_intersection() {
        let ray = Ray {
            origin: [0.0, 0.0, 0.0],
            direction: [1.0, 0.0, 0.0],
        };

        let bounds = BoundingBox {
            min: [5.0, -1.0, -1.0],
            max: [6.0, 1.0, 1.0],
        };

        let hit = Raycast::ray_intersects_bounds(&ray, &bounds);
        assert!(hit.is_some());
        assert!((hit.unwrap() - 5.0).abs() < 0.001);
    }
}
