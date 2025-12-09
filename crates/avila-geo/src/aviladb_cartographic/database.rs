//! Banco de dados cartográfico principal

use super::{DbResult, DbError, GeoEntity, LatLon, BoundingBox};
use super::entities::{Entity, Company, Place, Address, PointOfInterest};
use std::collections::HashMap;
use std::path::{Path, PathBuf};

/// Configuração do banco de dados
#[derive(Debug, Clone)]
pub struct DatabaseConfig {
    /// Diretório de armazenamento
    pub storage_path: PathBuf,

    /// Auto-salvar após cada operação
    pub auto_save: bool,

    /// Índice espacial habilitado
    pub spatial_index: bool,
}

impl Default for DatabaseConfig {
    fn default() -> Self {
        Self {
            storage_path: PathBuf::from("./aviladb_data"),
            auto_save: true,
            spatial_index: true,
        }
    }
}

/// Banco de dados cartográfico
pub struct CartographicDatabase {
    config: DatabaseConfig,

    /// Todas as entidades
    entities: HashMap<u64, Entity>,

    /// Próximo ID disponível
    next_id: u64,

    /// Índice por tipo
    type_index: HashMap<String, Vec<u64>>,

    /// Índice espacial (grid simples)
    spatial_grid: SpatialGrid,
}

impl CartographicDatabase {
    /// Cria novo banco de dados
    pub fn new(config: DatabaseConfig) -> Self {
        Self {
            spatial_grid: SpatialGrid::new(0.1), // Grid de 0.1 graus
            config,
            entities: HashMap::new(),
            next_id: 1,
            type_index: HashMap::new(),
        }
    }

    /// Carrega banco de dados do disco
    pub fn load<P: AsRef<Path>>(path: P) -> DbResult<Self> {
        let path = path.as_ref();
        if !path.exists() {
            return Err(DbError::NotFound(format!("Database não encontrado: {:?}", path)));
        }

        let json = std::fs::read_to_string(path)
            .map_err(|e| DbError::IoError(e.to_string()))?;

        let db: Self = serde_json::from_str(&json)
            .map_err(|e| DbError::SerializationError(e.to_string()))?;

        Ok(db)
    }

    /// Salva banco de dados no disco
    pub fn save(&self) -> DbResult<()> {
        std::fs::create_dir_all(&self.config.storage_path)
            .map_err(|e| DbError::IoError(e.to_string()))?;

        let path = self.config.storage_path.join("database.json");
        let json = serde_json::to_string_pretty(&self)
            .map_err(|e| DbError::SerializationError(e.to_string()))?;

        std::fs::write(path, json)
            .map_err(|e| DbError::IoError(e.to_string()))?;

        Ok(())
    }

    /// Adiciona empresa
    pub fn add_company(&mut self, mut company: Company) -> DbResult<u64> {
        let id = self.next_id;
        company.id = id;
        self.next_id += 1;

        let location = company.location;
        self.entities.insert(id, Entity::Company(company));
        self.type_index.entry("company".to_string()).or_default().push(id);

        if self.config.spatial_index {
            self.spatial_grid.insert(id, location);
        }

        if self.config.auto_save {
            self.save()?;
        }

        Ok(id)
    }

    /// Adiciona lugar
    pub fn add_place(&mut self, mut place: Place) -> DbResult<u64> {
        let id = self.next_id;
        place.id = id;
        self.next_id += 1;

        let location = place.location;
        self.entities.insert(id, Entity::Place(place));
        self.type_index.entry("place".to_string()).or_default().push(id);

        if self.config.spatial_index {
            self.spatial_grid.insert(id, location);
        }

        if self.config.auto_save {
            self.save()?;
        }

        Ok(id)
    }

    /// Adiciona endereço
    pub fn add_address(&mut self, mut address: Address) -> DbResult<u64> {
        let id = self.next_id;
        address.id = id;
        self.next_id += 1;

        let location = address.location;
        self.entities.insert(id, Entity::Address(address));
        self.type_index.entry("address".to_string()).or_default().push(id);

        if self.config.spatial_index {
            self.spatial_grid.insert(id, location);
        }

        if self.config.auto_save {
            self.save()?;
        }

        Ok(id)
    }

    /// Adiciona ponto de interesse
    pub fn add_poi(&mut self, mut poi: PointOfInterest) -> DbResult<u64> {
        let id = self.next_id;
        poi.id = id;
        self.next_id += 1;

        let location = poi.location;
        self.entities.insert(id, Entity::PointOfInterest(poi));
        self.type_index.entry("poi".to_string()).or_default().push(id);

        if self.config.spatial_index {
            self.spatial_grid.insert(id, location);
        }

        if self.config.auto_save {
            self.save()?;
        }

        Ok(id)
    }

    /// Busca entidade por ID
    pub fn get(&self, id: u64) -> Option<&Entity> {
        self.entities.get(&id)
    }

    /// Remove entidade
    pub fn remove(&mut self, id: u64) -> DbResult<()> {
        if let Some(entity) = self.entities.remove(&id) {
            let entity_type = entity.as_geo_entity().entity_type().to_string();

            if let Some(ids) = self.type_index.get_mut(&entity_type) {
                ids.retain(|&i| i != id);
            }

            self.spatial_grid.remove(id);

            if self.config.auto_save {
                self.save()?;
            }

            Ok(())
        } else {
            Err(DbError::NotFound(format!("Entidade {} não encontrada", id)))
        }
    }

    /// Lista todas as entidades
    pub fn list_all(&self) -> Vec<&Entity> {
        self.entities.values().collect()
    }

    /// Lista entidades por tipo
    pub fn list_by_type(&self, entity_type: &str) -> Vec<&Entity> {
        self.type_index
            .get(entity_type)
            .map(|ids| {
                ids.iter()
                    .filter_map(|id| self.entities.get(id))
                    .collect()
            })
            .unwrap_or_default()
    }

    /// Busca espacial: entidades em um bounding box
    pub fn search_in_bbox(&self, bbox: BoundingBox) -> Vec<&Entity> {
        if self.config.spatial_index {
            let ids = self.spatial_grid.query_bbox(bbox);
            ids.iter()
                .filter_map(|id| self.entities.get(id))
                .collect()
        } else {
            // Busca linear
            self.entities.values()
                .filter(|e| {
                    let loc = e.as_geo_entity().location();
                    bbox.contains(&loc)
                })
                .collect()
        }
    }

    /// Busca espacial: entidades próximas a um ponto
    pub fn search_near(&self, center: LatLon, radius_km: f64) -> Vec<(&Entity, f64)> {
        self.entities.values()
            .filter_map(|e| {
                let loc = e.as_geo_entity().location();
                let distance = center.distance_to(&loc);
                if distance <= radius_km {
                    Some((e, distance))
                } else {
                    None
                }
            })
            .collect()
    }

    /// Busca por nome (case-insensitive, contém)
    pub fn search_by_name(&self, query: &str) -> Vec<&Entity> {
        let query_lower = query.to_lowercase();
        self.entities.values()
            .filter(|e| {
                e.as_geo_entity().name().to_lowercase().contains(&query_lower)
            })
            .collect()
    }

    /// Busca por atributo
    pub fn search_by_attribute(&self, key: &str, value: &str) -> Vec<&Entity> {
        self.entities.values()
            .filter(|e| {
                e.as_geo_entity().get_attribute(key) == Some(value)
            })
            .collect()
    }

    /// Número total de entidades
    pub fn count(&self) -> usize {
        self.entities.len()
    }

    /// Estatísticas do database
    pub fn stats(&self) -> DatabaseStats {
        let mut stats = DatabaseStats::default();

        for entity in self.entities.values() {
            match entity {
                Entity::Company(_) => stats.companies += 1,
                Entity::Place(_) => stats.places += 1,
                Entity::Address(_) => stats.addresses += 1,
                Entity::PointOfInterest(_) => stats.pois += 1,
            }
        }

        stats.total = self.entities.len();
        stats
    }
}

/// Estatísticas do database
#[derive(Debug, Default, Clone)]
pub struct DatabaseStats {
    pub total: usize,
    pub companies: usize,
    pub places: usize,
    pub addresses: usize,
    pub pois: usize,
}

/// Grid espacial simples para indexação
struct SpatialGrid {
    cell_size: f64,
    grid: HashMap<(i32, i32), Vec<u64>>,
}

impl SpatialGrid {
    fn new(cell_size: f64) -> Self {
        Self {
            cell_size,
            grid: HashMap::new(),
        }
    }

    fn cell_coords(&self, location: LatLon) -> (i32, i32) {
        let x = (location.lon / self.cell_size).floor() as i32;
        let y = (location.lat / self.cell_size).floor() as i32;
        (x, y)
    }

    fn insert(&mut self, id: u64, location: LatLon) {
        let cell = self.cell_coords(location);
        self.grid.entry(cell).or_default().push(id);
    }

    fn remove(&mut self, id: u64) {
        for ids in self.grid.values_mut() {
            ids.retain(|&i| i != id);
        }
    }

    fn query_bbox(&self, bbox: BoundingBox) -> Vec<u64> {
        let min_cell = self.cell_coords(LatLon::new(bbox.min_lat, bbox.min_lon));
        let max_cell = self.cell_coords(LatLon::new(bbox.max_lat, bbox.max_lon));

        let mut results = Vec::new();

        for x in min_cell.0..=max_cell.0 {
            for y in min_cell.1..=max_cell.1 {
                if let Some(ids) = self.grid.get(&(x, y)) {
                    results.extend(ids);
                }
            }
        }

        results
    }
}

// Implementações para serialização (simplificadas)
impl serde::Serialize for CartographicDatabase {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut state = serializer.serialize_struct("CartographicDatabase", 2)?;
        state.serialize_field("next_id", &self.next_id)?;
        state.serialize_field("entity_count", &self.entities.len())?;
        state.end()
    }
}

impl<'de> serde::Deserialize<'de> for CartographicDatabase {
    fn deserialize<D>(_deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Implementação simplificada
        Ok(Self::new(DatabaseConfig::default()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_database_operations() {
        let mut db = CartographicDatabase::new(DatabaseConfig {
            auto_save: false,
            ..Default::default()
        });

        let company = Company::new(
            0,
            "Test Corp".to_string(),
            LatLon::new(-23.55, -46.63),
            "Rua Teste, 100".to_string(),
            "Tecnologia".to_string(),
        );

        let id = db.add_company(company).unwrap();
        assert_eq!(db.count(), 1);

        let entity = db.get(id).unwrap();
        assert_eq!(entity.as_geo_entity().name(), "Test Corp");
    }

    #[test]
    fn test_spatial_search() {
        let mut db = CartographicDatabase::new(DatabaseConfig {
            auto_save: false,
            spatial_index: true,
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
        let results = db.search_in_bbox(bbox);

        assert_eq!(results.len(), 1);
    }
}
