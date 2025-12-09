//! Mapas temáticos

use super::{DbError, DbResult, GeoEntity};
use super::database::CartographicDatabase;
use super::symbology::{SymbolStyle, ColorRamp, Classifier, CategoricalSymbology, ClassificationMethod};
use super::entities::Entity;
use crate::cartography::{Map, Viewport, LatLon, BoundingBox};
use std::collections::HashMap;

/// Tema do mapa
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MapTheme {
    /// Símb graduado por quantidade/valor
    GraduatedSymbol,

    /// Símbolos categóricos (diferentes por categoria)
    Categorical,

    /// Mapa de calor (densidade)
    Heatmap,

    /// Proporcionais (tamanho varia com valor)
    Proportional,

    /// Simples (todos iguais)
    Simple,
}

/// Camada temática
pub struct ThematicLayer {
    pub name: String,
    pub theme: MapTheme,
    pub visible: bool,
    pub opacity: f32,
    pub entities: Vec<u64>, // IDs das entidades
}

impl ThematicLayer {
    pub fn new(name: String, theme: MapTheme) -> Self {
        Self {
            name,
            theme,
            visible: true,
            opacity: 1.0,
            entities: Vec::new(),
        }
    }

    pub fn add_entity(&mut self, id: u64) {
        self.entities.push(id);
    }
}

/// Mapa temático
pub struct ThematicMap {
    pub title: String,
    pub description: String,
    pub layers: Vec<ThematicLayer>,
    pub viewport: Viewport,
    pub show_labels: bool,
    pub show_legend: bool,
}

impl ThematicMap {
    pub fn new(title: String) -> Self {
        Self {
            title,
            description: String::new(),
            layers: Vec::new(),
            viewport: Viewport::default(),
            show_labels: true,
            show_legend: true,
        }
    }

    pub fn add_layer(&mut self, layer: ThematicLayer) {
        self.layers.push(layer);
    }

    pub fn set_viewport(&mut self, viewport: Viewport) {
        self.viewport = viewport;
    }

    /// Ajusta viewport para mostrar todas as entidades
    pub fn fit_to_entities(&mut self, db: &CartographicDatabase) {
        let mut min_lat = 90.0;
        let mut max_lat = -90.0;
        let mut min_lon = 180.0;
        let mut max_lon = -180.0;

        for layer in &self.layers {
            for &id in &layer.entities {
                if let Some(entity) = db.get(id) {
                    let loc = entity.as_geo_entity().location();
                    min_lat = min_lat.min(loc.lat);
                    max_lat = max_lat.max(loc.lat);
                    min_lon = min_lon.min(loc.lon);
                    max_lon = max_lon.max(loc.lon);
                }
            }
        }

        // Adiciona margem de 10%
        let lat_margin = (max_lat - min_lat) * 0.1;
        let lon_margin = (max_lon - min_lon) * 0.1;

        let bbox = BoundingBox::new(
            min_lat - lat_margin,
            max_lat + lat_margin,
            min_lon - lon_margin,
            max_lon + lon_margin,
        );

        self.viewport = Viewport::from_bbox(bbox);
    }
}

/// Builder para criar mapas temáticos facilmente
pub struct ThematicMapBuilder<'a> {
    db: &'a CartographicDatabase,
    map: ThematicMap,
}

impl<'a> ThematicMapBuilder<'a> {
    pub fn new(db: &'a CartographicDatabase, title: String) -> Self {
        Self {
            db,
            map: ThematicMap::new(title),
        }
    }

    /// Adiciona camada com todas as empresas
    pub fn add_companies_layer(mut self, name: String) -> Self {
        let mut layer = ThematicLayer::new(name, MapTheme::Simple);

        for entity in self.db.list_by_type("company") {
            layer.add_entity(entity.as_geo_entity().id());
        }

        self.map.add_layer(layer);
        self
    }

    /// Adiciona camada com todas as empresas por setor (categórico)
    pub fn add_companies_by_sector(mut self, name: String) -> Self {
        let mut layer = ThematicLayer::new(name, MapTheme::Categorical);

        for entity in self.db.list_by_type("company") {
            layer.add_entity(entity.as_geo_entity().id());
        }

        self.map.add_layer(layer);
        self
    }

    /// Adiciona camada com lugares
    pub fn add_places_layer(mut self, name: String) -> Self {
        let mut layer = ThematicLayer::new(name, MapTheme::Simple);

        for entity in self.db.list_by_type("place") {
            layer.add_entity(entity.as_geo_entity().id());
        }

        self.map.add_layer(layer);
        self
    }

    /// Adiciona camada com lugares por categoria
    pub fn add_places_by_category(mut self, name: String) -> Self {
        let mut layer = ThematicLayer::new(name, MapTheme::Categorical);

        for entity in self.db.list_by_type("place") {
            layer.add_entity(entity.as_geo_entity().id());
        }

        self.map.add_layer(layer);
        self
    }

    /// Adiciona camada com endereços
    pub fn add_addresses_layer(mut self, name: String) -> Self {
        let mut layer = ThematicLayer::new(name, MapTheme::Simple);

        for entity in self.db.list_by_type("address") {
            layer.add_entity(entity.as_geo_entity().id());
        }

        self.map.add_layer(layer);
        self
    }

    /// Adiciona camada com POIs
    pub fn add_pois_layer(mut self, name: String) -> Self {
        let mut layer = ThematicLayer::new(name, MapTheme::Simple);

        for entity in self.db.list_by_type("poi") {
            layer.add_entity(entity.as_geo_entity().id());
        }

        self.map.add_layer(layer);
        self
    }

    /// Adiciona camada customizada por filtro
    pub fn add_custom_layer<F>(mut self, name: String, theme: MapTheme, filter: F) -> Self
    where
        F: Fn(&Entity) -> bool,
    {
        let mut layer = ThematicLayer::new(name, theme);

        for entity in self.db.list_all() {
            if filter(entity) {
                layer.add_entity(entity.as_geo_entity().id());
            }
        }

        self.map.add_layer(layer);
        self
    }

    /// Define descrição do mapa
    pub fn description(mut self, desc: String) -> Self {
        self.map.description = desc;
        self
    }

    /// Ajusta viewport automaticamente
    pub fn auto_fit(mut self) -> Self {
        self.map.fit_to_entities(self.db);
        self
    }

    /// Centraliza em coordenada específica
    pub fn center_at(mut self, coord: LatLon) -> Self {
        self.map.viewport.center_at(coord);
        self
    }

    /// Define se mostra labels
    pub fn show_labels(mut self, show: bool) -> Self {
        self.map.show_labels = show;
        self
    }

    /// Define se mostra legenda
    pub fn show_legend(mut self, show: bool) -> Self {
        self.map.show_legend = show;
        self
    }

    /// Constrói o mapa temático
    pub fn build(self) -> ThematicMap {
        self.map
    }
}

/// Renderizador de mapas temáticos
pub struct ThematicRenderer {
    /// Simbologia categórica
    categorical_symbology: HashMap<String, CategoricalSymbology>,

    /// Rampas de cor por camada
    color_ramps: HashMap<String, ColorRamp>,

    /// Classificadores por camada
    classifiers: HashMap<String, Classifier>,
}

impl ThematicRenderer {
    pub fn new() -> Self {
        Self {
            categorical_symbology: HashMap::new(),
            color_ramps: HashMap::new(),
            classifiers: HashMap::new(),
        }
    }

    /// Define simbologia categórica para uma camada
    pub fn set_categorical(&mut self, layer_name: String, symbology: CategoricalSymbology) {
        self.categorical_symbology.insert(layer_name, symbology);
    }

    /// Define rampa de cor para uma camada
    pub fn set_color_ramp(&mut self, layer_name: String, ramp: ColorRamp) {
        self.color_ramps.insert(layer_name, ramp);
    }

    /// Define classificador para uma camada
    pub fn set_classifier(&mut self, layer_name: String, classifier: Classifier) {
        self.classifiers.insert(layer_name, classifier);
    }

    /// Obtém símbolo para uma entidade em uma camada
    pub fn get_symbol_for_entity(
        &self,
        layer_name: &str,
        entity: &Entity,
        _layer_theme: &MapTheme,
    ) -> SymbolStyle {
        // Tenta obter simbologia categórica
        if let Some(symbology) = self.categorical_symbology.get(layer_name) {
            // Determina categoria baseado no tipo de entidade
            let category = match entity {
                Entity::Company(c) => &c.sector,
                Entity::Place(p) => &p.category,
                Entity::Address(a) => &a.address_type,
                Entity::PointOfInterest(poi) => &poi.poi_type,
            };

            return symbology.get_symbol(category).clone();
        }

        // Padrão
        SymbolStyle::default()
    }
}

impl Default for ThematicRenderer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::aviladb_cartographic::database::DatabaseConfig;

    #[test]
    fn test_thematic_map_builder() {
        let db = CartographicDatabase::new(DatabaseConfig::default());

        let map = ThematicMapBuilder::new(&db, "Mapa de Teste".to_string())
            .add_companies_layer("Empresas".to_string())
            .description("Mapa temático de teste".to_string())
            .show_labels(true)
            .build();

        assert_eq!(map.title, "Mapa de Teste");
        assert_eq!(map.layers.len(), 1);
    }
}
