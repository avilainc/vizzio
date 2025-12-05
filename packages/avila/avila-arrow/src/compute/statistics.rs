//! Statistical functions for arrays - pure Rust implementation

use crate::error::Result;

/// Calculate mean (average) of i32 array
pub fn mean_i32(values: &[i32]) -> f64 {
    if values.is_empty() {
        return 0.0;
    }
    let sum: i64 = values.iter().map(|&x| x as i64).sum();
    sum as f64 / values.len() as f64
}

/// Calculate mean of f64 array
pub fn mean_f64(values: &[f64]) -> f64 {
    if values.is_empty() {
        return 0.0;
    }
    let sum: f64 = values.iter().sum();
    sum / values.len() as f64
}

/// Calculate variance of f64 array
pub fn variance_f64(values: &[f64]) -> f64 {
    if values.len() < 2 {
        return 0.0;
    }

    let mean = mean_f64(values);
    let sum_sq_diff: f64 = values.iter().map(|&x| (x - mean).powi(2)).sum();
    sum_sq_diff / values.len() as f64
}

/// Calculate sample variance (n-1 denominator)
pub fn sample_variance_f64(values: &[f64]) -> f64 {
    if values.len() < 2 {
        return 0.0;
    }

    let mean = mean_f64(values);
    let sum_sq_diff: f64 = values.iter().map(|&x| (x - mean).powi(2)).sum();
    sum_sq_diff / (values.len() - 1) as f64
}

/// Calculate standard deviation
pub fn std_dev_f64(values: &[f64]) -> f64 {
    variance_f64(values).sqrt()
}

/// Calculate sample standard deviation
pub fn sample_std_dev_f64(values: &[f64]) -> f64 {
    sample_variance_f64(values).sqrt()
}

/// Calculate covariance between two arrays
pub fn covariance_f64(x: &[f64], y: &[f64]) -> Result<f64> {
    if x.len() != y.len() {
        return Err(crate::error::ArrowError::ArrayLengthMismatch {
            expected: x.len(),
            actual: y.len(),
        });
    }

    if x.len() < 2 {
        return Ok(0.0);
    }

    let mean_x = mean_f64(x);
    let mean_y = mean_f64(y);

    let cov: f64 = x
        .iter()
        .zip(y.iter())
        .map(|(&xi, &yi)| (xi - mean_x) * (yi - mean_y))
        .sum();

    Ok(cov / x.len() as f64)
}

/// Calculate Pearson correlation coefficient
pub fn correlation_f64(x: &[f64], y: &[f64]) -> Result<f64> {
    if x.len() != y.len() {
        return Err(crate::error::ArrowError::ArrayLengthMismatch {
            expected: x.len(),
            actual: y.len(),
        });
    }

    let cov = covariance_f64(x, y)?;
    let std_x = std_dev_f64(x);
    let std_y = std_dev_f64(y);

    if std_x == 0.0 || std_y == 0.0 {
        return Ok(0.0);
    }

    Ok(cov / (std_x * std_y))
}

/// Calculate skewness (third moment)
pub fn skewness_f64(values: &[f64]) -> f64 {
    if values.len() < 3 {
        return 0.0;
    }

    let mean = mean_f64(values);
    let std = std_dev_f64(values);

    if std == 0.0 {
        return 0.0;
    }

    let n = values.len() as f64;
    let sum_cubed: f64 = values.iter().map(|&x| ((x - mean) / std).powi(3)).sum();

    (n / ((n - 1.0) * (n - 2.0))) * sum_cubed
}

/// Calculate kurtosis (fourth moment)
pub fn kurtosis_f64(values: &[f64]) -> f64 {
    if values.len() < 4 {
        return 0.0;
    }

    let mean = mean_f64(values);
    let std = std_dev_f64(values);

    if std == 0.0 {
        return 0.0;
    }

    let n = values.len() as f64;
    let sum_fourth: f64 = values.iter().map(|&x| ((x - mean) / std).powi(4)).sum();

    let kurtosis = (n * (n + 1.0) / ((n - 1.0) * (n - 2.0) * (n - 3.0))) * sum_fourth
        - (3.0 * (n - 1.0).powi(2) / ((n - 2.0) * (n - 3.0)));

    kurtosis
}

/// Calculate quantile (percentile)
pub fn quantile_f64(values: &[f64], q: f64) -> f64 {
    if values.is_empty() {
        return 0.0;
    }

    let mut sorted = values.to_vec();
    sorted.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));

    let index = (q * (sorted.len() - 1) as f64).round() as usize;
    sorted[index]
}

/// Calculate quartiles (Q1, Q2/median, Q3)
pub fn quartiles_f64(values: &[f64]) -> (f64, f64, f64) {
    let q1 = quantile_f64(values, 0.25);
    let q2 = quantile_f64(values, 0.50);
    let q3 = quantile_f64(values, 0.75);
    (q1, q2, q3)
}

/// Calculate interquartile range (IQR)
pub fn iqr_f64(values: &[f64]) -> f64 {
    let (q1, _, q3) = quartiles_f64(values);
    q3 - q1
}

/// Calculate z-scores (standardized values)
pub fn z_scores_f64(values: &[f64]) -> Vec<f64> {
    let mean = mean_f64(values);
    let std = std_dev_f64(values);

    if std == 0.0 {
        return vec![0.0; values.len()];
    }

    values.iter().map(|&x| (x - mean) / std).collect()
}

/// Calculate moving average (simple)
pub fn moving_average_f64(values: &[f64], window: usize) -> Vec<f64> {
    if window == 0 || window > values.len() {
        return Vec::new();
    }

    let mut result = Vec::new();

    for i in 0..=(values.len() - window) {
        let sum: f64 = values[i..i + window].iter().sum();
        result.push(sum / window as f64);
    }

    result
}

/// Calculate entropy (Shannon entropy)
pub fn entropy_i32(values: &[i32]) -> f64 {
    use std::collections::HashMap;

    if values.is_empty() {
        return 0.0;
    }

    let mut counts = HashMap::new();
    for &v in values {
        *counts.entry(v).or_insert(0) += 1;
    }

    let total = values.len() as f64;
    let mut entropy = 0.0;

    for count in counts.values() {
        let p = *count as f64 / total;
        if p > 0.0 {
            entropy -= p * p.log2();
        }
    }

    entropy
}

/// Calculate coefficient of variation
pub fn coefficient_of_variation_f64(values: &[f64]) -> f64 {
    let mean = mean_f64(values);
    if mean == 0.0 {
        return 0.0;
    }
    let std = std_dev_f64(values);
    std / mean
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mean_i32() {
        let values = vec![1, 2, 3, 4, 5];
        assert_eq!(mean_i32(&values), 3.0);
    }

    #[test]
    fn test_mean_f64() {
        let values = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        assert_eq!(mean_f64(&values), 3.0);
    }

    #[test]
    fn test_variance_f64() {
        let values = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let variance = variance_f64(&values);
        assert!((variance - 2.0).abs() < 0.01);
    }

    #[test]
    fn test_std_dev_f64() {
        let values = vec![2.0, 4.0, 4.0, 4.0, 5.0, 5.0, 7.0, 9.0];
        let std = std_dev_f64(&values);
        assert!((std - 2.0).abs() < 0.1);
    }

    #[test]
    fn test_covariance_f64() {
        let x = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let y = vec![2.0, 4.0, 6.0, 8.0, 10.0];
        let cov = covariance_f64(&x, &y).unwrap();
        assert!(cov > 0.0);
    }

    #[test]
    fn test_correlation_f64() {
        let x = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let y = vec![2.0, 4.0, 6.0, 8.0, 10.0];
        let corr = correlation_f64(&x, &y).unwrap();
        assert!((corr - 1.0).abs() < 0.01); // Perfect correlation
    }

    #[test]
    fn test_quartiles_f64() {
        let values = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0];
        let (q1, q2, q3) = quartiles_f64(&values);
        assert_eq!(q1, 3.0);
        assert_eq!(q2, 5.0);
        assert_eq!(q3, 7.0);
    }

    #[test]
    fn test_z_scores_f64() {
        let values = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let z_scores = z_scores_f64(&values);
        assert_eq!(z_scores.len(), 5);
        // Mean of z-scores should be ~0
        let mean_z = mean_f64(&z_scores);
        assert!(mean_z.abs() < 0.01);
    }
}
