//! Window functions for arrays - pure Rust implementation

use crate::error::Result;

/// Rolling sum with a fixed window size
pub fn rolling_sum_i32(values: &[i32], window_size: usize) -> Result<Vec<i32>> {
    if window_size == 0 || window_size > values.len() {
        return Err(crate::error::ArrowError::ComputeError(
            format!("Invalid window size: {}", window_size),
        ));
    }

    let mut result = Vec::with_capacity(values.len());
    let mut sum = 0;

    // Initialize first window
    for i in 0..window_size {
        sum += values[i];
    }
    result.push(sum);

    // Slide window
    for i in window_size..values.len() {
        sum = sum - values[i - window_size] + values[i];
        result.push(sum);
    }

    Ok(result)
}

/// Rolling sum for f64
pub fn rolling_sum_f64(values: &[f64], window_size: usize) -> Result<Vec<f64>> {
    if window_size == 0 || window_size > values.len() {
        return Err(crate::error::ArrowError::ComputeError(
            format!("Invalid window size: {}", window_size),
        ));
    }

    let mut result = Vec::with_capacity(values.len());
    let mut sum = 0.0;

    for i in 0..window_size {
        sum += values[i];
    }
    result.push(sum);

    for i in window_size..values.len() {
        sum = sum - values[i - window_size] + values[i];
        result.push(sum);
    }

    Ok(result)
}

/// Rolling mean (average) with a fixed window size
pub fn rolling_mean_f64(values: &[f64], window_size: usize) -> Result<Vec<f64>> {
    let sums = rolling_sum_f64(values, window_size)?;
    Ok(sums.iter().map(|&s| s / window_size as f64).collect())
}

/// Rolling minimum
pub fn rolling_min_i32(values: &[i32], window_size: usize) -> Result<Vec<i32>> {
    if window_size == 0 || window_size > values.len() {
        return Err(crate::error::ArrowError::ComputeError(
            format!("Invalid window size: {}", window_size),
        ));
    }

    let mut result = Vec::with_capacity(values.len() - window_size + 1);

    for i in 0..=(values.len() - window_size) {
        let window = &values[i..i + window_size];
        result.push(*window.iter().min().unwrap());
    }

    Ok(result)
}

/// Rolling maximum
pub fn rolling_max_i32(values: &[i32], window_size: usize) -> Result<Vec<i32>> {
    if window_size == 0 || window_size > values.len() {
        return Err(crate::error::ArrowError::ComputeError(
            format!("Invalid window size: {}", window_size),
        ));
    }

    let mut result = Vec::with_capacity(values.len() - window_size + 1);

    for i in 0..=(values.len() - window_size) {
        let window = &values[i..i + window_size];
        result.push(*window.iter().max().unwrap());
    }

    Ok(result)
}

/// Rolling standard deviation
pub fn rolling_std_f64(values: &[f64], window_size: usize) -> Result<Vec<f64>> {
    if window_size == 0 || window_size > values.len() {
        return Err(crate::error::ArrowError::ComputeError(
            format!("Invalid window size: {}", window_size),
        ));
    }

    let mut result = Vec::with_capacity(values.len() - window_size + 1);

    for i in 0..=(values.len() - window_size) {
        let window = &values[i..i + window_size];
        let mean: f64 = window.iter().sum::<f64>() / window_size as f64;
        let variance: f64 = window
            .iter()
            .map(|&x| {
                let diff = x - mean;
                diff * diff
            })
            .sum::<f64>()
            / window_size as f64;
        result.push(variance.sqrt());
    }

    Ok(result)
}

/// Cumulative sum
pub fn cumsum_i32(values: &[i32]) -> Vec<i32> {
    let mut result = Vec::with_capacity(values.len());
    let mut sum = 0;

    for &v in values {
        sum += v;
        result.push(sum);
    }

    result
}

/// Cumulative sum for f64
pub fn cumsum_f64(values: &[f64]) -> Vec<f64> {
    let mut result = Vec::with_capacity(values.len());
    let mut sum = 0.0;

    for &v in values {
        sum += v;
        result.push(sum);
    }

    result
}

/// Cumulative product
pub fn cumprod_i32(values: &[i32]) -> Vec<i32> {
    let mut result = Vec::with_capacity(values.len());
    let mut prod = 1;

    for &v in values {
        prod *= v;
        result.push(prod);
    }

    result
}

/// Cumulative maximum
pub fn cummax_i32(values: &[i32]) -> Vec<i32> {
    let mut result = Vec::with_capacity(values.len());
    let mut max = i32::MIN;

    for &v in values {
        max = max.max(v);
        result.push(max);
    }

    result
}

/// Cumulative minimum
pub fn cummin_i32(values: &[i32]) -> Vec<i32> {
    let mut result = Vec::with_capacity(values.len());
    let mut min = i32::MAX;

    for &v in values {
        min = min.min(v);
        result.push(min);
    }

    result
}

/// Exponential moving average
pub fn ema_f64(values: &[f64], alpha: f64) -> Vec<f64> {
    if values.is_empty() {
        return Vec::new();
    }

    let mut result = Vec::with_capacity(values.len());
    let mut ema = values[0];
    result.push(ema);

    for &v in &values[1..] {
        ema = alpha * v + (1.0 - alpha) * ema;
        result.push(ema);
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rolling_sum_i32() {
        let values = vec![1, 2, 3, 4, 5];
        let result = rolling_sum_i32(&values, 3).unwrap();
        assert_eq!(result, vec![6, 9, 12]); // [1+2+3, 2+3+4, 3+4+5]
    }

    #[test]
    fn test_cumsum_i32() {
        let values = vec![1, 2, 3, 4, 5];
        let result = cumsum_i32(&values);
        assert_eq!(result, vec![1, 3, 6, 10, 15]);
    }

    #[test]
    fn test_cumprod_i32() {
        let values = vec![1, 2, 3, 4];
        let result = cumprod_i32(&values);
        assert_eq!(result, vec![1, 2, 6, 24]);
    }

    #[test]
    fn test_cummax_i32() {
        let values = vec![3, 1, 4, 1, 5];
        let result = cummax_i32(&values);
        assert_eq!(result, vec![3, 3, 4, 4, 5]);
    }

    #[test]
    fn test_ema_f64() {
        let values = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let result = ema_f64(&values, 0.5);
        assert_eq!(result.len(), 5);
        assert_eq!(result[0], 1.0);
    }

    #[test]
    fn test_rolling_mean_f64() {
        let values = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let result = rolling_mean_f64(&values, 3).unwrap();
        assert_eq!(result, vec![2.0, 3.0, 4.0]); // [6/3, 9/3, 12/3]
    }
}
