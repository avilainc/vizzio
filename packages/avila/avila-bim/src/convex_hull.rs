//! Convex Hull algorithms (Rust puro)

/// Convex Hull 2D (Graham Scan)
pub struct ConvexHull;

impl ConvexHull {
    /// Calcular convex hull 2D usando Graham Scan
    pub fn compute_2d(points: &[[f64; 2]]) -> Vec<[f64; 2]> {
        if points.len() < 3 {
            return points.to_vec();
        }

        let mut sorted_points = points.to_vec();

        // Encontrar ponto mais baixo (menor y, depois menor x)
        let mut min_idx = 0;
        for i in 1..sorted_points.len() {
            if sorted_points[i][1] < sorted_points[min_idx][1] ||
               (sorted_points[i][1] == sorted_points[min_idx][1] && 
                sorted_points[i][0] < sorted_points[min_idx][0]) {
                min_idx = i;
            }
        }

        sorted_points.swap(0, min_idx);
        let pivot = sorted_points[0];

        // Ordenar por ângulo polar
        sorted_points[1..].sort_by(|a, b| {
            let angle_a = Self::polar_angle(pivot, *a);
            let angle_b = Self::polar_angle(pivot, *b);
            angle_a.partial_cmp(&angle_b).unwrap()
        });

        let mut hull = Vec::new();
        hull.push(sorted_points[0]);
        hull.push(sorted_points[1]);

        for i in 2..sorted_points.len() {
            while hull.len() > 1 && 
                  Self::cross_product_sign(hull[hull.len() - 2], hull[hull.len() - 1], sorted_points[i]) <= 0.0 {
                hull.pop();
            }
            hull.push(sorted_points[i]);
        }

        hull
    }

    /// Calcular ângulo polar em relação ao pivot
    fn polar_angle(pivot: [f64; 2], point: [f64; 2]) -> f64 {
        let dx = point[0] - pivot[0];
        let dy = point[1] - pivot[1];
        dy.atan2(dx)
    }

    /// Produto vetorial 2D (determina orientação)
    fn cross_product_sign(o: [f64; 2], a: [f64; 2], b: [f64; 2]) -> f64 {
        (a[0] - o[0]) * (b[1] - o[1]) - (a[1] - o[1]) * (b[0] - o[0])
    }

    /// Convex Hull 3D simplificado (Gift Wrapping)
    pub fn compute_3d_simplified(points: &[[f64; 3]]) -> Vec<Vec<usize>> {
        if points.len() < 4 {
            return Vec::new();
        }

        let mut faces = Vec::new();

        // Encontrar ponto mais baixo
        let mut min_idx = 0;
        for i in 1..points.len() {
            if points[i][2] < points[min_idx][2] {
                min_idx = i;
            }
        }

        // Iniciar com triângulo base
        // TODO: Implementação completa do Quickhull ou Gift Wrapping
        // Por ora, retornar estrutura vazia
        
        faces
    }

    /// Verificar se ponto está dentro do convex hull 2D
    pub fn point_inside_hull_2d(point: [f64; 2], hull: &[[f64; 2]]) -> bool {
        if hull.len() < 3 {
            return false;
        }

        for i in 0..hull.len() {
            let j = (i + 1) % hull.len();
            let cross = Self::cross_product_sign(hull[i], hull[j], point);
            
            if cross < 0.0 {
                return false;
            }
        }

        true
    }

    /// Calcular área do convex hull 2D
    pub fn hull_area_2d(hull: &[[f64; 2]]) -> f64 {
        if hull.len() < 3 {
            return 0.0;
        }

        let mut area = 0.0;
        for i in 0..hull.len() {
            let j = (i + 1) % hull.len();
            area += hull[i][0] * hull[j][1];
            area -= hull[j][0] * hull[i][1];
        }

        (area / 2.0).abs()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convex_hull_2d() {
        let points = vec![
            [0.0, 0.0],
            [1.0, 0.0],
            [0.5, 0.5], // Ponto interior
            [1.0, 1.0],
            [0.0, 1.0],
        ];

        let hull = ConvexHull::compute_2d(&points);
        
        // Hull deve ter 4 pontos (os cantos do quadrado)
        assert_eq!(hull.len(), 4);
    }

    #[test]
    fn test_point_inside_hull() {
        let hull = vec![
            [0.0, 0.0],
            [2.0, 0.0],
            [2.0, 2.0],
            [0.0, 2.0],
        ];

        assert!(ConvexHull::point_inside_hull_2d([1.0, 1.0], &hull));
        assert!(!ConvexHull::point_inside_hull_2d([3.0, 3.0], &hull));
    }

    #[test]
    fn test_hull_area() {
        let hull = vec![
            [0.0, 0.0],
            [2.0, 0.0],
            [2.0, 2.0],
            [0.0, 2.0],
        ];

        let area = ConvexHull::hull_area_2d(&hull);
        assert!((area - 4.0).abs() < 1e-10);
    }
}
