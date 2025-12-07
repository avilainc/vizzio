//! Curve utilities (Rust puro)

use std::f64::consts::PI;

/// Utilitários para curvas paramétricas
pub struct Curve;

impl Curve {
    /// Avaliar curva de Bézier cúbica
    pub fn cubic_bezier(t: f64, p0: [f64; 3], p1: [f64; 3], p2: [f64; 3], p3: [f64; 3]) -> [f64; 3] {
        let t2 = t * t;
        let t3 = t2 * t;
        let mt = 1.0 - t;
        let mt2 = mt * mt;
        let mt3 = mt2 * mt;

        [
            mt3 * p0[0] + 3.0 * mt2 * t * p1[0] + 3.0 * mt * t2 * p2[0] + t3 * p3[0],
            mt3 * p0[1] + 3.0 * mt2 * t * p1[1] + 3.0 * mt * t2 * p2[1] + t3 * p3[1],
            mt3 * p0[2] + 3.0 * mt2 * t * p1[2] + 3.0 * mt * t2 * p2[2] + t3 * p3[2],
        ]
    }

    /// Derivada de curva de Bézier cúbica
    pub fn cubic_bezier_derivative(t: f64, p0: [f64; 3], p1: [f64; 3], p2: [f64; 3], p3: [f64; 3]) -> [f64; 3] {
        let t2 = t * t;
        let mt = 1.0 - t;
        let mt2 = mt * mt;

        [
            3.0 * mt2 * (p1[0] - p0[0]) + 6.0 * mt * t * (p2[0] - p1[0]) + 3.0 * t2 * (p3[0] - p2[0]),
            3.0 * mt2 * (p1[1] - p0[1]) + 6.0 * mt * t * (p2[1] - p1[1]) + 3.0 * t2 * (p3[1] - p2[1]),
            3.0 * mt2 * (p1[2] - p0[2]) + 6.0 * mt * t * (p2[2] - p1[2]) + 3.0 * t2 * (p3[2] - p2[2]),
        ]
    }

    /// Tesselar curva de Bézier cúbica
    pub fn tessellate_cubic_bezier(
        p0: [f64; 3],
        p1: [f64; 3],
        p2: [f64; 3],
        p3: [f64; 3],
        segments: usize,
    ) -> Vec<[f64; 3]> {
        let mut points = Vec::with_capacity(segments + 1);

        for i in 0..=segments {
            let t = i as f64 / segments as f64;
            points.push(Self::cubic_bezier(t, p0, p1, p2, p3));
        }

        points
    }

    /// Avaliar círculo 2D
    pub fn circle_2d(angle: f64, center: [f64; 2], radius: f64) -> [f64; 2] {
        [
            center[0] + radius * angle.cos(),
            center[1] + radius * angle.sin(),
        ]
    }

    /// Tesselar círculo 2D
    pub fn tessellate_circle_2d(center: [f64; 2], radius: f64, segments: usize) -> Vec<[f64; 2]> {
        let mut points = Vec::with_capacity(segments);

        for i in 0..segments {
            let angle = (i as f64 / segments as f64) * 2.0 * PI;
            points.push(Self::circle_2d(angle, center, radius));
        }

        points
    }

    /// Avaliar elipse 2D
    pub fn ellipse_2d(angle: f64, center: [f64; 2], radius_x: f64, radius_y: f64) -> [f64; 2] {
        [
            center[0] + radius_x * angle.cos(),
            center[1] + radius_y * angle.sin(),
        ]
    }

    /// Tesselar elipse 2D
    pub fn tessellate_ellipse_2d(
        center: [f64; 2],
        radius_x: f64,
        radius_y: f64,
        segments: usize,
    ) -> Vec<[f64; 2]> {
        let mut points = Vec::with_capacity(segments);

        for i in 0..segments {
            let angle = (i as f64 / segments as f64) * 2.0 * PI;
            points.push(Self::ellipse_2d(angle, center, radius_x, radius_y));
        }

        points
    }

    /// Avaliar arco circular 2D
    pub fn arc_2d(
        angle: f64,
        center: [f64; 2],
        radius: f64,
        start_angle: f64,
        end_angle: f64,
    ) -> [f64; 2] {
        let actual_angle = start_angle + angle * (end_angle - start_angle);
        Self::circle_2d(actual_angle, center, radius)
    }

    /// Tesselar arco 2D
    pub fn tessellate_arc_2d(
        center: [f64; 2],
        radius: f64,
        start_angle: f64,
        end_angle: f64,
        segments: usize,
    ) -> Vec<[f64; 2]> {
        let mut points = Vec::with_capacity(segments + 1);

        for i in 0..=segments {
            let t = i as f64 / segments as f64;
            points.push(Self::arc_2d(t, center, radius, start_angle, end_angle));
        }

        points
    }

    /// Calcular comprimento aproximado de curva
    pub fn curve_length_approx(points: &[[f64; 3]]) -> f64 {
        let mut length = 0.0;

        for i in 0..points.len() - 1 {
            let dx = points[i + 1][0] - points[i][0];
            let dy = points[i + 1][1] - points[i][1];
            let dz = points[i + 1][2] - points[i][2];
            length += (dx * dx + dy * dy + dz * dz).sqrt();
        }

        length
    }

    /// Interpolar linearmente entre dois pontos
    pub fn lerp(t: f64, a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
        [
            a[0] + t * (b[0] - a[0]),
            a[1] + t * (b[1] - a[1]),
            a[2] + t * (b[2] - a[2]),
        ]
    }

    /// Catmull-Rom spline (interpolação suave)
    pub fn catmull_rom(t: f64, p0: [f64; 3], p1: [f64; 3], p2: [f64; 3], p3: [f64; 3]) -> [f64; 3] {
        let t2 = t * t;
        let t3 = t2 * t;

        let q0 = -t3 + 2.0 * t2 - t;
        let q1 = 3.0 * t3 - 5.0 * t2 + 2.0;
        let q2 = -3.0 * t3 + 4.0 * t2 + t;
        let q3 = t3 - t2;

        [
            0.5 * (p0[0] * q0 + p1[0] * q1 + p2[0] * q2 + p3[0] * q3),
            0.5 * (p0[1] * q0 + p1[1] * q1 + p2[1] * q2 + p3[1] * q3),
            0.5 * (p0[2] * q0 + p1[2] * q1 + p2[2] * q2 + p3[2] * q3),
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cubic_bezier() {
        let p0 = [0.0, 0.0, 0.0];
        let p1 = [1.0, 2.0, 0.0];
        let p2 = [2.0, 2.0, 0.0];
        let p3 = [3.0, 0.0, 0.0];

        let mid = Curve::cubic_bezier(0.5, p0, p1, p2, p3);

        // Ponto médio deve estar aproximadamente em (1.5, 1.5, 0)
        assert!((mid[0] - 1.5).abs() < 0.1);
        assert!((mid[1] - 1.5).abs() < 0.1);
    }

    #[test]
    fn test_circle_tessellation() {
        let center = [0.0, 0.0];
        let radius = 1.0;
        let points = Curve::tessellate_circle_2d(center, radius, 8);

        assert_eq!(points.len(), 8);

        // Primeiro ponto deve estar em (1, 0)
        assert!((points[0][0] - 1.0).abs() < 1e-10);
        assert!((points[0][1] - 0.0).abs() < 1e-10);
    }

    #[test]
    fn test_lerp() {
        let a = [0.0, 0.0, 0.0];
        let b = [10.0, 10.0, 10.0];

        let mid = Curve::lerp(0.5, a, b);
        assert_eq!(mid, [5.0, 5.0, 5.0]);
    }
}
