//! # Exemplo Completo: AvilaDB Cartográfico
//!
//! Demonstra como criar seu banco de dados cartográfico pessoal,
//! adicionar dados (empresas, lugares, endereços) e gerar mapas temáticos.

use avila_geo::aviladb_cartographic::*;
use avila_geo::cartography::LatLon;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🗺️  AvilaDB Cartográfico - Sistema de Banco de Dados Geográfico Pessoal");
    println!("=======================================================================\n");

    // 1. CRIAR O BANCO DE DADOS
    println!("📁 Criando banco de dados...");
    let mut db = CartographicDatabase::new(DatabaseConfig {
        storage_path: "meu_database_geo".into(),
        auto_save: false, // Desabilitado para demo
        spatial_index: true,
    });

    // 2. ADICIONAR EMPRESAS
    println!("\n🏢 Adicionando empresas...");

    let empresa1 = Company::new(
        0,
        "Tech Solutions".to_string(),
        LatLon::new(-23.5505, -46.6333), // São Paulo
        "Av. Paulista, 1000".to_string(),
        "Tecnologia".to_string(),
    )
    .with_employees(150)
    .with_revenue(5_000_000.0)
    .with_contact("www.techsolutions.com".into(), "(11) 1234-5678".into());

    let id1 = db.add_company(empresa1)?;
    println!("   ✓ Tech Solutions (ID: {})", id1);

    let empresa2 = Company::new(
        0,
        "Food Express".to_string(),
        LatLon::new(-23.5629, -46.6544), // São Paulo
        "Rua Augusta, 500".to_string(),
        "Alimentação".to_string(),
    )
    .with_employees(25);

    let id2 = db.add_company(empresa2)?;
    println!("   ✓ Food Express (ID: {})", id2);

    let empresa3 = Company::new(
        0,
        "Auto Center".to_string(),
        LatLon::new(-22.9068, -43.1729), // Rio de Janeiro
        "Av. Atlântica, 200".to_string(),
        "Automotivo".to_string(),
    )
    .with_employees(40);

    let id3 = db.add_company(empresa3)?;
    println!("   ✓ Auto Center (ID: {})", id3);

    // 3. ADICIONAR LUGARES
    println!("\n🍽️  Adicionando lugares...");

    let lugar1 = Place::new(
        0,
        "Restaurante Sabor Brasileiro".to_string(),
        LatLon::new(-23.5489, -46.6388),
        "Restaurante".to_string(),
    )
    .with_rating(4.5)
    .with_description("Culinária regional brasileira".into());

    db.add_place(lugar1)?;
    println!("   ✓ Restaurante Sabor Brasileiro");

    let lugar2 = Place::new(
        0,
        "Hospital São Lucas".to_string(),
        LatLon::new(-23.5558, -46.6396),
        "Hospital".to_string(),
    )
    .with_description("Hospital geral".into());

    db.add_place(lugar2)?;
    println!("   ✓ Hospital São Lucas");

    let lugar3 = Place::new(
        0,
        "Escola Municipal".to_string(),
        LatLon::new(-23.5575, -46.6398),
        "Escola".to_string(),
    );

    db.add_place(lugar3)?;
    println!("   ✓ Escola Municipal");

    // 4. ADICIONAR ENDEREÇOS
    println!("\n🏠 Adicionando endereços...");

    let endereco1 = Address::new(
        0,
        LatLon::new(-23.5500, -46.6400),
        "Rua das Flores".to_string(),
        "123".to_string(),
        "Jardim Paulista".to_string(),
        "São Paulo".to_string(),
        "SP".to_string(),
        "01234-567".to_string(),
    );

    db.add_address(endereco1)?;
    println!("   ✓ Rua das Flores, 123");

    // 5. ADICIONAR PONTOS DE INTERESSE
    println!("\n📍 Adicionando pontos de interesse...");

    let mut poi1 = PointOfInterest::new(
        0,
        "Parque Ibirapuera".to_string(),
        LatLon::new(-23.5873, -46.6573),
        "Parque".to_string(),
    );
    poi1.importance = 5;
    poi1.notes = "Maior parque urbano de São Paulo".to_string();
    poi1.mark_visited("2024-01-15".to_string());

    db.add_poi(poi1)?;
    println!("   ✓ Parque Ibirapuera");

    // 6. ESTATÍSTICAS DO DATABASE
    println!("\n📊 Estatísticas do Database:");
    let stats = db.stats();
    println!("   Total de entidades: {}", stats.total);
    println!("   Empresas: {}", stats.companies);
    println!("   Lugares: {}", stats.places);
    println!("   Endereços: {}", stats.addresses);
    println!("   POIs: {}", stats.pois);

    // 7. CONSULTAS
    println!("\n🔍 Executando consultas...");

    // Busca por nome
    let results = db.search_by_name("Tech");
    println!("\n   Busca por 'Tech': {} resultados", results.len());
    for entity in results {
        println!("      - {}", entity.as_geo_entity().name());
    }

    // Busca espacial (próximo à Paulista)
    let centro_paulista = LatLon::new(-23.5505, -46.6333);
    let proximos = db.search_near(centro_paulista, 5.0); // 5km
    println!("\n   Próximos à Av. Paulista (5km): {} resultados", proximos.len());
    for (entity, dist) in proximos {
        println!("      - {} ({:.2} km)", entity.as_geo_entity().name(), dist);
    }

    // Consulta avançada com QueryBuilder
    println!("\n   Consulta: Empresas do setor Tecnologia");
    let tech_companies = QueryBuilder::new(&db)
        .only_companies()
        .name_contains("Tech".to_string())
        .execute()?;
    println!("      Encontradas: {} empresas", tech_companies.len());

    // 8. CRIAR MAPA TEMÁTICO
    println!("\n🗺️  Criando mapas temáticos...");

    // Mapa 1: Todas as empresas
    let mapa_empresas = ThematicMapBuilder::new(&db, "Mapa de Empresas".to_string())
        .description("Distribuição geográfica das empresas cadastradas".to_string())
        .add_companies_layer("Todas as Empresas".to_string())
        .auto_fit()
        .show_labels(true)
        .show_legend(true)
        .build();

    println!("   ✓ Mapa de Empresas criado");
    println!("      Título: {}", mapa_empresas.title);
    println!("      Camadas: {}", mapa_empresas.layers.len());

    // Mapa 2: Empresas por setor (categórico)
    let mapa_setores = ThematicMapBuilder::new(&db, "Empresas por Setor".to_string())
        .description("Empresas classificadas por setor de atuação".to_string())
        .add_companies_by_sector("Setores Econômicos".to_string())
        .auto_fit()
        .build();

    println!("   ✓ Mapa de Setores criado");

    // Mapa 3: Lugares por categoria
    let mapa_lugares = ThematicMapBuilder::new(&db, "Mapa de Serviços".to_string())
        .description("Restaurantes, hospitais, escolas e outros serviços".to_string())
        .add_places_by_category("Serviços e Locais".to_string())
        .auto_fit()
        .build();

    println!("   ✓ Mapa de Serviços criado");

    // Mapa 4: Customizado
    let mapa_custom = ThematicMapBuilder::new(&db, "Mapa Customizado".to_string())
        .description("Apenas entidades com mais de 30 funcionários".to_string())
        .add_custom_layer(
            "Grandes Empresas".to_string(),
            MapTheme::Proportional,
            |entity| {
                if let Entity::Company(c) = entity {
                    c.employees.unwrap_or(0) > 30
                } else {
                    false
                }
            },
        )
        .auto_fit()
        .build();

    println!("   ✓ Mapa Customizado criado");

    // 9. EXPORTAR MAPAS
    println!("\n💾 Exportando mapas...");

    // Exportar para JSON
    let json_output = MapExporter::to_json(&db, &mapa_empresas)?;
    std::fs::write("mapa_empresas.json", json_output)?;
    println!("   ✓ mapa_empresas.json");

    // Exportar para GeoJSON
    let geojson_output = MapExporter::to_geojson(&db, &mapa_empresas)?;
    std::fs::write("mapa_empresas.geojson", geojson_output)?;
    println!("   ✓ mapa_empresas.geojson (compatível com QGIS, ArcGIS)");

    // Exportar para CSV
    let csv_output = MapExporter::to_csv(&db, &mapa_empresas)?;
    std::fs::write("mapa_empresas.csv", csv_output)?;
    println!("   ✓ mapa_empresas.csv (abre no Excel)");

    // Exportar para HTML interativo
    let html_output = MapExporter::to_html(&db, &mapa_empresas)?;
    std::fs::write("mapa_empresas.html", html_output)?;
    println!("   ✓ mapa_empresas.html (mapa interativo no navegador)");

    // 10. SIMBOLOGIA CUSTOMIZADA
    println!("\n🎨 Configurando simbologia...");

    let mut renderer = ThematicRenderer::new();

    // Simbologia por setor (empresas)
    let mut symbology_setores = CategoricalSymbology::new();

    symbology_setores.add_category(
        "Tecnologia".to_string(),
        SymbolStyle::simple([0, 123, 255, 255], 12.0), // Azul
        "Empresas de Tecnologia".to_string(),
    );

    symbology_setores.add_category(
        "Alimentação".to_string(),
        SymbolStyle::simple([255, 165, 0, 255], 10.0), // Laranja
        "Empresas de Alimentação".to_string(),
    );

    symbology_setores.add_category(
        "Automotivo".to_string(),
        SymbolStyle::simple([128, 128, 128, 255], 10.0), // Cinza
        "Empresas Automotivas".to_string(),
    );

    renderer.set_categorical("Setores Econômicos".to_string(), symbology_setores);
    println!("   ✓ Simbologia por setor configurada");

    // Simbologia por categoria (lugares)
    let mut symbology_lugares = CategoricalSymbology::new();

    symbology_lugares.add_category(
        "Restaurante".to_string(),
        SymbolStyle::simple([255, 0, 0, 255], 10.0), // Vermelho
        "Restaurantes".to_string(),
    );

    symbology_lugares.add_category(
        "Hospital".to_string(),
        SymbolStyle::simple([0, 255, 0, 255], 12.0), // Verde
        "Hospitais".to_string(),
    );

    symbology_lugares.add_category(
        "Escola".to_string(),
        SymbolStyle::simple([255, 255, 0, 255], 10.0), // Amarelo
        "Escolas".to_string(),
    );

    renderer.set_categorical("Serviços e Locais".to_string(), symbology_lugares);
    println!("   ✓ Simbologia por categoria configurada");

    // Rampa de cores para dados quantitativos
    let rampa_faturamento = ColorRamp::traffic_light();
    renderer.set_color_ramp("Faturamento".to_string(), rampa_faturamento);
    println!("   ✓ Rampa de cores configurada");

    // 11. SALVAR DATABASE
    println!("\n💾 Salvando database...");
    db.save()?;
    println!("   ✓ Database salvo em: meu_database_geo/");

    println!("\n✅ Demonstração completa!");
    println!("\n📁 Arquivos gerados:");
    println!("   - mapa_empresas.json");
    println!("   - mapa_empresas.geojson");
    println!("   - mapa_empresas.csv");
    println!("   - mapa_empresas.html (abra no navegador!)");
    println!("\n🎓 Próximos passos:");
    println!("   1. Abra mapa_empresas.html no navegador para ver mapa interativo");
    println!("   2. Importe mapa_empresas.geojson no QGIS ou Google My Maps");
    println!("   3. Abra mapa_empresas.csv no Excel para análise tabular");
    println!("   4. Use os dados para criar seus próprios mapas temáticos!");

    Ok(())
}
