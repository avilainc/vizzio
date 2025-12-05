//! Exportação de resultados para formatos de visualização

use ndarray::{Array1, Array2, ArrayView2};
use std::collections::HashMap;

/// Formato de exportação
#[derive(Debug, Clone, Copy)]
pub enum ExportFormat {
    JSON,
    CSV,
    GeoJSON,
}

/// ExportEngine - exporta dados de clustering para diversos formatos
pub struct ExportEngine;

impl ExportEngine {
    /// Exporta clustering para JSON
    pub fn to_json(
        data: &ArrayView2<f64>,
        labels: &Array1<usize>,
        feature_names: &[String],
    ) -> String {
        let mut points = Vec::new();

        for (i, label) in labels.iter().enumerate() {
            let mut features: HashMap<String, f64> = HashMap::new();

            for (j, name) in feature_names.iter().enumerate() {
                if j < data.ncols() {
                    features.insert(name.clone(), data[[i, j]]);
                }
            }

            let features_json: Vec<String> = features
                .iter()
                .map(|(k, v)| format!(r#""{}": {:.6}"#, k, v))
                .collect();

            points.push(format!(
                r#"{{"id": {}, "cluster": {}, "features": {{{}}}}}"#,
                i,
                label,
                features_json.join(", ")
            ));
        }

        format!(r#"{{"points": [{}]}}"#, points.join(", "))
    }

    /// Exporta clustering para CSV
    pub fn to_csv(
        data: &ArrayView2<f64>,
        labels: &Array1<usize>,
        feature_names: &[String],
    ) -> String {
        let mut lines = Vec::new();

        // Cabeçalho
        let mut header = feature_names.to_vec();
        header.push("cluster".to_string());
        lines.push(header.join(","));

        // Dados
        for (i, label) in labels.iter().enumerate() {
            let mut row: Vec<String> = (0..data.ncols())
                .map(|j| format!("{:.6}", data[[i, j]]))
                .collect();
            row.push(label.to_string());
            lines.push(row.join(","));
        }

        lines.join("\n")
    }

    /// Exporta para GeoJSON (assumindo que primeiras 2 features são lon/lat)
    pub fn to_geojson(
        data: &ArrayView2<f64>,
        labels: &Array1<usize>,
        feature_names: &[String],
    ) -> Result<String, String> {
        if data.ncols() < 2 {
            return Err("Dados precisam ter pelo menos 2 dimensões para GeoJSON".to_string());
        }

        let mut features = Vec::new();

        for (i, label) in labels.iter().enumerate() {
            let lon = data[[i, 0]];
            let lat = data[[i, 1]];

            let mut properties: HashMap<String, String> = HashMap::new();
            properties.insert("cluster".to_string(), label.to_string());

            for (j, name) in feature_names.iter().enumerate().skip(2) {
                if j < data.ncols() {
                    properties.insert(name.clone(), format!("{:.6}", data[[i, j]]));
                }
            }

            let props_json: Vec<String> = properties
                .iter()
                .map(|(k, v)| format!(r#""{}": {}"#, k, v))
                .collect();

            features.push(format!(
                r#"{{"type": "Feature", "geometry": {{"type": "Point", "coordinates": [{:.6}, {:.6}]}}, "properties": {{{}}}}}"#,
                lon, lat, props_json.join(", ")
            ));
        }

        Ok(format!(
            r#"{{"type": "FeatureCollection", "features": [{}]}}"#,
            features.join(", ")
        ))
    }

    /// Exporta centroides de clusters
    pub fn export_centroids(
        data: &ArrayView2<f64>,
        labels: &Array1<usize>,
        format: ExportFormat,
    ) -> String {
        let n_clusters = labels.iter().max().unwrap_or(&0) + 1;
        let n_features = data.ncols();

        // Calcula centroides
        let mut centroids = Array2::zeros((n_clusters, n_features));
        let mut counts = vec![0; n_clusters];

        for (i, &label) in labels.iter().enumerate() {
            for j in 0..n_features {
                centroids[[label, j]] += data[[i, j]];
            }
            counts[label] += 1;
        }

        for cluster_id in 0..n_clusters {
            if counts[cluster_id] > 0 {
                for j in 0..n_features {
                    centroids[[cluster_id, j]] /= counts[cluster_id] as f64;
                }
            }
        }

        match format {
            ExportFormat::JSON => Self::centroids_to_json(&centroids, n_clusters),
            ExportFormat::CSV => Self::centroids_to_csv(&centroids, n_clusters),
            ExportFormat::GeoJSON => Self::centroids_to_geojson(&centroids, n_clusters)
                .unwrap_or_else(|e| format!(r#"{{"error": "{}"}}"#, e)),
        }
    }

    fn centroids_to_json(centroids: &Array2<f64>, n_clusters: usize) -> String {
        let mut clusters_json = Vec::new();

        for i in 0..n_clusters {
            let coords: Vec<String> = (0..centroids.ncols())
                .map(|j| format!("{:.6}", centroids[[i, j]]))
                .collect();

            clusters_json.push(format!(
                r#"{{"cluster": {}, "centroid": [{}]}}"#,
                i,
                coords.join(", ")
            ));
        }

        format!(r#"{{"centroids": [{}]}}"#, clusters_json.join(", "))
    }

    fn centroids_to_csv(centroids: &Array2<f64>, n_clusters: usize) -> String {
        let mut lines = Vec::new();

        // Cabeçalho
        let n_features = centroids.ncols();
        let mut header: Vec<String> = (0..n_features)
            .map(|i| format!("feature_{}", i))
            .collect();
        header.insert(0, "cluster".to_string());
        lines.push(header.join(","));

        // Centroides
        for i in 0..n_clusters {
            let mut row: Vec<String> = vec![i.to_string()];
            for j in 0..n_features {
                row.push(format!("{:.6}", centroids[[i, j]]));
            }
            lines.push(row.join(","));
        }

        lines.join("\n")
    }

    fn centroids_to_geojson(centroids: &Array2<f64>, n_clusters: usize) -> Result<String, String> {
        if centroids.ncols() < 2 {
            return Err("Centroides precisam ter pelo menos 2 dimensões".to_string());
        }

        let mut features = Vec::new();

        for i in 0..n_clusters {
            let lon = centroids[[i, 0]];
            let lat = centroids[[i, 1]];

            features.push(format!(
                r#"{{"type": "Feature", "geometry": {{"type": "Point", "coordinates": [{:.6}, {:.6}]}}, "properties": {{"cluster": {}, "type": "centroid"}}}}"#,
                lon, lat, i
            ));
        }

        Ok(format!(
            r#"{{"type": "FeatureCollection", "features": [{}]}}"#,
            features.join(", ")
        ))
    }

    /// Exporta matriz de confusão (para comparação de clusters)
    pub fn export_confusion_matrix(
        labels_true: &Array1<usize>,
        labels_pred: &Array1<usize>,
    ) -> String {
        let n_true = labels_true.iter().max().unwrap_or(&0) + 1;
        let n_pred = labels_pred.iter().max().unwrap_or(&0) + 1;

        let mut matrix = Array2::zeros((n_true, n_pred));

        for (&true_label, &pred_label) in labels_true.iter().zip(labels_pred.iter()) {
            matrix[[true_label, pred_label]] += 1;
        }

        let mut rows = Vec::new();
        for i in 0..n_true {
            let row: Vec<String> = (0..n_pred)
                .map(|j| matrix[[i, j]].to_string())
                .collect();
            rows.push(format!(r#"[{}]"#, row.join(", ")));
        }

        format!(r#"{{"matrix": [{}]}}"#, rows.join(", "))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ndarray::array;

    #[test]
    fn test_json_export() {
        let data = array![[1.0, 2.0], [3.0, 4.0]];
        let labels = array![0, 1];
        let names = vec!["x".to_string(), "y".to_string()];

        let json = ExportEngine::to_json(&data.view(), &labels, &names);
        assert!(json.contains("points"));
        assert!(json.contains("cluster"));
    }

    #[test]
    fn test_csv_export() {
        let data = array![[1.0, 2.0], [3.0, 4.0]];
        let labels = array![0, 1];
        let names = vec!["x".to_string(), "y".to_string()];

        let csv = ExportEngine::to_csv(&data.view(), &labels, &names);
        assert!(csv.contains("x,y,cluster"));
    }
}
