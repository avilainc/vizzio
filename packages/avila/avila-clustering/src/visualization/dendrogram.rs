//! Geração de dados para dendrogramas (clustering hierárquico)

use ndarray::Array1;
use std::collections::HashMap;

/// Estrutura de um nó no dendrograma
#[derive(Debug, Clone)]
pub struct DendrogramNode {
    pub id: usize,
    pub left: Option<Box<DendrogramNode>>,
    pub right: Option<Box<DendrogramNode>>,
    pub distance: f64,
    pub count: usize,
}

impl DendrogramNode {
    pub fn leaf(id: usize) -> Self {
        Self {
            id,
            left: None,
            right: None,
            distance: 0.0,
            count: 1,
        }
    }

    pub fn merge(left: DendrogramNode, right: DendrogramNode, distance: f64, id: usize) -> Self {
        let count = left.count + right.count;
        Self {
            id,
            left: Some(Box::new(left)),
            right: Some(Box::new(right)),
            distance,
            count,
        }
    }

    pub fn is_leaf(&self) -> bool {
        self.left.is_none() && self.right.is_none()
    }
}

/// DendrogramBuilder - constrói estrutura de dendrograma
pub struct DendrogramBuilder {
    root: Option<DendrogramNode>,
}

impl DendrogramBuilder {
    pub fn new() -> Self {
        Self { root: None }
    }

    /// Constrói dendrograma a partir de linkage matrix
    /// linkage: [(cluster_i, cluster_j, distance, size), ...]
    pub fn from_linkage(&mut self, linkage: &[(usize, usize, f64, usize)]) -> Result<(), String> {
        if linkage.is_empty() {
            return Err("Linkage matrix vazia".to_string());
        }

        let n_leaves = linkage.len() + 1;
        let mut nodes: HashMap<usize, DendrogramNode> = HashMap::new();

        // Cria nós folha
        for i in 0..n_leaves {
            nodes.insert(i, DendrogramNode::leaf(i));
        }

        // Constrói árvore bottom-up
        for (merge_idx, &(left_id, right_id, distance, size)) in linkage.iter().enumerate() {
            let new_id = n_leaves + merge_idx;

            let left = nodes.remove(&left_id).ok_or("Nó esquerdo não encontrado")?;
            let right = nodes.remove(&right_id).ok_or("Nó direito não encontrado")?;

            let merged = DendrogramNode::merge(left, right, distance, new_id);
            nodes.insert(new_id, merged);
        }

        // A raiz é o último nó criado
        let root_id = n_leaves + linkage.len() - 1;
        self.root = nodes.remove(&root_id);

        Ok(())
    }

    /// Exporta para formato JSON
    pub fn to_json(&self) -> Result<String, String> {
        let root = self.root.as_ref().ok_or("Dendrograma não construído")?;
        Ok(Self::node_to_json(root))
    }

    fn node_to_json(node: &DendrogramNode) -> String {
        if node.is_leaf() {
            format!(
                r#"{{"id": {}, "type": "leaf", "distance": {:.4}}}"#,
                node.id, node.distance
            )
        } else {
            let left_json = Self::node_to_json(node.left.as_ref().unwrap());
            let right_json = Self::node_to_json(node.right.as_ref().unwrap());

            format!(
                r#"{{"id": {}, "type": "internal", "distance": {:.4}, "count": {}, "left": {}, "right": {}}}"#,
                node.id, node.distance, node.count, left_json, right_json
            )
        }
    }

    /// Gera lista de coordenadas para plotagem
    pub fn to_coordinates(&self) -> Result<Vec<DendrogramCoordinate>, String> {
        let root = self.root.as_ref().ok_or("Dendrograma não construído")?;
        let mut coords = Vec::new();
        let mut x_pos = 0.0;

        Self::generate_coordinates(root, &mut coords, &mut x_pos, 0.0);

        Ok(coords)
    }

    fn generate_coordinates(
        node: &DendrogramNode,
        coords: &mut Vec<DendrogramCoordinate>,
        x_pos: &mut f64,
        y_offset: f64,
    ) -> f64 {
        if node.is_leaf() {
            let x = *x_pos;
            *x_pos += 1.0;

            coords.push(DendrogramCoordinate {
                x,
                y: y_offset,
                node_id: node.id,
                is_leaf: true,
            });

            return x;
        }

        let left_x = Self::generate_coordinates(
            node.left.as_ref().unwrap(),
            coords,
            x_pos,
            y_offset + node.distance,
        );

        let right_x = Self::generate_coordinates(
            node.right.as_ref().unwrap(),
            coords,
            x_pos,
            y_offset + node.distance,
        );

        let x = (left_x + right_x) / 2.0;
        let y = y_offset;

        coords.push(DendrogramCoordinate {
            x,
            y,
            node_id: node.id,
            is_leaf: false,
        });

        x
    }

    /// Corta o dendrograma em n clusters
    pub fn cut(&self, n_clusters: usize) -> Result<Array1<usize>, String> {
        let root = self.root.as_ref().ok_or("Dendrograma não construído")?;

        // Encontra limiar de distância para n clusters
        let mut distances = Vec::new();
        Self::collect_distances(root, &mut distances);
        distances.sort_by(|a, b| b.partial_cmp(a).unwrap());

        if n_clusters == 0 || n_clusters > distances.len() + 1 {
            return Err("Número de clusters inválido".to_string());
        }

        let threshold = if n_clusters == 1 {
            f64::INFINITY
        } else {
            distances[n_clusters - 2]
        };

        // Atribui labels
        let mut labels = Vec::new();
        let mut cluster_id = 0;
        Self::assign_labels(root, threshold, &mut labels, &mut cluster_id);

        Ok(Array1::from_vec(labels))
    }

    fn collect_distances(node: &DendrogramNode, distances: &mut Vec<f64>) {
        if !node.is_leaf() {
            distances.push(node.distance);
            Self::collect_distances(node.left.as_ref().unwrap(), distances);
            Self::collect_distances(node.right.as_ref().unwrap(), distances);
        }
    }

    fn assign_labels(
        node: &DendrogramNode,
        threshold: f64,
        labels: &mut Vec<usize>,
        cluster_id: &mut usize,
    ) {
        if node.is_leaf() {
            labels.push(*cluster_id);
        } else if node.distance > threshold {
            *cluster_id += 1;
            Self::assign_labels(node.left.as_ref().unwrap(), threshold, labels, cluster_id);
            *cluster_id += 1;
            Self::assign_labels(node.right.as_ref().unwrap(), threshold, labels, cluster_id);
        } else {
            Self::assign_labels(node.left.as_ref().unwrap(), threshold, labels, cluster_id);
            Self::assign_labels(node.right.as_ref().unwrap(), threshold, labels, cluster_id);
        }
    }
}

impl Default for DendrogramBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// Coordenada de um nó no dendrograma para plotagem
#[derive(Debug, Clone)]
pub struct DendrogramCoordinate {
    pub x: f64,
    pub y: f64,
    pub node_id: usize,
    pub is_leaf: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dendrogram_building() {
        let linkage = vec![
            (0, 1, 0.5, 2),
            (2, 3, 0.8, 2),
            (4, 5, 1.2, 4),
        ];

        let mut builder = DendrogramBuilder::new();
        builder.from_linkage(&linkage).unwrap();

        let coords = builder.to_coordinates().unwrap();
        assert!(!coords.is_empty());
    }
}
