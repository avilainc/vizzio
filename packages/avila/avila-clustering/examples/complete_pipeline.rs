//! Exemplo de uso completo do novo pipeline de clustering
//!
//! Demonstra o fluxo: preprocessing -> clustering -> postprocessing -> visualization

use avila_clustering::preprocessing::{StandardScaler, DataTransformer};
use avila_clustering::algorithms::kmeans::KMeans;
use avila_clustering::postprocessing::{ClusterExplainer, ClusterLabeler};
use avila_clustering::visualization::{ProjectionEngine, ProjectionType, ExportEngine, ExportFormat};
use avila_clustering::metrics::validation::silhouette_score;
use ndarray::{Array2, array};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 Pipeline Completo de Clustering - avila-clustering\n");

    // 1. Dados de exemplo (sintéticos)
    let data = generate_sample_data();
    println!("✓ Dados carregados: {} amostras, {} features", data.nrows(), data.ncols());

    // 2. Pré-processamento
    println!("\n📊 Pré-processamento:");
    let mut scaler = StandardScaler::new();
    let data_scaled = scaler.fit_transform(&data.view())?;
    println!("  ✓ Normalização aplicada (StandardScaler)");

    // 3. Clustering
    println!("\n🎯 Clustering:");
    let n_clusters = 3;
    let mut kmeans = KMeans::new(n_clusters);
    let labels = kmeans.fit(&data_scaled.view())?;
    println!("  ✓ KMeans executado com k={}", n_clusters);

    // 4. Métricas de validação
    println!("\n📈 Métricas de Validação:");
    let silhouette = silhouette_score(&data_scaled.view(), &labels)?;
    println!("  • Silhouette Score: {:.4}", silhouette);

    // 5. Pós-processamento e explicação
    println!("\n🔍 Pós-processamento:");
    let explainer = ClusterExplainer::new();
    let importance = explainer.feature_importance(&data_scaled.view(), &labels);
    println!("  ✓ Feature importance calculada para {} clusters", importance.len());

    let feature_names = vec!["feature_0".to_string(), "feature_1".to_string(), "feature_2".to_string()];
    let labeler = ClusterLabeler::new(feature_names.clone());
    let cluster_names = labeler.generate_cluster_names(&data_scaled.view(), &labels);
    println!("  ✓ Nomes automáticos gerados:");
    for (cluster_id, name) in &cluster_names {
        println!("    - Cluster {}: {}", cluster_id, name);
    }

    // 6. Visualização
    println!("\n🎨 Visualização:");
    let projection_engine = ProjectionEngine::new(ProjectionType::RandomProjection);
    let projected_2d = projection_engine.project_2d(&data_scaled.view())?;
    println!("  ✓ Projeção 2D gerada ({} pontos)", projected_2d.nrows());

    // 7. Exportação
    println!("\n💾 Exportação:");
    let json_export = ExportEngine::to_json(&data_scaled.view(), &labels, &feature_names);
    println!("  ✓ JSON gerado ({} bytes)", json_export.len());

    let csv_export = ExportEngine::to_csv(&data_scaled.view(), &labels, &feature_names);
    println!("  ✓ CSV gerado ({} linhas)", csv_export.lines().count());

    let centroids_json = ExportEngine::export_centroids(&data_scaled.view(), &labels, ExportFormat::JSON);
    println!("  ✓ Centroides exportados");

    println!("\n✅ Pipeline completo executado com sucesso!");
    println!("\n📝 Resumo:");
    println!("  • Amostras processadas: {}", data.nrows());
    println!("  • Clusters identificados: {}", n_clusters);
    println!("  • Qualidade (Silhouette): {:.4}", silhouette);

    Ok(())
}

fn generate_sample_data() -> Array2<f64> {
    // Gera dados sintéticos com 3 clusters
    let mut data = Vec::new();

    // Cluster 1: centrado em (0, 0, 0)
    for i in 0..30 {
        let noise = (i as f64 * 0.1) % 1.0 - 0.5;
        data.push(vec![noise, noise * 0.5, noise * 0.3]);
    }

    // Cluster 2: centrado em (5, 5, 5)
    for i in 0..30 {
        let noise = (i as f64 * 0.1) % 1.0 - 0.5;
        data.push(vec![5.0 + noise, 5.0 + noise * 0.5, 5.0 + noise * 0.3]);
    }

    // Cluster 3: centrado em (-5, 5, -5)
    for i in 0..30 {
        let noise = (i as f64 * 0.1) % 1.0 - 0.5;
        data.push(vec![-5.0 + noise, 5.0 + noise * 0.5, -5.0 + noise * 0.3]);
    }

    let flat: Vec<f64> = data.into_iter().flatten().collect();
    Array2::from_shape_vec((90, 3), flat).unwrap()
}
