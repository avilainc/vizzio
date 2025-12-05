//! Exemplo de uso do AVX API Core
//!
//! Demonstra como criar um servidor HTTP simples usando apenas código próprio.

use avx_api_core::{
    http::{Method, Request, Response, Router, Server, StatusCode},
    json::JsonValue,
};

fn main() -> Result<(), String> {
    println!("🚀 AVX API Core - Native Implementation Example");
    println!("================================================\n");

    let router = Router::new()
        // Endpoint simples de ping
        .get("/ping", |_req: &Request| {
            Response::new(StatusCode::OK).with_text("pong")
        })
        // Endpoint com JSON
        .get("/info", |_req: &Request| {
            let info = JsonValue::object(vec![
                ("service", JsonValue::String("avx-example".to_string())),
                ("version", JsonValue::String("1.0.0".to_string())),
                ("native", JsonValue::Bool(true)),
                ("dependencies", JsonValue::Number(0.0)),
            ]);
            Response::new(StatusCode::OK).with_json(&info.to_string())
        })
        // Endpoint com array
        .get("/features", |_req: &Request| {
            let features = JsonValue::array(vec![
                JsonValue::String("Native HTTP Server".to_string()),
                JsonValue::String("Custom JSON Parser".to_string()),
                JsonValue::String("Zero External Dependencies".to_string()),
                JsonValue::String("Type-safe Routing".to_string()),
            ]);
            
            let response = JsonValue::object(vec![
                ("features", features),
                ("count", JsonValue::Number(4.0)),
            ]);
            
            Response::new(StatusCode::OK).with_json(&response.to_string())
        })
        // Echo endpoint que retorna dados do request
        .get("/echo", |req: &Request| {
            let echo = JsonValue::object(vec![
                ("method", JsonValue::String(req.method.as_str().to_string())),
                ("path", JsonValue::String(req.path.clone())),
                ("version", JsonValue::String(req.version.clone())),
            ]);
            Response::new(StatusCode::OK).with_json(&echo.to_string())
        });

    let addr = "127.0.0.1:3000".parse().unwrap();
    
    println!("📡 Server listening on http://{}", addr);
    println!("\nAvailable endpoints:");
    println!("  • GET  http://{}/ping", addr);
    println!("  • GET  http://{}/info", addr);
    println!("  • GET  http://{}/features", addr);
    println!("  • GET  http://{}/echo\n", addr);
    println!("Press Ctrl+C to stop\n");

    Server::bind(addr, router)?.serve()
}
