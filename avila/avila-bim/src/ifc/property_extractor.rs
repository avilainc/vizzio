//! Property extractor from IFC entities

use crate::bim_core::*;
use std::collections::HashMap;

/// Extrator de propriedades IFC
pub struct PropertyExtractor;

impl PropertyExtractor {
    /// Extrair IfcPropertySet → Properties
    pub fn extract_property_set(
        pset_name: &str,
        properties: HashMap<String, String>,
    ) -> Properties {
        let mut props = Properties::new();

        for (key, value) in properties {
            // Tentar inferir tipo do valor
            if let Ok(int_val) = value.parse::<i64>() {
                props.set(key, PropertyValue::Integer(int_val));
            } else if let Ok(float_val) = value.parse::<f64>() {
                props.set(key, PropertyValue::Float(float_val));
            } else if value.eq_ignore_ascii_case("true") || value.eq_ignore_ascii_case("false") {
                let bool_val = value.eq_ignore_ascii_case("true");
                props.set(key, PropertyValue::Boolean(bool_val));
            } else {
                props.set(key, PropertyValue::String(value));
            }
        }

        props
    }

    /// Extrair propriedades quantitativas (IfcQuantitySet)
    pub fn extract_quantities(
        quantities: HashMap<String, f64>,
    ) -> Properties {
        let mut props = Properties::new();

        for (key, value) in quantities {
            // Identificar tipo por nome
            let prop_value = if key.contains("Length") || key.contains("Width") || key.contains("Height") {
                PropertyValue::Length(value, LengthUnit::Meter)
            } else if key.contains("Area") {
                PropertyValue::Area(value)
            } else if key.contains("Volume") {
                PropertyValue::Volume(value)
            } else {
                PropertyValue::Float(value)
            };

            props.set(key, prop_value);
        }

        props
    }

    /// Extrair material
    pub fn extract_material(
        name: String,
        color: Option<[f32; 4]>,
    ) -> Material {
        Material {
            name,
            color,
            metallic: 0.0,
            roughness: 0.8,
            textures: HashMap::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_properties() {
        let mut props_map = HashMap::new();
        props_map.insert("LoadBearing".to_string(), "true".to_string());
        props_map.insert("FireRating".to_string(), "120".to_string());

        let props = PropertyExtractor::extract_property_set("Pset_WallCommon", props_map);

        assert!(matches!(
            props.get("LoadBearing"),
            Some(PropertyValue::Boolean(true))
        ));
        assert!(matches!(
            props.get("FireRating"),
            Some(PropertyValue::Integer(120))
        ));
    }
}
