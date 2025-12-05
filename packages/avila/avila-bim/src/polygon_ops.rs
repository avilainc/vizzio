//! Operações avançadas em polígonos 2D (Rust puro)

/// Operações em polígonos 2D
pub struct PolygonOps;

impl PolygonOps {
    /// Calcular área de polígono (shoelace formula)
    pub fn area(polygon: &[[f64; 2]]) -> f64 {
        if polygon.len() < 3 {
            return 0.0;
        }

        let mut area = 0.0;
        for i in 0..polygon.len() {
            let j = (i + 1) % polygon.len();
            area += polygon[i][0] * polygon[j][1];
            area -= polygon[j][0] * polygon[i][1];
        }

        (area / 2.0).abs()
    }

    /// Calcular perímetro
    pub fn perimeter(polygon: &[[f64; 2]]) -> f64 {
        let mut perimeter = 0.0;

        for i in 0..polygon.len() {
            let j = (i + 1) % polygon.len();
            let dx = polygon[j][0] - polygon[i][0];
            let dy = polygon[j][1] - polygon[i][1];
            perimeter += (dx * dx + dy * dy).sqrt();
        }

        perimeter
    }

    /// Calcular centroide (centro de massa)
    pub fn centroid(polygon: &[[f64; 2]]) -> [f64; 2] {
        if polygon.is_empty() {
            return [0.0, 0.0];
        }

        let mut cx = 0.0;
        let mut cy = 0.0;
        let mut area = 0.0;

        for i in 0..polygon.len() {
            let j = (i + 1) % polygon.len();
            let cross = polygon[i][0] * polygon[j][1] - polygon[j][0] * polygon[i][1];

            cx += (polygon[i][0] + polygon[j][0]) * cross;
            cy += (polygon[i][1] + polygon[j][1]) * cross;
            area += cross;
        }

        area *= 0.5;

        if area.abs() < 1e-10 {
            // Fallback: média dos pontos
            let mut sum_x = 0.0;
            let mut sum_y = 0.0;
            for p in polygon {
                sum_x += p[0];
                sum_y += p[1];
            }
            return [sum_x / polygon.len() as f64, sum_y / polygon.len() as f64];
        }

        [cx / (6.0 * area), cy / (6.0 * area)]
    }

    /// Verificar se polígono é convexo
    pub fn is_convex(polygon: &[[f64; 2]]) -> bool {
        if polygon.len() < 3 {
            return false;
        }

        let mut sign = 0.0;

        for i in 0..polygon.len() {
            let p1 = polygon[i];
            let p2 = polygon[(i + 1) % polygon.len()];
            let p3 = polygon[(i + 2) % polygon.len()];

            let cross = Self::cross_product_2d(
                [p2[0] - p1[0], p2[1] - p1[1]],
                [p3[0] - p2[0], p3[1] - p2[1]],
            );

            if cross.abs() > 1e-10 {
                if sign == 0.0 {
                    sign = cross;
                } else if sign * cross < 0.0 {
                    return false;
                }
            }
        }

        true
    }

    fn cross_product_2d(a: [f64; 2], b: [f64; 2]) -> f64 {
        a[0] * b[1] - a[1] * b[0]
    }

    /// Verificar se ponto está dentro do polígono (ray casting)
    pub fn point_inside(point: [f64; 2], polygon: &[[f64; 2]]) -> bool {
        let mut inside = false;
        let n = polygon.len();

        for i in 0..n {
            let j = (i + 1) % n;
            let vi = polygon[i];
            let vj = polygon[j];

            if ((vi[1] > point[1]) != (vj[1] > point[1])) &&
               (point[0] < (vj[0] - vi[0]) * (point[1] - vi[1]) / (vj[1] - vi[1]) + vi[0]) {
                inside = !inside;
            }
        }

        inside
    }

    /// Simplificar polígono (Douglas-Peucker)
    pub fn simplify(polygon: &[[f64; 2]], epsilon: f64) -> Vec<[f64; 2]> {
        if polygon.len() < 3 {
            return polygon.to_vec();
        }

        let mut result = Vec::new();
        Self::douglas_peucker(polygon, epsilon, &mut result);
        result
    }

    fn douglas_peucker(points: &[[f64; 2]], epsilon: f64, result: &mut Vec<[f64; 2]>) {
        if points.len() < 2 {
            return;
        }

        if points.len() == 2 {
            result.push(points[0]);
            result.push(points[1]);
            return;
        }

        // Encontrar ponto mais distante da linha
        let mut max_dist = 0.0;
        let mut max_index = 0;

        for i in 1..points.len() - 1 {
            let dist = Self::point_line_distance(points[i], points[0], points[points.len() - 1]);
            if dist > max_dist {
                max_dist = dist;
                max_index = i;
            }
        }

        // Se distância máxima > epsilon, dividir recursivamente
        if max_dist > epsilon {
            let mut left = Vec::new();
            Self::douglas_peucker(&points[..=max_index], epsilon, &mut left);

            let mut right = Vec::new();
            Self::douglas_peucker(&points[max_index..], epsilon, &mut right);

            // Combinar resultados (remover ponto duplicado)
            result.extend(left.iter().take(left.len() - 1));
            result.extend(&right);
        } else {
            result.push(points[0]);
            result.push(points[points.len() - 1]);
        }
    }

    fn point_line_distance(point: [f64; 2], line_start: [f64; 2], line_end: [f64; 2]) -> f64 {
        let dx = line_end[0] - line_start[0];
        let dy = line_end[1] - line_start[1];
        let len_sq = dx * dx + dy * dy;

        if len_sq < 1e-10 {
            let px = point[0] - line_start[0];
            let py = point[1] - line_start[1];
            return (px * px + py * py).sqrt();
        }

        let num = (dy * point[0] - dx * point[1] + line_end[0] * line_start[1] - line_end[1] * line_start[0]).abs();
        num / len_sq.sqrt()
    }

    /// Calcular bounding box
    pub fn bounding_box(polygon: &[[f64; 2]]) -> ([f64; 2], [f64; 2]) {
        if polygon.is_empty() {
            return ([0.0, 0.0], [0.0, 0.0]);
        }

        let mut min = polygon[0];
        let mut max = polygon[0];

        for p in &polygon[1..] {
            min[0] = min[0].min(p[0]);
            min[1] = min[1].min(p[1]);
            max[0] = max[0].max(p[0]);
            max[1] = max[1].max(p[1]);
        }

        (min, max)
    }

    /// Offset de polígono (inset/outset)
    pub fn offset(polygon: &[[f64; 2]], distance: f64) -> Vec<[f64; 2]> {
        if polygon.len() < 3 {
            return polygon.to_vec();
        }

        let mut result = Vec::with_capacity(polygon.len());

        for i in 0..polygon.len() {
            let prev = polygon[(i + polygon.len() - 1) % polygon.len()];
            let curr = polygon[i];
            let next = polygon[(i + 1) % polygon.len()];

            // Vetores das arestas
            let v1 = Self::normalize([curr[0] - prev[0], curr[1] - prev[1]]);
            let v2 = Self::normalize([next[0] - curr[0], next[1] - curr[1]]);

            // Normais (perpendiculares)
            let n1 = [-v1[1], v1[0]];
            let n2 = [-v2[1], v2[0]];

            // Bissetriz
            let bisector = Self::normalize([n1[0] + n2[0], n1[1] + n2[1]]);

            // Calcular offset
            let dot = n1[0] * bisector[0] + n1[1] * bisector[1];
            let offset_distance = if dot.abs() > 1e-10 {
                distance / dot
            } else {
                distance
            };

            result.push([
                curr[0] + bisector[0] * offset_distance,
                curr[1] + bisector[1] * offset_distance,
            ]);
        }

        result
    }

    fn normalize(v: [f64; 2]) -> [f64; 2] {
        let len = (v[0] * v[0] + v[1] * v[1]).sqrt();
        if len > 1e-10 {
            [v[0] / len, v[1] / len]
        } else {
            [0.0, 0.0]
        }
    }

    /// Reverter ordem dos vértices
    pub fn reverse(polygon: &[[f64; 2]]) -> Vec<[f64; 2]> {
        polygon.iter().rev().copied().collect()
    }

    /// Verificar orientação (horário ou anti-horário)
    pub fn is_clockwise(polygon: &[[f64; 2]]) -> bool {
        Self::area(polygon) < 0.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_polygon_area() {
        let square = vec![
            [0.0, 0.0],
            [2.0, 0.0],
            [2.0, 2.0],
            [0.0, 2.0],
        ];

        let area = PolygonOps::area(&square);
        assert!((area - 4.0).abs() < 1e-10);
    }

    #[test]
    fn test_polygon_centroid() {
        let square = vec![
            [0.0, 0.0],
            [2.0, 0.0],
            [2.0, 2.0],
            [0.0, 2.0],
        ];

        let centroid = PolygonOps::centroid(&square);
        assert!((centroid[0] - 1.0).abs() < 1e-10);
        assert!((centroid[1] - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_point_inside() {
        let square = vec![
            [0.0, 0.0],
            [2.0, 0.0],
            [2.0, 2.0],
            [0.0, 2.0],
        ];

        assert!(PolygonOps::point_inside([1.0, 1.0], &square));
        assert!(!PolygonOps::point_inside([3.0, 3.0], &square));
    }

    #[test]
    fn test_is_convex() {
        let square = vec![
            [0.0, 0.0],
            [2.0, 0.0],
            [2.0, 2.0],
            [0.0, 2.0],
        ];

        assert!(PolygonOps::is_convex(&square));
    }
}
