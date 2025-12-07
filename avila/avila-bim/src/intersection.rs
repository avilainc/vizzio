//! Intersection algorithms (Rust puro)

/// Algoritmos de interseção geométrica
pub struct Intersection;

impl Intersection {
    /// Interseção ray-triângulo (Möller-Trumbore)
    pub fn ray_triangle(
        ray_origin: [f64; 3],
        ray_direction: [f64; 3],
        v0: [f64; 3],
        v1: [f64; 3],
        v2: [f64; 3],
    ) -> Option<f64> {
        const EPSILON: f64 = 1e-8;

        let edge1 = [v1[0] - v0[0], v1[1] - v0[1], v1[2] - v0[2]];
        let edge2 = [v2[0] - v0[0], v2[1] - v0[1], v2[2] - v0[2]];

        let h = Self::cross(ray_direction, edge2);
        let a = Self::dot(edge1, h);

        if a.abs() < EPSILON {
            return None; // Ray paralelo ao triângulo
        }

        let f = 1.0 / a;
        let s = [ray_origin[0] - v0[0], ray_origin[1] - v0[1], ray_origin[2] - v0[2]];
        let u = f * Self::dot(s, h);

        if u < 0.0 || u > 1.0 {
            return None;
        }

        let q = Self::cross(s, edge1);
        let v = f * Self::dot(ray_direction, q);

        if v < 0.0 || u + v > 1.0 {
            return None;
        }

        let t = f * Self::dot(edge2, q);

        if t > EPSILON {
            Some(t)
        } else {
            None
        }
    }

    /// Interseção ray-AABB (bounding box)
    pub fn ray_aabb(
        ray_origin: [f64; 3],
        ray_direction: [f64; 3],
        box_min: [f64; 3],
        box_max: [f64; 3],
    ) -> Option<f64> {
        let mut tmin = f64::NEG_INFINITY;
        let mut tmax = f64::INFINITY;

        for i in 0..3 {
            if ray_direction[i].abs() < 1e-8 {
                // Ray paralelo ao eixo
                if ray_origin[i] < box_min[i] || ray_origin[i] > box_max[i] {
                    return None;
                }
            } else {
                let inv_d = 1.0 / ray_direction[i];
                let mut t0 = (box_min[i] - ray_origin[i]) * inv_d;
                let mut t1 = (box_max[i] - ray_origin[i]) * inv_d;

                if inv_d < 0.0 {
                    std::mem::swap(&mut t0, &mut t1);
                }

                tmin = tmin.max(t0);
                tmax = tmax.min(t1);

                if tmax <= tmin {
                    return None;
                }
            }
        }

        if tmin >= 0.0 {
            Some(tmin)
        } else if tmax >= 0.0 {
            Some(0.0)
        } else {
            None
        }
    }

    /// Interseção AABB-AABB
    pub fn aabb_aabb(
        min1: [f64; 3],
        max1: [f64; 3],
        min2: [f64; 3],
        max2: [f64; 3],
    ) -> bool {
        for i in 0..3 {
            if max1[i] < min2[i] || min1[i] > max2[i] {
                return false;
            }
        }
        true
    }

    /// Interseção esfera-esfera
    pub fn sphere_sphere(
        center1: [f64; 3],
        radius1: f64,
        center2: [f64; 3],
        radius2: f64,
    ) -> bool {
        let dx = center1[0] - center2[0];
        let dy = center1[1] - center2[1];
        let dz = center1[2] - center2[2];
        let dist_sq = dx * dx + dy * dy + dz * dz;
        let sum_radii = radius1 + radius2;

        dist_sq <= sum_radii * sum_radii
    }

    /// Interseção linha-linha 2D
    pub fn line_line_2d(
        p1: [f64; 2],
        p2: [f64; 2],
        p3: [f64; 2],
        p4: [f64; 2],
    ) -> Option<[f64; 2]> {
        let x1 = p1[0];
        let y1 = p1[1];
        let x2 = p2[0];
        let y2 = p2[1];
        let x3 = p3[0];
        let y3 = p3[1];
        let x4 = p4[0];
        let y4 = p4[1];

        let denom = (x1 - x2) * (y3 - y4) - (y1 - y2) * (x3 - x4);

        if denom.abs() < 1e-10 {
            return None; // Linhas paralelas
        }

        let t = ((x1 - x3) * (y3 - y4) - (y1 - y3) * (x3 - x4)) / denom;
        let u = -((x1 - x2) * (y1 - y3) - (y1 - y2) * (x1 - x3)) / denom;

        if t >= 0.0 && t <= 1.0 && u >= 0.0 && u <= 1.0 {
            let x = x1 + t * (x2 - x1);
            let y = y1 + t * (y2 - y1);
            Some([x, y])
        } else {
            None
        }
    }

    /// Ponto mais próximo em segmento de linha
    pub fn closest_point_on_segment(
        point: [f64; 3],
        line_start: [f64; 3],
        line_end: [f64; 3],
    ) -> [f64; 3] {
        let line = [
            line_end[0] - line_start[0],
            line_end[1] - line_start[1],
            line_end[2] - line_start[2],
        ];

        let to_point = [
            point[0] - line_start[0],
            point[1] - line_start[1],
            point[2] - line_start[2],
        ];

        let line_length_sq = line[0] * line[0] + line[1] * line[1] + line[2] * line[2];

        if line_length_sq < 1e-10 {
            return line_start;
        }

        let t = (Self::dot(to_point, line) / line_length_sq).clamp(0.0, 1.0);

        [
            line_start[0] + t * line[0],
            line_start[1] + t * line[1],
            line_start[2] + t * line[2],
        ]
    }

    #[inline]
    fn dot(a: [f64; 3], b: [f64; 3]) -> f64 {
        a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
    }

    #[inline]
    fn cross(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
        [
            a[1] * b[2] - a[2] * b[1],
            a[2] * b[0] - a[0] * b[2],
            a[0] * b[1] - a[1] * b[0],
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ray_triangle() {
        let ray_origin = [0.0, 0.0, 0.0];
        let ray_direction = [0.0, 0.0, 1.0];

        let v0 = [-1.0, -1.0, 5.0];
        let v1 = [1.0, -1.0, 5.0];
        let v2 = [0.0, 1.0, 5.0];

        let hit = Intersection::ray_triangle(ray_origin, ray_direction, v0, v1, v2);
        assert!(hit.is_some());
        assert!((hit.unwrap() - 5.0).abs() < 1e-6);
    }

    #[test]
    fn test_ray_aabb() {
        let ray_origin = [0.0, 0.0, 0.0];
        let ray_direction = [1.0, 0.0, 0.0];

        let box_min = [5.0, -1.0, -1.0];
        let box_max = [6.0, 1.0, 1.0];

        let hit = Intersection::ray_aabb(ray_origin, ray_direction, box_min, box_max);
        assert!(hit.is_some());
        assert!((hit.unwrap() - 5.0).abs() < 1e-6);
    }

    #[test]
    fn test_aabb_aabb() {
        let min1 = [0.0, 0.0, 0.0];
        let max1 = [2.0, 2.0, 2.0];

        let min2 = [1.0, 1.0, 1.0];
        let max2 = [3.0, 3.0, 3.0];

        assert!(Intersection::aabb_aabb(min1, max1, min2, max2));

        let min3 = [10.0, 10.0, 10.0];
        let max3 = [12.0, 12.0, 12.0];

        assert!(!Intersection::aabb_aabb(min1, max1, min3, max3));
    }

    #[test]
    fn test_sphere_sphere() {
        let center1 = [0.0, 0.0, 0.0];
        let center2 = [3.0, 0.0, 0.0];

        assert!(Intersection::sphere_sphere(center1, 2.0, center2, 2.0));
        assert!(!Intersection::sphere_sphere(center1, 1.0, center2, 1.0));
    }
}
