//! Comparação entre algoritmos

use std::collections::HashMap;

pub struct AlgorithmComparison {
    results: HashMap<String, ComparisonResult>,
}

pub struct ComparisonResult {
    pub time_seconds: f64,
    pub quality_score: f64,
    pub memory_mb: f64,
}

impl AlgorithmComparison {
    pub fn new() -> Self {
        Self {
            results: HashMap::new(),
        }
    }

    pub fn add_result(&mut self, algorithm: String, result: ComparisonResult) {
        self.results.insert(algorithm, result);
    }

    pub fn get_best_by_time(&self) -> Option<(&String, &ComparisonResult)> {
        self.results
            .iter()
            .min_by(|a, b| a.1.time_seconds.partial_cmp(&b.1.time_seconds).unwrap())
    }

    pub fn report(&self) -> String {
        let mut lines = vec!["Algorithm Comparison:".to_string()];
        for (algo, result) in &self.results {
            lines.push(format!(
                "  {}: {:.3}s, quality={:.3}, memory={:.1}MB",
                algo, result.time_seconds, result.quality_score, result.memory_mb
            ));
        }
        lines.join("\n")
    }
}

impl Default for AlgorithmComparison {
    fn default() -> Self {
        Self::new()
    }
}
