//! IFC schema validator

use crate::bim_core::*;

/// Erros de validação
#[derive(Debug, Clone)]
pub struct ValidationError {
    pub element_guid: Option<IfcGuid>,
    pub error_type: ValidationErrorType,
    pub message: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ValidationErrorType {
    MissingRequired,
    InvalidType,
    InvalidReference,
    SchemaViolation,
}

/// Validador IFC
pub struct IfcValidator {
    strict_mode: bool,
}

impl IfcValidator {
    pub fn new(strict_mode: bool) -> Self {
        Self { strict_mode }
    }

    /// Validar modelo completo
    pub fn validate_model(&self, model: &BimModel) -> Vec<ValidationError> {
        let mut errors = Vec::new();

        // Validar hierarquia
        errors.extend(self.validate_hierarchy(&model.hierarchy));

        // Validar elementos
        for element in model.elements.values() {
            errors.extend(self.validate_element(element));
        }

        errors
    }

    /// Validar hierarquia
    fn validate_hierarchy(&self, _hierarchy: &Hierarchy) -> Vec<ValidationError> {
        let mut errors = Vec::new();

        // TODO: Validar estrutura espacial
        // - Projeto deve ter pelo menos 1 site ou building
        // - Building deve ter pelo menos 1 storey

        errors
    }

    /// Validar elemento
    fn validate_element(&self, element: &BimElement) -> Vec<ValidationError> {
        let mut errors = Vec::new();

        // Validar GUID
        if element.guid.as_str().len() != 22 {
            errors.push(ValidationError {
                element_guid: Some(element.guid.clone()),
                error_type: ValidationErrorType::InvalidType,
                message: "Invalid GUID format".to_string(),
            });
        }

        // Validar tipo de elemento
        if !self.is_valid_element_type(&element.element_type) {
            errors.push(ValidationError {
                element_guid: Some(element.guid.clone()),
                error_type: ValidationErrorType::InvalidType,
                message: format!("Unknown element type: {}", element.element_type),
            });
        }

        // Validar geometria (se presente)
        if let Some(ref geometry) = element.geometry {
            if geometry.mesh.is_some() && geometry.bounds.min[0] > geometry.bounds.max[0] {
                errors.push(ValidationError {
                    element_guid: Some(element.guid.clone()),
                    error_type: ValidationErrorType::InvalidType,
                    message: "Invalid bounding box".to_string(),
                });
            }
        }

        errors
    }

    /// Validar tipo de elemento IFC
    fn is_valid_element_type(&self, element_type: &str) -> bool {
        matches!(
            element_type,
            "IfcWall" | "IfcSlab" | "IfcColumn" | "IfcBeam" | "IfcDoor" | "IfcWindow"
                | "IfcRoof" | "IfcStair" | "IfcRailing" | "IfcCurtainWall"
        )
    }
}

impl Default for IfcValidator {
    fn default() -> Self {
        Self::new(false)
    }
}
