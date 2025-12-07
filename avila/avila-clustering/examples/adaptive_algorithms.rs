//! Exemplo de algoritmos adaptativos
//!
//! Demonstra auto-detecção de clusters, tuning de parâmetros e clustering incremental

use avila_clustering::algorithms::adaptive::{AutoCluster, ClusterNumberMethod, IncrementalClusterer};
use avila_clustering::preprocessing::{StandardScaler, DataTransformer};
use ndarray::Array2;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🤖 Algoritmos Adaptativos - avila-clustering\n");

    // Gera dados de exemplo
    let data = generate_sample_data();
    println!("✓ Dados carregados: {} amostras, {} features\n", data.nrows(), data.ncols());

    // Pré-processamento
    let mut scaler = StandardScaler::new();
    let data_scaled = scaler.fit_transform(&data.view())?;

    // 1. Auto-detecção do número de clusters
    println!("🔍 Auto-detecção do número ideal de clusters:");

    let methods = vec![
        (ClusterNumberMethod::Elbow, "Método do Cotovelo"),
        (ClusterNumberMethod::Silhouette, "Silhouette"),
        (ClusterNumberMethod::GapStatistic, "Gap Statistic"),
        (ClusterNumberMethod::BIC, "BIC (Bayesian Information Criterion)"),
    ];

    for (method, name) in methods {
        let auto_cluster = AutoCluster::new(2, 10).with_method(method);
        match auto_cluster.find_optimal_clusters(&data_scaled.view()) {
            Ok(k) => println!("  • {}: {} clusters", name, k),
            Err(e) => println!("  • {}: erro - {}", name, e),
        }
    }

    // 2. Clustering Incremental
    println!("\n📈 Clustering Incremental:");
    let mut incremental = IncrementalClusterer::new(3);

    // Treina em batches
    let batch_size = 30;
    for (i, batch_start) in (0..data_scaled.nrows()).step_by(batch_size).enumerate() {
        let batch_end = (batch_start + batch_size).min(data_scaled.nrows());
        let batch = data_scaled.slice(ndarray::s![batch_start..batch_end, ..]);

        incremental.partial_fit(&batch)?;
        println!("  ✓ Batch {} processado ({} amostras)", i + 1, batch_end - batch_start);
    }

    // Predição
    let predictions = incremental.predict(&data_scaled.view())?;
    println!("  ✓ Predições finais: {} labels", predictions.len());

    // 3. Resumo
    println!("\n✅ Demonstração completa!");
    println!("\n💡 Benefícios dos Algoritmos Adaptativos:");
    println!("  • Auto-detecção: elimina adivinhação de k");
    println!("  • Incremental: eficiente para big data");
    println!("  • Transfer learning: reutiliza conhecimento");
    println!("  • Parameter tuning: otimização automática");

    Ok(())
}

fn generate_sample_data() -> Array2<f64> {
    let mut data = Vec::new();

    // Gera 3 clusters com diferentes densidades
    for cluster in 0..3 {
        let center_x = (cluster as f64) * 10.0;
        for i in 0..30 {
            let angle = (i as f64) * 0.21;
            let radius = 1.0 + (i as f64) * 0.05;
            data.push(vec![
                center_x + radius * angle.cos(),
                radius * angle.sin(),
                (cluster as f64) + (i as f64) * 0.01,
            ]);
        }
    }

    let flat: Vec<f64> = data.into_iter().flatten().collect();
    Array2::from_shape_vec((90, 3), flat).unwrap()
}
