//! Definição de continentes e seus países

use crate::error::GeoError;
use super::coordinates::{LatLon, BoundingBox};
use super::renderer::Style;

/// Trait para representar um continente
pub trait Continent: Send + Sync {
    /// Nome do continente
    fn name(&self) -> &str;

    /// Lista de países no continente
    fn countries(&self) -> Vec<Country>;

    /// Bounding box do continente
    fn bounding_box(&self) -> BoundingBox;

    /// População total
    fn population(&self) -> u64;

    /// Área em km²
    fn area(&self) -> f64;
}

/// Representa um país
#[derive(Clone)]
pub struct Country {
    /// Nome do país
    pub name: String,

    /// Código ISO 3166-1 alpha-3
    pub code: String,

    /// Capital
    pub capital: LatLon,

    /// Fronteira (polígono simplificado)
    pub boundary: Vec<LatLon>,

    /// População
    pub population: u64,

    /// Área em km²
    pub area: f64,

    /// Estilo de renderização
    pub style: Style,
}

impl Country {
    pub fn new(name: &str, code: &str, capital: LatLon) -> Self {
        Self {
            name: name.to_string(),
            code: code.to_string(),
            capital,
            boundary: Vec::new(),
            population: 0,
            area: 0.0,
            style: Style::default(),
        }
    }

    pub fn with_boundary(mut self, boundary: Vec<LatLon>) -> Self {
        self.boundary = boundary;
        self
    }

    pub fn with_population(mut self, population: u64) -> Self {
        self.population = population;
        self
    }

    pub fn with_area(mut self, area: f64) -> Self {
        self.area = area;
        self
    }
}

/// Europa
pub struct Europe;

impl Europe {
    pub fn new() -> Self {
        Self
    }

    fn load_countries() -> Vec<Country> {
        vec![
            // Portugal
            Country::new("Portugal", "PRT", LatLon::new(38.7223, -9.1393))
                .with_boundary(vec![
                    LatLon::new(42.15, -8.67), // NO
                    LatLon::new(42.15, -6.19), // NE
                    LatLon::new(38.0, -7.0),   // E
                    LatLon::new(37.0, -9.0),   // S
                    LatLon::new(37.0, -9.5),   // SO
                    LatLon::new(42.15, -8.67), // Volta ao início
                ])
                .with_population(10_300_000)
                .with_area(92_090.0),

            // Espanha
            Country::new("Espanha", "ESP", LatLon::new(40.4168, -3.7038))
                .with_boundary(vec![
                    LatLon::new(43.79, -7.04), // NO
                    LatLon::new(43.79, 3.32),  // NE
                    LatLon::new(36.0, 3.32),   // SE
                    LatLon::new(36.0, -6.36),  // SO
                    LatLon::new(43.79, -7.04), // Volta
                ])
                .with_population(47_400_000)
                .with_area(505_990.0),

            // França
            Country::new("França", "FRA", LatLon::new(48.8566, 2.3522))
                .with_boundary(vec![
                    LatLon::new(51.09, 2.54),  // N
                    LatLon::new(51.09, 8.23),  // NE
                    LatLon::new(42.33, 3.16),  // S
                    LatLon::new(43.5, -1.48),  // SO
                    LatLon::new(51.09, 2.54),  // Volta
                ])
                .with_population(67_750_000)
                .with_area(551_695.0),

            // Alemanha
            Country::new("Alemanha", "DEU", LatLon::new(52.5200, 13.4050))
                .with_boundary(vec![
                    LatLon::new(55.06, 8.65),  // NO
                    LatLon::new(54.98, 14.41), // NE
                    LatLon::new(47.27, 13.84), // SE
                    LatLon::new(47.53, 7.59),  // SO
                    LatLon::new(55.06, 8.65),  // Volta
                ])
                .with_population(83_200_000)
                .with_area(357_022.0),

            // Itália
            Country::new("Itália", "ITA", LatLon::new(41.9028, 12.4964))
                .with_boundary(vec![
                    LatLon::new(47.09, 6.63),  // N
                    LatLon::new(46.64, 13.69), // NE
                    LatLon::new(36.65, 15.08), // S
                    LatLon::new(38.77, 8.18),  // O
                    LatLon::new(47.09, 6.63),  // Volta
                ])
                .with_population(59_550_000)
                .with_area(301_340.0),

            // Reino Unido
            Country::new("Reino Unido", "GBR", LatLon::new(51.5074, -0.1278))
                .with_boundary(vec![
                    LatLon::new(60.85, -1.27), // N Escócia
                    LatLon::new(58.63, 1.77),  // NE
                    LatLon::new(50.05, 1.76),  // SE
                    LatLon::new(50.05, -5.71), // SO
                    LatLon::new(60.85, -1.27), // Volta
                ])
                .with_population(67_330_000)
                .with_area(242_495.0),

            // Polónia
            Country::new("Polónia", "POL", LatLon::new(52.2297, 21.0122))
                .with_boundary(vec![
                    LatLon::new(54.84, 14.12), // NO
                    LatLon::new(54.36, 24.15), // NE
                    LatLon::new(49.00, 24.15), // SE
                    LatLon::new(49.00, 14.12), // SO
                    LatLon::new(54.84, 14.12), // Volta
                ])
                .with_population(38_000_000)
                .with_area(312_696.0),

            // Ucrânia
            Country::new("Ucrânia", "UKR", LatLon::new(50.4501, 30.5234))
                .with_boundary(vec![
                    LatLon::new(52.38, 22.14), // NO
                    LatLon::new(52.38, 40.23), // NE
                    LatLon::new(44.39, 40.23), // SE
                    LatLon::new(45.47, 22.14), // SO
                    LatLon::new(52.38, 22.14), // Volta
                ])
                .with_population(41_000_000)
                .with_area(603_628.0),

            // Grécia
            Country::new("Grécia", "GRC", LatLon::new(37.9838, 23.7275))
                .with_boundary(vec![
                    LatLon::new(41.75, 19.37), // NO
                    LatLon::new(41.75, 28.25), // NE
                    LatLon::new(34.80, 28.25), // SE
                    LatLon::new(36.39, 19.37), // SO
                    LatLon::new(41.75, 19.37), // Volta
                ])
                .with_population(10_640_000)
                .with_area(131_957.0),

            // Suécia
            Country::new("Suécia", "SWE", LatLon::new(59.3293, 18.0686))
                .with_boundary(vec![
                    LatLon::new(69.06, 11.03), // NO
                    LatLon::new(69.06, 24.17), // NE
                    LatLon::new(55.34, 24.17), // SE
                    LatLon::new(55.34, 11.03), // SO
                    LatLon::new(69.06, 11.03), // Volta
                ])
                .with_population(10_420_000)
                .with_area(450_295.0),
        ]
    }
}

impl Default for Europe {
    fn default() -> Self {
        Self::new()
    }
}

impl Continent for Europe {
    fn name(&self) -> &str {
        "Europa"
    }

    fn countries(&self) -> Vec<Country> {
        Self::load_countries()
    }

    fn bounding_box(&self) -> BoundingBox {
        // Europa: Islândia ao Urais, Escandinávia ao Mediterrâneo
        BoundingBox::new(
            35.0,  // Sul (Creta, Grécia)
            71.0,  // Norte (Cabo Norte, Noruega)
            -25.0, // Oeste (Islândia)
            45.0,  // Leste (Urais, Rússia)
        )
    }

    fn population(&self) -> u64 {
        748_000_000
    }

    fn area(&self) -> f64 {
        10_180_000.0
    }
}

/// África
pub struct Africa;

impl Africa {
    pub fn new() -> Self {
        Self
    }

    fn load_countries() -> Vec<Country> {
        vec![
            // Egito
            Country::new("Egito", "EGY", LatLon::new(30.0444, 31.2357))
                .with_boundary(vec![
                    LatLon::new(31.67, 25.00), // NO
                    LatLon::new(31.67, 36.90), // NE
                    LatLon::new(22.00, 36.90), // SE
                    LatLon::new(22.00, 25.00), // SO
                    LatLon::new(31.67, 25.00), // Volta
                ])
                .with_population(104_000_000)
                .with_area(1_010_408.0),

            // África do Sul
            Country::new("África do Sul", "ZAF", LatLon::new(-25.7479, 28.2293))
                .with_boundary(vec![
                    LatLon::new(-22.13, 16.46), // NO
                    LatLon::new(-22.13, 32.89), // NE
                    LatLon::new(-34.84, 32.89), // SE
                    LatLon::new(-34.84, 16.46), // SO
                    LatLon::new(-22.13, 16.46), // Volta
                ])
                .with_population(60_140_000)
                .with_area(1_221_037.0),

            // Nigéria
            Country::new("Nigéria", "NGA", LatLon::new(9.0765, 7.3986))
                .with_boundary(vec![
                    LatLon::new(13.89, 2.69),  // NO
                    LatLon::new(13.89, 14.68), // NE
                    LatLon::new(4.27, 14.68),  // SE
                    LatLon::new(4.27, 2.69),   // SO
                    LatLon::new(13.89, 2.69),  // Volta
                ])
                .with_population(218_500_000)
                .with_area(923_768.0),

            // Etiópia
            Country::new("Etiópia", "ETH", LatLon::new(9.0250, 38.7469))
                .with_boundary(vec![
                    LatLon::new(14.89, 32.99), // NO
                    LatLon::new(14.89, 48.00), // NE
                    LatLon::new(3.40, 48.00),  // SE
                    LatLon::new(3.40, 32.99),  // SO
                    LatLon::new(14.89, 32.99), // Volta
                ])
                .with_population(120_280_000)
                .with_area(1_104_300.0),

            // Quénia
            Country::new("Quénia", "KEN", LatLon::new(-1.2921, 36.8219))
                .with_boundary(vec![
                    LatLon::new(5.03, 33.91),  // NO
                    LatLon::new(5.03, 41.91),  // NE
                    LatLon::new(-4.68, 41.91), // SE
                    LatLon::new(-4.68, 33.91), // SO
                    LatLon::new(5.03, 33.91),  // Volta
                ])
                .with_population(54_030_000)
                .with_area(580_367.0),

            // Argélia
            Country::new("Argélia", "DZA", LatLon::new(36.7538, 3.0588))
                .with_boundary(vec![
                    LatLon::new(37.09, -8.67), // NO
                    LatLon::new(37.09, 12.00), // NE
                    LatLon::new(19.00, 12.00), // SE
                    LatLon::new(19.00, -8.67), // SO
                    LatLon::new(37.09, -8.67), // Volta
                ])
                .with_population(44_620_000)
                .with_area(2_381_741.0),

            // Marrocos
            Country::new("Marrocos", "MAR", LatLon::new(34.0209, -6.8416))
                .with_boundary(vec![
                    LatLon::new(35.93, -13.17), // NO
                    LatLon::new(35.93, -0.99),  // NE
                    LatLon::new(27.66, -0.99),  // SE
                    LatLon::new(27.66, -13.17), // SO
                    LatLon::new(35.93, -13.17), // Volta
                ])
                .with_population(37_460_000)
                .with_area(446_550.0),
        ]
    }
}

impl Default for Africa {
    fn default() -> Self {
        Self::new()
    }
}

impl Continent for Africa {
    fn name(&self) -> &str {
        "África"
    }

    fn countries(&self) -> Vec<Country> {
        Self::load_countries()
    }

    fn bounding_box(&self) -> BoundingBox {
        BoundingBox::new(
            -35.0, // Sul (Cabo da Boa Esperança)
            37.0,  // Norte (Tunísia)
            -18.0, // Oeste (Cabo Verde)
            52.0,  // Leste (Somália)
        )
    }

    fn population(&self) -> u64 {
        1_400_000_000
    }

    fn area(&self) -> f64 {
        30_370_000.0
    }
}

/// Ásia
pub struct Asia;

impl Asia {
    pub fn new() -> Self {
        Self
    }

    fn load_countries() -> Vec<Country> {
        vec![
            // China
            Country::new("China", "CHN", LatLon::new(39.9042, 116.4074))
                .with_boundary(vec![
                    LatLon::new(53.56, 73.50),  // NO
                    LatLon::new(53.56, 135.09), // NE
                    LatLon::new(18.20, 135.09), // SE
                    LatLon::new(18.20, 73.50),  // SO
                    LatLon::new(53.56, 73.50),  // Volta
                ])
                .with_population(1_412_000_000)
                .with_area(9_596_961.0),

            // Índia
            Country::new("Índia", "IND", LatLon::new(28.6139, 77.2090))
                .with_boundary(vec![
                    LatLon::new(35.67, 68.11), // NO
                    LatLon::new(35.67, 97.40), // NE
                    LatLon::new(8.08, 97.40),  // SE
                    LatLon::new(8.08, 68.11),  // SO
                    LatLon::new(35.67, 68.11), // Volta
                ])
                .with_population(1_408_000_000)
                .with_area(3_287_263.0),

            // Japão
            Country::new("Japão", "JPN", LatLon::new(35.6762, 139.6503))
                .with_boundary(vec![
                    LatLon::new(45.55, 129.41), // NO
                    LatLon::new(45.55, 145.82), // NE
                    LatLon::new(24.04, 145.82), // SE
                    LatLon::new(24.04, 129.41), // SO
                    LatLon::new(45.55, 129.41), // Volta
                ])
                .with_population(125_500_000)
                .with_area(377_975.0),

            // Coreia do Sul
            Country::new("Coreia do Sul", "KOR", LatLon::new(37.5665, 126.9780))
                .with_boundary(vec![
                    LatLon::new(38.61, 124.61), // NO
                    LatLon::new(38.61, 131.87), // NE
                    LatLon::new(33.11, 131.87), // SE
                    LatLon::new(33.11, 124.61), // SO
                    LatLon::new(38.61, 124.61), // Volta
                ])
                .with_population(51_740_000)
                .with_area(100_210.0),

            // Tailândia
            Country::new("Tailândia", "THA", LatLon::new(13.7563, 100.5018))
                .with_boundary(vec![
                    LatLon::new(20.46, 97.35),  // NO
                    LatLon::new(20.46, 105.64), // NE
                    LatLon::new(5.61, 105.64),  // SE
                    LatLon::new(5.61, 97.35),   // SO
                    LatLon::new(20.46, 97.35),  // Volta
                ])
                .with_population(71_800_000)
                .with_area(513_120.0),

            // Arábia Saudita
            Country::new("Arábia Saudita", "SAU", LatLon::new(24.7136, 46.6753))
                .with_boundary(vec![
                    LatLon::new(32.15, 34.56), // NO
                    LatLon::new(32.15, 55.67), // NE
                    LatLon::new(16.00, 55.67), // SE
                    LatLon::new(16.00, 34.56), // SO
                    LatLon::new(32.15, 34.56), // Volta
                ])
                .with_population(35_950_000)
                .with_area(2_149_690.0),

            // Turquia
            Country::new("Turquia", "TUR", LatLon::new(39.9334, 32.8597))
                .with_boundary(vec![
                    LatLon::new(42.11, 26.04), // NO
                    LatLon::new(42.11, 44.79), // NE
                    LatLon::new(35.82, 44.79), // SE
                    LatLon::new(35.82, 26.04), // SO
                    LatLon::new(42.11, 26.04), // Volta
                ])
                .with_population(85_340_000)
                .with_area(783_562.0),
        ]
    }
}

impl Default for Asia {
    fn default() -> Self {
        Self::new()
    }
}

impl Continent for Asia {
    fn name(&self) -> &str {
        "Ásia"
    }

    fn countries(&self) -> Vec<Country> {
        Self::load_countries()
    }

    fn bounding_box(&self) -> BoundingBox {
        BoundingBox::new(
            -10.0,  // Sul (Indonésia)
            80.0,   // Norte (Sibéria)
            25.0,   // Oeste (Turquia)
            180.0,  // Leste (Rússia oriental)
        )
    }

    fn population(&self) -> u64 {
        4_700_000_000
    }

    fn area(&self) -> f64 {
        44_580_000.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_europe_countries() {
        let europe = Europe::new();
        let countries = europe.countries();
        assert!(countries.len() >= 10);
    }

    #[test]
    fn test_africa_bbox() {
        let africa = Africa::new();
        let bbox = africa.bounding_box();
        assert!(bbox.contains(&LatLon::new(0.0, 20.0))); // África central
    }

    #[test]
    fn test_asia_population() {
        let asia = Asia::new();
        assert!(asia.population() > 4_000_000_000);
    }
}
