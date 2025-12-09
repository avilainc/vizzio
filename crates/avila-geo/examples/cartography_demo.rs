//! Exemplo completo: Mapa da Europa, África e Ásia
//!
//! Este exemplo demonstra como criar e renderizar mapas dos três continentes
//! com diferentes projeções e estilos.

use avila_geo::cartography::{
    Map,
    SvgRenderer,
    Mercator,
    AlbersEqualArea,
    Robinson,
    RenderOptions,
    CityDatabase,
    ZoomLevel,
    LatLon,
};
use std::fs;
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🗺️  Avila Cartography Framework - Demo");
    println!("=====================================\n");

    // Cria diretório de saída
    let output_dir = "output/maps";
    fs::create_dir_all(output_dir)?;

    // 1. Mapa da Europa com projeção Mercator
    println!("📍 Criando mapa da Europa...");
    create_europe_map(output_dir)?;

    // 2. Mapa da África com projeção Albers
    println!("🌍 Criando mapa da África...");
    create_africa_map(output_dir)?;

    // 3. Mapa da Ásia com projeção Albers
    println!("🗾 Criando mapa da Ásia...");
    create_asia_map(output_dir)?;

    // 4. Mapa combinado (Europa + África + Ásia) com Robinson
    println!("🌐 Criando mapa combinado...");
    create_combined_map(output_dir)?;

    // 5. Mapa interativo com cidades
    println!("🏙️  Criando mapa com cidades...");
    create_cities_map(output_dir)?;

    println!("\n✅ Todos os mapas foram criados com sucesso!");
    println!("📂 Arquivos salvos em: {}", output_dir);

    Ok(())
}

/// Cria mapa da Europa
fn create_europe_map(output_dir: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut map = Map::new(vec!["europe"])?;

    // Configura projeção Mercator
    map.set_projection(Box::new(Mercator::new()));

    // Configura opções de renderização
    let mut options = RenderOptions::default();
    options.width = 1920;
    options.height = 1080;
    options.show_graticule = true;
    options.show_labels = true;

    // Adiciona cidades importantes
    for city in CityDatabase::european_cities() {
        map.add_feature(Box::new(city));
    }

    // Renderiza
    let mut renderer = SvgRenderer::new();
    let svg_data = map.render(&mut renderer)?;

    // Salva arquivo
    let path = Path::new(output_dir).join("europe_mercator.svg");
    fs::write(&path, svg_data)?;
    println!("   ✓ Europa (Mercator): {}", path.display());

    Ok(())
}

/// Cria mapa da África
fn create_africa_map(output_dir: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut map = Map::new(vec!["africa"])?;

    // Projeção Albers otimizada para África
    map.set_projection(Box::new(AlbersEqualArea::for_africa()));

    let mut options = RenderOptions::default();
    options.width = 1200;
    options.height = 1400;
    options.background_color = [240, 248, 255, 255];

    // Adiciona cidades
    for city in CityDatabase::african_cities() {
        map.add_feature(Box::new(city));
    }

    let mut renderer = SvgRenderer::new();
    let svg_data = map.render(&mut renderer)?;

    let path = Path::new(output_dir).join("africa_albers.svg");
    fs::write(&path, svg_data)?;
    println!("   ✓ África (Albers): {}", path.display());

    Ok(())
}

/// Cria mapa da Ásia
fn create_asia_map(output_dir: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut map = Map::new(vec!["asia"])?;

    // Projeção Albers otimizada para Ásia
    map.set_projection(Box::new(AlbersEqualArea::for_asia()));

    let mut options = RenderOptions::default();
    options.width = 1800;
    options.height = 1200;

    // Adiciona cidades
    for city in CityDatabase::asian_cities() {
        map.add_feature(Box::new(city));
    }

    let mut renderer = SvgRenderer::new();
    let svg_data = map.render(&mut renderer)?;

    let path = Path::new(output_dir).join("asia_albers.svg");
    fs::write(&path, svg_data)?;
    println!("   ✓ Ásia (Albers): {}", path.display());

    Ok(())
}

/// Cria mapa combinado dos três continentes
fn create_combined_map(output_dir: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut map = Map::new(vec!["europe", "africa", "asia"])?;

    // Robinson é ideal para mapas-múndi
    map.set_projection(Box::new(Robinson::new()));

    let mut options = RenderOptions::default();
    options.width = 2560;
    options.height = 1440;
    options.show_graticule = true;

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

    let path = Path::new(output_dir).join("combined_robinson.svg");
    fs::write(&path, svg_data)?;
    println!("   ✓ Mapa Combinado (Robinson): {}", path.display());

    Ok(())
}

/// Cria mapa focado em cidades
fn create_cities_map(output_dir: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut map = Map::new(vec!["europe", "africa", "asia"])?;

    map.set_projection(Box::new(Mercator::new()));

    // Centraliza em um ponto interessante (Mediterrâneo)
    map.center_at(LatLon::new(35.0, 20.0));
    map.zoom_in(); // Zoom para ver melhor

    let mut options = RenderOptions::default();
    options.width = 1920;
    options.height = 1080;
    options.show_labels = true;

    // Adiciona cidades principais
    let cities = [
        CityDatabase::european_cities(),
        CityDatabase::african_cities(),
        CityDatabase::asian_cities(),
    ].concat();

    for city in cities {
        map.add_feature(Box::new(city));
    }

    let mut renderer = SvgRenderer::new();
    let svg_data = map.render(&mut renderer)?;

    let path = Path::new(output_dir).join("cities_detail.svg");
    fs::write(&path, svg_data)?;
    println!("   ✓ Mapa de Cidades: {}", path.display());

    Ok(())
}
