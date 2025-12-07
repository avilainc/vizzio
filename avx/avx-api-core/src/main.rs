//! AVX API Core Service
//!
//! Main service entry point - 100% código próprio, sem dependências externas.

use avx_api_core::{
    http::{Method, Request, Response, Router, Server, StatusCode},
    json::JsonValue,
    ForecastService, SERVICE_NAME,
};
use avx_config::AvxConfig;
use avx_telemetry::{self, AvxContext, AvxMetrics};
use std::sync::{Arc, Mutex};

fn main() -> Result<(), String> {
    let cfg = config::load();
    let ctx = context::from_config(&cfg);
    observability::init_tracing(&ctx);

    let metrics = Arc::new(Mutex::new(AvxMetrics::new()));
    let forecast_service = Arc::new(ForecastService::new());

    let router = routing::build(metrics, forecast_service);
    let addr = config::http_addr(&cfg)?;

    println!("Starting {} on {}", SERVICE_NAME, addr);
    Server::bind(addr, router)?.serve()
}

mod config {
    use avx_api_core::DEFAULT_BIND_ADDR;
    use avx_config::AvxConfig;
    use std::net::SocketAddr;

    /// Loads configuration from environment or defaults
    pub fn load() -> AvxConfig {
        let mut cfg = AvxConfig::load().unwrap_or_else(|_| AvxConfig::with_defaults());
        cfg.http.bind_addr = DEFAULT_BIND_ADDR.into();
        cfg
    }

    /// Parses HTTP address from configuration
    pub fn http_addr(cfg: &AvxConfig) -> Result<SocketAddr, String> {
        cfg.http.bind_addr.parse().map_err(|e| format!("Invalid address: {}", e))
    }
}

mod context {
    use super::{AvxConfig, AvxContext};

    pub fn from_config(cfg: &AvxConfig) -> AvxContext {
        AvxContext {
            stack: cfg.stack.clone(),
            layer: cfg.layer.clone(),
            env: cfg.env.clone(),
            cluster: cfg.cluster.clone(),
            mesh: cfg.mesh.clone(),
        }
    }
}

mod observability {
    use super::{avx_telemetry, AvxContext};

    pub fn init_tracing(ctx: &AvxContext) {
        avx_telemetry::init_tracing(ctx);
    }
}

mod routing {
    use super::{Arc, AvxMetrics, ForecastService, JsonValue, Method, Mutex, Request, Response, Router, StatusCode, SERVICE_NAME};

    pub fn build(
        metrics: Arc<Mutex<AvxMetrics>>,
        forecast_service: Arc<ForecastService>,
    ) -> Router {
        Router::new()
            .get("/core/ping", move |_req: &Request| {
                Response::new(StatusCode::OK).with_text("pong from avx-api-core")
            })
            .get("/core/status", move |_req: &Request| {
                let status = JsonValue::object(vec![
                    ("service", JsonValue::String(SERVICE_NAME.to_string())),
                    ("status", JsonValue::String("healthy".to_string())),
                    ("version", JsonValue::String(env!("CARGO_PKG_VERSION").to_string())),
                ]);
                Response::new(StatusCode::OK).with_json(&status.to_string())
            })
            .get("/core/forecast", move |_req: &Request| {
                let series = vec![100.0, 105.0, 98.0, 110.0, 115.0, 102.0, 108.0, 112.0, 107.0, 120.0];
                let steps = 5;

                let metrics_guard = metrics.lock().unwrap();
                match metrics_guard.forecast_metric(series.clone(), steps) {
                    Ok(forecast) => {
                        println!("[INFO] Generated forecast for request rates");
                        let response = JsonValue::object(vec![
                            ("service", JsonValue::String(SERVICE_NAME.to_string())),
                            ("historical_data", JsonValue::from(series)),
                            ("forecast", JsonValue::from(forecast)),
                            ("forecast_steps", JsonValue::Number(steps as f64)),
                            ("model", JsonValue::String("ARIMA(1,1,1)".to_string())),
                        ]);
                        Response::new(StatusCode::OK).with_json(&response.to_string())
                    }
                    Err(error) => {
                        let response = JsonValue::object(vec![
                            ("error", JsonValue::String(error)),
                            ("service", JsonValue::String(SERVICE_NAME.to_string())),
                        ]);
                        Response::new(StatusCode::INTERNAL_SERVER_ERROR).with_json(&response.to_string())
                    }
                }
            })
    }
}
