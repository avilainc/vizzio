//! Exemplo de visualização e exportação
//!
//! Demonstra projeções, dendrogramas, layouts de grafos e exportação em múltiplos formatos

use avila_clustering::preprocessing::{StandardScaler, DataTransformer};
use avila_clustering::algorithms::kmeans::KMeans;
use avila_clustering::visualization::{
    ProjectionEngine, ProjectionType, ProjectedData,
    DendrogramBuilder, GraphLayoutEngine, LayoutAlgorithm, GraphVisualization,
    ExportEngine, ExportFormat
};
use ndarray::Array2;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🎨 Visualização e Exportação - avila-clustering\n");

    let data = generate_sample_data();
    println!("✓ Dados carregados: {} amostras, {} features\n", data.nrows(), data.ncols());

    // Pré-processamento
    let mut scaler = StandardScaler::new();
    let data_scaled = scaler.fit_transform(&data.view())?;

    // Clustering
    let mut kmeans = KMeans::new(3);
    let labels = kmeans.fit(&data_scaled.view())?;
    println!("✓ Clustering realizado (k=3)\n");

    // 1. Projeções para Visualização
    println!("📊 Projeções Dimensionais:");

    // Projeção 2D
    let projection_2d = ProjectionEngine::new(ProjectionType::PCA);
    let coords_2d = projection_2d.project_2d(&data_scaled.view())?;

    let mut projected = ProjectedData::new_2d(coords_2d)?
        .with_labels(labels.clone());
    projected.auto_color();

    println!("  ✓ Projeção 2D (PCA): {} pontos", projected.coordinates.nrows());

    // Projeção 3D
    let projection_3d = ProjectionEngine::new(ProjectionType::RandomProjection);
    let coords_3d = projection_3d.project_3d(&data_scaled.view())?;
    let projected_3d = ProjectedData::new_3d(coords_3d)?
        .with_labels(labels.clone());

    println!("  ✓ Projeção 3D (Random): {} pontos", projected_3d.coordinates.nrows());

    // 2. Dendrograma (Clustering Hierárquico)
    println!("\n🌳 Dendrograma:");

    // Simula linkage matrix: (cluster_i, cluster_j, distance, size)
    let linkage = vec![
        (0, 1, 0.5, 2),
        (2, 3, 0.8, 2),
        (4, 5, 1.0, 4),
    ];

    let mut dendrogram = DendrogramBuilder::new();
    dendrogram.from_linkage(&linkage)?;

    let dendro_json = dendrogram.to_json()?;
    println!("  ✓ Estrutura gerada: {} bytes", dendro_json.len());

    let dendro_coords = dendrogram.to_coordinates()?;
    println!("  ✓ Coordenadas para plotagem: {} nós", dendro_coords.len());

    // Corta dendrograma em 2 clusters
    let cut_labels = dendrogram.cut(2)?;
    println!("  ✓ Dendrograma cortado em 2 clusters");

    // 3. Layout de Grafos
    println!("\n🕸️  Layout de Grafos:");

    // Cria matriz de adjacência simples
    let mut adjacency = Array2::zeros((6, 6));
    adjacency[[0, 1]] = 1.0;
    adjacency[[1, 2]] = 1.0;
    adjacency[[2, 3]] = 1.0;
    adjacency[[3, 4]] = 1.0;
    adjacency[[4, 5]] = 1.0;
    adjacency[[5, 0]] = 1.0;

    // Layout circular
    let circular_engine = GraphLayoutEngine::new(LayoutAlgorithm::Circular);
    let circular_positions = circular_engine.compute_layout(&adjacency)?;
    println!("  ✓ Layout circular: {} nós", circular_positions.nrows());

    // Layout force-directed
    let force_engine = GraphLayoutEngine::new(LayoutAlgorithm::ForceDirected)
        .with_iterations(50);
    let force_positions = force_engine.compute_layout(&adjacency)?;
    println!("  ✓ Layout force-directed: {} nós", force_positions.nrows());

    // Cria visualização de grafo
    let edges = vec![(0, 1, 1.0), (1, 2, 1.0), (2, 3, 1.0)];
    let graph_viz = GraphVisualization::new(force_positions)
        .with_edges(edges)
        .with_labels(vec!["A".to_string(), "B".to_string(), "C".to_string(),
                         "D".to_string(), "E".to_string(), "F".to_string()]);

    let graph_json = graph_viz.to_json();
    println!("  ✓ Grafo exportado: {} bytes", graph_json.len());

    // 4. Exportação em Múltiplos Formatos
    println!("\n💾 Exportação:");

    let feature_names = vec!["x".to_string(), "y".to_string(), "z".to_string()];

    // JSON
    let json = ExportEngine::to_json(&data_scaled.view(), &labels, &feature_names);
    println!("  ✓ JSON: {} bytes", json.len());
    // Salva preview
    if json.len() > 100 {
        println!("    Preview: {}...", &json[..100]);
    }

    // CSV
    let csv = ExportEngine::to_csv(&data_scaled.view(), &labels, &feature_names);
    let csv_lines = csv.lines().count();
    println!("  ✓ CSV: {} linhas", csv_lines);

    // GeoJSON (assumindo primeiras 2 features são lon/lat)
    match ExportEngine::to_geojson(&data_scaled.view(), &labels, &feature_names) {
        Ok(geojson) => {
            println!("  ✓ GeoJSON: {} bytes", geojson.len());
            if geojson.len() > 100 {
                println!("    Preview: {}...", &geojson[..100]);
            }
        }
        Err(e) => println!("  ⚠ GeoJSON: {}", e),
    }

    // Centroides
    let centroids_json = ExportEngine::export_centroids(&data_scaled.view(), &labels, ExportFormat::JSON);
    println!("  ✓ Centroides (JSON): {} bytes", centroids_json.len());

    let centroids_csv = ExportEngine::export_centroids(&data_scaled.view(), &labels, ExportFormat::CSV);
    println!("  ✓ Centroides (CSV): {} linhas", centroids_csv.lines().count());

    // 5. Salvando arquivos (simulado)
    println!("\n📁 Arquivos Gerados (simulado):");
    println!("  • clustering_results.json");
    println!("  • clustering_results.csv");
    println!("  • clustering_geo.geojson");
    println!("  • centroids.json");
    println!("  • dendrogram.json");
    println!("  • graph_layout.json");

    println!("\n✅ Visualização completa!");
    println!("\n💡 Integrações Suportadas:");
    println!("  • D3.js: use os JSONs de grafos e dendrogramas");
    println!("  • Plotly: use projeções 2D/3D");
    println!("  • Leaflet/Mapbox: use GeoJSON para mapas");
    println!("  • Tableau/Power BI: use CSV para dashboards");

    Ok(())
}

fn generate_sample_data() -> Array2<f64> {
    let mut data = Vec::new();

    for cluster in 0..3 {
        let center = [
            (cluster as f64) * 5.0,
            ((cluster + 1) as f64) * 3.0,
            (cluster as f64) * 2.0,
        ];

        for i in 0..30 {
            let noise_factor = (i as f64 * 0.1).sin();
            data.push(vec![
                center[0] + noise_factor * 0.8,
                center[1] + noise_factor * 0.6,
                center[2] + noise_factor * 0.4,
            ]);
        }
    }

    let flat: Vec<f64> = data.into_iter().flatten().collect();
    Array2::from_shape_vec((90, 3), flat).unwrap()
}
