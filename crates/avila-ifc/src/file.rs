use crate::entities::IfcEntity;
use crate::error::{IfcError, Result};
use crate::step_parser::{StepFile, StepParser};
use indexmap::IndexMap;
use std::collections::HashMap;
use std::path::Path;

/// Main IFC file representation
pub struct IfcFile {
    step_file: StepFile,
    entities_by_type: HashMap<String, Vec<i64>>,
    spatial_structure: SpatialStructure,
}

#[derive(Debug, Default)]
pub struct SpatialStructure {
    pub project: Option<i64>,
    pub sites: Vec<i64>,
    pub buildings: Vec<i64>,
    pub building_storeys: Vec<i64>,
}

impl IfcFile {
    /// Open and parse an IFC file from a path
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self> {
        let mut parser = StepParser::new();
        let step_file = parser.parse_file(path)?;
        Self::from_step_file(step_file)
    }

    /// Create an IfcFile from a parsed STEP file
    pub fn from_step_file(step_file: StepFile) -> Result<Self> {
        let mut entities_by_type: HashMap<String, Vec<i64>> = HashMap::new();

        // Index entities by type
        for (id, entity) in &step_file.entities {
            entities_by_type
                .entry(entity.entity_type.clone())
                .or_insert_with(Vec::new)
                .push(*id);
        }

        // Build spatial structure
        let spatial_structure = Self::build_spatial_structure(&step_file, &entities_by_type)?;

        Ok(Self {
            step_file,
            entities_by_type,
            spatial_structure,
        })
    }

    fn build_spatial_structure(
        step_file: &StepFile,
        entities_by_type: &HashMap<String, Vec<i64>>,
    ) -> Result<SpatialStructure> {
        let mut structure = SpatialStructure::default();

        // Find project
        if let Some(projects) = entities_by_type.get("IFCPROJECT") {
            structure.project = projects.first().copied();
        }

        // Find sites
        if let Some(sites) = entities_by_type.get("IFCSITE") {
            structure.sites = sites.clone();
        }

        // Find buildings
        if let Some(buildings) = entities_by_type.get("IFCBUILDING") {
            structure.buildings = buildings.clone();
        }

        // Find building storeys
        if let Some(storeys) = entities_by_type.get("IFCBUILDINGSTOREY") {
            structure.building_storeys = storeys.clone();
        }

        Ok(structure)
    }

    /// Get the IFC schema version
    pub fn schema(&self) -> Option<&str> {
        self.step_file
            .header
            .schema_identifiers
            .first()
            .map(|s| s.as_str())
    }

    /// Get the file name from header
    pub fn file_name(&self) -> &str {
        &self.step_file.header.file_name
    }

    /// Get the originating system
    pub fn originating_system(&self) -> &str {
        &self.step_file.header.originating_system
    }

    /// Get an entity by ID
    pub fn get_entity(&self, id: i64) -> Option<IfcEntity> {
        self.step_file
            .entities
            .get(&id)
            .map(|e| IfcEntity::from(e.clone()))
    }

    /// Get all entities of a specific type
    pub fn get_entities_by_type(&self, entity_type: &str) -> Vec<IfcEntity> {
        self.entities_by_type
            .get(entity_type)
            .map(|ids| {
                ids.iter()
                    .filter_map(|id| self.get_entity(*id))
                    .collect()
            })
            .unwrap_or_default()
    }

    /// Get all entity types present in the file
    pub fn get_entity_types(&self) -> Vec<String> {
        self.entities_by_type.keys().cloned().collect()
    }

    /// Count entities of a specific type
    pub fn count_entities_by_type(&self, entity_type: &str) -> usize {
        self.entities_by_type
            .get(entity_type)
            .map(|v| v.len())
            .unwrap_or(0)
    }

    /// Get total number of entities
    pub fn total_entities(&self) -> usize {
        self.step_file.entities.len()
    }

    /// Get the project entity
    pub fn get_project(&self) -> Option<IfcEntity> {
        self.spatial_structure
            .project
            .and_then(|id| self.get_entity(id))
    }

    /// Get all site entities
    pub fn get_sites(&self) -> Vec<IfcEntity> {
        self.spatial_structure
            .sites
            .iter()
            .filter_map(|id| self.get_entity(*id))
            .collect()
    }

    /// Get all building entities
    pub fn get_buildings(&self) -> Vec<IfcEntity> {
        self.spatial_structure
            .buildings
            .iter()
            .filter_map(|id| self.get_entity(*id))
            .collect()
    }

    /// Get all building storey entities
    pub fn get_building_storeys(&self) -> Vec<IfcEntity> {
        self.spatial_structure
            .building_storeys
            .iter()
            .filter_map(|id| self.get_entity(*id))
            .collect()
    }

    /// Get entities by filter predicate
    pub fn filter_entities<F>(&self, predicate: F) -> Vec<IfcEntity>
    where
        F: Fn(&IfcEntity) -> bool,
    {
        self.step_file
            .entities
            .values()
            .map(|e| IfcEntity::from(e.clone()))
            .filter(predicate)
            .collect()
    }

    /// Resolve an entity reference
    pub fn resolve_reference(&self, reference: i64) -> Result<IfcEntity> {
        self.get_entity(reference)
            .ok_or_else(|| IfcError::InvalidReference(reference))
    }

    /// Get entities containing a specific string in their name
    pub fn find_by_name(&self, name: &str) -> Vec<IfcEntity> {
        self.filter_entities(|entity| {
            entity
                .get_string_attribute(2) // Name is typically at index 2
                .ok()
                .flatten()
                .map(|n| n.to_lowercase().contains(&name.to_lowercase()))
                .unwrap_or(false)
        })
    }

    /// Get statistics about the IFC file
    pub fn statistics(&self) -> FileStatistics {
        let mut stats = FileStatistics::default();

        stats.total_entities = self.total_entities();
        stats.schema = self.schema().unwrap_or("Unknown").to_string();
        stats.entity_type_count = self.entities_by_type.len();

        // Count common entity types
        stats.walls = self.count_entities_by_type("IFCWALL");
        stats.slabs = self.count_entities_by_type("IFCSLAB");
        stats.beams = self.count_entities_by_type("IFCBEAM");
        stats.columns = self.count_entities_by_type("IFCCOLUMN");
        stats.doors = self.count_entities_by_type("IFCDOOR");
        stats.windows = self.count_entities_by_type("IFCWINDOW");
        stats.building_storeys = self.count_entities_by_type("IFCBUILDINGSTOREY");

        stats
    }
}

#[derive(Debug, Default)]
pub struct FileStatistics {
    pub total_entities: usize,
    pub schema: String,
    pub entity_type_count: usize,
    pub walls: usize,
    pub slabs: usize,
    pub beams: usize,
    pub columns: usize,
    pub doors: usize,
    pub windows: usize,
    pub building_storeys: usize,
}

impl std::fmt::Display for FileStatistics {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "IFC File Statistics")?;
        writeln!(f, "====================")?;
        writeln!(f, "Schema: {}", self.schema)?;
        writeln!(f, "Total Entities: {}", self.total_entities)?;
        writeln!(f, "Entity Types: {}", self.entity_type_count)?;
        writeln!(f)?;
        writeln!(f, "Common Elements:")?;
        writeln!(f, "  Walls: {}", self.walls)?;
        writeln!(f, "  Slabs: {}", self.slabs)?;
        writeln!(f, "  Beams: {}", self.beams)?;
        writeln!(f, "  Columns: {}", self.columns)?;
        writeln!(f, "  Doors: {}", self.doors)?;
        writeln!(f, "  Windows: {}", self.windows)?;
        writeln!(f, "  Building Storeys: {}", self.building_storeys)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_file_statistics_display() {
        let stats = FileStatistics {
            total_entities: 1000,
            schema: "IFC2X3".to_string(),
            entity_type_count: 50,
            walls: 100,
            slabs: 50,
            beams: 75,
            columns: 80,
            doors: 20,
            windows: 30,
            building_storeys: 5,
        };

        let display = format!("{}", stats);
        assert!(display.contains("IFC2X3"));
        assert!(display.contains("1000"));
    }
}
