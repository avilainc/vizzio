//! Attribute Table

use std::collections::{HashMap, HashSet};

pub struct AttributeTable {
    pub layer_id: String,
    pub features: Vec<Feature>,
    pub selected_features: HashSet<usize>,
    pub filter: Option<String>,
}

#[derive(Debug, Clone)]
pub struct Feature {
    pub id: String,
    pub geometry_type: String,
    pub attributes: HashMap<String, String>,
}

impl Feature {
    pub fn new(id: String, geometry_type: String) -> Self {
        Self {
            id,
            geometry_type,
            attributes: HashMap::new(),
        }
    }

    pub fn with_attributes(id: String, geometry_type: String, attributes: HashMap<String, String>) -> Self {
        Self {
            id,
            geometry_type,
            attributes,
        }
    }

    pub fn set_attribute(&mut self, key: String, value: String) {
        self.attributes.insert(key, value);
    }

    pub fn get_attribute(&self, key: &str) -> Option<&String> {
        self.attributes.get(key)
    }
}

impl AttributeTable {
    pub fn new(layer_id: String) -> Self {
        Self {
            layer_id,
            features: Vec::new(),
            selected_features: HashSet::new(),
            filter: None,
        }
    }

    pub fn with_capacity(layer_id: String, capacity: usize) -> Self {
        Self {
            layer_id,
            features: Vec::with_capacity(capacity),
            selected_features: HashSet::new(),
            filter: None,
        }
    }

    pub fn add_feature(&mut self, feature: Feature) {
        self.features.push(feature);
    }

    pub fn select_feature(&mut self, index: usize) -> bool {
        index < self.features.len() && self.selected_features.insert(index)
    }

    pub fn deselect_feature(&mut self, index: usize) -> bool {
        self.selected_features.remove(&index)
    }

    pub fn toggle_selection(&mut self, index: usize) -> bool {
        if index >= self.features.len() {
            return false;
        }
        if self.selected_features.contains(&index) {
            self.selected_features.remove(&index);
            false
        } else {
            self.selected_features.insert(index);
            true
        }
    }

    pub fn select_all(&mut self) {
        self.selected_features = (0..self.features.len()).collect();
    }

    pub fn is_selected(&self, index: usize) -> bool {
        self.selected_features.contains(&index)
    }

    pub fn clear_selection(&mut self) {
        self.selected_features.clear();
    }

    pub fn apply_filter(&mut self, filter: String) {
        self.filter = Some(filter);
    }

    pub fn clear_filter(&mut self) {
        self.filter = None;
    }

    pub fn get_filtered_features(&self) -> Vec<(usize, &Feature)> {
        let normalized = self
            .filter
            .as_ref()
            .map(|filter| filter.trim().to_lowercase())
            .filter(|filter| !filter.is_empty());

        match normalized {
            Some(filter) => self
                .features
                .iter()
                .enumerate()
                .filter(|(_, feature)| feature.matches(&filter))
                .collect(),
            None => self.features.iter().enumerate().collect(),
        }
    }

    pub fn feature_count(&self) -> usize {
        self.features.len()
    }

    pub fn selected_count(&self) -> usize {
        self.selected_features.len()
    }

    pub fn get_feature(&self, index: usize) -> Option<&Feature> {
        self.features.get(index)
    }

    pub fn get_feature_mut(&mut self, index: usize) -> Option<&mut Feature> {
        self.features.get_mut(index)
    }

    pub fn remove_feature(&mut self, index: usize) -> Option<Feature> {
        if index < self.features.len() {
            self.selected_features.remove(&index);
            // Ajustar índices de seleção
            self.selected_features = self
                .selected_features
                .iter()
                .filter_map(|&i| if i > index { Some(i - 1) } else if i < index { Some(i) } else { None })
                .collect();
            Some(self.features.remove(index))
        } else {
            None
        }
    }
}

impl Feature {
    fn matches(&self, filter: &str) -> bool {
        self.id.to_lowercase().contains(filter)
            || self.geometry_type.to_lowercase().contains(filter)
            || self.attributes.iter().any(|(key, value)| {
                key.to_lowercase().contains(filter) || value.to_lowercase().contains(filter)
            })
    }
}
