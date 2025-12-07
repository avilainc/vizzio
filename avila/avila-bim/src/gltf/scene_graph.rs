//! glTF Scene graph builder

use crate::bim_core::*;
use std::collections::HashMap;

/// Construtor de scene graph glTF
pub struct SceneGraphBuilder {
    nodes: Vec<GltfNode>,
    root_nodes: Vec<usize>,
    guid_to_node: HashMap<String, usize>,
}

impl SceneGraphBuilder {
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            root_nodes: Vec::new(),
            guid_to_node: HashMap::new(),
        }
    }

    /// Construir a partir da hierarquia espacial
    pub fn from_spatial_structure(&mut self, spatial: &SpatialNode) -> usize {
        let node_idx = self.add_node(GltfNode {
            name: spatial.name.clone(),
            children: Vec::new(),
            mesh_index: None,
            transform: [1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0],
        });

        self.guid_to_node.insert(spatial.guid.as_str().to_string(), node_idx);

        // Processar filhos recursivamente
        for child in &spatial.children {
            let child_idx = self.from_spatial_structure(child);
            self.nodes[node_idx].children.push(child_idx);
        }

        node_idx
    }

    /// Adicionar nó
    pub fn add_node(&mut self, node: GltfNode) -> usize {
        let idx = self.nodes.len();
        self.nodes.push(node);
        idx
    }

    /// Definir raiz
    pub fn set_root(&mut self, node_idx: usize) {
        self.root_nodes.push(node_idx);
    }

    /// Obter nós
    pub fn nodes(&self) -> &[GltfNode] {
        &self.nodes
    }

    /// Obter raízes
    pub fn root_nodes(&self) -> &[usize] {
        &self.root_nodes
    }
}

impl Default for SceneGraphBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// Nó glTF
#[derive(Debug, Clone)]
pub struct GltfNode {
    pub name: String,
    pub children: Vec<usize>,
    pub mesh_index: Option<usize>,
    pub transform: [f64; 16],
}
