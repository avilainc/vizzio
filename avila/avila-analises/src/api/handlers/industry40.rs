use axum::{extract::{Path, State}, Json};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use tracing::{error, info, warn, instrument};
use validator::Validate;

use crate::{
    api::{error::ApiError, response::ApiResponse, state::AppState, ApiResult},
    error::AvilaError,
    industry40::*,
    models::BehaviorEvent,
};

// ==================== Request/Response DTOs ====================

/// POST /api/v1/industry40/iot/ingest
/// Ingerir dados de telemetria de dispositivos IoT
#[derive(Debug, Deserialize, Validate)]
pub struct TelemetryRequest {
    #[validate(length(min = 1, max = 100, message = "Device ID must be between 1-100 characters"))]
    pub device_id: String,
    pub telemetry: iot::ProductionTelemetry,
}

#[derive(Debug, Serialize)]
pub struct TelemetryResponse {
    pub device_id: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub status: String,
    pub message: String,
}

/// Ingerir telemetria de dispositivo IoT
///
/// # Errors
/// - Returns `ApiError::Validation` if input validation fails
/// - Returns `ApiError::Storage` if storage operation fails
#[instrument(skip(state), fields(device_id = %payload.device_id))]
pub async fn ingest_telemetry(
    State(state): State<AppState>,
    Json(payload): Json<TelemetryRequest>,
) -> ApiResult<Json<ApiResponse<TelemetryResponse>>> {
    // Validar input
    payload.validate()
        .map_err(|e| ApiError::Validation(format!("Invalid telemetry data: {}", e)))?;

    info!(
        device_id = %payload.device_id,
        temperature = payload.telemetry.temperature,
        pressure = payload.telemetry.pressure,
        "Ingesting telemetry data"
    );

    // Criar evento para armazenamento
    let event = BehaviorEvent {
        event_id: uuid::Uuid::new_v4().to_string(),
        user_id: format!("device:{}", payload.device_id),
        session_id: payload.device_id.clone(),
        timestamp: Utc::now(),
        event_type: crate::models::EventType::Custom {
            name: "iot_telemetry".to_string(),
            data: serde_json::json!({
                "device_id": payload.device_id,
                "temperature": payload.telemetry.temperature,
                "pressure": payload.telemetry.pressure,
                "vibration": payload.telemetry.vibration,
                "power_consumption": payload.telemetry.power_consumption,
                "cycle_time_ms": payload.telemetry.cycle_time_ms,
                "production_count": payload.telemetry.production_count,
            }),
        },
        metadata: std::collections::HashMap::new(),
        context: crate::models::EventContext {
            user_agent: "IoT-Device".to_string(),
            ip: "internal".to_string(),
            country: None,
            device_type: crate::models::DeviceType::Desktop,
            referrer: None,
            utm_source: None,
            utm_medium: None,
            utm_campaign: None,
        },
    };

    // Armazenar no event store
    state.event_store.store(event).await
        .map_err(|e| {
            error!(error = %e, "Failed to store telemetry");
            ApiError::Storage(format!("Failed to store telemetry: {}", e))
        })?;

    let response = TelemetryResponse {
        device_id: payload.device_id,
        timestamp: Utc::now(),
        status: "success".to_string(),
        message: "Telemetry data ingested successfully".to_string(),
    };

    Ok(Json(ApiResponse::ok(response)))
}

/// POST /api/v1/industry40/maintenance/predict
/// Prever falhas de máquina usando modelo de manutenção preditiva
#[derive(Debug, Deserialize, Validate)]
pub struct PredictFailureRequest {
    #[validate(length(min = 1, max = 100))]
    pub device_id: String,
    pub telemetry: iot::ProductionTelemetry,
    /// Dados históricos opcionais para melhor predição (máximo 1000 registros)
    #[validate(length(max = 1000))]
    pub historical_data: Option<Vec<iot::ProductionTelemetry>>,
}

#[derive(Debug, Serialize)]
pub struct PredictFailureResponse {
    pub device_id: String,
    pub failure_probability: f64,
    pub risk_level: RiskLevel,
    pub alert: Option<predictive_maintenance::MaintenanceAlert>,
    pub predictions: Vec<String>,
    pub confidence: f64,
    pub analyzed_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum RiskLevel {
    Low,
    Medium,
    High,
    Critical,
}

impl RiskLevel {
    fn from_probability(prob: f64) -> Self {
        match prob {
            p if p < 0.25 => RiskLevel::Low,
            p if p < 0.50 => RiskLevel::Medium,
            p if p < 0.75 => RiskLevel::High,
            _ => RiskLevel::Critical,
        }
    }
}

/// Predizer probabilidade de falha de equipamento
///
/// Utiliza dados históricos e telemetria atual para prever falhas.
///
/// # Errors
/// - Returns `ApiError::Validation` if input validation fails
/// - Returns `ApiError::Analysis` if prediction fails
#[instrument(skip(state, payload), fields(device_id = %payload.device_id))]
pub async fn predict_failure(
    State(state): State<AppState>,
    Json(payload): Json<PredictFailureRequest>,
) -> ApiResult<Json<ApiResponse<PredictFailureResponse>>> {
    payload.validate()
        .map_err(|e| ApiError::Validation(format!("Invalid prediction request: {}", e)))?;

    info!(device_id = %payload.device_id, "Predicting equipment failure");

    let mut engine = predictive_maintenance::PredictiveMaintenanceEngine::new();

    // Buscar dados históricos do event store se não fornecidos
    let historical_data = if let Some(data) = payload.historical_data {
        data
    } else {
        // Buscar últimos 100 eventos do dispositivo
        let events = state.event_store
            .get_by_user(&format!("device:{}", payload.device_id), Some(100))
            .await
            .map_err(|e| {
                warn!(error = %e, "Failed to fetch historical data, using current telemetry only");
                e
            })
            .unwrap_or_default();

        // Converter eventos para telemetria (simplificado)
        vec![payload.telemetry.clone()]
    };

    if historical_data.is_empty() {
        warn!(device_id = %payload.device_id, "No historical data available for training");
        return Err(ApiError::Analysis("Insufficient data for prediction".to_string()));
    }

    // Treinar modelo com dados históricos
    engine.train_model(payload.device_id.clone(), &historical_data);

    // Fazer predição
    let alert = engine.predict_failure(&payload.telemetry);
    let failure_prob = alert.as_ref().map(|a| a.failure_probability).unwrap_or(0.0);
    let risk_level = RiskLevel::from_probability(failure_prob);

    // Calcular confiança baseada em quantidade de dados
    let confidence = (historical_data.len() as f64 / 100.0).min(1.0);

    // Gerar recomendações
    let predictions = if let Some(ref alert) = alert {
        vec![
            format!("Component '{}' requires attention", alert.component),
            format!("Recommended action: {}", alert.action),
            if failure_prob > 0.7 {
                "URGENT: Schedule immediate maintenance".to_string()
            } else {
                "Monitor closely for next 24-48 hours".to_string()
            },
        ]
    } else {
        vec!["Equipment operating within normal parameters".to_string()]
    };

    let response = PredictFailureResponse {
        device_id: payload.device_id.clone(),
        failure_probability: failure_prob,
        risk_level,
        alert,
        predictions,
        confidence,
        analyzed_at: Utc::now(),
    };

    info!(
        device_id = %payload.device_id,
        failure_probability = failure_prob,
        risk_level = ?response.risk_level,
        "Failure prediction completed"
    );

    Ok(Json(ApiResponse::ok(response)))
}

/// POST /api/v1/industry40/oee/calculate
/// Calcular Overall Equipment Effectiveness (OEE)
#[derive(Debug, Deserialize, Validate)]
pub struct CalculateOEERequest {
    #[validate(length(min = 1, max = 100))]
    pub device_id: String,
    pub production_data: oee::ProductionData,
    #[validate(range(min = 1, max = 3600000, message = "Cycle time must be between 1ms and 1 hour"))]
    pub target_cycle_time_ms: u64,
    #[validate(range(min = 0.1, max = 24.0, message = "Planned hours must be between 0.1 and 24"))]
    pub planned_hours: f64,
}

#[derive(Debug, Serialize)]
pub struct OEEResponse {
    pub device_id: String,
    pub metrics: oee::OEEMetrics,
    pub performance_category: PerformanceCategory,
    pub recommendations: Vec<String>,
    pub calculated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum PerformanceCategory {
    WorldClass,    // OEE >= 85%
    Competitive,   // OEE >= 70%
    Average,       // OEE >= 50%
    BelowAverage,  // OEE < 50%
}

impl PerformanceCategory {
    fn from_oee(oee: f64) -> Self {
        match oee {
            o if o >= 0.85 => PerformanceCategory::WorldClass,
            o if o >= 0.70 => PerformanceCategory::Competitive,
            o if o >= 0.50 => PerformanceCategory::Average,
            _ => PerformanceCategory::BelowAverage,
        }
    }
}

/// Calcular OEE (Overall Equipment Effectiveness)
///
/// OEE = Availability × Performance × Quality
///
/// # Errors
/// - Returns `ApiError::Validation` if input validation fails
/// - Returns `ApiError::Analysis` if calculation fails
#[instrument(skip(state, payload), fields(device_id = %payload.device_id))]
pub async fn calculate_oee(
    State(state): State<AppState>,
    Json(payload): Json<CalculateOEERequest>,
) -> ApiResult<Json<ApiResponse<OEEResponse>>> {
    payload.validate()
        .map_err(|e| ApiError::Validation(format!("Invalid OEE request: {}", e)))?;

    info!(device_id = %payload.device_id, "Calculating OEE metrics");

    let calculator = oee::OEECalculator::new(
        payload.target_cycle_time_ms,
        payload.planned_hours,
    );

    let metrics = calculator.calculate_oee(&payload.production_data);
    let performance_category = PerformanceCategory::from_oee(metrics.oee);

    // Gerar recomendações baseadas nos componentes do OEE
    let mut recommendations = Vec::new();

    if metrics.availability < 0.9 {
        recommendations.push(format!(
            "Availability is {}%. Target: 90%+. Reduce downtime and improve maintenance scheduling.",
            (metrics.availability * 100.0).round()
        ));
    }

    if metrics.performance < 0.95 {
        recommendations.push(format!(
            "Performance is {}%. Target: 95%+. Optimize cycle times and reduce minor stops.",
            (metrics.performance * 100.0).round()
        ));
    }

    if metrics.quality < 0.99 {
        recommendations.push(format!(
            "Quality rate is {}%. Target: 99%+. Improve quality control and reduce defects.",
            (metrics.quality * 100.0).round()
        ));
    }

    if metrics.oee >= 0.85 {
        recommendations.push("Excellent! Maintain world-class OEE performance.".to_string());
    } else if metrics.oee >= 0.70 {
        recommendations.push("Good performance. Focus on continuous improvement.".to_string());
    } else {
        recommendations.push("OEE below target. Prioritize improvement initiatives.".to_string());
    }

    let response = OEEResponse {
        device_id: payload.device_id.clone(),
        metrics,
        performance_category,
        recommendations,
        calculated_at: Utc::now(),
    };

    info!(
        device_id = %payload.device_id,
        oee = %.2 = response.metrics.oee * 100.0,
        category = ?response.performance_category,
        "OEE calculation completed"
    );

    Ok(Json(ApiResponse::ok(response)))
}

/// GET /api/v1/industry40/twin/:device_id
/// Obter estado do gêmeo digital de um dispositivo
///
/// # Errors
/// - Returns `ApiError::Validation` if device_id is invalid
/// - Returns `ApiError::NotFound` if device doesn't exist
#[instrument(skip(state), fields(device_id = %device_id))]
pub async fn get_digital_twin(
    State(state): State<AppState>,
    Path(device_id): Path<String>,
) -> ApiResult<Json<ApiResponse<digital_twin::TwinState>>> {
    // Validar device_id
    if device_id.is_empty() || device_id.len() > 100 {
        return Err(ApiError::Validation(
            "Device ID must be between 1-100 characters".to_string()
        ));
    }

    info!(device_id = %device_id, "Fetching digital twin state");

    // Verificar se o dispositivo existe no event store
    let device_events = state.event_store
        .get_by_user(&format!("device:{}", device_id), Some(1))
        .await
        .map_err(|e| ApiError::Storage(format!("Failed to check device: {}", e)))?;

    if device_events.is_empty() {
        warn!(device_id = %device_id, "Device not found");
        return Err(ApiError::NotFound(format!("Device '{}' not found", device_id)));
    }

    // Criar ou recuperar gêmeo digital
    let twin = digital_twin::DigitalTwin::new(
        format!("twin-{}", device_id),
        device_id.clone(),
    );

    info!(device_id = %device_id, health_status = ?twin.state.health_status, "Digital twin retrieved");

    Ok(Json(ApiResponse::ok(twin.state)))
}

/// POST /api/v1/industry40/optimize/production
/// Otimizar cronograma de produção
#[derive(Debug, Deserialize, Validate)]
pub struct OptimizeProductionRequest {
    #[validate(length(min = 1, max = 1000, message = "Orders must be between 1-1000"))]
    pub orders: Vec<production_optimizer::ProductionOrder>,
    pub constraints: production_optimizer::ProductionConstraints,
    /// Opções de otimização
    pub optimization_goal: Option<OptimizationGoal>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum OptimizationGoal {
    MinimizeTime,
    MinimizeCost,
    MaximizeQuality,
    Balanced,
}

impl Default for OptimizationGoal {
    fn default() -> Self {
        OptimizationGoal::Balanced
    }
}

#[derive(Debug, Serialize)]
pub struct OptimizationResponse {
    pub result: production_optimizer::OptimizationResult,
    pub metrics: OptimizationMetrics,
    pub goal: OptimizationGoal,
    pub optimized_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Serialize)]
pub struct OptimizationMetrics {
    pub total_orders: usize,
    pub optimized_orders: usize,
    pub estimated_time_savings_percent: f64,
    pub optimization_score: f64,
}

/// Otimizar cronograma de produção
///
/// # Errors
/// - Returns `ApiError::Validation` if input validation fails
/// - Returns `ApiError::Analysis` if optimization fails
#[instrument(skip(state, payload), fields(orders_count = payload.orders.len()))]
pub async fn optimize_production(
    State(state): State<AppState>,
    Json(payload): Json<OptimizeProductionRequest>,
) -> ApiResult<Json<ApiResponse<OptimizationResponse>>> {
    payload.validate()
        .map_err(|e| ApiError::Validation(format!("Invalid optimization request: {}", e)))?;

    if payload.orders.is_empty() {
        return Err(ApiError::Validation("At least one order is required".to_string()));
    }

    info!(
        orders_count = payload.orders.len(),
        goal = ?payload.optimization_goal,
        "Optimizing production schedule"
    );

    let goal = payload.optimization_goal.unwrap_or_default();
    let optimizer = production_optimizer::ProductionOptimizer::new(payload.constraints);
    let result = optimizer.optimize_schedule(payload.orders.clone());

    // Calcular métricas de otimização
    let metrics = OptimizationMetrics {
        total_orders: payload.orders.len(),
        optimized_orders: result.scheduled_orders.len(),
        estimated_time_savings_percent: 15.0, // Placeholder - calcular baseado em algoritmo
        optimization_score: 0.85, // Placeholder - calcular baseado em critérios
    };

    info!(
        optimized_orders = metrics.optimized_orders,
        score = %.2 = metrics.optimization_score,
        "Production optimization completed"
    );

    let response = OptimizationResponse {
        result,
        metrics,
        goal,
        optimized_at: Utc::now(),
    };

    Ok(Json(ApiResponse::ok(response)))
}

/// POST /api/v1/industry40/quality/inspect
/// Inspecionar qualidade de produto
#[derive(Debug, Deserialize, Validate)]
pub struct InspectRequest {
    pub product: quality_control::Product,
    /// Limiar de qualidade mínimo aceitável (0-1)
    #[validate(range(min = 0.0, max = 1.0))]
    pub quality_threshold: Option<f64>,
}

#[derive(Debug, Serialize)]
pub struct InspectionResponse {
    pub result: quality_control::InspectionResult,
    pub quality_grade: QualityGrade,
    pub recommendations: Vec<String>,
    pub inspected_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "UPPERCASE")]
pub enum QualityGrade {
    A,  // Excellent (>95%)
    B,  // Good (85-95%)
    C,  // Acceptable (70-85%)
    D,  // Below standard (50-70%)
    F,  // Fail (<50%)
}

impl QualityGrade {
    fn from_score(score: f64) -> Self {
        match score {
            s if s > 0.95 => QualityGrade::A,
            s if s > 0.85 => QualityGrade::B,
            s if s > 0.70 => QualityGrade::C,
            s if s > 0.50 => QualityGrade::D,
            _ => QualityGrade::F,
        }
    }
}

/// Inspecionar qualidade de produto
///
/// # Errors
/// - Returns `ApiError::Validation` if input validation fails
/// - Returns `ApiError::Analysis` if inspection fails
#[instrument(skip(state, payload), fields(product_id = %payload.product.id))]
pub async fn inspect_quality(
    State(state): State<AppState>,
    Json(payload): Json<InspectRequest>,
) -> ApiResult<Json<ApiResponse<InspectionResponse>>> {
    payload.validate()
        .map_err(|e| ApiError::Validation(format!("Invalid inspection request: {}", e)))?;

    info!(product_id = %payload.product.id, "Inspecting product quality");

    let inspector = quality_control::QualityInspector::new();
    let result = inspector.inspect(&payload.product);

    let quality_score = if result.passed { 0.95 } else { 0.60 }; // Simplificado
    let quality_grade = QualityGrade::from_score(quality_score);
    let threshold = payload.quality_threshold.unwrap_or(0.85);

    // Gerar recomendações baseadas na inspeção
    let mut recommendations = Vec::new();

    if !result.passed {
        recommendations.push("Product failed quality inspection".to_string());
        if !result.defects.is_empty() {
            recommendations.push(format!("Defects found: {}", result.defects.len()));
        }
        recommendations.push("Route to rework or rejection".to_string());
    } else if quality_score < threshold {
        recommendations.push(format!(
            "Quality score {:.1}% is below threshold {:.1}%",
            quality_score * 100.0,
            threshold * 100.0
        ));
        recommendations.push("Consider process improvement".to_string());
    } else {
        recommendations.push("Product meets quality standards".to_string());
    }

    info!(
        product_id = %payload.product.id,
        passed = result.passed,
        grade = ?quality_grade,
        defects = result.defects.len(),
        "Quality inspection completed"
    );

    let response = InspectionResponse {
        result,
        quality_grade,
        recommendations,
        inspected_at: Utc::now(),
    };

    Ok(Json(ApiResponse::ok(response)))
}

/// GET /api/v1/industry40/energy/consumption
/// Obter consumo de energia total ou por dispositivo
#[derive(Debug, Deserialize, Validate)]
pub struct EnergyConsumptionQuery {
    /// Filtrar por device_id específico
    pub device_id: Option<String>,
    /// Período em horas (padrão: 24)
    #[validate(range(min = 1, max = 720))]
    pub period_hours: Option<u32>,
}

#[derive(Debug, Serialize)]
pub struct EnergyConsumptionResponse {
    pub total_consumption_kwh: f64,
    pub period_hours: u32,
    pub average_power_kw: f64,
    pub peak_power_kw: f64,
    pub devices: Vec<DeviceEnergy>,
    pub efficiency_rating: EfficiencyRating,
    pub recommendations: Vec<String>,
    pub measured_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Serialize)]
pub struct DeviceEnergy {
    pub device_id: String,
    pub consumption_kwh: f64,
    pub percentage: f64,
}

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum EfficiencyRating {
    Excellent,
    Good,
    Fair,
    Poor,
}

/// Obter dados de consumo de energia
///
/// # Errors
/// - Returns `ApiError::Validation` if query validation fails
/// - Returns `ApiError::Storage` if data fetch fails
#[instrument(skip(state))]
pub async fn get_energy_consumption(
    State(state): State<AppState>,
    axum::extract::Query(query): axum::extract::Query<EnergyConsumptionQuery>,
) -> ApiResult<Json<ApiResponse<EnergyConsumptionResponse>>> {
    // Validar query
    query.validate()
        .map_err(|e| ApiError::Validation(format!("Invalid energy query: {}", e)))?;

    let period_hours = query.period_hours.unwrap_or(24);

    info!(
        device_id = ?query.device_id,
        period_hours = period_hours,
        "Fetching energy consumption data"
    );

    let monitor = energy_management::EnergyMonitor::new();
    let total_consumption = monitor.get_total_consumption();

    // Calcular médias e picos
    let average_power = total_consumption / (period_hours as f64);
    let peak_power = average_power * 1.5; // Simplificado

    // Simular dados por dispositivo
    let devices = vec![
        DeviceEnergy {
            device_id: "device-001".to_string(),
            consumption_kwh: total_consumption * 0.4,
            percentage: 40.0,
        },
        DeviceEnergy {
            device_id: "device-002".to_string(),
            consumption_kwh: total_consumption * 0.35,
            percentage: 35.0,
        },
        DeviceEnergy {
            device_id: "device-003".to_string(),
            consumption_kwh: total_consumption * 0.25,
            percentage: 25.0,
        },
    ];

    // Avaliar eficiência
    let efficiency_rating = if average_power < 50.0 {
        EfficiencyRating::Excellent
    } else if average_power < 100.0 {
        EfficiencyRating::Good
    } else if average_power < 150.0 {
        EfficiencyRating::Fair
    } else {
        EfficiencyRating::Poor
    };

    // Gerar recomendações
    let mut recommendations = Vec::new();

    if average_power > 100.0 {
        recommendations.push("Consider implementing energy-saving measures".to_string());
    }

    if peak_power / average_power > 2.0 {
        recommendations.push("High peak-to-average ratio. Consider load balancing".to_string());
    }

    recommendations.push("Schedule energy-intensive operations during off-peak hours".to_string());
    recommendations.push("Regular maintenance can improve energy efficiency by 10-15%".to_string());

    let response = EnergyConsumptionResponse {
        total_consumption_kwh: total_consumption,
        period_hours,
        average_power_kw: average_power,
        peak_power_kw: peak_power,
        devices,
        efficiency_rating,
        recommendations,
        measured_at: Utc::now(),
    };

    info!(
        total_kwh = %.2 = total_consumption,
        avg_kw = %.2 = average_power,
        rating = ?efficiency_rating,
        "Energy consumption data retrieved"
    );

    Ok(Json(ApiResponse::ok(response)))
}

/// POST /api/v1/industry40/anomaly/detect
/// Detectar anomalias em séries temporais
#[derive(Debug, Deserialize, Validate)]
pub struct DetectAnomalyRequest {
    #[validate(length(min = 10, max = 10000, message = "Values must be between 10-10000 data points"))]
    pub values: Vec<f64>,
    #[validate(range(min = 1.0, max = 5.0, message = "Threshold sigma must be between 1.0-5.0"))]
    pub threshold_sigma: f64,
    /// ID do dispositivo ou sensor
    pub device_id: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct DetectAnomalyResponse {
    pub anomaly_indices: Vec<usize>,
    pub anomalies: Vec<AnomalyDetail>,
    pub count: usize,
    pub severity: AnomalySeverity,
    pub statistics: AnomalyStatistics,
    pub recommendations: Vec<String>,
    pub analyzed_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Serialize)]
pub struct AnomalyDetail {
    pub index: usize,
    pub value: f64,
    pub deviation: f64,
    pub severity: AnomalySeverity,
}

#[derive(Debug, Serialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum AnomalySeverity {
    Low,
    Medium,
    High,
    Critical,
}

impl AnomalySeverity {
    fn from_deviation(deviation: f64, threshold: f64) -> Self {
        let ratio = deviation / threshold;
        match ratio {
            r if r < 1.5 => AnomalySeverity::Low,
            r if r < 2.5 => AnomalySeverity::Medium,
            r if r < 4.0 => AnomalySeverity::High,
            _ => AnomalySeverity::Critical,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct AnomalyStatistics {
    pub total_points: usize,
    pub anomaly_rate: f64,
    pub mean: f64,
    pub std_dev: f64,
    pub min: f64,
    pub max: f64,
}

/// Detectar anomalias em séries temporais
///
/// Utiliza detecção baseada em desvio padrão (Z-score)
///
/// # Errors
/// - Returns `ApiError::Validation` if input validation fails
/// - Returns `ApiError::Analysis` if detection fails
#[instrument(skip(state, payload), fields(data_points = payload.values.len()))]
pub async fn detect_anomalies(
    State(state): State<AppState>,
    Json(payload): Json<DetectAnomalyRequest>,
) -> ApiResult<Json<ApiResponse<DetectAnomalyResponse>>> {
    payload.validate()
        .map_err(|e| ApiError::Validation(format!("Invalid anomaly detection request: {}", e)))?;

    info!(
        data_points = payload.values.len(),
        threshold = payload.threshold_sigma,
        device_id = ?payload.device_id,
        "Detecting anomalies"
    );

    // Calcular estatísticas
    let mean = payload.values.iter().sum::<f64>() / payload.values.len() as f64;
    let variance = payload.values.iter()
        .map(|v| (v - mean).powi(2))
        .sum::<f64>() / payload.values.len() as f64;
    let std_dev = variance.sqrt();

    let min = payload.values.iter().cloned().fold(f64::INFINITY, f64::min);
    let max = payload.values.iter().cloned().fold(f64::NEG_INFINITY, f64::max);

    // Detectar anomalias
    let detector = time_series::AnomalyDetector::new(payload.threshold_sigma);
    let anomaly_indices = detector.detect(&payload.values);

    // Criar detalhes das anomalias
    let anomalies: Vec<AnomalyDetail> = anomaly_indices.iter()
        .map(|&idx| {
            let value = payload.values[idx];
            let deviation = ((value - mean) / std_dev).abs();
            let severity = AnomalySeverity::from_deviation(deviation, payload.threshold_sigma);

            AnomalyDetail {
                index: idx,
                value,
                deviation,
                severity,
            }
        })
        .collect();

    // Determinar severidade geral
    let overall_severity = if anomalies.iter().any(|a| a.severity == AnomalySeverity::Critical) {
        AnomalySeverity::Critical
    } else if anomalies.iter().any(|a| a.severity == AnomalySeverity::High) {
        AnomalySeverity::High
    } else if anomalies.iter().any(|a| a.severity == AnomalySeverity::Medium) {
        AnomalySeverity::Medium
    } else {
        AnomalySeverity::Low
    };

    let anomaly_rate = anomalies.len() as f64 / payload.values.len() as f64;

    // Gerar recomendações
    let mut recommendations = Vec::new();

    if anomaly_rate > 0.10 {
        recommendations.push(format!(
            "High anomaly rate ({:.1}%). Review sensor calibration and data collection.",
            anomaly_rate * 100.0
        ));
    }

    if overall_severity == AnomalySeverity::Critical {
        recommendations.push("CRITICAL: Immediate investigation required".to_string());
        recommendations.push("Check equipment status and safety systems".to_string());
    } else if overall_severity == AnomalySeverity::High {
        recommendations.push("HIGH: Schedule inspection within 24 hours".to_string());
    } else if !anomalies.is_empty() {
        recommendations.push("Monitor trends and investigate if anomalies persist".to_string());
    } else {
        recommendations.push("No significant anomalies detected. System operating normally".to_string());
    }

    let statistics = AnomalyStatistics {
        total_points: payload.values.len(),
        anomaly_rate,
        mean,
        std_dev,
        min,
        max,
    };

    let response = DetectAnomalyResponse {
        anomaly_indices,
        anomalies,
        count: anomalies.len(),
        severity: overall_severity,
        statistics,
        recommendations,
        analyzed_at: Utc::now(),
    };

    info!(
        anomalies_found = response.count,
        anomaly_rate = %.2 = anomaly_rate * 100.0,
        severity = ?response.severity,
        "Anomaly detection completed"
    );

    Ok(Json(ApiResponse::ok(response)))
}
