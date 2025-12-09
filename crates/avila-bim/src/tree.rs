//! Sistema de árvore hierárquica para modelos IFC
//!
//! Organiza elementos IFC por tipo e permite navegação/filtragem

use alloc::vec::Vec;
use alloc::string::String;
use alloc::collections::BTreeMap;

/// Nó da árvore hierárquica
#[derive(Debug, Clone)]
pub struct TreeNode {
    /// ID único do nó
    pub id: String,
    /// Nome exibido
    pub label: String,
    /// Tipo do nó (group, entity)
    pub node_type: NodeType,
    /// ID da entidade IFC (se for entity)
    pub entity_id: Option<u32>,
    /// Índice da geometria correspondente
    pub geometry_index: Option<usize>,
    /// Filhos deste nó
    pub children: Vec<TreeNode>,
    /// Está expandido na UI
    pub expanded: bool,
    /// Está visível no viewer
    pub visible: bool,
}

/// Tipo de nó
#[derive(Debug, Clone, PartialEq)]
pub enum NodeType {
    /// Grupo (pasta)
    Group,
    /// Entidade individual
    Entity,
}

impl TreeNode {
    /// Cria novo nó grupo
    pub fn new_group(id: String, label: String) -> Self {
        Self {
            id,
            label,
            node_type: NodeType::Group,
            entity_id: None,
            geometry_index: None,
            children: Vec::new(),
            expanded: true,
            visible: true,
        }
    }

    /// Cria novo nó entidade
    pub fn new_entity(
        id: String,
        label: String,
        entity_id: u32,
        geometry_index: usize,
    ) -> Self {
        Self {
            id,
            label,
            node_type: NodeType::Entity,
            entity_id: Some(entity_id),
            geometry_index: Some(geometry_index),
            children: Vec::new(),
            expanded: false,
            visible: true,
        }
    }

    /// Adiciona filho
    pub fn add_child(&mut self, child: TreeNode) {
        self.children.push(child);
    }

    /// Conta total de entidades (recursivo)
    pub fn count_entities(&self) -> usize {
        let mut count = if self.node_type == NodeType::Entity { 1 } else { 0 };
        for child in &self.children {
            count += child.count_entities();
        }
        count
    }

    /// Busca nó por ID (recursivo)
    pub fn find_node_mut(&mut self, id: &str) -> Option<&mut TreeNode> {
        if self.id == id {
            return Some(self);
        }
        for child in &mut self.children {
            if let Some(node) = child.find_node_mut(id) {
                return Some(node);
            }
        }
        None
    }

    /// Coleta todos os índices de geometria visíveis
    pub fn collect_visible_geometry_indices(&self, indices: &mut Vec<usize>) {
        if self.visible {
            if let Some(idx) = self.geometry_index {
                indices.push(idx);
            }
            for child in &self.children {
                child.collect_visible_geometry_indices(indices);
            }
        }
    }

    /// Define visibilidade recursivamente
    pub fn set_visibility_recursive(&mut self, visible: bool) {
        self.visible = visible;
        for child in &mut self.children {
            child.set_visibility_recursive(visible);
        }
    }
}

/// Constrói árvore hierárquica a partir das geometrias
pub fn build_tree_from_geometries(
    geometries: &[crate::IfcGeometry],
) -> TreeNode {
    let mut root = TreeNode::new_group("root".to_string(), "Modelo IFC".to_string());

    // Agrupa por tipo
    let mut groups: BTreeMap<String, Vec<(usize, &crate::IfcGeometry)>> = BTreeMap::new();

    for (idx, geom) in geometries.iter().enumerate() {
        groups
            .entry(geom.entity_type.clone())
            .or_insert_with(Vec::new)
            .push((idx, geom));
    }

    // Cria nós por tipo
    for (entity_type, items) in groups {
        let mut group = TreeNode::new_group(
            format!("group_{}", entity_type),
            format!("{} ({})", entity_type, items.len()),
        );

        for (idx, geom) in items {
            let entity = TreeNode::new_entity(
                format!("entity_{}_{}", entity_type, geom.entity_id),
                format!("#{} - {}", geom.entity_id, entity_type),
                geom.entity_id,
                idx,
            );
            group.add_child(entity);
        }

        root.add_child(group);
    }

    root
}

/// Serializa árvore para JSON
pub fn tree_to_json(node: &TreeNode) -> String {
    let mut json = String::from("{");

    json.push_str(&format!("\"id\":\"{}\"", node.id));
    json.push_str(&format!(",\"label\":\"{}\"", node.label));
    json.push_str(&format!(",\"type\":\"{}\"",
        if node.node_type == NodeType::Group { "group" } else { "entity" }));

    if let Some(entity_id) = node.entity_id {
        json.push_str(&format!(",\"entityId\":{}", entity_id));
    }

    if let Some(geom_idx) = node.geometry_index {
        json.push_str(&format!(",\"geometryIndex\":{}", geom_idx));
    }

    json.push_str(&format!(",\"expanded\":{}", node.expanded));
    json.push_str(&format!(",\"visible\":{}", node.visible));

    if !node.children.is_empty() {
        json.push_str(",\"children\":[");
        for (i, child) in node.children.iter().enumerate() {
            if i > 0 {
                json.push(',');
            }
            json.push_str(&tree_to_json(child));
        }
        json.push(']');
    }

    json.push('}');
    json
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tree_node_creation() {
        let mut root = TreeNode::new_group("root".to_string(), "Root".to_string());
        let child = TreeNode::new_entity("e1".to_string(), "Entity 1".to_string(), 123, 0);
        root.add_child(child);

        assert_eq!(root.children.len(), 1);
        assert_eq!(root.count_entities(), 1);
    }

    #[test]
    fn test_visibility_toggle() {
        let mut root = TreeNode::new_group("root".to_string(), "Root".to_string());
        let child = TreeNode::new_entity("e1".to_string(), "Entity 1".to_string(), 123, 0);
        root.add_child(child);

        root.set_visibility_recursive(false);
        assert!(!root.visible);
        assert!(!root.children[0].visible);
    }
}
