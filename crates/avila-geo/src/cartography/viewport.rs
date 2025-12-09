//! Sistema de viewport e níveis de zoom

use super::coordinates::{LatLon, BoundingBox};

/// Níveis de zoom
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ZoomLevel {
    /// Mundo inteiro
    World = 0,
    /// Continente
    Continent = 1,
    /// País
    Country = 2,
    /// Região/Estado
    Region = 3,
    /// Cidade
    City = 4,
    /// Bairro
    Neighborhood = 5,
    /// Rua
    Street = 6,
}

impl ZoomLevel {
    /// Fator de escala para o nível de zoom
    pub fn scale_factor(&self) -> f64 {
        match self {
            ZoomLevel::World => 1.0,
            ZoomLevel::Continent => 2.0,
            ZoomLevel::Country => 4.0,
            ZoomLevel::Region => 8.0,
            ZoomLevel::City => 16.0,
            ZoomLevel::Neighborhood => 32.0,
            ZoomLevel::Street => 64.0,
        }
    }

    /// Próximo nível de zoom (zoom in)
    pub fn next(&self) -> Option<ZoomLevel> {
        match self {
            ZoomLevel::World => Some(ZoomLevel::Continent),
            ZoomLevel::Continent => Some(ZoomLevel::Country),
            ZoomLevel::Country => Some(ZoomLevel::Region),
            ZoomLevel::Region => Some(ZoomLevel::City),
            ZoomLevel::City => Some(ZoomLevel::Neighborhood),
            ZoomLevel::Neighborhood => Some(ZoomLevel::Street),
            ZoomLevel::Street => None,
        }
    }

    /// Nível anterior (zoom out)
    pub fn previous(&self) -> Option<ZoomLevel> {
        match self {
            ZoomLevel::World => None,
            ZoomLevel::Continent => Some(ZoomLevel::World),
            ZoomLevel::Country => Some(ZoomLevel::Continent),
            ZoomLevel::Region => Some(ZoomLevel::Country),
            ZoomLevel::City => Some(ZoomLevel::Region),
            ZoomLevel::Neighborhood => Some(ZoomLevel::City),
            ZoomLevel::Street => Some(ZoomLevel::Neighborhood),
        }
    }
}

/// Viewport (área visível do mapa)
pub struct Viewport {
    /// Centro do viewport
    center: LatLon,

    /// Bounding box da área visível
    bbox: BoundingBox,

    /// Nível de zoom
    zoom_level: ZoomLevel,

    /// Largura em pixels
    width_px: u32,

    /// Altura em pixels
    height_px: u32,
}

impl Viewport {
    /// Cria viewport a partir de um bounding box
    pub fn from_bbox(bbox: BoundingBox) -> Self {
        Self {
            center: bbox.center(),
            bbox,
            zoom_level: ZoomLevel::Continent,
            width_px: 1920,
            height_px: 1080,
        }
    }

    /// Cria viewport centrado em uma coordenada
    pub fn centered_at(center: LatLon, zoom_level: ZoomLevel) -> Self {
        let scale = zoom_level.scale_factor();
        let delta_lat = 45.0 / scale;
        let delta_lon = 80.0 / scale;

        let bbox = BoundingBox::new(
            center.lat - delta_lat,
            center.lat + delta_lat,
            center.lon - delta_lon,
            center.lon + delta_lon,
        );

        Self {
            center,
            bbox,
            zoom_level,
            width_px: 1920,
            height_px: 1080,
        }
    }

    /// Retorna o centro do viewport
    pub fn center(&self) -> LatLon {
        self.center
    }

    /// Retorna o bounding box
    pub fn bbox(&self) -> BoundingBox {
        self.bbox
    }

    /// Retorna o nível de zoom
    pub fn zoom_level(&self) -> ZoomLevel {
        self.zoom_level
    }

    /// Largura em pixels
    pub fn width(&self) -> u32 {
        self.width_px
    }

    /// Altura em pixels
    pub fn height(&self) -> u32 {
        self.height_px
    }

    /// Define dimensões em pixels
    pub fn set_dimensions(&mut self, width: u32, height: u32) {
        self.width_px = width;
        self.height_px = height;
    }

    /// Zoom in (aumenta o zoom)
    pub fn zoom_in(&mut self) {
        if let Some(next_level) = self.zoom_level.next() {
            self.zoom_level = next_level;
            self.update_bbox();
        }
    }

    /// Zoom out (diminui o zoom)
    pub fn zoom_out(&mut self) {
        if let Some(prev_level) = self.zoom_level.previous() {
            self.zoom_level = prev_level;
            self.update_bbox();
        }
    }

    /// Pan (move o viewport)
    pub fn pan(&mut self, delta_lat: f64, delta_lon: f64) {
        let new_lat = (self.center.lat + delta_lat).clamp(-85.0, 85.0);
        let new_lon = (self.center.lon + delta_lon).clamp(-180.0, 180.0);

        self.center = LatLon::new(new_lat, new_lon);
        self.update_bbox();
    }

    /// Centraliza em uma coordenada
    pub fn center_at(&mut self, coord: LatLon) {
        self.center = coord;
        self.update_bbox();
    }

    /// Atualiza o bounding box baseado no centro e zoom
    fn update_bbox(&mut self) {
        let scale = self.zoom_level.scale_factor();
        let delta_lat = 45.0 / scale;
        let delta_lon = 80.0 / scale;

        self.bbox = BoundingBox::new(
            (self.center.lat - delta_lat).max(-90.0),
            (self.center.lat + delta_lat).min(90.0),
            (self.center.lon - delta_lon).max(-180.0),
            (self.center.lon + delta_lon).min(180.0),
        );
    }

    /// Retorna o passo da grade de coordenadas baseado no zoom
    pub fn graticule_step(&self) -> i32 {
        match self.zoom_level {
            ZoomLevel::World => 30,
            ZoomLevel::Continent => 15,
            ZoomLevel::Country => 5,
            ZoomLevel::Region => 2,
            ZoomLevel::City => 1,
            ZoomLevel::Neighborhood => 1,
            ZoomLevel::Street => 1,
        }
    }

    /// Verifica se uma coordenada está visível no viewport
    pub fn is_visible(&self, coord: &LatLon) -> bool {
        self.bbox.contains(coord)
    }

    /// Retorna a resolução em graus por pixel
    pub fn resolution(&self) -> (f64, f64) {
        let lat_per_px = self.bbox.height() / self.height_px as f64;
        let lon_per_px = self.bbox.width() / self.width_px as f64;
        (lat_per_px, lon_per_px)
    }
}

impl Default for Viewport {
    fn default() -> Self {
        // Viewport do mundo centrado no meridiano de Greenwich
        Self::centered_at(LatLon::new(20.0, 10.0), ZoomLevel::World)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zoom_levels() {
        let mut viewport = Viewport::default();

        let initial_zoom = viewport.zoom_level();
        viewport.zoom_in();
        assert!(viewport.zoom_level() > initial_zoom);

        viewport.zoom_out();
        assert_eq!(viewport.zoom_level(), initial_zoom);
    }

    #[test]
    fn test_pan() {
        let mut viewport = Viewport::centered_at(LatLon::new(0.0, 0.0), ZoomLevel::Country);

        viewport.pan(10.0, 10.0);
        let center = viewport.center();

        assert!((center.lat - 10.0).abs() < 0.01);
        assert!((center.lon - 10.0).abs() < 0.01);
    }

    #[test]
    fn test_visibility() {
        let viewport = Viewport::centered_at(LatLon::new(50.0, 10.0), ZoomLevel::Continent);

        assert!(viewport.is_visible(&LatLon::new(50.0, 10.0))); // Centro
        assert!(!viewport.is_visible(&LatLon::new(-50.0, -100.0))); // Longe
    }
}
