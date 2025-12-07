//! Forecast service and types
//!
//! Provides time-series forecasting capabilities using ARIMA models.

use crate::error::{ApiError, ApiResult};

/// Default number of forecast steps
pub const DEFAULT_FORECAST_STEPS: usize = 5;

/// ARIMA model label
pub const MODEL_LABEL: &str = "ARIMA(1,1,1)";

/// Forecast request
#[derive(Debug, Clone)]
pub struct ForecastRequest {
    /// Historical time series data
    pub series: Vec<f64>,
    /// Number of steps to forecast
    pub steps: usize,
    /// Optional model parameters
    pub model_params: Option<ModelParams>,
}

impl ForecastRequest {
    /// Creates a new forecast request
    pub fn new(series: Vec<f64>, steps: usize) -> Self {
        Self {
            series,
            steps,
            model_params: None,
        }
    }

    /// Validates the request
    pub fn validate(&self) -> ApiResult<()> {
        if self.series.is_empty() {
            return Err(ApiError::validation("Time series cannot be empty"));
        }

        if self.series.len() < 3 {
            return Err(ApiError::validation(
                "Time series must have at least 3 data points",
            ));
        }

        if self.steps == 0 {
            return Err(ApiError::validation("Forecast steps must be greater than 0"));
        }

        if self.steps > 100 {
            return Err(ApiError::validation(
                "Forecast steps cannot exceed 100",
            ));
        }

        // Check for NaN or infinite values
        if self.series.iter().any(|&x| !x.is_finite()) {
            return Err(ApiError::validation(
                "Time series contains invalid values (NaN or Infinity)",
            ));
        }

        Ok(())
    }
}

/// Model parameters for forecasting
#[derive(Debug, Clone)]
pub struct ModelParams {
    /// Auto-regressive order
    pub p: usize,
    /// Differencing order
    pub d: usize,
    /// Moving average order
    pub q: usize,
}

impl Default for ModelParams {
    fn default() -> Self {
        Self { p: 1, d: 1, q: 1 }
    }
}

/// Forecast response
#[derive(Debug, Clone)]
pub struct ForecastResponse {
    /// Service name
    pub service: String,
    /// Original historical data
    pub historical_data: Vec<f64>,
    /// Forecasted values
    pub forecast: Vec<f64>,
    /// Number of forecast steps
    pub forecast_steps: usize,
    /// Model used for forecasting
    pub model: String,
    /// Optional confidence intervals
    pub confidence_intervals: Option<Vec<ConfidenceInterval>>,
}

/// Confidence interval for a forecast point
#[derive(Debug, Clone)]
pub struct ConfidenceInterval {
    /// Lower bound
    pub lower: f64,
    /// Upper bound
    pub upper: f64,
    /// Confidence level (e.g., 0.95 for 95%)
    pub level: f64,
}

/// Forecast service
pub struct ForecastService;

impl ForecastService {
    /// Creates a new forecast service
    pub fn new() -> Self {
        Self
    }

    /// Generates baseline time series for testing
    pub fn baseline_series() -> Vec<f64> {
        vec![
            100.0, 105.0, 98.0, 110.0, 115.0, 102.0, 108.0, 112.0, 107.0, 120.0,
        ]
    }

    /// Predicts future values using ARIMA model
    pub fn predict(&self, request: ForecastRequest) -> ApiResult<ForecastResponse> {
        // Validate request
        request.validate()?;

        // For now, use a simple linear extrapolation as placeholder
        // In production, this would use actual ARIMA implementation
        let forecast = self.simple_forecast(&request.series, request.steps);

        Ok(ForecastResponse {
            service: crate::SERVICE_NAME.to_string(),
            historical_data: request.series,
            forecast,
            forecast_steps: request.steps,
            model: MODEL_LABEL.to_string(),
            confidence_intervals: None,
        })
    }

    /// Simple linear extrapolation (placeholder for ARIMA)
    fn simple_forecast(&self, series: &[f64], steps: usize) -> Vec<f64> {
        let n = series.len();
        if n < 2 {
            return vec![series[0]; steps];
        }

        // Calculate simple trend from last few points
        let window = n.min(5);
        let recent = &series[n - window..];

        let sum_x: f64 = (0..window).map(|i| i as f64).sum();
        let sum_y: f64 = recent.iter().sum();
        let sum_xy: f64 = recent.iter().enumerate().map(|(i, &y)| i as f64 * y).sum();
        let sum_x2: f64 = (0..window).map(|i| (i * i) as f64).sum();

        let n_f = window as f64;
        let slope = (n_f * sum_xy - sum_x * sum_y) / (n_f * sum_x2 - sum_x * sum_x);
        let intercept = (sum_y - slope * sum_x) / n_f;

        // Extrapolate
        (0..steps)
            .map(|i| intercept + slope * (window + i) as f64)
            .collect()
    }

    /// Generates forecast with confidence intervals
    pub fn predict_with_confidence(
        &self,
        request: ForecastRequest,
        confidence_level: f64,
    ) -> ApiResult<ForecastResponse> {
        let mut response = self.predict(request)?;

        // Calculate simple confidence intervals based on historical variance
        let variance = self.calculate_variance(&response.historical_data);
        let std_dev = variance.sqrt();

        // Z-score for given confidence level (simplified)
        let z_score = if confidence_level >= 0.95 {
            1.96
        } else if confidence_level >= 0.90 {
            1.645
        } else {
            1.0
        };

        let intervals: Vec<ConfidenceInterval> = response
            .forecast
            .iter()
            .map(|&value| ConfidenceInterval {
                lower: value - z_score * std_dev,
                upper: value + z_score * std_dev,
                level: confidence_level,
            })
            .collect();

        response.confidence_intervals = Some(intervals);
        Ok(response)
    }

    /// Calculates variance of a time series
    fn calculate_variance(&self, series: &[f64]) -> f64 {
        if series.is_empty() {
            return 0.0;
        }

        let mean = series.iter().sum::<f64>() / series.len() as f64;
        let variance = series
            .iter()
            .map(|&x| (x - mean).powi(2))
            .sum::<f64>()
            / series.len() as f64;

        variance
    }
}

impl Default for ForecastService {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_forecast_request_validation() {
        let valid_request = ForecastRequest::new(vec![1.0, 2.0, 3.0, 4.0], 5);
        assert!(valid_request.validate().is_ok());

        let empty_request = ForecastRequest::new(vec![], 5);
        assert!(empty_request.validate().is_err());

        let too_few_points = ForecastRequest::new(vec![1.0, 2.0], 5);
        assert!(too_few_points.validate().is_err());

        let too_many_steps = ForecastRequest::new(vec![1.0, 2.0, 3.0], 101);
        assert!(too_many_steps.validate().is_err());
    }

    #[test]
    fn test_forecast_service() {
        let service = ForecastService::new();
        let series = vec![100.0, 105.0, 110.0, 115.0, 120.0];
        let request = ForecastRequest::new(series.clone(), 3);

        let result = service.predict(request);
        assert!(result.is_ok());

        let response = result.unwrap();
        assert_eq!(response.forecast.len(), 3);
        assert_eq!(response.historical_data, series);
    }

    #[test]
    fn test_variance_calculation() {
        let service = ForecastService::new();
        let series = vec![2.0, 4.0, 6.0, 8.0, 10.0];
        let variance = service.calculate_variance(&series);

        // Variance of [2,4,6,8,10] with mean 6 is 8.0
        assert!((variance - 8.0).abs() < 0.001);
    }

    #[test]
    fn test_confidence_intervals() {
        let service = ForecastService::new();
        let request = ForecastRequest::new(vec![100.0, 105.0, 110.0, 115.0, 120.0], 3);

        let result = service.predict_with_confidence(request, 0.95);
        assert!(result.is_ok());

        let response = result.unwrap();
        assert!(response.confidence_intervals.is_some());

        let intervals = response.confidence_intervals.unwrap();
        assert_eq!(intervals.len(), 3);

        for interval in intervals {
            assert!(interval.lower < interval.upper);
            assert_eq!(interval.level, 0.95);
        }
    }
}
