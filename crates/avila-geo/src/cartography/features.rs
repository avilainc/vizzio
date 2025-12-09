//! Features geográficas (cidades, rios, montanhas, etc)

use crate::error::GeoError;
use super::coordinates::{LatLon, Point2D};
use super::projection::Projection;
use super::viewport::Viewport;
use super::renderer::{MapRenderer, Style};

/// Trait para features geográficas
pub trait Feature: Send + Sync {
    /// Renderiza a feature
    fn render(
        &self,
        renderer: &mut dyn MapRenderer,
        projection: &Box<dyn Projection>,
        viewport: &Viewport,
    ) -> Result<(), GeoError>;

    /// Nome da feature
    fn name(&self) -> &str;

    /// Tipo da feature
    fn feature_type(&self) -> FeatureType;
}

/// Tipos de features
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FeatureType {
    Country,
    City,
    River,
    Mountain,
    Lake,
    Road,
    Border,
}

/// Cidade
pub struct City {
    pub name: String,
    pub location: LatLon,
    pub population: u64,
    pub is_capital: bool,
}

impl City {
    pub fn new(name: &str, location: LatLon, population: u64) -> Self {
        Self {
            name: name.to_string(),
            location,
            population,
            is_capital: false,
        }
    }

    pub fn capital(name: &str, location: LatLon, population: u64) -> Self {
        Self {
            name: name.to_string(),
            location,
            population,
            is_capital: true,
        }
    }

    fn marker_size(&self) -> f64 {
        if self.is_capital {
            8.0
        } else if self.population > 5_000_000 {
            6.0
        } else if self.population > 1_000_000 {
            4.0
        } else {
            2.0
        }
    }
}

impl Feature for City {
    fn render(
        &self,
        renderer: &mut dyn MapRenderer,
        projection: &Box<dyn Projection>,
        viewport: &Viewport,
    ) -> Result<(), GeoError> {
        if !viewport.is_visible(&self.location) {
            return Ok(());
        }

        let center = projection.project(&self.location)?;
        let size = self.marker_size();

        // Desenha círculo para a cidade
        let circle_points = self.create_circle(center, size);
        let mut style = Style::default();
        style.fill_color = if self.is_capital {
            [255, 0, 0, 255] // Vermelho para capitais
        } else {
            [0, 0, 0, 255] // Preto para outras cidades
        };

        renderer.draw_polygon(&circle_points, &style)?;

        // Desenha nome se cidade for grande o suficiente
        if self.population > 1_000_000 {
            let label_pos = Point2D::new(center.x + size + 2.0, center.y);
            renderer.draw_label(&self.name, label_pos)?;
        }

        Ok(())
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn feature_type(&self) -> FeatureType {
        FeatureType::City
    }
}

impl City {
    fn create_circle(&self, center: Point2D, radius: f64) -> Vec<Point2D> {
        let segments = 16;
        let mut points = Vec::with_capacity(segments);

        for i in 0..segments {
            let angle = 2.0 * std::f64::consts::PI * i as f64 / segments as f64;
            let x = center.x + radius * angle.cos();
            let y = center.y + radius * angle.sin();
            points.push(Point2D::new(x, y));
        }

        points
    }
}

/// Rio
pub struct River {
    pub name: String,
    pub path: Vec<LatLon>,
    pub length_km: f64,
}

impl River {
    pub fn new(name: &str, path: Vec<LatLon>) -> Self {
        let length = Self::calculate_length(&path);
        Self {
            name: name.to_string(),
            path,
            length_km: length,
        }
    }

    fn calculate_length(path: &[LatLon]) -> f64 {
        let mut total = 0.0;
        for i in 0..path.len().saturating_sub(1) {
            total += path[i].distance_to(&path[i + 1]);
        }
        total
    }
}

impl Feature for River {
    fn render(
        &self,
        renderer: &mut dyn MapRenderer,
        projection: &Box<dyn Projection>,
        viewport: &Viewport,
    ) -> Result<(), GeoError> {
        // Projeta todos os pontos do rio
        let projected: Result<Vec<Point2D>, _> = self.path
            .iter()
            .filter(|coord| viewport.is_visible(coord))
            .map(|coord| projection.project(coord))
            .collect();

        let points = projected?;

        if points.len() < 2 {
            return Ok(());
        }

        // Estilo de rio
        let mut style = Style::water();
        style.stroke_width = (self.length_km / 1000.0).max(0.5) as f32;

        renderer.draw_line(&points, &style)?;

        Ok(())
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn feature_type(&self) -> FeatureType {
        FeatureType::River
    }
}

/// Montanha
pub struct Mountain {
    pub name: String,
    pub peak: LatLon,
    pub elevation_m: f64,
}

impl Mountain {
    pub fn new(name: &str, peak: LatLon, elevation_m: f64) -> Self {
        Self {
            name: name.to_string(),
            peak,
            elevation_m,
        }
    }
}

impl Feature for Mountain {
    fn render(
        &self,
        renderer: &mut dyn MapRenderer,
        projection: &Box<dyn Projection>,
        viewport: &Viewport,
    ) -> Result<(), GeoError> {
        if !viewport.is_visible(&self.peak) {
            return Ok(());
        }

        let center = projection.project(&self.peak)?;
        let size = (self.elevation_m / 1000.0).min(10.0);

        // Desenha triângulo para montanha
        let triangle = vec![
            Point2D::new(center.x, center.y - size),
            Point2D::new(center.x - size * 0.866, center.y + size * 0.5),
            Point2D::new(center.x + size * 0.866, center.y + size * 0.5),
        ];

        let mut style = Style::default();
        style.fill_color = [139, 90, 43, 255]; // Marrom

        renderer.draw_polygon(&triangle, &style)?;

        // Label com elevação
        let label = format!("{} ({}m)", self.name, self.elevation_m as i32);
        let label_pos = Point2D::new(center.x, center.y + size + 5.0);
        renderer.draw_label(&label, label_pos)?;

        Ok(())
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn feature_type(&self) -> FeatureType {
        FeatureType::Mountain
    }
}

/// Catálogo de cidades importantes
pub struct CityDatabase;

impl CityDatabase {
    /// Retorna cidades importantes da Europa
    pub fn european_cities() -> Vec<City> {
        vec![
            City::capital("Lisboa", LatLon::new(38.7223, -9.1393), 505_000),
            City::capital("Madrid", LatLon::new(40.4168, -3.7038), 3_300_000),
            City::capital("Paris", LatLon::new(48.8566, 2.3522), 2_160_000),
            City::capital("Londres", LatLon::new(51.5074, -0.1278), 9_000_000),
            City::capital("Berlim", LatLon::new(52.5200, 13.4050), 3_650_000),
            City::capital("Roma", LatLon::new(41.9028, 12.4964), 2_870_000),
            City::capital("Atenas", LatLon::new(37.9838, 23.7275), 664_000),
            City::new("Barcelona", LatLon::new(41.3874, 2.1686), 1_620_000),
            City::new("Milão", LatLon::new(45.4642, 9.1900), 1_350_000),
            City::new("Munique", LatLon::new(48.1351, 11.5820), 1_470_000),
        ]
    }

    /// Retorna cidades importantes da África
    pub fn african_cities() -> Vec<City> {
        vec![
            City::capital("Cairo", LatLon::new(30.0444, 31.2357), 20_900_000),
            City::capital("Lagos", LatLon::new(6.5244, 3.3792), 14_860_000),
            City::capital("Kinshasa", LatLon::new(-4.3217, 15.3125), 14_340_000),
            City::capital("Joanesburgo", LatLon::new(-26.2041, 28.0473), 5_635_000),
            City::capital("Nairobi", LatLon::new(-1.2921, 36.8219), 4_400_000),
            City::capital("Casablanca", LatLon::new(33.5731, -7.5898), 3_360_000),
            City::capital("Adis Abeba", LatLon::new(9.0250, 38.7469), 3_040_000),
        ]
    }

    /// Retorna cidades importantes da Ásia
    pub fn asian_cities() -> Vec<City> {
        vec![
            City::capital("Tóquio", LatLon::new(35.6762, 139.6503), 37_400_000),
            City::capital("Pequim", LatLon::new(39.9042, 116.4074), 21_540_000),
            City::capital("Mumbai", LatLon::new(19.0760, 72.8777), 20_410_000),
            City::capital("Xangai", LatLon::new(31.2304, 121.4737), 27_060_000),
            City::capital("Délhi", LatLon::new(28.7041, 77.1025), 30_290_000),
            City::capital("Seul", LatLon::new(37.5665, 126.9780), 9_770_000),
            City::capital("Jacarta", LatLon::new(-6.2088, 106.8456), 10_560_000),
            City::capital("Bangkok", LatLon::new(13.7563, 100.5018), 10_720_000),
            City::new("Hong Kong", LatLon::new(22.3193, 114.1694), 7_500_000),
            City::new("Singapura", LatLon::new(1.3521, 103.8198), 5_690_000),
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_city_database() {
        let european = CityDatabase::european_cities();
        assert!(european.len() >= 10);

        let african = CityDatabase::african_cities();
        assert!(african.len() >= 5);

        let asian = CityDatabase::asian_cities();
        assert!(asian.len() >= 8);
    }
}
