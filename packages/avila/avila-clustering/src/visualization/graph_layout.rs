//! Layout de grafos para visualização de clustering baseado em grafos

use ndarray::{Array1, Array2};
use std::collections::HashMap;

/// Algoritmo de layout
#[derive(Debug, Clone, Copy)]
pub enum LayoutAlgorithm {
    ForceDirected,
    Circular,
    Spring,
}

/// GraphLayoutEngine - calcula posições de nós para visualização
pub struct GraphLayoutEngine {
    algorithm: LayoutAlgorithm,
    iterations: usize,
}

impl GraphLayoutEngine {
    pub fn new(algorithm: LayoutAlgorithm) -> Self {
        Self {
            algorithm,
            iterations: 100,
        }
    }

    pub fn with_iterations(mut self, iterations: usize) -> Self {
        self.iterations = iterations;
        self
    }

    /// Calcula layout a partir de matriz de adjacência
    pub fn compute_layout(
        &self,
        adjacency: &Array2<f64>,
    ) -> Result<Array2<f64>, String> {
        match self.algorithm {
            LayoutAlgorithm::ForceDirected => self.force_directed_layout(adjacency),
            LayoutAlgorithm::Circular => self.circular_layout(adjacency),
            LayoutAlgorithm::Spring => self.spring_layout(adjacency),
        }
    }

    // Implementações de algoritmos

    fn force_directed_layout(&self, adjacency: &Array2<f64>) -> Result<Array2<f64>, String> {
        let n_nodes = adjacency.nrows();
        if n_nodes != adjacency.ncols() {
            return Err("Matriz de adjacência deve ser quadrada".to_string());
        }

        // Inicializa posições aleatoriamente
        let mut positions = Array2::zeros((n_nodes, 2));
        for i in 0..n_nodes {
            positions[[i, 0]] = (i as f64 * 17.0).sin();
            positions[[i, 1]] = (i as f64 * 13.0).cos();
        }

        let k = (1.0 / (n_nodes as f64).sqrt()).max(0.1); // Distância ideal
        let temperature_decay = 0.95;
        let mut temperature = 1.0;

        for _ in 0..self.iterations {
            let mut forces = Array2::zeros((n_nodes, 2));

            // Forças repulsivas (todos os pares)
            for i in 0..n_nodes {
                for j in (i + 1)..n_nodes {
                    let dx = positions[[i, 0]] - positions[[j, 0]];
                    let dy = positions[[i, 1]] - positions[[j, 1]];
                    let dist = (dx * dx + dy * dy).sqrt().max(0.01);

                    let repulsion = k * k / dist;
                    let fx = (dx / dist) * repulsion;
                    let fy = (dy / dist) * repulsion;

                    forces[[i, 0]] += fx;
                    forces[[i, 1]] += fy;
                    forces[[j, 0]] -= fx;
                    forces[[j, 1]] -= fy;
                }
            }

            // Forças atrativas (arestas)
            for i in 0..n_nodes {
                for j in 0..n_nodes {
                    if adjacency[[i, j]] > 0.0 {
                        let dx = positions[[i, 0]] - positions[[j, 0]];
                        let dy = positions[[i, 1]] - positions[[j, 1]];
                        let dist = (dx * dx + dy * dy).sqrt().max(0.01);

                        let attraction = dist * dist / k * adjacency[[i, j]];
                        let fx = (dx / dist) * attraction;
                        let fy = (dy / dist) * attraction;

                        forces[[i, 0]] -= fx;
                        forces[[i, 1]] -= fy;
                    }
                }
            }

            // Atualiza posições
            for i in 0..n_nodes {
                let fx = forces[[i, 0]];
                let fy = forces[[i, 1]];
                let force_mag = (fx * fx + fy * fy).sqrt().max(0.01);

                let displacement = temperature.min(force_mag);
                positions[[i, 0]] += (fx / force_mag) * displacement;
                positions[[i, 1]] += (fy / force_mag) * displacement;
            }

            temperature *= temperature_decay;
        }

        Ok(positions)
    }

    fn circular_layout(&self, adjacency: &Array2<f64>) -> Result<Array2<f64>, String> {
        let n_nodes = adjacency.nrows();
        let mut positions = Array2::zeros((n_nodes, 2));

        let angle_step = 2.0 * std::f64::consts::PI / n_nodes as f64;

        for i in 0..n_nodes {
            let angle = i as f64 * angle_step;
            positions[[i, 0]] = angle.cos();
            positions[[i, 1]] = angle.sin();
        }

        Ok(positions)
    }

    fn spring_layout(&self, adjacency: &Array2<f64>) -> Result<Array2<f64>, String> {
        // Similar ao force-directed, mas com parâmetros diferentes
        let n_nodes = adjacency.nrows();
        let mut positions = Array2::zeros((n_nodes, 2));

        // Inicialização em grid
        let grid_size = (n_nodes as f64).sqrt().ceil() as usize;
        for i in 0..n_nodes {
            positions[[i, 0]] = (i % grid_size) as f64;
            positions[[i, 1]] = (i / grid_size) as f64;
        }

        let k = 1.0;
        let c_rep = 1.0;
        let c_spring = 1.0;

        for _ in 0..self.iterations {
            let mut forces = Array2::zeros((n_nodes, 2));

            // Forças de mola (arestas)
            for i in 0..n_nodes {
                for j in 0..n_nodes {
                    if i != j && adjacency[[i, j]] > 0.0 {
                        let dx = positions[[j, 0]] - positions[[i, 0]];
                        let dy = positions[[j, 1]] - positions[[i, 1]];
                        let dist = (dx * dx + dy * dy).sqrt().max(0.01);

                        let spring_force = c_spring * (dist - k);
                        forces[[i, 0]] += (dx / dist) * spring_force * adjacency[[i, j]];
                        forces[[i, 1]] += (dy / dist) * spring_force * adjacency[[i, j]];
                    }
                }
            }

            // Forças repulsivas
            for i in 0..n_nodes {
                for j in 0..n_nodes {
                    if i != j {
                        let dx = positions[[j, 0]] - positions[[i, 0]];
                        let dy = positions[[j, 1]] - positions[[i, 1]];
                        let dist_sq = (dx * dx + dy * dy).max(0.01);

                        forces[[i, 0]] -= c_rep * dx / dist_sq;
                        forces[[i, 1]] -= c_rep * dy / dist_sq;
                    }
                }
            }

            // Atualiza posições
            let damping = 0.9;
            for i in 0..n_nodes {
                positions[[i, 0]] += forces[[i, 0]] * 0.01 * damping;
                positions[[i, 1]] += forces[[i, 1]] * 0.01 * damping;
            }
        }

        Ok(positions)
    }
}

/// Estrutura de dados de grafo para visualização
#[derive(Debug, Clone)]
pub struct GraphVisualization {
    pub node_positions: Array2<f64>,
    pub edges: Vec<(usize, usize, f64)>, // (from, to, weight)
    pub node_labels: Option<Vec<String>>,
    pub node_sizes: Option<Vec<f64>>,
}

impl GraphVisualization {
    pub fn new(node_positions: Array2<f64>) -> Self {
        Self {
            node_positions,
            edges: Vec::new(),
            node_labels: None,
            node_sizes: None,
        }
    }

    pub fn with_edges(mut self, edges: Vec<(usize, usize, f64)>) -> Self {
        self.edges = edges;
        self
    }

    pub fn with_labels(mut self, labels: Vec<String>) -> Self {
        self.node_labels = Some(labels);
        self
    }

    pub fn with_sizes(mut self, sizes: Vec<f64>) -> Self {
        self.node_sizes = Some(sizes);
        self
    }

    /// Exporta para formato JSON
    pub fn to_json(&self) -> String {
        let mut nodes_json = Vec::new();

        for i in 0..self.node_positions.nrows() {
            let label = self.node_labels
                .as_ref()
                .and_then(|l| l.get(i))
                .cloned()
                .unwrap_or_else(|| format!("node_{}", i));

            let size = self.node_sizes
                .as_ref()
                .and_then(|s| s.get(i))
                .copied()
                .unwrap_or(1.0);

            nodes_json.push(format!(
                r#"{{"id": {}, "label": "{}", "x": {:.4}, "y": {:.4}, "size": {:.4}}}"#,
                i, label, self.node_positions[[i, 0]], self.node_positions[[i, 1]], size
            ));
        }

        let edges_json: Vec<String> = self.edges
            .iter()
            .map(|(from, to, weight)| {
                format!(
                    r#"{{"source": {}, "target": {}, "weight": {:.4}}}"#,
                    from, to, weight
                )
            })
            .collect();

        format!(
            r#"{{"nodes": [{}], "edges": [{}]}}"#,
            nodes_json.join(", "),
            edges_json.join(", ")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ndarray::array;

    #[test]
    fn test_circular_layout() {
        let adj = array![
            [0.0, 1.0, 0.0],
            [1.0, 0.0, 1.0],
            [0.0, 1.0, 0.0],
        ];

        let engine = GraphLayoutEngine::new(LayoutAlgorithm::Circular);
        let positions = engine.compute_layout(&adj).unwrap();

        assert_eq!(positions.nrows(), 3);
        assert_eq!(positions.ncols(), 2);
    }
}
