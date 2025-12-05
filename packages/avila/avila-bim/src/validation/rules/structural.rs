//! Structural validation rules

use crate::bim_core::*;

/// Regras de validação estrutural
pub struct StructuralRules;

impl StructuralRules {
    /// Validar elementos estruturais
    pub fn validate_structural_elements(model: &BimModel) -> Vec<String> {
        let mut warnings = Vec::new();

        // Verificar paredes estruturais
        let walls = model.filter_by_type("IfcWall");
        for wall in walls {
            if let Some(prop) = wall.get_property("LoadBearing") {
                if matches!(prop, PropertyValue::Boolean(true)) {
                    // Verificar espessura mínima
                    if let Some(thickness_prop) = wall.get_property("Width") {
                        if let PropertyValue::Length(thickness, _) = thickness_prop {
                            if *thickness < 0.15 {
                                warnings.push(format!(
                                    "Load-bearing wall {} has insufficient thickness: {}m",
                                    wall.guid.as_str(),
                                    thickness
                                ));
                            }
                        }
                    }
                }
            }
        }

        // Verificar colunas
        let columns = model.filter_by_type("IfcColumn");
        if columns.is_empty() && !walls.is_empty() {
            warnings.push("Model has walls but no columns".to_string());
        }

        warnings
    }

    /// Validar lajes
    pub fn validate_slabs(model: &BimModel) -> Vec<String> {
        let mut warnings = Vec::new();

        let slabs = model.filter_by_type("IfcSlab");
        for slab in slabs {
            // Verificar espessura
            if let Some(PropertyValue::Length(thickness, _)) = slab.get_property("Thickness") {
                if *thickness < 0.1 {
                    warnings.push(format!(
                        "Slab {} has insufficient thickness: {}m",
                        slab.guid.as_str(),
                        thickness
                    ));
                }
            }
        }

        warnings
    }
}
