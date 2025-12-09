//! Sistema de consultas espaciais e por atributos

use super::{DbResult, DbError, GeoEntity};
use super::database::CartographicDatabase;
use super::entities::Entity;
use crate::cartography::{LatLon, BoundingBox};

/// Consulta espacial
#[derive(Debug, Clone)]
pub enum SpatialQuery {
    /// Dentro de um bounding box
    InBoundingBox(BoundingBox),

    /// Próximo a um ponto (raio em km)
    Near { center: LatLon, radius_km: f64 },

    /// Interseção com polígono (simplificado)
    IntersectsPolygon(Vec<LatLon>),
}

/// Consulta por atributo
#[derive(Debug, Clone)]
pub enum AttributeQuery {
    /// Nome contém texto
    NameContains(String),

    /// Atributo igual a valor
    AttributeEquals { key: String, value: String },

    /// Atributo contém valor
    AttributeContains { key: String, value: String },

    /// Tipo de entidade
    EntityType(String),
}

/// Builder de consultas
pub struct QueryBuilder<'a> {
    db: &'a CartographicDatabase,
    spatial_filters: Vec<SpatialQuery>,
    attribute_filters: Vec<AttributeQuery>,
}

impl<'a> QueryBuilder<'a> {
    pub fn new(db: &'a CartographicDatabase) -> Self {
        Self {
            db,
            spatial_filters: Vec::new(),
            attribute_filters: Vec::new(),
        }
    }

    /// Adiciona filtro espacial - bounding box
    pub fn in_bbox(mut self, bbox: BoundingBox) -> Self {
        self.spatial_filters.push(SpatialQuery::InBoundingBox(bbox));
        self
    }

    /// Adiciona filtro espacial - proximidade
    pub fn near(mut self, center: LatLon, radius_km: f64) -> Self {
        self.spatial_filters.push(SpatialQuery::Near { center, radius_km });
        self
    }

    /// Adiciona filtro - nome contém
    pub fn name_contains(mut self, text: String) -> Self {
        self.attribute_filters.push(AttributeQuery::NameContains(text));
        self
    }

    /// Adiciona filtro - atributo igual
    pub fn attribute_equals(mut self, key: String, value: String) -> Self {
        self.attribute_filters.push(AttributeQuery::AttributeEquals { key, value });
        self
    }

    /// Adiciona filtro - tipo de entidade
    pub fn entity_type(mut self, entity_type: String) -> Self {
        self.attribute_filters.push(AttributeQuery::EntityType(entity_type));
        self
    }

    /// Filtra apenas empresas
    pub fn only_companies(self) -> Self {
        self.entity_type("company".to_string())
    }

    /// Filtra apenas lugares
    pub fn only_places(self) -> Self {
        self.entity_type("place".to_string())
    }

    /// Filtra apenas endereços
    pub fn only_addresses(self) -> Self {
        self.entity_type("address".to_string())
    }

    /// Filtra apenas POIs
    pub fn only_pois(self) -> Self {
        self.entity_type("poi".to_string())
    }

    /// Executa a consulta
    pub fn execute(self) -> DbResult<Vec<&'a Entity>> {
        // Começa com todas as entidades
        let mut results: Vec<&Entity> = self.db.list_all();

        // Aplica filtros espaciais
        for filter in &self.spatial_filters {
            results = self.apply_spatial_filter(results, filter);
        }

        // Aplica filtros de atributos
        for filter in &self.attribute_filters {
            results = self.apply_attribute_filter(results, filter);
        }

        Ok(results)
    }

    fn apply_spatial_filter(
        &self,
        entities: Vec<&'a Entity>,
        filter: &SpatialQuery,
    ) -> Vec<&'a Entity> {
        match filter {
            SpatialQuery::InBoundingBox(bbox) => {
                entities
                    .into_iter()
                    .filter(|e| {
                        let loc = e.as_geo_entity().location();
                        bbox.contains(&loc)
                    })
                    .collect()
            }
            SpatialQuery::Near { center, radius_km } => {
                entities
                    .into_iter()
                    .filter(|e| {
                        let loc = e.as_geo_entity().location();
                        center.distance_to(&loc) <= *radius_km
                    })
                    .collect()
            }
            SpatialQuery::IntersectsPolygon(_polygon) => {
                // TODO: Implementar verificação de ponto em polígono
                entities
            }
        }
    }

    fn apply_attribute_filter(
        &self,
        entities: Vec<&'a Entity>,
        filter: &AttributeQuery,
    ) -> Vec<&'a Entity> {
        match filter {
            AttributeQuery::NameContains(text) => {
                let text_lower = text.to_lowercase();
                entities
                    .into_iter()
                    .filter(|e| {
                        e.as_geo_entity()
                            .name()
                            .to_lowercase()
                            .contains(&text_lower)
                    })
                    .collect()
            }
            AttributeQuery::AttributeEquals { key, value } => {
                entities
                    .into_iter()
                    .filter(|e| e.as_geo_entity().get_attribute(key) == Some(value.as_str()))
                    .collect()
            }
            AttributeQuery::AttributeContains { key, value } => {
                let value_lower = value.to_lowercase();
                entities
                    .into_iter()
                    .filter(|e| {
                        if let Some(attr_value) = e.as_geo_entity().get_attribute(key) {
                            attr_value.to_lowercase().contains(&value_lower)
                        } else {
                            false
                        }
                    })
                    .collect()
            }
            AttributeQuery::EntityType(entity_type) => {
                entities
                    .into_iter()
                    .filter(|e| e.as_geo_entity().entity_type() == entity_type.as_str())
                    .collect()
            }
        }
    }
}

/// Estatísticas de consulta
pub struct QueryStats {
    pub total_found: usize,
    pub by_type: std::collections::HashMap<String, usize>,
}

impl QueryStats {
    pub fn from_results(entities: &[&Entity]) -> Self {
        let mut by_type = std::collections::HashMap::new();

        for entity in entities {
            let entity_type = entity.as_geo_entity().entity_type().to_string();
            *by_type.entry(entity_type).or_insert(0) += 1;
        }

        Self {
            total_found: entities.len(),
            by_type,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::aviladb_cartographic::{
        database::{CartographicDatabase, DatabaseConfig},
        entities::Company,
    };

    #[test]
    fn test_query_builder() {
        let mut db = CartographicDatabase::new(DatabaseConfig {
            auto_save: false,
            ..Default::default()
        });

        let company = Company::new(
            0,
            "Test Corp".to_string(),
            LatLon::new(-23.55, -46.63),
            "Rua Teste".to_string(),
            "Tech".to_string(),
        );

        db.add_company(company).unwrap();

        let results = QueryBuilder::new(&db)
            .only_companies()
            .name_contains("Test".to_string())
            .execute()
            .unwrap();

        assert_eq!(results.len(), 1);
    }

    #[test]
    fn test_spatial_query() {
        let mut db = CartographicDatabase::new(DatabaseConfig {
            auto_save: false,
            ..Default::default()
        });

        let company = Company::new(
            0,
            "Central Corp".to_string(),
            LatLon::new(0.0, 0.0),
            "Centro".to_string(),
            "Tech".to_string(),
        );

        db.add_company(company).unwrap();

        let bbox = BoundingBox::new(-1.0, 1.0, -1.0, 1.0);

        let results = QueryBuilder::new(&db)
            .in_bbox(bbox)
            .execute()
            .unwrap();

        assert_eq!(results.len(), 1);
    }
}
