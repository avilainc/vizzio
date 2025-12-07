//! Exemplo de clustering com restrições
//!
//! Demonstra semi-supervised, balanced, fairness-aware e spatial clustering

use avila_clustering::algorithms::constrained::{
    SemiSupervisedClusterer, BalancedClusterer, FairClusterer, FairnessMetric, SpatialClusterer
};
use avila_clustering::preprocessing::{StandardScaler, DataTransformer};
use ndarray::Array2;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("⚖️  Clustering com Restrições - avila-clustering\n");

    let data = generate_sample_data();
    println!("✓ Dados carregados: {} amostras, {} features\n", data.nrows(), data.ncols());

    let mut scaler = StandardScaler::new();
    let data_scaled = scaler.fit_transform(&data.view())?;

    // 1. Semi-Supervised Clustering
    println!("🎯 Semi-Supervised Clustering:");
    println!("  Adicionando restrições de pares...");

    let mut semi_supervised = SemiSupervisedClusterer::new(3);

    // Must-link: pontos que devem estar no mesmo cluster
    semi_supervised.add_must_link(0, 1);
    semi_supervised.add_must_link(2, 3);
    println!("  ✓ Must-link constraints: 2 pares");

    // Cannot-link: pontos que NÃO devem estar no mesmo cluster
    semi_supervised.add_cannot_link(0, 50);
    semi_supervised.add_cannot_link(10, 60);
    println!("  ✓ Cannot-link constraints: 2 pares");

    let labels_semi = semi_supervised.fit(&data_scaled.view())?;
    println!("  ✓ Clustering concluído respeitando restrições");

    // 2. Balanced Clustering
    println!("\n⚖️  Balanced Clustering:");
    println!("  Forçando clusters de tamanho equilibrado...");

    let balanced = BalancedClusterer::new(3)
        .with_size_constraints(25, 35);

    let labels_balanced = balanced.fit(&data_scaled.view())?;

    // Verifica tamanhos
    let mut sizes = vec![0; 3];
    for &label in labels_balanced.iter() {
        sizes[label] += 1;
    }
    println!("  ✓ Tamanhos dos clusters:");
    for (i, size) in sizes.iter().enumerate() {
        println!("    - Cluster {}: {} amostras", i, size);
    }

    // 3. Fairness-Aware Clustering
    println!("\n🤝 Fairness-Aware Clustering:");
    println!("  Garantindo fairness em atributos sensíveis...");

    let sensitive_features = vec![2]; // Feature 2 é sensível (ex: gênero, raça)
    let fair = FairClusterer::new(3, sensitive_features)
        .with_metric(FairnessMetric::DemographicParity);

    let labels_fair = fair.fit(&data_scaled.view())?;
    println!("  ✓ Clustering justo aplicado (Demographic Parity)");

    // 4. Spatial Clustering
    println!("\n🗺️  Spatial Clustering:");
    println!("  Aplicando restrições geoespaciais...");

    let spatial = SpatialClusterer::new(3, 5.0) // max_distance = 5.0
        .with_coordinate_indices(0, 1); // features 0 e 1 são coordenadas

    let labels_spatial = spatial.fit(&data_scaled.view())?;
    println!("  ✓ Clusters geoespacialmente contíguos");

    // 5. Comparação de resultados
    println!("\n📊 Comparação de Resultados:");
    println!("  • Semi-supervised: {} clusters únicos", count_unique(&labels_semi));
    println!("  • Balanced: {} clusters únicos", count_unique(&labels_balanced));
    println!("  • Fairness-aware: {} clusters únicos", count_unique(&labels_fair));
    println!("  • Spatial: {} clusters únicos", count_unique(&labels_spatial));

    println!("\n✅ Demonstração completa!");
    println!("\n💡 Casos de Uso:");
    println!("  • Semi-supervised: quando você tem conhecimento parcial");
    println!("  • Balanced: distribuição equilibrada de clientes/recursos");
    println!("  • Fairness: evitar discriminação em decisões algorítmicas");
    println!("  • Spatial: segmentação geográfica, zoneamento urbano");

    Ok(())
}

fn generate_sample_data() -> Array2<f64> {
    let mut data = Vec::new();

    // Gera dados com padrão espacial
    for cluster in 0..3 {
        let base_x = (cluster as f64) * 8.0;
        let base_y = if cluster % 2 == 0 { 0.0 } else { 8.0 };

        for i in 0..30 {
            let noise_x = ((i * 7) % 20) as f64 * 0.1 - 1.0;
            let noise_y = ((i * 13) % 20) as f64 * 0.1 - 1.0;
            let sensitive_attr = if i % 2 == 0 { 0.0 } else { 1.0 }; // Atributo sensível binário

            data.push(vec![
                base_x + noise_x,
                base_y + noise_y,
                sensitive_attr,
            ]);
        }
    }

    let flat: Vec<f64> = data.into_iter().flatten().collect();
    Array2::from_shape_vec((90, 3), flat).unwrap()
}

fn count_unique(labels: &ndarray::Array1<usize>) -> usize {
    let mut unique = labels.to_vec();
    unique.sort_unstable();
    unique.dedup();
    unique.len()
}
