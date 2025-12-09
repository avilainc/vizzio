use crate::error::{IfcError, Result};
use crate::step_parser::{StepEntity, StepValue};
use indexmap::IndexMap;
use std::collections::HashMap;

/// Represents an IFC entity with typed access to attributes
#[derive(Debug, Clone)]
pub struct IfcEntity {
    pub id: i64,
    pub entity_type: String,
    pub attributes: Vec<StepValue>,
}

impl From<StepEntity> for IfcEntity {
    fn from(step_entity: StepEntity) -> Self {
        Self {
            id: step_entity.id,
            entity_type: step_entity.entity_type,
            attributes: step_entity.attributes,
        }
    }
}

impl IfcEntity {
    pub fn get_string_attribute(&self, index: usize) -> Result<Option<String>> {
        match self.attributes.get(index) {
            Some(StepValue::String(s)) => Ok(Some(s.clone())),
            Some(StepValue::Null) => Ok(None),
            Some(_) => Err(IfcError::InvalidAttribute(format!(
                "Attribute {} is not a string",
                index
            ))),
            None => Ok(None),
        }
    }

    pub fn get_integer_attribute(&self, index: usize) -> Result<Option<i64>> {
        match self.attributes.get(index) {
            Some(StepValue::Integer(i)) => Ok(Some(*i)),
            Some(StepValue::Null) => Ok(None),
            Some(_) => Err(IfcError::InvalidAttribute(format!(
                "Attribute {} is not an integer",
                index
            ))),
            None => Ok(None),
        }
    }

    pub fn get_real_attribute(&self, index: usize) -> Result<Option<f64>> {
        match self.attributes.get(index) {
            Some(StepValue::Real(r)) => Ok(Some(*r)),
            Some(StepValue::Integer(i)) => Ok(Some(*i as f64)),
            Some(StepValue::Null) => Ok(None),
            Some(_) => Err(IfcError::InvalidAttribute(format!(
                "Attribute {} is not a real number",
                index
            ))),
            None => Ok(None),
        }
    }

    pub fn get_entity_ref_attribute(&self, index: usize) -> Result<Option<i64>> {
        match self.attributes.get(index) {
            Some(StepValue::EntityRef(id)) => Ok(Some(*id)),
            Some(StepValue::Null) => Ok(None),
            Some(_) => Err(IfcError::InvalidAttribute(format!(
                "Attribute {} is not an entity reference",
                index
            ))),
            None => Ok(None),
        }
    }

    pub fn get_enum_attribute(&self, index: usize) -> Result<Option<String>> {
        match self.attributes.get(index) {
            Some(StepValue::Enum(e)) => Ok(Some(e.clone())),
            Some(StepValue::Null) => Ok(None),
            Some(_) => Err(IfcError::InvalidAttribute(format!(
                "Attribute {} is not an enum",
                index
            ))),
            None => Ok(None),
        }
    }

    pub fn get_list_attribute(&self, index: usize) -> Result<Option<Vec<StepValue>>> {
        match self.attributes.get(index) {
            Some(StepValue::List(list)) => Ok(Some(list.clone())),
            Some(StepValue::Null) => Ok(None),
            Some(_) => Err(IfcError::InvalidAttribute(format!(
                "Attribute {} is not a list",
                index
            ))),
            None => Ok(None),
        }
    }
}

// Common IFC entity types
#[derive(Debug, Clone)]
pub struct IfcProject {
    pub id: i64,
    pub global_id: String,
    pub name: String,
    pub description: Option<String>,
}

#[derive(Debug, Clone)]
pub struct IfcBuilding {
    pub id: i64,
    pub global_id: String,
    pub name: String,
    pub description: Option<String>,
    pub elevation: Option<f64>,
}

#[derive(Debug, Clone)]
pub struct IfcBuildingStorey {
    pub id: i64,
    pub global_id: String,
    pub name: String,
    pub description: Option<String>,
    pub elevation: f64,
}

#[derive(Debug, Clone)]
pub struct IfcSite {
    pub id: i64,
    pub global_id: String,
    pub name: String,
    pub description: Option<String>,
}

#[derive(Debug, Clone)]
pub struct IfcWall {
    pub id: i64,
    pub global_id: String,
    pub name: String,
    pub description: Option<String>,
}

#[derive(Debug, Clone)]
pub struct IfcSlab {
    pub id: i64,
    pub global_id: String,
    pub name: String,
    pub description: Option<String>,
}

#[derive(Debug, Clone)]
pub struct IfcBeam {
    pub id: i64,
    pub global_id: String,
    pub name: String,
    pub description: Option<String>,
}

#[derive(Debug, Clone)]
pub struct IfcColumn {
    pub id: i64,
    pub global_id: String,
    pub name: String,
    pub description: Option<String>,
}

#[derive(Debug, Clone)]
pub struct IfcDoor {
    pub id: i64,
    pub global_id: String,
    pub name: String,
    pub description: Option<String>,
}

#[derive(Debug, Clone)]
pub struct IfcWindow {
    pub id: i64,
    pub global_id: String,
    pub name: String,
    pub description: Option<String>,
}

// Geometry types
#[derive(Debug, Clone)]
pub struct IfcCartesianPoint {
    pub id: i64,
    pub coordinates: Vec<f64>,
}

#[derive(Debug, Clone)]
pub struct IfcDirection {
    pub id: i64,
    pub direction_ratios: Vec<f64>,
}

#[derive(Debug, Clone)]
pub struct IfcAxis2Placement3D {
    pub id: i64,
    pub location: i64, // Reference to IfcCartesianPoint
    pub axis: Option<i64>, // Reference to IfcDirection
    pub ref_direction: Option<i64>, // Reference to IfcDirection
}

// Property types
#[derive(Debug, Clone)]
pub struct IfcPropertySingleValue {
    pub id: i64,
    pub name: String,
    pub description: Option<String>,
    pub nominal_value: Option<StepValue>,
}

#[derive(Debug, Clone)]
pub struct IfcPropertySet {
    pub id: i64,
    pub global_id: String,
    pub name: String,
    pub description: Option<String>,
    pub properties: Vec<i64>, // References to properties
}

// Material types
#[derive(Debug, Clone)]
pub struct IfcMaterial {
    pub id: i64,
    pub name: String,
    pub description: Option<String>,
    pub category: Option<String>,
}

// Relationship types
#[derive(Debug, Clone)]
pub struct IfcRelAggregates {
    pub id: i64,
    pub relating_object: i64,
    pub related_objects: Vec<i64>,
}

#[derive(Debug, Clone)]
pub struct IfcRelContainedInSpatialStructure {
    pub id: i64,
    pub related_elements: Vec<i64>,
    pub relating_structure: i64,
}

#[derive(Debug, Clone)]
pub struct IfcRelDefinesByProperties {
    pub id: i64,
    pub related_objects: Vec<i64>,
    pub relating_property_definition: i64,
}

/// Entity type registry for quick lookup
pub struct EntityTypeRegistry {
    type_map: HashMap<String, fn(&IfcEntity) -> Result<Box<dyn std::any::Any>>>,
}

impl EntityTypeRegistry {
    pub fn new() -> Self {
        let mut registry = Self {
            type_map: HashMap::new(),
        };
        registry.register_standard_types();
        registry
    }

    fn register_standard_types(&mut self) {
        // Register common IFC types
        // This would be expanded to include all IFC2X3 entities
    }

    pub fn is_registered(&self, entity_type: &str) -> bool {
        self.type_map.contains_key(entity_type)
    }
}

impl Default for EntityTypeRegistry {
    fn default() -> Self {
        Self::new()
    }
}
