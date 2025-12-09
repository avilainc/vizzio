# 🗺️ Avila Cartography Framework

Framework completo de visualização cartográfica para mapas da **Europa**, **África** e **Ásia**, desenvolvido 100% em Rust nativo sem dependências externas críticas.

## ✨ Características

### 🌍 Continentes Suportados
- **Europa**: 10+ países principais com fronteiras, capitais e dados demográficos
- **África**: 7+ países principais cobrindo Norte, Sul, Leste e Oeste
- **Ásia**: 7+ países principais incluindo China, Índia, Japão e mais

### 📐 Projeções Cartográficas
- **Mercator** (Web Mercator EPSG:3857) - Usado por Google Maps, OpenStreetMap
- **Robinson** - Projeção de compromisso para mapas-múndi
- **Albers Equal Area** - Preserva áreas, ideal para continentes específicos
  - Configurações otimizadas para Europa, África e Ásia

### 🎨 Renderização
- **SVG** (vetorial) - Escalável, ideal para web e impressão
- **JSON** - Dados estruturados para processamento adicional
- Grade de coordenadas (meridianos e paralelos)
- Estilos customizáveis (cores, espessuras, opacidade)
- Multi-camadas (países, cidades, rios, montanhas)

### 🏙️ Features Geográficas
- **Cidades**: 27+ cidades principais com população e localização
  - Marcadores diferenciados para capitais
  - Tamanho proporcional à população
- **Países**: Fronteiras políticas, áreas, populações
- **Rios**: Renderização de cursos d'água
- **Montanhas**: Picos com elevação

### 🔍 Interatividade
- Sistema de **viewport** com zoom e pan
- 7 níveis de zoom (Mundo → Rua)
- Centralização em coordenadas específicas
- Cálculo automático de bounding boxes

### 🧩 Sistema de Tiles
- Compatível com tiles XYZ (OpenStreetMap)
- Grade de tiles para renderização eficiente
- Hierarquia de tiles (parent/children)
- URLs para tiles OSM

## 🚀 Quick Start

```rust
use avila_geo::cartography::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Cria mapa da Europa
    let mut map = Map::new(vec!["europe"])?;

    // 2. Configura projeção
    map.set_projection(Box::new(Mercator::new()));

    // 3. Adiciona cidades
    for city in CityDatabase::european_cities() {
        map.add_feature(Box::new(city));
    }

    // 4. Renderiza em SVG
    let mut renderer = SvgRenderer::new();
    let svg_data = map.render(&mut renderer)?;

    // 5. Salva arquivo
    std::fs::write("europa.svg", svg_data)?;

    Ok(())
}
```

## 📊 Exemplos de Uso

### Mapa da África com Albers

```rust
let mut map = Map::new(vec!["africa"])?;
map.set_projection(Box::new(AlbersEqualArea::for_africa()));

for city in CityDatabase::african_cities() {
    map.add_feature(Box::new(city));
}

let mut renderer = SvgRenderer::new();
let svg_data = map.render(&mut renderer)?;
```

### Mapa Combinado (3 Continentes)

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
let svg_data = map.render(&mut renderer)?;
```

### Viewport Interativo

```rust
let mut map = Map::new(vec!["europe"])?;

// Centraliza em Paris
map.center_at(LatLon::new(48.8566, 2.3522));

// Zoom in
map.zoom_in();
map.zoom_in();

// Pan (move o mapa)
map.pan(5.0, 10.0); // delta_lat, delta_lon
```

### Cálculos Geográficos

```rust
use avila_geo::cartography::LatLon;

// Distância entre Lisboa e Paris (Haversine)
let lisbon = LatLon::new(38.7223, -9.1393);
let paris = LatLon::new(48.8566, 2.3522);
let distance_km = lisbon.distance_to(&paris);

println!("Distância: {:.0} km", distance_km);
// Output: Distância: 1450 km
```

### Sistema de Tiles

```rust
use avila_geo::cartography::{Tile, TileGrid};

// Tile específico
let tile = Tile::new(132, 85, 8); // x, y, zoom
let bbox = tile.bbox();
let osm_url = tile.osm_url();

// Grade de tiles para uma região
let bbox = BoundingBox::new(35.0, 71.0, -25.0, 45.0); // Europa
let grid = TileGrid::from_bbox(bbox, 6);

for tile in grid.tiles() {
    println!("Tile: {}", tile); // 6/132/85
}
```

## 🎯 Dados Incluídos

### Europa (10 países)
- Portugal, Espanha, França, Alemanha, Itália
- Reino Unido, Polónia, Ucrânia, Grécia, Suécia

### África (7 países)
- Egito, África do Sul, Nigéria, Etiópia
- Quénia, Argélia, Marrocos

### Ásia (7 países)
- China, Índia, Japão, Coreia do Sul
- Tailândia, Arábia Saudita, Turquia

### Cidades (27 totais)
- **Europa**: Lisboa, Madrid, Paris, Londres, Berlim, Roma, Atenas, Barcelona, Milão, Munique
- **África**: Cairo, Lagos, Kinshasa, Joanesburgo, Nairobi, Casablanca, Adis Abeba
- **Ásia**: Tóquio, Pequim, Mumbai, Xangai, Délhi, Seul, Jacarta, Bangkok, Hong Kong, Singapura

## 🏗️ Arquitetura

```
cartography/
├── mod.rs              # Entry point e Map struct
├── coordinates.rs      # LatLon, Point2D, BoundingBox
├── projection.rs       # Mercator, Robinson, Albers
├── continents.rs       # Europe, Africa, Asia
├── renderer.rs         # SVG, JSON renderers
├── viewport.rs         # Zoom, pan, viewport
├── features.rs         # Cities, Rivers, Mountains
├── tiles.rs            # Tile system (XYZ)
└── error.rs            # Error types
```

## 🔧 Configuração de Renderização

```rust
let mut options = RenderOptions::default();

// Dimensões
options.width = 1920;
options.height = 1080;

// Cor de fundo
options.background_color = [240, 248, 255, 255]; // Alice blue

// Grade de coordenadas
options.show_graticule = true;
options.graticule_style = Style::graticule();

// Labels
options.show_labels = true;

// DPI
options.dpi = 150;
```

## 📈 Performance

- **Zero dependências críticas**: Tudo implementado em Rust puro
- **Renderização eficiente**: Sistema de tiles para grandes áreas
- **Projeções otimizadas**: Cálculos matemáticos precisos
- **Compilação otimizada**: LTO e otimizações de release

## 🧪 Testes

```bash
# Executar todos os testes
cargo test --package avila-geo --lib cartography

# Teste específico
cargo test --package avila-geo --lib cartography::coordinates::tests

# Com output
cargo test --package avila-geo --lib cartography -- --nocapture
```

## 📦 Exemplo Completo

Execute o demo completo que gera todos os mapas:

```bash
cargo run --example cartography_demo --package avila-geo
```

Isso criará:
- `output/maps/europe_mercator.svg` - Europa em Mercator
- `output/maps/africa_albers.svg` - África em Albers
- `output/maps/asia_albers.svg` - Ásia em Albers
- `output/maps/combined_robinson.svg` - Mapa combinado em Robinson
- `output/maps/cities_detail.svg` - Mapa focado em cidades

## 🗺️ Formatos de Saída

### SVG (Recomendado)
- Vetorial, escalável
- Abre em navegadores
- Editável em Inkscape, Adobe Illustrator

### JSON
- Dados estruturados
- Fácil processamento
- Interoperável

## 🌐 Casos de Uso

- **Visualização de dados geográficos**
- **Dashboards de análise territorial**
- **Sistemas de informação geográfica (GIS)**
- **Educação e ensino de geografia**
- **Relatórios e apresentações**
- **Análise demográfica e estatística**
- **Planejamento urbano e regional**

## 🔮 Roadmap

- [ ] Mais países e cidades
- [ ] Lagos, mares e oceanos
- [ ] Rios principais detalhados
- [ ] Cadeias montanhosas
- [ ] Renderer PNG/raster
- [ ] Renderer PDF
- [ ] WebGL renderer
- [ ] Animações e transições
- [ ] Heatmaps e choropleth
- [ ] Dados GeoJSON

## 📝 Licença

MIT OR Apache-2.0

## 👥 Autores

- Nícolas Ávila <nicolas@avila.inc>
- Avila Development Team <dev@avila.inc>

---

**Avila Geo** - Geographic Intelligence for the Modern World 🌍
