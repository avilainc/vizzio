//! Clipping de polígonos (Sutherland-Hodgman) (Rust puro)

/// Clipping de polígonos
pub struct Clipper;

impl Clipper {
    /// Clipar polígono contra retângulo (Sutherland-Hodgman)
    pub fn clip_polygon_rect(
        polygon: &[[f64; 2]],
        rect_min: [f64; 2],
        rect_max: [f64; 2],
    ) -> Vec<[f64; 2]> {
        if polygon.is_empty() {
            return Vec::new();
        }

        let mut output = polygon.to_vec();

        // Clipar contra cada aresta do retângulo
        // Esquerda
        output = Self::clip_against_edge(&output, rect_min, [rect_min[0], rect_max[1]], true);

        // Direita
        output = Self::clip_against_edge(&output, [rect_max[0], rect_max[1]], rect_max, true);

        // Baixo
        output = Self::clip_against_edge(&output, [rect_max[0], rect_min[1]], rect_min, false);

        // Cima
        output = Self::clip_against_edge(&output, rect_max, [rect_min[0], rect_max[1]], false);

        output
    }

    fn clip_against_edge(
        polygon: &[[f64; 2]],
        edge_start: [f64; 2],
        edge_end: [f64; 2],
        left_side: bool,
    ) -> Vec<[f64; 2]> {
        if polygon.is_empty() {
            return Vec::new();
        }

        let mut output = Vec::new();

        for i in 0..polygon.len() {
            let curr = polygon[i];
            let next = polygon[(i + 1) % polygon.len()];

            let curr_inside = Self::is_inside(curr, edge_start, edge_end, left_side);
            let next_inside = Self::is_inside(next, edge_start, edge_end, left_side);

            if curr_inside {
                output.push(curr);

                if !next_inside {
                    if let Some(intersection) = Self::line_intersection(curr, next, edge_start, edge_end) {
                        output.push(intersection);
                    }
                }
            } else if next_inside {
                if let Some(intersection) = Self::line_intersection(curr, next, edge_start, edge_end) {
                    output.push(intersection);
                }
            }
        }

        output
    }

    fn is_inside(point: [f64; 2], edge_start: [f64; 2], edge_end: [f64; 2], left_side: bool) -> bool {
        let cross = (edge_end[0] - edge_start[0]) * (point[1] - edge_start[1]) -
                    (edge_end[1] - edge_start[1]) * (point[0] - edge_start[0]);

        if left_side {
            cross >= 0.0
        } else {
            cross <= 0.0
        }
    }

    fn line_intersection(
        p1: [f64; 2],
        p2: [f64; 2],
        p3: [f64; 2],
        p4: [f64; 2],
    ) -> Option<[f64; 2]> {
        let x1 = p1[0]; let y1 = p1[1];
        let x2 = p2[0]; let y2 = p2[1];
        let x3 = p3[0]; let y3 = p3[1];
        let x4 = p4[0]; let y4 = p4[1];

        let denom = (x1 - x2) * (y3 - y4) - (y1 - y2) * (x3 - x4);

        if denom.abs() < 1e-10 {
            return None; // Linhas paralelas
        }

        let t = ((x1 - x3) * (y3 - y4) - (y1 - y3) * (x3 - x4)) / denom;

        Some([
            x1 + t * (x2 - x1),
            y1 + t * (y2 - y1),
        ])
    }

    /// Clipar linha contra retângulo (Cohen-Sutherland)
    pub fn clip_line_rect(
        p1: [f64; 2],
        p2: [f64; 2],
        rect_min: [f64; 2],
        rect_max: [f64; 2],
    ) -> Option<([f64; 2], [f64; 2])> {
        const INSIDE: u8 = 0; // 0000
        const LEFT: u8 = 1;   // 0001
        const RIGHT: u8 = 2;  // 0010
        const BOTTOM: u8 = 4; // 0100
        const TOP: u8 = 8;    // 1000

        let compute_code = |x: f64, y: f64| -> u8 {
            let mut code = INSIDE;
            if x < rect_min[0] {
                code |= LEFT;
            } else if x > rect_max[0] {
                code |= RIGHT;
            }
            if y < rect_min[1] {
                code |= BOTTOM;
            } else if y > rect_max[1] {
                code |= TOP;
            }
            code
        };

        let mut x1 = p1[0];
        let mut y1 = p1[1];
        let mut x2 = p2[0];
        let mut y2 = p2[1];

        let mut code1 = compute_code(x1, y1);
        let mut code2 = compute_code(x2, y2);

        loop {
            if (code1 | code2) == 0 {
                // Ambos dentro
                return Some(([x1, y1], [x2, y2]));
            } else if (code1 & code2) != 0 {
                // Ambos fora no mesmo lado
                return None;
            }

            // Clipar contra uma aresta
            let code_out = if code1 != 0 { code1 } else { code2 };

            let x: f64;
            let y: f64;

            if (code_out & TOP) != 0 {
                x = x1 + (x2 - x1) * (rect_max[1] - y1) / (y2 - y1);
                y = rect_max[1];
            } else if (code_out & BOTTOM) != 0 {
                x = x1 + (x2 - x1) * (rect_min[1] - y1) / (y2 - y1);
                y = rect_min[1];
            } else if (code_out & RIGHT) != 0 {
                y = y1 + (y2 - y1) * (rect_max[0] - x1) / (x2 - x1);
                x = rect_max[0];
            } else {
                y = y1 + (y2 - y1) * (rect_min[0] - x1) / (x2 - x1);
                x = rect_min[0];
            }

            if code_out == code1 {
                x1 = x;
                y1 = y;
                code1 = compute_code(x1, y1);
            } else {
                x2 = x;
                y2 = y;
                code2 = compute_code(x2, y2);
            }
        }
    }

    /// Clipar ponto contra retângulo
    pub fn point_in_rect(point: [f64; 2], rect_min: [f64; 2], rect_max: [f64; 2]) -> bool {
        point[0] >= rect_min[0] && point[0] <= rect_max[0] &&
        point[1] >= rect_min[1] && point[1] <= rect_max[1]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clip_polygon() {
        let polygon = vec![
            [-1.0, -1.0],
            [3.0, -1.0],
            [3.0, 3.0],
            [-1.0, 3.0],
        ];

        let clipped = Clipper::clip_polygon_rect(&polygon, [0.0, 0.0], [2.0, 2.0]);

        // Deve resultar em quadrado [0,0] a [2,2]
        assert!(clipped.len() >= 4);
    }

    #[test]
    fn test_clip_line() {
        let p1 = [-1.0, 0.5];
        let p2 = [3.0, 0.5];

        let result = Clipper::clip_line_rect(p1, p2, [0.0, 0.0], [2.0, 2.0]);

        assert!(result.is_some());
        let (clipped_p1, clipped_p2) = result.unwrap();

        // Linha deve ser clipada para [0, 0.5] a [2, 0.5]
        assert!((clipped_p1[0] - 0.0).abs() < 1e-10);
        assert!((clipped_p2[0] - 2.0).abs() < 1e-10);
    }

    #[test]
    fn test_point_in_rect() {
        assert!(Clipper::point_in_rect([1.0, 1.0], [0.0, 0.0], [2.0, 2.0]));
        assert!(!Clipper::point_in_rect([3.0, 3.0], [0.0, 0.0], [2.0, 2.0]));
    }
}
