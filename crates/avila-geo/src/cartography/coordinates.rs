//! Sistema de coordenadas geográficas e cartesianas

use std::fmt;

/// Coordenada geográfica (latitude, longitude)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LatLon {
    /// Latitude em graus (-90 a 90)
    pub lat: f64,
    /// Longitude em graus (-180 a 180)
    pub lon: f64,
}

impl LatLon {
    /// Cria uma nova coordenada geográfica
    pub fn new(lat: f64, lon: f64) -> Self {
        assert!(lat >= -90.0 && lat <= 90.0, "Latitude deve estar entre -90 e 90");
        assert!(lon >= -180.0 && lon <= 180.0, "Longitude deve estar entre -180 e 180");
        Self { lat, lon }
    }

    /// Converte para radianos
    pub fn to_radians(&self) -> (f64, f64) {
        (self.lat.to_radians(), self.lon.to_radians())
    }

    /// Calcula distância para outra coordenada (fórmula de Haversine)
    pub fn distance_to(&self, other: &LatLon) -> f64 {
        const EARTH_RADIUS_KM: f64 = 6371.0;

        let (lat1, lon1) = self.to_radians();
        let (lat2, lon2) = other.to_radians();

        let dlat = lat2 - lat1;
        let dlon = lon2 - lon1;

        let a = (dlat / 2.0).sin().powi(2)
              + lat1.cos() * lat2.cos() * (dlon / 2.0).sin().powi(2);
        let c = 2.0 * a.sqrt().atan2((1.0 - a).sqrt());

        EARTH_RADIUS_KM * c
    }
}

impl fmt::Display for LatLon {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let lat_dir = if self.lat >= 0.0 { "N" } else { "S" };
        let lon_dir = if self.lon >= 0.0 { "E" } else { "W" };
        write!(f, "{:.4}°{}, {:.4}°{}", self.lat.abs(), lat_dir, self.lon.abs(), lon_dir)
    }
}

/// Ponto 2D no espaço cartesiano (coordenadas de tela)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point2D {
    pub x: f64,
    pub y: f64,
}

impl Point2D {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    pub fn distance_to(&self, other: &Point2D) -> f64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        (dx * dx + dy * dy).sqrt()
    }
}

/// Bounding box (caixa delimitadora)
#[derive(Debug, Clone, Copy)]
pub struct BoundingBox {
    pub min_lat: f64,
    pub max_lat: f64,
    pub min_lon: f64,
    pub max_lon: f64,
}

impl BoundingBox {
    pub fn new(min_lat: f64, max_lat: f64, min_lon: f64, max_lon: f64) -> Self {
        Self { min_lat, max_lat, min_lon, max_lon }
    }

    /// Retorna o centro do bounding box
    pub fn center(&self) -> LatLon {
        LatLon::new(
            (self.min_lat + self.max_lat) / 2.0,
            (self.min_lon + self.max_lon) / 2.0,
        )
    }

    /// Retorna a largura em graus
    pub fn width(&self) -> f64 {
        self.max_lon - self.min_lon
    }

    /// Retorna a altura em graus
    pub fn height(&self) -> f64 {
        self.max_lat - self.min_lat
    }

    /// Verifica se contém uma coordenada
    pub fn contains(&self, coord: &LatLon) -> bool {
        coord.lat >= self.min_lat && coord.lat <= self.max_lat &&
        coord.lon >= self.min_lon && coord.lon <= self.max_lon
    }

    /// Expande o bounding box para incluir uma coordenada
    pub fn expand_to_include(&mut self, coord: &LatLon) {
        self.min_lat = self.min_lat.min(coord.lat);
        self.max_lat = self.max_lat.max(coord.lat);
        self.min_lon = self.min_lon.min(coord.lon);
        self.max_lon = self.max_lon.max(coord.lon);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_latlon_distance() {
        // Distância aproximada entre Lisboa e Paris
        let lisbon = LatLon::new(38.7223, -9.1393);
        let paris = LatLon::new(48.8566, 2.3522);
        let distance = lisbon.distance_to(&paris);

        // Distância real é ~1450 km
        assert!((distance - 1450.0).abs() < 50.0);
    }

    #[test]
    fn test_bounding_box() {
        let bbox = BoundingBox::new(35.0, 71.0, -25.0, 45.0);
        let center = bbox.center();

        assert_eq!(center.lat, 53.0);
        assert_eq!(center.lon, 10.0);
    }
}
