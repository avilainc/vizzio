// Feature extraction for ML
use std::error::Error;

/// Feature extractor for binaries
pub struct FeatureExtractor {
    feature_types: Vec<FeatureType>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum FeatureType {
    ByteHistogram,
    OpcodeNgrams,
    StringFeatures,
    ImportFeatures,
    SectionFeatures,
    EntropFeatures,
    GraphFeatures,
}

impl FeatureExtractor {
    pub fn new() -> Self {
        Self {
            feature_types: vec![
                FeatureType::ByteHistogram,
                FeatureType::OpcodeNgrams,
                FeatureType::StringFeatures,
                FeatureType::ImportFeatures,
                FeatureType::SectionFeatures,
                FeatureType::EntropFeatures,
            ],
        }
    }

    /// Extract all features from binary
    pub fn extract(&self, binary_data: &[u8]) -> Result<Vec<f32>, Box<dyn Error>> {
        let mut features = Vec::new();

        // Byte histogram (256 features)
        features.extend(self.byte_histogram(binary_data));

        // String entropy
        features.push(self.string_entropy(binary_data));

        // File size (normalized)
        features.push((binary_data.len() as f32).log10());

        // TODO: Add more features

        Ok(features)
    }

    /// Calculate byte histogram
    fn byte_histogram(&self, data: &[u8]) -> Vec<f32> {
        let mut histogram = vec![0.0; 256];
        let total = data.len() as f32;

        for &byte in data {
            histogram[byte as usize] += 1.0;
        }

        // Normalize
        for count in &mut histogram {
            *count /= total;
        }

        histogram
    }

    /// Calculate string entropy
    fn string_entropy(&self, data: &[u8]) -> f32 {
        let mut freq = vec![0; 256];

        for &byte in data {
            freq[byte as usize] += 1;
        }

        let len = data.len() as f32;
        let mut entropy = 0.0;

        for count in freq {
            if count > 0 {
                let p = count as f32 / len;
                entropy -= p * p.log2();
            }
        }

        entropy
    }

    /// Extract opcode n-grams
    pub fn extract_opcode_ngrams(&self, opcodes: &[u8], n: usize) -> Vec<Vec<u8>> {
        opcodes.windows(n).map(|w| w.to_vec()).collect()
    }

    /// Extract API call sequences
    pub fn extract_api_sequences(&self, imports: &[String]) -> Vec<String> {
        imports.to_vec()
    }

    /// Extract graph features from CFG
    pub fn extract_graph_features(&self) -> GraphFeatures {
        GraphFeatures {
            num_nodes: 0,
            num_edges: 0,
            avg_degree: 0.0,
            clustering_coefficient: 0.0,
            cyclomatic_complexity: 0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct GraphFeatures {
    pub num_nodes: usize,
    pub num_edges: usize,
    pub avg_degree: f32,
    pub clustering_coefficient: f32,
    pub cyclomatic_complexity: usize,
}
