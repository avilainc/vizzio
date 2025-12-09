//! # Avila Cartography Framework
//!
//! Framework completo para visualização de mapas geográficos da Europa, África e Ásia.
//!
//! ## Características
//! - Projeções cartográficas (Mercator, Robinson, Albers)
//! - Renderização de fronteiras políticas
//! - Sistema de coordenadas geográficas
//! - Zoom e pan interativo
//! - Camadas de informação (países, cidades, rios, montanhas)

pub mod error;
pub mod projection;
pub mod renderer;
pub mod continents;
pub mod coordinates;
pub mod features;
pub mod tiles;
pub mod viewport;

pub use error::{GeoError, GeoResult};pub use projection::{Projection, Mercator, Robinson, AlbersEqualArea};
pub use renderer::{MapRenderer, RenderOptions, OutputFormat, SvgRenderer, JsonRenderer, Style};
pub use continents::{Europe, Africa, Asia, Continent, Country};
pub use coordinates::{LatLon, Point2D, BoundingBox};
pub use features::{City, River, Mountain, Feature, CityDatabase, FeatureType};
pub use viewport::{Viewport, ZoomLevel};
pub use tiles::{Tile, TileGrid};

/// Representa um mapa cartográfico completo
pub struct Map {
    /// Projeção cartográfica utilizada
    projection: Box<dyn Projection>,

    /// Viewport atual (área visível)
    viewport: Viewport,

    /// Continentes incluídos no mapa
    continents: Vec<Box<dyn Continent>>,

    /// Features geográficas (países, cidades, etc)
    features: Vec<Box<dyn Feature>>,

    /// Opções de renderização
    render_options: RenderOptions,
}

impl Map {
    /// Cria um novo mapa com os continentes especificados
    pub fn new(continents: Vec<&str>) -> Result<Self, GeoError> {
        let mut continent_objects: Vec<Box<dyn Continent>> = Vec::new();

        for name in continents {
            match name.to_lowercase().as_str() {
                "europe" | "europa" => continent_objects.push(Box::new(Europe::new())),
                "africa" | "áfrica" => continent_objects.push(Box::new(Africa::new())),
                "asia" | "ásia" => continent_objects.push(Box::new(Asia::new())),
                _ => return Err(GeoError::InvalidContinent(name.to_string())),
            }
        }

        // Calcula viewport para incluir todos os continentes
        let bbox = Self::calculate_bounding_box(&continent_objects);
        let viewport = Viewport::from_bbox(bbox);

        Ok(Self {
            projection: Box::new(Mercator::new()),
            viewport,
            continents: continent_objects,
            features: Vec::new(),
            render_options: RenderOptions::default(),
        })
    }

    /// Define a projeção cartográfica
    pub fn set_projection(&mut self, projection: Box<dyn Projection>) {
        self.projection = projection;
    }

    /// Adiciona uma feature geográfica ao mapa
    pub fn add_feature(&mut self, feature: Box<dyn Feature>) {
        self.features.push(feature);
    }

    /// Renderiza o mapa
    pub fn render(&self, renderer: &mut dyn MapRenderer) -> Result<Vec<u8>, GeoError> {
        renderer.begin(self.viewport.width(), self.viewport.height())?;

        // Renderiza fundo
        renderer.draw_background(&self.render_options)?;

        // Renderiza cada continente
        for continent in &self.continents {
            self.render_continent(renderer, continent.as_ref())?;
        }

        // Renderiza features adicionais
        for feature in &self.features {
            feature.render(renderer, &self.projection, &self.viewport)?;
        }

        // Renderiza grade de coordenadas se habilitado
        if self.render_options.show_graticule {
            self.render_graticule(renderer)?;
        }

        renderer.end()
    }

    /// Renderiza um continente
    fn render_continent(&self, renderer: &mut dyn MapRenderer, continent: &dyn Continent) -> Result<(), GeoError> {
        let countries = continent.countries();

        for country in countries {
            // Converte coordenadas geográficas para coordenadas de tela
            let screen_coords = self.project_boundary(&country.boundary)?;

            // Desenha o polígono do país
            renderer.draw_polygon(&screen_coords, &country.style)?;

            // Desenha nome do país se zoom for adequado
            if self.viewport.zoom_level() >= ZoomLevel::Country {
                let label_pos = self.projection.project(&country.capital)?;
                renderer.draw_label(&country.name, label_pos)?;
            }
        }

        Ok(())
    }

    /// Renderiza grade de coordenadas (meridianos e paralelos)
    fn render_graticule(&self, renderer: &mut dyn MapRenderer) -> Result<(), GeoError> {
        let step = self.viewport.graticule_step();

        // Meridianos (longitude)
        for lon in (-180..=180).step_by(step as usize) {
            let mut points = Vec::new();
            for lat in (-90..=90).step_by(5) {
                let point = self.projection.project(&LatLon::new(lat as f64, lon as f64))?;
                points.push(point);
            }
            renderer.draw_line(&points, &self.render_options.graticule_style)?;
        }

        // Paralelos (latitude)
        for lat in (-90..=90).step_by(step as usize) {
            let mut points = Vec::new();
            for lon in (-180..=180).step_by(5) {
                let point = self.projection.project(&LatLon::new(lat as f64, lon as f64))?;
                points.push(point);
            }
            renderer.draw_line(&points, &self.render_options.graticule_style)?;
        }

        Ok(())
    }

    /// Projeta uma fronteira (sequência de coordenadas) para coordenadas de tela
    fn project_boundary(&self, boundary: &[LatLon]) -> Result<Vec<Point2D>, GeoError> {
        boundary.iter()
            .map(|coord| self.projection.project(coord))
            .collect()
    }

    /// Calcula bounding box que engloba todos os continentes
    fn calculate_bounding_box(continents: &[Box<dyn Continent>]) -> BoundingBox {
        let mut min_lat = 90.0;
        let mut max_lat = -90.0;
        let mut min_lon = 180.0;
        let mut max_lon = -180.0;

        for continent in continents {
            let bbox = continent.bounding_box();
            min_lat = min_lat.min(bbox.min_lat);
            max_lat = max_lat.max(bbox.max_lat);
            min_lon = min_lon.min(bbox.min_lon);
            max_lon = max_lon.max(bbox.max_lon);
        }

        BoundingBox::new(min_lat, max_lat, min_lon, max_lon)
    }

    /// Zoom in no mapa
    pub fn zoom_in(&mut self) {
        self.viewport.zoom_in();
    }

    /// Zoom out no mapa
    pub fn zoom_out(&mut self) {
        self.viewport.zoom_out();
    }

    /// Move o viewport (pan)
    pub fn pan(&mut self, delta_x: f64, delta_y: f64) {
        self.viewport.pan(delta_x, delta_y);
    }

    /// Centraliza o mapa em uma coordenada específica
    pub fn center_at(&mut self, coord: LatLon) {
        self.viewport.center_at(coord);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_map_europe() {
        let map = Map::new(vec!["europe"]);
        assert!(map.is_ok());
    }

    #[test]
    fn test_create_map_multiple_continents() {
        let map = Map::new(vec!["europe", "africa", "asia"]);
        assert!(map.is_ok());
    }

    #[test]
    fn test_invalid_continent() {
        let map = Map::new(vec!["atlantis"]);
        assert!(map.is_err());
    }
}
