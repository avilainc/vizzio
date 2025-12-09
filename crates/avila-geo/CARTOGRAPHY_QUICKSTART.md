# 🎯 Quick Start - Cartography Framework

## Instalação

O framework de cartografia já está integrado no `avila-geo`:

```toml
[dependencies]
avila-geo = { version = "0.1.0", path = "../avila-geo" }
```

## Exemplo Básico

### 1. Mapa da Europa

```rust
use avila_geo::cartography::*;

let mut map = Map::new(vec!["europe"])?;
map.set_projection(Box::new(Mercator::new()));

// Adiciona cidades
for city in CityDatabase::european_cities() {
    map.add_feature(Box::new(city));
}

// Renderiza
let mut renderer = SvgRenderer::new();
let svg = map.render(&mut renderer)?;
std::fs::write("europa.svg", svg)?;
```

### 2. Mapa da África

```rust
let mut map = Map::new(vec!["africa"])?;
map.set_projection(Box::new(AlbersEqualArea::for_africa()));

for city in CityDatabase::african_cities() {
    map.add_feature(Box::new(city));
}

let mut renderer = SvgRenderer::new();
let svg = map.render(&mut renderer)?;
std::fs::write("africa.svg", svg)?;
```

### 3. Mapa Combinado

```rust
let mut map = Map::new(vec!["europe", "africa", "asia"])?;
map.set_projection(Box::new(Robinson::new()));

// Adiciona todas as cidades
for city in CityDatabase::european_cities() {
    map.add_feature(Box::new(city));
}
for city in CityDatabase::african_cities() {
    map.add_feature(Box::new(city));
}
for city in CityDatabase::asian_cities() {
    map.add_feature(Box::new(city));
}

let mut renderer = SvgRenderer::new();
let svg = map.render(&mut renderer)?;
std::fs::write("mundo.svg", svg)?;
```

## Coordenadas e Distâncias

```rust
use avila_geo::cartography::LatLon;

// Criar coordenadas
let lisboa = LatLon::new(38.7223, -9.1393);
let paris = LatLon::new(48.8566, 2.3522);

// Calcular distância (Haversine)
let distancia_km = lisboa.distance_to(&paris);
println!("Distância Lisboa-Paris: {:.0} km", distancia_km);
// Output: Distância Lisboa-Paris: 1450 km

// Formatar coordenadas
println!("{}", lisboa);
// Output: 38.7223°N, 9.1393°W
```

## Viewport e Zoom

```rust
let mut map = Map::new(vec!["europe"])?;

// Centralizar em uma coordenada
map.center_at(LatLon::new(48.8566, 2.3522)); // Paris

// Zoom in/out
map.zoom_in();
map.zoom_in();
map.zoom_out();

// Pan (mover mapa)
map.pan(5.0, 10.0); // delta_lat, delta_lon
```

## Sistema de Tiles

```rust
use avila_geo::cartography::{Tile, TileGrid, BoundingBox};

// Tile específico
let tile = Tile::new(132, 85, 8); // x, y, zoom

// Bounding box do tile
let bbox = tile.bbox();

// URL para OpenStreetMap
let url = tile.osm_url();
println!("{}", url);
// https://tile.openstreetmap.org/8/132/85.png

// Grade de tiles para uma região
let europa_bbox = BoundingBox::new(35.0, 71.0, -25.0, 45.0);
let grid = TileGrid::from_bbox(europa_bbox, 6);

for tile in grid.tiles() {
    println!("Tile: {}", tile);
}
```

## Projeções Cartográficas

### Mercator (Web Mercator)
```rust
let projection = Mercator::new();
map.set_projection(Box::new(projection));
```
- Usado por Google Maps, OpenStreetMap
- Preserva ângulos
- Distorce áreas próximo aos polos

### Robinson
```rust
let projection = Robinson::new();
map.set_projection(Box::new(projection));
```
- Projeção de compromisso
- Boa para mapas-múndi
- Equilibra distorções

### Albers Equal Area
```rust
// Otimizada para Europa
let projection = AlbersEqualArea::for_europe();

// Ou para África
let projection = AlbersEqualArea::for_africa();

// Ou para Ásia
let projection = AlbersEqualArea::for_asia();

map.set_projection(Box::new(projection));
```
- Preserva áreas
- Ideal para continentes específicos

## Customização

### Opções de Renderização

```rust
let mut options = RenderOptions::default();

// Dimensões
options.width = 1920;
options.height = 1080;

// Cor de fundo (RGBA)
options.background_color = [240, 248, 255, 255];

// Grade de coordenadas
options.show_graticule = true;

// Labels
options.show_labels = true;

// DPI
options.dpi = 150;
```

### Estilos

```rust
use avila_geo::cartography::Style;

// Estilo personalizado
let mut style = Style::default();
style.fill_color = [220, 220, 200, 255];    // Bege
style.stroke_color = [100, 100, 100, 255];  // Cinza
style.stroke_width = 1.5;
style.opacity = 0.8;

// Estilos pré-definidos
let land_style = Style::land();
let water_style = Style::water();
let grid_style = Style::graticule();
```

## Features Geográficas

### Adicionar Cidades

```rust
use avila_geo::cartography::City;

let lisboa = City::capital(
    "Lisboa",
    LatLon::new(38.7223, -9.1393),
    505_000
);

map.add_feature(Box::new(lisboa));
```

### Adicionar Rios

```rust
use avila_geo::cartography::River;

let danubio = River::new(
    "Danúbio",
    vec![
        LatLon::new(48.2082, 16.3738), // Viena
        LatLon::new(47.4979, 19.0402), // Budapeste
        // ... mais pontos
    ]
);

map.add_feature(Box::new(danubio));
```

### Adicionar Montanhas

```rust
use avila_geo::cartography::Mountain;

let mont_blanc = Mountain::new(
    "Mont Blanc",
    LatLon::new(45.8326, 6.8652),
    4808.0 // metros
);

map.add_feature(Box::new(mont_blanc));
```

## Formatos de Saída

### SVG (Recomendado)

```rust
let mut renderer = SvgRenderer::new();
let svg_data = map.render(&mut renderer)?;
std::fs::write("mapa.svg", svg_data)?;
```

### JSON

```rust
let mut renderer = JsonRenderer::new();
let json_data = map.render(&mut renderer)?;
std::fs::write("mapa.json", json_data)?;
```

## Executar Exemplos

```bash
# Showcase (demonstração rápida)
cargo run --example cartography_showcase

# Demo completo (gera todos os mapas)
cargo run --example cartography_demo
```

## Testes

```bash
# Todos os testes do módulo cartography
cargo test --package avila-geo cartography

# Teste específico
cargo test --package avila-geo cartography::coordinates::tests

# Com output detalhado
cargo test --package avila-geo cartography -- --nocapture
```

## Documentação

```bash
# Gerar documentação
cargo doc --package avila-geo --open

# Ver apenas o módulo cartography
cargo doc --package avila-geo --no-deps --open
```

---

**🗺️ Happy Mapping!**

Para mais informações, consulte [CARTOGRAPHY_README.md](./CARTOGRAPHY_README.md)
