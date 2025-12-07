//! Spatial partitioning with K-D Tree (Rust puro)

/// K-D Tree para busca espacial eficiente
pub struct KdTree {
    root: Option<Box<KdNode>>,
    dimension: usize,
}

struct KdNode {
    point: Vec<f64>,
    data_index: usize,
    left: Option<Box<KdNode>>,
    right: Option<Box<KdNode>>,
    axis: usize,
}

impl KdTree {
    /// Criar K-D Tree vazia
    pub fn new(dimension: usize) -> Self {
        Self {
            root: None,
            dimension,
        }
    }

    /// Construir K-D Tree a partir de pontos
    pub fn build(points: &[Vec<f64>]) -> Self {
        if points.is_empty() {
            return Self::new(3);
        }

        let dimension = points[0].len();
        let mut tree = Self::new(dimension);

        let mut indexed_points: Vec<(Vec<f64>, usize)> = points
            .iter()
            .enumerate()
            .map(|(i, p)| (p.clone(), i))
            .collect();

        tree.root = Self::build_recursive(&mut indexed_points, 0, dimension);
        tree
    }

    fn build_recursive(
        points: &mut [(Vec<f64>, usize)],
        depth: usize,
        dimension: usize,
    ) -> Option<Box<KdNode>> {
        if points.is_empty() {
            return None;
        }

        let axis = depth % dimension;

        // Ordenar pontos pelo eixo atual
        points.sort_by(|a, b| a.0[axis].partial_cmp(&b.0[axis]).unwrap());

        let median = points.len() / 2;
        let (point, data_index) = points[median].clone();

        let left = Self::build_recursive(&mut points[..median], depth + 1, dimension);
        let right = Self::build_recursive(&mut points[median + 1..], depth + 1, dimension);

        Some(Box::new(KdNode {
            point,
            data_index,
            left,
            right,
            axis,
        }))
    }

    /// Buscar K vizinhos mais próximos
    pub fn k_nearest(&self, query: &[f64], k: usize) -> Vec<(usize, f64)> {
        let mut nearest = Vec::new();

        if let Some(ref root) = self.root {
            Self::k_nearest_recursive(root, query, k, &mut nearest);
        }

        nearest.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
        nearest.truncate(k);
        nearest
    }

    fn k_nearest_recursive(
        node: &KdNode,
        query: &[f64],
        k: usize,
        nearest: &mut Vec<(usize, f64)>,
    ) {
        let dist = Self::distance_squared(&node.point, query);

        // Adicionar ponto atual se necessário
        if nearest.len() < k {
            nearest.push((node.data_index, dist));
            nearest.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
        } else if dist < nearest[nearest.len() - 1].1 {
            nearest[nearest.len() - 1] = (node.data_index, dist);
            nearest.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
        }

        let axis = node.axis;
        let diff = query[axis] - node.point[axis];

        // Decidir qual lado explorar primeiro
        let (primary, secondary) = if diff < 0.0 {
            (&node.left, &node.right)
        } else {
            (&node.right, &node.left)
        };

        if let Some(ref child) = primary {
            Self::k_nearest_recursive(child, query, k, nearest);
        }

        // Verificar se precisa explorar outro lado
        if nearest.len() < k || diff * diff < nearest[nearest.len() - 1].1 {
            if let Some(ref child) = secondary {
                Self::k_nearest_recursive(child, query, k, nearest);
            }
        }
    }

    /// Buscar pontos dentro de raio
    pub fn radius_search(&self, query: &[f64], radius: f64) -> Vec<(usize, f64)> {
        let mut results = Vec::new();
        let radius_sq = radius * radius;

        if let Some(ref root) = self.root {
            Self::radius_search_recursive(root, query, radius_sq, &mut results);
        }

        results
    }

    fn radius_search_recursive(
        node: &KdNode,
        query: &[f64],
        radius_sq: f64,
        results: &mut Vec<(usize, f64)>,
    ) {
        let dist_sq = Self::distance_squared(&node.point, query);

        if dist_sq <= radius_sq {
            results.push((node.data_index, dist_sq.sqrt()));
        }

        let axis = node.axis;
        let diff = query[axis] - node.point[axis];

        let (primary, secondary) = if diff < 0.0 {
            (&node.left, &node.right)
        } else {
            (&node.right, &node.left)
        };

        if let Some(ref child) = primary {
            Self::radius_search_recursive(child, query, radius_sq, results);
        }

        if diff * diff <= radius_sq {
            if let Some(ref child) = secondary {
                Self::radius_search_recursive(child, query, radius_sq, results);
            }
        }
    }

    fn distance_squared(a: &[f64], b: &[f64]) -> f64 {
        a.iter()
            .zip(b.iter())
            .map(|(x, y)| (x - y) * (x - y))
            .sum()
    }

    /// Contar pontos na árvore
    pub fn size(&self) -> usize {
        Self::count_nodes(&self.root)
    }

    fn count_nodes(node: &Option<Box<KdNode>>) -> usize {
        match node {
            None => 0,
            Some(n) => 1 + Self::count_nodes(&n.left) + Self::count_nodes(&n.right),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kdtree_build() {
        let points = vec![
            vec![1.0, 2.0, 3.0],
            vec![4.0, 5.0, 6.0],
            vec![7.0, 8.0, 9.0],
        ];

        let tree = KdTree::build(&points);
        assert_eq!(tree.size(), 3);
    }

    #[test]
    fn test_kdtree_nearest() {
        let points = vec![
            vec![0.0, 0.0, 0.0],
            vec![1.0, 0.0, 0.0],
            vec![0.0, 1.0, 0.0],
            vec![5.0, 5.0, 5.0],
        ];

        let tree = KdTree::build(&points);
        let query = vec![0.5, 0.0, 0.0];

        let nearest = tree.k_nearest(&query, 2);
        assert_eq!(nearest.len(), 2);

        // Primeiro deve ser index 1 (1,0,0) ou index 0 (0,0,0)
        assert!(nearest[0].0 <= 1);
    }

    #[test]
    fn test_kdtree_radius_search() {
        let points = vec![
            vec![0.0, 0.0, 0.0],
            vec![1.0, 0.0, 0.0],
            vec![0.0, 1.0, 0.0],
            vec![10.0, 10.0, 10.0],
        ];

        let tree = KdTree::build(&points);
        let query = vec![0.0, 0.0, 0.0];

        let results = tree.radius_search(&query, 1.5);

        // Deve encontrar 3 pontos próximos (indices 0, 1, 2)
        assert_eq!(results.len(), 3);
    }
}
