//! Entidades geográficas personalizadas

use super::{GeoEntity, LatLon};
use std::collections::HashMap;

/// Empresa
#[derive(Debug, Clone)]
pub struct Company {
    pub id: u64,
    pub name: String,
    pub location: LatLon,
    pub address: String,
    pub sector: String,          // Setor: "Varejo", "Tecnologia", "Alimentação", etc
    pub employees: Option<u32>,   // Número de funcionários
    pub revenue: Option<f64>,     // Faturamento anual
    pub website: Option<String>,
    pub phone: Option<String>,
    pub notes: String,
    pub attributes: HashMap<String, String>,
}

impl Company {
    pub fn new(id: u64, name: String, location: LatLon, address: String, sector: String) -> Self {
        Self {
            id,
            name,
            location,
            address,
            sector,
            employees: None,
            revenue: None,
            website: None,
            phone: None,
            notes: String::new(),
            attributes: HashMap::new(),
        }
    }

    pub fn with_employees(mut self, employees: u32) -> Self {
        self.employees = Some(employees);
        self
    }

    pub fn with_revenue(mut self, revenue: f64) -> Self {
        self.revenue = Some(revenue);
        self
    }

    pub fn with_contact(mut self, website: String, phone: String) -> Self {
        self.website = Some(website);
        self.phone = Some(phone);
        self
    }

    pub fn add_attribute(&mut self, key: String, value: String) {
        self.attributes.insert(key, value);
    }
}

impl GeoEntity for Company {
    fn id(&self) -> u64 { self.id }
    fn name(&self) -> &str { &self.name }
    fn location(&self) -> LatLon { self.location }
    fn entity_type(&self) -> &str { "company" }
    fn attributes(&self) -> &HashMap<String, String> { &self.attributes }
}

/// Local/Lugar genérico
#[derive(Debug, Clone)]
pub struct Place {
    pub id: u64,
    pub name: String,
    pub location: LatLon,
    pub category: String,         // "Restaurante", "Hospital", "Escola", "Parque", etc
    pub description: String,
    pub rating: Option<f32>,      // Avaliação de 0.0 a 5.0
    pub capacity: Option<u32>,    // Capacidade de pessoas
    pub opening_hours: Option<String>,
    pub attributes: HashMap<String, String>,
}

impl Place {
    pub fn new(id: u64, name: String, location: LatLon, category: String) -> Self {
        Self {
            id,
            name,
            location,
            category,
            description: String::new(),
            rating: None,
            capacity: None,
            opening_hours: None,
            attributes: HashMap::new(),
        }
    }

    pub fn with_rating(mut self, rating: f32) -> Self {
        self.rating = Some(rating.clamp(0.0, 5.0));
        self
    }

    pub fn with_description(mut self, description: String) -> Self {
        self.description = description;
        self
    }

    pub fn add_attribute(&mut self, key: String, value: String) {
        self.attributes.insert(key, value);
    }
}

impl GeoEntity for Place {
    fn id(&self) -> u64 { self.id }
    fn name(&self) -> &str { &self.name }
    fn location(&self) -> LatLon { self.location }
    fn entity_type(&self) -> &str { "place" }
    fn attributes(&self) -> &HashMap<String, String> { &self.attributes }
}

/// Endereço residencial/comercial
#[derive(Debug, Clone)]
pub struct Address {
    pub id: u64,
    pub location: LatLon,
    pub street: String,
    pub number: String,
    pub complement: Option<String>,
    pub neighborhood: String,
    pub city: String,
    pub state: String,
    pub postal_code: String,
    pub country: String,
    pub address_type: String,     // "Residencial", "Comercial", "Industrial"
    pub resident_name: Option<String>,
    pub attributes: HashMap<String, String>,
}

impl Address {
    pub fn new(
        id: u64,
        location: LatLon,
        street: String,
        number: String,
        neighborhood: String,
        city: String,
        state: String,
        postal_code: String,
    ) -> Self {
        Self {
            id,
            location,
            street,
            number,
            complement: None,
            neighborhood,
            city,
            state,
            postal_code,
            country: "Brasil".to_string(),
            address_type: "Residencial".to_string(),
            resident_name: None,
            attributes: HashMap::new(),
        }
    }

    pub fn full_address(&self) -> String {
        format!(
            "{}, {} - {} - {}, {} - CEP {}",
            self.street,
            self.number,
            self.neighborhood,
            self.city,
            self.state,
            self.postal_code
        )
    }

    pub fn add_attribute(&mut self, key: String, value: String) {
        self.attributes.insert(key, value);
    }
}

impl GeoEntity for Address {
    fn id(&self) -> u64 { self.id }
    fn name(&self) -> &str { &self.street }
    fn location(&self) -> LatLon { self.location }
    fn entity_type(&self) -> &str { "address" }
    fn attributes(&self) -> &HashMap<String, String> { &self.attributes }
}

/// Ponto de Interesse (POI)
#[derive(Debug, Clone)]
pub struct PointOfInterest {
    pub id: u64,
    pub name: String,
    pub location: LatLon,
    pub poi_type: String,         // "Marco histórico", "Vista panorâmica", "Evento", etc
    pub importance: u8,           // 1-5 (importância)
    pub visited: bool,
    pub visit_date: Option<String>,
    pub photos: Vec<String>,      // Paths para fotos
    pub notes: String,
    pub attributes: HashMap<String, String>,
}

impl PointOfInterest {
    pub fn new(id: u64, name: String, location: LatLon, poi_type: String) -> Self {
        Self {
            id,
            name,
            location,
            poi_type,
            importance: 3,
            visited: false,
            visit_date: None,
            photos: Vec::new(),
            notes: String::new(),
            attributes: HashMap::new(),
        }
    }

    pub fn mark_visited(&mut self, date: String) {
        self.visited = true;
        self.visit_date = Some(date);
    }

    pub fn add_photo(&mut self, photo_path: String) {
        self.photos.push(photo_path);
    }

    pub fn add_attribute(&mut self, key: String, value: String) {
        self.attributes.insert(key, value);
    }
}

impl GeoEntity for PointOfInterest {
    fn id(&self) -> u64 { self.id }
    fn name(&self) -> &str { &self.name }
    fn location(&self) -> LatLon { self.location }
    fn entity_type(&self) -> &str { "poi" }
    fn attributes(&self) -> &HashMap<String, String> { &self.attributes }
}

/// Enum para facilitar trabalho com diferentes tipos
#[derive(Debug, Clone)]
pub enum Entity {
    Company(Company),
    Place(Place),
    Address(Address),
    PointOfInterest(PointOfInterest),
}

impl Entity {
    pub fn as_geo_entity(&self) -> &dyn GeoEntity {
        match self {
            Entity::Company(c) => c as &dyn GeoEntity,
            Entity::Place(p) => p as &dyn GeoEntity,
            Entity::Address(a) => a as &dyn GeoEntity,
            Entity::PointOfInterest(poi) => poi as &dyn GeoEntity,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_company_creation() {
        let company = Company::new(
            1,
            "Tech Corp".to_string(),
            LatLon::new(-23.55, -46.63),
            "Av. Paulista, 1000".to_string(),
            "Tecnologia".to_string(),
        ).with_employees(50);

        assert_eq!(company.name, "Tech Corp");
        assert_eq!(company.employees, Some(50));
    }

    #[test]
    fn test_place_with_rating() {
        let place = Place::new(
            2,
            "Restaurante Bom Gosto".to_string(),
            LatLon::new(-23.56, -46.65),
            "Restaurante".to_string(),
        ).with_rating(4.5);

        assert_eq!(place.rating, Some(4.5));
    }

    #[test]
    fn test_address_formatting() {
        let addr = Address::new(
            3,
            LatLon::new(-23.55, -46.63),
            "Rua das Flores".to_string(),
            "123".to_string(),
            "Centro".to_string(),
            "São Paulo".to_string(),
            "SP".to_string(),
            "01234-567".to_string(),
        );

        let full = addr.full_address();
        assert!(full.contains("Rua das Flores"));
        assert!(full.contains("123"));
    }
}
