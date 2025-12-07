//! Triangulation algorithms (Rust puro)

/// Triangulador de polígonos 2D (Ear Clipping)
pub struct Triangulator;

impl Triangulator {
    /// Triangular polígono 2D simples (ear clipping)
    /// Recebe pontos 2D [(x, y), ...] e retorna índices de triângulos
    pub fn triangulate_polygon(points: &[[f64; 2]]) -> Vec<u32> {
        if points.len() < 3 {
            return Vec::new();
        }

        let mut indices = Vec::new();
        let mut remaining: Vec<usize> = (0..points.len()).collect();

        while remaining.len() > 3 {
            let mut ear_found = false;

            for i in 0..remaining.len() {
                let prev = remaining[(i + remaining.len() - 1) % remaining.len()];
                let curr = remaining[i];
                let next = remaining[(i + 1) % remaining.len()];

                if Self::is_ear(&points, &remaining, prev, curr, next) {
                    // Adicionar triângulo
                    indices.push(prev as u32);
                    indices.push(curr as u32);
                    indices.push(next as u32);

                    // Remover vértice do meio
                    remaining.remove(i);
                    ear_found = true;
                    break;
                }
            }

            if !ear_found {
                // Polígono não pode ser triangulado (pode ser inválido)
                break;
            }
        }

        // Adicionar último triângulo
        if remaining.len() == 3 {
            indices.push(remaining[0] as u32);
            indices.push(remaining[1] as u32);
            indices.push(remaining[2] as u32);
        }

        indices
    }

    /// Verificar se vértice é uma "orelha" (ear)
    fn is_ear(points: &[[f64; 2]], remaining: &[usize], prev: usize, curr: usize, next: usize) -> bool {
        let p0 = points[prev];
        let p1 = points[curr];
        let p2 = points[next];

        // Verificar se é convexo
        if !Self::is_convex(p0, p1, p2) {
            return false;
        }

        // Verificar se nenhum outro ponto está dentro do triângulo
        for &idx in remaining {
            if idx == prev || idx == curr || idx == next {
                continue;
            }

            if Self::point_in_triangle(points[idx], p0, p1, p2) {
                return false;
            }
        }

        true
    }

    /// Verificar se três pontos formam vértice convexo (sentido anti-horário)
    fn is_convex(p0: [f64; 2], p1: [f64; 2], p2: [f64; 2]) -> bool {
        let cross = (p1[0] - p0[0]) * (p2[1] - p0[1]) - (p1[1] - p0[1]) * (p2[0] - p0[0]);
        cross > 0.0
    }

    /// Verificar se ponto está dentro de triângulo (barycentric coordinates)
    fn point_in_triangle(p: [f64; 2], a: [f64; 2], b: [f64; 2], c: [f64; 2]) -> bool {
        let v0 = [c[0] - a[0], c[1] - a[1]];
        let v1 = [b[0] - a[0], b[1] - a[1]];
        let v2 = [p[0] - a[0], p[1] - a[1]];

        let dot00 = v0[0] * v0[0] + v0[1] * v0[1];
        let dot01 = v0[0] * v1[0] + v0[1] * v1[1];
        let dot02 = v0[0] * v2[0] + v0[1] * v2[1];
        let dot11 = v1[0] * v1[0] + v1[1] * v1[1];
        let dot12 = v1[0] * v2[0] + v1[1] * v2[1];

        let inv_denom = 1.0 / (dot00 * dot11 - dot01 * dot01);
        let u = (dot11 * dot02 - dot01 * dot12) * inv_denom;
        let v = (dot00 * dot12 - dot01 * dot02) * inv_denom;

        (u >= 0.0) && (v >= 0.0) && (u + v < 1.0)
    }

    /// Calcular área de polígono 2D (Shoelace formula)
    pub fn polygon_area(points: &[[f64; 2]]) -> f64 {
        if points.len() < 3 {
            return 0.0;
        }

        let mut area = 0.0;
        for i in 0..points.len() {
            let j = (i + 1) % points.len();
            area += points[i][0] * points[j][1];
            area -= points[j][0] * points[i][1];
        }

        (area / 2.0).abs()
    }

    /// Verificar se polígono é convexo
    pub fn is_convex_polygon(points: &[[f64; 2]]) -> bool {
        if points.len() < 3 {
            return false;
        }

        let mut sign = 0.0;

        for i in 0..points.len() {
            let p0 = points[i];
            let p1 = points[(i + 1) % points.len()];
            let p2 = points[(i + 2) % points.len()];

            let cross = (p1[0] - p0[0]) * (p2[1] - p0[1]) - (p1[1] - p0[1]) * (p2[0] - p0[0]);

            if i == 0 {
                sign = cross;
            } else if cross * sign < 0.0 {
                return false;
            }
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_triangulate_square() {
        let points = vec![
            [0.0, 0.0],
            [1.0, 0.0],
            [1.0, 1.0],
            [0.0, 1.0],
        ];

        let indices = Triangulator::triangulate_polygon(&points);
        assert_eq!(indices.len(), 6); // 2 triângulos
    }

    #[test]
    fn test_polygon_area() {
        let square = vec![
            [0.0, 0.0],
            [2.0, 0.0],
            [2.0, 2.0],
            [0.0, 2.0],
        ];

        let area = Triangulator::polygon_area(&square);
        assert!((area - 4.0).abs() < 1e-10);
    }

    #[test]
    fn test_is_convex() {
        let square = vec![
            [0.0, 0.0],
            [1.0, 0.0],
            [1.0, 1.0],
            [0.0, 1.0],
        ];

        assert!(Triangulator::is_convex_polygon(&square));
    }
}
