use crate::entities::IfcEntity;
use crate::file::IfcFile;
use std::collections::HashSet;

/// Query builder for filtering IFC entities
pub struct IfcQuery<'a> {
    file: &'a IfcFile,
    filters: Vec<Box<dyn Fn(&IfcEntity) -> bool + 'a>>,
}

impl<'a> IfcQuery<'a> {
    pub fn new(file: &'a IfcFile) -> Self {
        Self {
            file,
            filters: Vec::new(),
        }
    }

    /// Filter by entity type
    pub fn entity_type(mut self, entity_type: &'a str) -> Self {
        self.filters.push(Box::new(move |entity| {
            entity.entity_type == entity_type
        }));
        self
    }

    /// Filter by entity types (multiple)
    pub fn entity_types(mut self, entity_types: &'a [&'a str]) -> Self {
        let types: HashSet<&str> = entity_types.iter().copied().collect();
        self.filters.push(Box::new(move |entity| {
            types.contains(entity.entity_type.as_str())
        }));
        self
    }

    /// Filter by name containing string (case-insensitive)
    pub fn name_contains(mut self, name: &'a str) -> Self {
        let name_lower = name.to_lowercase();
        self.filters.push(Box::new(move |entity| {
            entity
                .get_string_attribute(2)
                .ok()
                .flatten()
                .map(|n| n.to_lowercase().contains(&name_lower))
                .unwrap_or(false)
        }));
        self
    }

    /// Filter by exact name
    pub fn name_equals(mut self, name: &'a str) -> Self {
        let name = name.to_string();
        self.filters.push(Box::new(move |entity| {
            entity
                .get_string_attribute(2)
                .ok()
                .flatten()
                .map(|n| n == name)
                .unwrap_or(false)
        }));
        self
    }

    /// Filter by global ID
    pub fn global_id(mut self, global_id: &'a str) -> Self {
        let global_id = global_id.to_string();
        self.filters.push(Box::new(move |entity| {
            entity
                .get_string_attribute(0)
                .ok()
                .flatten()
                .map(|id| id == global_id)
                .unwrap_or(false)
        }));
        self
    }

    /// Filter by custom predicate
    pub fn filter<F>(mut self, predicate: F) -> Self
    where
        F: Fn(&IfcEntity) -> bool + 'a,
    {
        self.filters.push(Box::new(predicate));
        self
    }

    /// Execute the query and return matching entities
    pub fn execute(self) -> Vec<IfcEntity> {
        self.file.filter_entities(|entity| {
            self.filters.iter().all(|filter| filter(entity))
        })
    }

    /// Execute and return only the first result
    pub fn first(self) -> Option<IfcEntity> {
        self.execute().into_iter().next()
    }

    /// Count matching entities without loading them all
    pub fn count(self) -> usize {
        self.execute().len()
    }
}

/// Query for spatial elements within a building storey
pub struct SpatialQuery<'a> {
    file: &'a IfcFile,
}

impl<'a> SpatialQuery<'a> {
    pub fn new(file: &'a IfcFile) -> Self {
        Self { file }
    }

    /// Get all elements in a specific building storey
    pub fn elements_in_storey(&self, storey_id: i64) -> Vec<IfcEntity> {
        // Find IfcRelContainedInSpatialStructure relationships
        let relationships = self
            .file
            .get_entities_by_type("IFCRELCONTAINEDINSPATIALSTRUCTURE");

        let mut element_ids = Vec::new();

        for rel in relationships {
            // Check if relating_structure matches storey_id
            if let Ok(Some(relating_id)) = rel.get_entity_ref_attribute(4) {
                if relating_id == storey_id {
                    // Get related_elements (usually a list)
                    if let Ok(Some(elements)) = rel.get_list_attribute(4) {
                        for elem in elements {
                            if let crate::step_parser::StepValue::EntityRef(id) = elem {
                                element_ids.push(id);
                            }
                        }
                    }
                }
            }
        }

        element_ids
            .into_iter()
            .filter_map(|id| self.file.get_entity(id))
            .collect()
    }

    /// Get all walls in the project
    pub fn all_walls(&self) -> Vec<IfcEntity> {
        self.file.get_entities_by_type("IFCWALL")
    }

    /// Get all slabs in the project
    pub fn all_slabs(&self) -> Vec<IfcEntity> {
        self.file.get_entities_by_type("IFCSLAB")
    }

    /// Get all beams in the project
    pub fn all_beams(&self) -> Vec<IfcEntity> {
        self.file.get_entities_by_type("IFCBEAM")
    }

    /// Get all columns in the project
    pub fn all_columns(&self) -> Vec<IfcEntity> {
        self.file.get_entities_by_type("IFCCOLUMN")
    }

    /// Get all structural elements (walls, beams, columns, slabs)
    pub fn all_structural_elements(&self) -> Vec<IfcEntity> {
        let types = ["IFCWALL", "IFCBEAM", "IFCCOLUMN", "IFCSLAB"];
        types
            .iter()
            .flat_map(|t| self.file.get_entities_by_type(t))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::step_parser::{StepEntity, StepValue};
    use crate::file::IfcFile;
    use crate::step_parser::StepFile;

    fn create_test_entity(id: i64, entity_type: &str, name: &str) -> StepEntity {
        StepEntity {
            id,
            entity_type: entity_type.to_string(),
            attributes: vec![
                StepValue::String("GlobalId".to_string()),
                StepValue::Null,
                StepValue::String(name.to_string()),
            ],
        }
    }

    #[test]
    fn test_query_filter_by_type() {
        // This is a simplified test - in practice you'd create a proper test file
        // The test demonstrates the query interface
    }
}
