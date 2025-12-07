# Industry 4.0 API - Production Guide

## Overview

A API Industry 4.0 da Avila Analytics fornece endpoints de nível de produção para:
- Ingestão de telemetria IoT
- Manutenção preditiva
- Cálculo de OEE (Overall Equipment Effectiveness)
- Gêmeos digitais
- Otimização de produção
- Inspeção de qualidade
- Monitoramento de energia
- Detecção de anomalias

## Features de Produção

### ✅ Validação de Input
Todos os endpoints utilizam `validator` para validação rigorosa:
```rust
#[derive(Deserialize, Validate)]
pub struct TelemetryRequest {
    #[validate(length(min = 1, max = 100))]
    pub device_id: String,
    pub telemetry: ProductionTelemetry,
}
```

### ✅ Logging Estruturado
Logging com `tracing` para observabilidade:
```rust
#[instrument(skip(state), fields(device_id = %payload.device_id))]
pub async fn ingest_telemetry(...) {
    info!(temperature = telemetry.temperature, "Ingesting telemetry");
}
```

### ✅ Error Handling Tipado
Erros específicos e informativos:
```rust
.map_err(|e| ApiError::Validation(format!("Invalid data: {}", e)))?
```

### ✅ Métricas e Análise
Respostas enriquecidas com métricas e recomendações:
- Risk levels (Low, Medium, High, Critical)
- Performance categories
- Quality grades
- Efficiency ratings

## API Endpoints

### 1. Ingestão de Telemetria IoT

**POST** `/api/v1/industry40/iot/ingest`

```json
{
  "device_id": "device-001",
  "telemetry": {
    "temperature": 75.5,
    "pressure": 120.0,
    "vibration": 0.05,
    "power_consumption": 45.2,
    "cycle_time_ms": 1250,
    "production_count": 1000
  }
}
```

**Response:**
```json
{
  "success": true,
  "data": {
    "device_id": "device-001",
    "timestamp": "2025-12-05T10:30:00Z",
    "status": "success",
    "message": "Telemetry data ingested successfully"
  }
}
```

**Features:**
- ✅ Armazenamento em event store
- ✅ Validação de device_id (1-100 chars)
- ✅ Logging estruturado
- ✅ Timestamps UTC

---

### 2. Manutenção Preditiva

**POST** `/api/v1/industry40/maintenance/predict`

```json
{
  "device_id": "device-001",
  "telemetry": { ... },
  "historical_data": [ ... ] // Opcional, máx 1000 registros
}
```

**Response:**
```json
{
  "success": true,
  "data": {
    "device_id": "device-001",
    "failure_probability": 0.72,
    "risk_level": "high",
    "alert": {
      "component": "Bearing A",
      "action": "Replace bearing",
      "failure_probability": 0.72
    },
    "predictions": [
      "Component 'Bearing A' requires attention",
      "Recommended action: Replace bearing",
      "Monitor closely for next 24-48 hours"
    ],
    "confidence": 0.85,
    "analyzed_at": "2025-12-05T10:30:00Z"
  }
}
```

**Risk Levels:**
- `low`: < 25% failure probability
- `medium`: 25-50%
- `high`: 50-75%
- `critical`: > 75%

**Features:**
- ✅ Busca automática de dados históricos
- ✅ Treinamento de modelo
- ✅ Cálculo de confiança
- ✅ Recomendações contextuais
- ✅ Alertas baseados em risco

---

### 3. Cálculo de OEE

**POST** `/api/v1/industry40/oee/calculate`

```json
{
  "device_id": "device-001",
  "production_data": {
    "planned_production_time_hours": 8.0,
    "actual_runtime_hours": 7.2,
    "ideal_cycle_time_ms": 1000,
    "actual_cycle_time_ms": 1100,
    "total_units_produced": 25000,
    "good_units": 24500,
    "defects": 500
  },
  "target_cycle_time_ms": 1000,
  "planned_hours": 8.0
}
```

**Response:**
```json
{
  "success": true,
  "data": {
    "device_id": "device-001",
    "metrics": {
      "oee": 0.82,
      "availability": 0.90,
      "performance": 0.95,
      "quality": 0.98
    },
    "performance_category": "competitive",
    "recommendations": [
      "Good performance. Focus on continuous improvement.",
      "Quality rate is 98%. Target: 99%+. Improve quality control."
    ],
    "calculated_at": "2025-12-05T10:30:00Z"
  }
}
```

**Performance Categories:**
- `world_class`: OEE ≥ 85%
- `competitive`: OEE ≥ 70%
- `average`: OEE ≥ 50%
- `below_average`: OEE < 50%

**OEE Formula:**
```
OEE = Availability × Performance × Quality

Availability = Runtime / Planned Time
Performance = (Ideal Cycle Time × Total Count) / Runtime
Quality = Good Count / Total Count
```

---

### 4. Gêmeo Digital

**GET** `/api/v1/industry40/twin/:device_id`

**Response:**
```json
{
  "success": true,
  "data": {
    "twin_id": "twin-device-001",
    "device_id": "device-001",
    "health_status": "healthy",
    "last_sync": "2025-12-05T10:30:00Z",
    "predictions": [],
    "sensors": {}
  }
}
```

**Features:**
- ✅ Validação de device_id
- ✅ Verificação de existência do dispositivo
- ✅ Logging de status de saúde

---

### 5. Otimização de Produção

**POST** `/api/v1/industry40/optimize/production`

```json
{
  "orders": [
    {
      "order_id": "ORD-001",
      "product_id": "PROD-A",
      "quantity": 1000,
      "priority": 1,
      "deadline": "2025-12-10T00:00:00Z"
    }
  ],
  "constraints": {
    "max_parallel_orders": 5,
    "available_machines": 10
  },
  "optimization_goal": "minimize_time" // ou "minimize_cost", "maximize_quality", "balanced"
}
```

**Response:**
```json
{
  "success": true,
  "data": {
    "result": {
      "scheduled_orders": [...],
      "estimated_completion": "2025-12-08T16:00:00Z"
    },
    "metrics": {
      "total_orders": 10,
      "optimized_orders": 10,
      "estimated_time_savings_percent": 15.0,
      "optimization_score": 0.85
    },
    "goal": "minimize_time",
    "optimized_at": "2025-12-05T10:30:00Z"
  }
}
```

**Optimization Goals:**
- `minimize_time`: Reduzir tempo total de produção
- `minimize_cost`: Minimizar custos operacionais
- `maximize_quality`: Maximizar qualidade do produto
- `balanced`: Equilíbrio entre todos os fatores

---

### 6. Inspeção de Qualidade

**POST** `/api/v1/industry40/quality/inspect`

```json
{
  "product": {
    "id": "PROD-12345",
    "batch": "BATCH-001",
    "dimensions": [100.0, 50.0, 25.0],
    "weight": 2.5
  },
  "quality_threshold": 0.85 // Opcional, padrão: 0.85
}
```

**Response:**
```json
{
  "success": true,
  "data": {
    "result": {
      "passed": true,
      "defects": [],
      "score": 0.95
    },
    "quality_grade": "A",
    "recommendations": [
      "Product meets quality standards"
    ],
    "inspected_at": "2025-12-05T10:30:00Z"
  }
}
```

**Quality Grades:**
- `A`: Excellent (> 95%)
- `B`: Good (85-95%)
- `C`: Acceptable (70-85%)
- `D`: Below standard (50-70%)
- `F`: Fail (< 50%)

---

### 7. Consumo de Energia

**GET** `/api/v1/industry40/energy/consumption?period_hours=24&device_id=device-001`

**Response:**
```json
{
  "success": true,
  "data": {
    "total_consumption_kwh": 1250.5,
    "period_hours": 24,
    "average_power_kw": 52.1,
    "peak_power_kw": 78.2,
    "devices": [
      {
        "device_id": "device-001",
        "consumption_kwh": 500.2,
        "percentage": 40.0
      }
    ],
    "efficiency_rating": "good",
    "recommendations": [
      "Schedule energy-intensive operations during off-peak hours",
      "Regular maintenance can improve energy efficiency by 10-15%"
    ],
    "measured_at": "2025-12-05T10:30:00Z"
  }
}
```

**Efficiency Ratings:**
- `excellent`: < 50 kW average
- `good`: 50-100 kW
- `fair`: 100-150 kW
- `poor`: > 150 kW

---

### 8. Detecção de Anomalias

**POST** `/api/v1/industry40/anomaly/detect`

```json
{
  "values": [45.2, 46.1, 45.8, 89.5, 46.0, ...], // 10-10000 pontos
  "threshold_sigma": 3.0, // 1.0-5.0 (padrão: 3.0)
  "device_id": "sensor-001" // Opcional
}
```

**Response:**
```json
{
  "success": true,
  "data": {
    "anomaly_indices": [3, 15, 42],
    "anomalies": [
      {
        "index": 3,
        "value": 89.5,
        "deviation": 4.2,
        "severity": "high"
      }
    ],
    "count": 3,
    "severity": "high",
    "statistics": {
      "total_points": 1000,
      "anomaly_rate": 0.003,
      "mean": 46.2,
      "std_dev": 2.1,
      "min": 42.0,
      "max": 89.5
    },
    "recommendations": [
      "HIGH: Schedule inspection within 24 hours"
    ],
    "analyzed_at": "2025-12-05T10:30:00Z"
  }
}
```

**Severity Levels:**
- `low`: < 1.5× threshold
- `medium`: 1.5-2.5× threshold
- `high`: 2.5-4× threshold
- `critical`: > 4× threshold

**Detection Method:**
Utiliza Z-score (desvio padrão):
```
Z = (x - μ) / σ
Anomaly if |Z| > threshold_sigma
```

---

## Error Handling

### Error Responses

```json
{
  "success": false,
  "error": {
    "code": "VALIDATION_ERROR",
    "message": "Invalid telemetry data: device_id must be between 1-100 characters"
  }
}
```

### Error Codes
- `VALIDATION_ERROR`: Input validation failed
- `NOT_FOUND`: Resource not found
- `STORAGE_ERROR`: Storage operation failed
- `ANALYSIS_ERROR`: Analysis/calculation failed
- `INTERNAL_ERROR`: Internal server error

---

## Best Practices

### 1. Always Validate Input
```rust
payload.validate()
    .map_err(|e| ApiError::Validation(...))?;
```

### 2. Use Structured Logging
```rust
#[instrument(skip(state), fields(device_id = %id))]
info!(metric = value, "Event description");
```

### 3. Provide Context in Errors
```rust
.map_err(|e| {
    error!(error = %e, "Operation failed");
    ApiError::Storage(format!("Context: {}", e))
})?
```

### 4. Return Rich Responses
- Include timestamps
- Provide recommendations
- Add contextual metrics
- Use clear categorizations

### 5. Handle Edge Cases
```rust
if data.is_empty() {
    return Err(ApiError::Validation("Empty data".to_string()));
}
```

---

## Observability

### Metrics
- Request latency
- Error rates
- Prediction accuracy
- OEE scores
- Energy consumption

### Tracing
Todos os endpoints incluem:
- Request ID
- User/Device ID
- Timestamps
- Duration
- Result status

### Logging Levels
- `ERROR`: Failures críticos
- `WARN`: Avisos (dados faltando, threshold exceeded)
- `INFO`: Operações normais
- `DEBUG`: Detalhes de debugging
- `TRACE`: Informação detalhada de fluxo

---

## Performance

### Timeouts
- Telemetry ingestion: < 100ms
- Predictions: < 500ms
- OEE calculation: < 200ms
- Anomaly detection: < 1s (dependent on data size)

### Rate Limits
- Telemetry: 1000 requests/min per device
- Analytics: 100 requests/min per user
- Batch operations: 10 requests/min

### Caching
- Digital twin state: 5 minutes
- Energy statistics: 15 minutes
- Historical predictions: 1 hour

---

## Security

### Authentication
Todos os endpoints requerem:
- Valid API key header
- JWT token (production)

### Authorization
- Device-level access control
- Role-based permissions
- Audit logging

### Data Protection
- PII anonymization
- Encrypted storage
- GDPR compliance

---

## Testing

### Unit Tests
```bash
cargo test --test industry40_tests
```

### Integration Tests
```bash
cargo test --test api_tests -- --include-ignored
```

### Load Tests
```bash
cargo bench --bench industry40_bench
```

---

## Deployment

### Docker
```dockerfile
FROM rust:1.70 as builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
COPY --from=builder /app/target/release/avila-analises /usr/local/bin/
CMD ["avila-analises"]
```

### Environment Variables
```bash
AVILA_LOG_LEVEL=info
AVILA_API_PORT=3000
AVILA_DB_URL=postgres://...
AVILA_REDIS_URL=redis://...
```

---

## Support

- Documentation: https://docs.avila-analytics.com
- Issues: https://github.com/avila/avila-analises/issues
- Discord: https://discord.gg/avila

**Version:** 0.1.0
**Last Updated:** December 5, 2025
