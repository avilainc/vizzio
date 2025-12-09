//! Sistema de tiles para renderização eficiente
//!
//! Usado para dividir o mapa em "ladrilhos" (tiles) para renderização e cache.

use super::coordinates::{LatLon, BoundingBox};
use std::fmt;

/// Representa um tile (ladrilho) do mapa
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Tile {
    /// Coordenada X do tile
    pub x: u32,
    /// Coordenada Y do tile
    pub y: u32,
    /// Nível de zoom
    pub zoom: u8,
}

impl Tile {
    /// Cria um novo tile
    pub fn new(x: u32, y: u32, zoom: u8) -> Self {
        Self { x, y, zoom }
    }

    /// Cria tile a partir de coordenada geográfica
    pub fn from_latlon(coord: LatLon, zoom: u8) -> Self {
        let n = 2_u32.pow(zoom as u32);

        let x = ((coord.lon + 180.0) / 360.0 * n as f64) as u32;
        let lat_rad = coord.lat.to_radians();
        let y = ((1.0 - (lat_rad.tan() + 1.0 / lat_rad.cos()).ln() / std::f64::consts::PI) / 2.0 * n as f64) as u32;

        Self { x, y, zoom }
    }

    /// Retorna o bounding box do tile
    pub fn bbox(&self) -> BoundingBox {
        let n = 2_f64.powi(self.zoom as i32);

        let lon_min = self.x as f64 / n * 360.0 - 180.0;
        let lon_max = (self.x + 1) as f64 / n * 360.0 - 180.0;

        let lat_rad_min = ((1.0 - 2.0 * (self.y + 1) as f64 / n) * std::f64::consts::PI).sinh().atan();
        let lat_rad_max = ((1.0 - 2.0 * self.y as f64 / n) * std::f64::consts::PI).sinh().atan();

        let lat_min = lat_rad_min.to_degrees();
        let lat_max = lat_rad_max.to_degrees();

        BoundingBox::new(lat_min, lat_max, lon_min, lon_max)
    }

    /// Retorna tiles vizinhos
    pub fn neighbors(&self) -> Vec<Tile> {
        let max = 2_u32.pow(self.zoom as u32);
        let mut neighbors = Vec::new();

        // Norte
        if self.y > 0 {
            neighbors.push(Tile::new(self.x, self.y - 1, self.zoom));
        }

        // Sul
        if self.y < max - 1 {
            neighbors.push(Tile::new(self.x, self.y + 1, self.zoom));
        }

        // Oeste
        if self.x > 0 {
            neighbors.push(Tile::new(self.x - 1, self.y, self.zoom));
        } else {
            neighbors.push(Tile::new(max - 1, self.y, self.zoom));
        }

        // Leste
        if self.x < max - 1 {
            neighbors.push(Tile::new(self.x + 1, self.y, self.zoom));
        } else {
            neighbors.push(Tile::new(0, self.y, self.zoom));
        }

        neighbors
    }

    /// Retorna o tile pai (zoom - 1)
    pub fn parent(&self) -> Option<Tile> {
        if self.zoom == 0 {
            None
        } else {
            Some(Tile::new(self.x / 2, self.y / 2, self.zoom - 1))
        }
    }

    /// Retorna os tiles filhos (zoom + 1)
    pub fn children(&self) -> Vec<Tile> {
        if self.zoom >= 20 {
            return Vec::new();
        }

        let zoom = self.zoom + 1;
        let x = self.x * 2;
        let y = self.y * 2;

        vec![
            Tile::new(x, y, zoom),
            Tile::new(x + 1, y, zoom),
            Tile::new(x, y + 1, zoom),
            Tile::new(x + 1, y + 1, zoom),
        ]
    }

    /// URL para tile do OpenStreetMap
    pub fn osm_url(&self) -> String {
        format!("https://tile.openstreetmap.org/{}/{}/{}.png", self.zoom, self.x, self.y)
    }
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}/{}/{}", self.zoom, self.x, self.y)
    }
}

/// Grade de tiles para uma área específica
pub struct TileGrid {
    /// Tiles na grade
    tiles: Vec<Tile>,
    /// Nível de zoom
    zoom: u8,
}

impl TileGrid {
    /// Cria grade de tiles para um bounding box
    pub fn from_bbox(bbox: BoundingBox, zoom: u8) -> Self {
        let nw = Tile::from_latlon(LatLon::new(bbox.max_lat, bbox.min_lon), zoom);
        let se = Tile::from_latlon(LatLon::new(bbox.min_lat, bbox.max_lon), zoom);

        let mut tiles = Vec::new();

        for x in nw.x..=se.x {
            for y in nw.y..=se.y {
                tiles.push(Tile::new(x, y, zoom));
            }
        }

        Self { tiles, zoom }
    }

    /// Número de tiles na grade
    pub fn count(&self) -> usize {
        self.tiles.len()
    }

    /// Itera sobre os tiles
    pub fn tiles(&self) -> &[Tile] {
        &self.tiles
    }

    /// Nível de zoom
    pub fn zoom(&self) -> u8 {
        self.zoom
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tile_from_latlon() {
        // Centro de Lisboa no zoom 10
        let tile = Tile::from_latlon(LatLon::new(38.7223, -9.1393), 10);
        assert_eq!(tile.zoom, 10);
    }

    #[test]
    fn test_tile_bbox() {
        let tile = Tile::new(0, 0, 0);
        let bbox = tile.bbox();

        // Tile 0/0/0 deve cobrir o mundo inteiro
        assert!(bbox.min_lat < -80.0);
        assert!(bbox.max_lat > 80.0);
        assert!((bbox.min_lon - (-180.0)).abs() < 0.01);
        assert!((bbox.max_lon - 180.0).abs() < 0.01);
    }

    #[test]
    fn test_tile_children() {
        let tile = Tile::new(0, 0, 0);
        let children = tile.children();

        assert_eq!(children.len(), 4);
        assert_eq!(children[0].zoom, 1);
    }

    #[test]
    fn test_tile_grid() {
        let bbox = BoundingBox::new(35.0, 71.0, -25.0, 45.0); // Europa
        let grid = TileGrid::from_bbox(bbox, 4);

        assert!(grid.count() > 0);
    }
}
