//! Projeções cartográficas
//!
//! Implementa diferentes projeções para converter coordenadas geográficas (lat/lon)
//! em coordenadas cartesianas 2D (x/y) para renderização em mapas.

use crate::error::GeoError;
use super::coordinates::{LatLon, Point2D};
use std::f64::consts::PI;

/// Trait para projeções cartográficas
pub trait Projection: Send + Sync {
    /// Projeta coordenada geográfica para coordenada cartesiana
    fn project(&self, coord: &LatLon) -> Result<Point2D, GeoError>;

    /// Projeta coordenada cartesiana de volta para geográfica (inversa)
    fn inverse(&self, point: &Point2D) -> Result<LatLon, GeoError>;

    /// Nome da projeção
    fn name(&self) -> &str;
}

/// Projeção de Mercator (Web Mercator - EPSG:3857)
///
/// Usada pelo Google Maps, OpenStreetMap, etc.
/// Preserva ângulos mas distorce áreas (especialmente próximo aos polos)
pub struct Mercator {
    /// Raio da Terra em metros
    earth_radius: f64,
}

impl Mercator {
    pub fn new() -> Self {
        Self {
            earth_radius: 6378137.0, // WGS84
        }
    }
}

impl Default for Mercator {
    fn default() -> Self {
        Self::new()
    }
}

impl Projection for Mercator {
    fn project(&self, coord: &LatLon) -> Result<Point2D, GeoError> {
        let lon_rad = coord.lon.to_radians();
        let lat_rad = coord.lat.to_radians();

        // Limita latitude para evitar infinito nos polos
        let lat_rad = lat_rad.clamp(-85.0_f64.to_radians(), 85.0_f64.to_radians());

        let x = self.earth_radius * lon_rad;
        let y = self.earth_radius * ((PI / 4.0 + lat_rad / 2.0).tan().ln());

        Ok(Point2D::new(x, y))
    }

    fn inverse(&self, point: &Point2D) -> Result<LatLon, GeoError> {
        let lon = (point.x / self.earth_radius).to_degrees();
        let lat = (2.0 * (point.y / self.earth_radius).exp().atan() - PI / 2.0).to_degrees();

        Ok(LatLon::new(lat, lon))
    }

    fn name(&self) -> &str {
        "Web Mercator (EPSG:3857)"
    }
}

/// Projeção de Robinson
///
/// Projeção de compromisso que equilibra distorção de área e forma.
/// Comumente usada para mapas-múndi.
pub struct Robinson;

impl Robinson {
    pub fn new() -> Self {
        Self
    }

    // Tabela de coeficientes para latitude
    const LAT_TABLE: [(f64, f64, f64); 19] = [
        (0.0, 1.0000, 0.0000),
        (5.0, 0.9986, 0.0620),
        (10.0, 0.9954, 0.1240),
        (15.0, 0.9900, 0.1860),
        (20.0, 0.9822, 0.2480),
        (25.0, 0.9730, 0.3100),
        (30.0, 0.9600, 0.3720),
        (35.0, 0.9427, 0.4340),
        (40.0, 0.9216, 0.4958),
        (45.0, 0.8962, 0.5571),
        (50.0, 0.8679, 0.6176),
        (55.0, 0.8350, 0.6769),
        (60.0, 0.7986, 0.7346),
        (65.0, 0.7597, 0.7903),
        (70.0, 0.7186, 0.8435),
        (75.0, 0.6732, 0.8936),
        (80.0, 0.6213, 0.9394),
        (85.0, 0.5722, 0.9761),
        (90.0, 0.5322, 1.0000),
    ];

    fn interpolate(lat: f64) -> (f64, f64) {
        let abs_lat = lat.abs();

        for i in 0..Self::LAT_TABLE.len() - 1 {
            let (lat1, x1, y1) = Self::LAT_TABLE[i];
            let (lat2, x2, y2) = Self::LAT_TABLE[i + 1];

            if abs_lat >= lat1 && abs_lat <= lat2 {
                let ratio = (abs_lat - lat1) / (lat2 - lat1);
                let x = x1 + ratio * (x2 - x1);
                let y = y1 + ratio * (y2 - y1);
                return (x, y);
            }
        }

        (Self::LAT_TABLE.last().unwrap().1, Self::LAT_TABLE.last().unwrap().2)
    }
}

impl Default for Robinson {
    fn default() -> Self {
        Self::new()
    }
}

impl Projection for Robinson {
    fn project(&self, coord: &LatLon) -> Result<Point2D, GeoError> {
        const SCALE: f64 = 100.0; // Fator de escala

        let (x_coef, y_coef) = Self::interpolate(coord.lat);

        let x = SCALE * x_coef * coord.lon.to_radians();
        let y = SCALE * y_coef * coord.lat.signum();

        Ok(Point2D::new(x, y))
    }

    fn inverse(&self, _point: &Point2D) -> Result<LatLon, GeoError> {
        // Inversa da Robinson é complexa, implementação simplificada
        Err(GeoError::UnsupportedOperation("Robinson inverse projection not implemented".into()))
    }

    fn name(&self) -> &str {
        "Robinson"
    }
}

/// Projeção Albers Equal Area (cônica)
///
/// Preserva áreas, ideal para mapas de continentes específicos.
/// Requer paralelos padrão para configuração.
pub struct AlbersEqualArea {
    /// Latitude de origem
    lat0: f64,
    /// Longitude de origem
    lon0: f64,
    /// Primeiro paralelo padrão
    lat1: f64,
    /// Segundo paralelo padrão
    lat2: f64,
    /// Parâmetros calculados
    n: f64,
    c: f64,
    rho0: f64,
}

impl AlbersEqualArea {
    /// Cria uma nova projeção Albers
    ///
    /// # Parâmetros
    /// - `lat0`: Latitude de origem
    /// - `lon0`: Longitude de origem
    /// - `lat1`: Primeiro paralelo padrão
    /// - `lat2`: Segundo paralelo padrão
    pub fn new(lat0: f64, lon0: f64, lat1: f64, lat2: f64) -> Self {
        let lat0_rad = lat0.to_radians();
        let lat1_rad = lat1.to_radians();
        let lat2_rad = lat2.to_radians();

        let n = (lat1_rad.cos().powi(2) + lat2_rad.sin().powi(2) -
                 lat2_rad.cos().powi(2) - lat1_rad.sin().powi(2)) /
                (2.0 * (lat2_rad.sin() - lat1_rad.sin()));

        let c = lat1_rad.cos().powi(2) + 2.0 * n * lat1_rad.sin();
        let rho0 = (c - 2.0 * n * lat0_rad.sin()).sqrt() / n;

        Self {
            lat0,
            lon0,
            lat1,
            lat2,
            n,
            c,
            rho0,
        }
    }

    /// Configuração para Europa
    pub fn for_europe() -> Self {
        Self::new(50.0, 10.0, 43.0, 62.0)
    }

    /// Configuração para África
    pub fn for_africa() -> Self {
        Self::new(0.0, 20.0, -15.0, 25.0)
    }

    /// Configuração para Ásia
    pub fn for_asia() -> Self {
        Self::new(40.0, 100.0, 15.0, 65.0)
    }
}

impl Projection for AlbersEqualArea {
    fn project(&self, coord: &LatLon) -> Result<Point2D, GeoError> {
        const SCALE: f64 = 1000000.0;

        let lat_rad = coord.lat.to_radians();
        let lon_rad = coord.lon.to_radians();
        let lon0_rad = self.lon0.to_radians();

        let theta = self.n * (lon_rad - lon0_rad);
        let rho = (self.c - 2.0 * self.n * lat_rad.sin()).sqrt() / self.n;

        let x = SCALE * rho * theta.sin();
        let y = SCALE * (self.rho0 - rho * theta.cos());

        Ok(Point2D::new(x, y))
    }

    fn inverse(&self, point: &Point2D) -> Result<LatLon, GeoError> {
        const SCALE: f64 = 1000000.0;

        let x = point.x / SCALE;
        let y = point.y / SCALE;

        let rho = ((x.powi(2) + (self.rho0 - y).powi(2)).sqrt()).copysign(self.n);
        let theta = (x / rho).atan();

        let lon = theta / self.n + self.lon0.to_radians();
        let lat = ((self.c - (rho * self.n).powi(2)) / (2.0 * self.n)).asin();

        Ok(LatLon::new(lat.to_degrees(), lon.to_degrees()))
    }

    fn name(&self) -> &str {
        "Albers Equal Area Conic"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mercator_projection() {
        let mercator = Mercator::new();
        let lisbon = LatLon::new(38.7223, -9.1393);

        let projected = mercator.project(&lisbon).unwrap();
        let inverse = mercator.inverse(&projected).unwrap();

        assert!((inverse.lat - lisbon.lat).abs() < 0.001);
        assert!((inverse.lon - lisbon.lon).abs() < 0.001);
    }

    #[test]
    fn test_robinson_projection() {
        let robinson = Robinson::new();
        let berlin = LatLon::new(52.5200, 13.4050);

        let projected = robinson.project(&berlin);
        assert!(projected.is_ok());
    }
}
