# Avila Analytics

[![Rust](https://img.shields.io/badge/rust-1.70%2B-orange.svg)](https://www.rust-lang.org)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

🚀 Biblioteca Rust de alta performance para analytics em tempo real, machine learning e Industry 4.0.

## ✨ Features

### 📊 Core Analytics
- **Event Tracking** - Captura e processamento de eventos em tempo real
- **Funnel Analysis** - Análise de conversão e identificação de drop-offs
- **User Segmentation** - Segmentação dinâmica baseada em comportamento
- **Cohort Analysis** - Análise de coortes e retenção
- **Dashboard Metrics** - Métricas agregadas para dashboards

### 🤖 Machine Learning
- **Classification** - Logistic Regression, Random Forest, XGBoost
- **Regression** - Linear, Ridge, Lasso, Polynomial
- **Clustering** - K-Means, DBSCAN, Hierarchical
- **Feature Engineering** - Scaling, encoding, selection
- **Model Registry** - Versionamento e deployment de modelos

### 🌊 Streaming Analytics
- **Real-time Processing** - Processamento de streams com baixa latência
- **Window Operations** - Tumbling, Sliding, Session windows
- **Stream Aggregations** - Agregações em tempo real
- **Kafka/Kinesis Integration** - Conectores para message queues

### 🏭 Industry 4.0
- **OEE Monitoring** - Overall Equipment Effectiveness
- **Predictive Maintenance** - Manutenção preditiva com ML
- **Quality Control** - Controle de qualidade automatizado
- **Digital Twin** - Gêmeos digitais de equipamentos
- **Production Optimizer** - Otimização de produção

## 🚀 Quick Start

### Instalação

```toml
[dependencies]
avila-analises = "0.1.0"
tokio = { version = "1.0", features = ["full"] }
serde_json = "1.0"
```

### Exemplo Básico

```rust
use avila_analises::*;
use serde_json::json;

#[tokio::main]
async fn main() -> Result<()> {
    // Inicializar analytics
    let analytics = Analytics::new().await?;

    // Rastrear evento
    analytics.track("user_signup", json!({
        "user_id": "123",
        "email": "user@example.com"
    })).await?;

    Ok(())
}
```

## 📖 Documentação

- [Guia de Início Rápido](docs/getting-started.md)
- [Arquitetura](docs/architecture.md)
- [API Reference](docs/api/README.md)
- [Tutoriais](docs/tutorials/README.md)
- [Exemplos](examples/README.md)

## 🎯 Exemplos

```bash
# Event tracking básico
cargo run --example basic_tracking

# Análise de funil
cargo run --example funnel_analysis

# Segmentação de usuários
cargo run --example user_segmentation

# Dashboard em tempo real
cargo run --example realtime_dashboard

# Machine Learning
cargo run --example ml_predictions

# Industry 4.0 OEE
cargo run --example industry40_oee
```

## 🏗️ Estrutura do Projeto

```
avila-analises/
├── src/
│   ├── api/              # REST API & WebSocket
│   ├── storage/          # Storage backends
│   ├── ml/              # Machine Learning
│   ├── streaming/       # Stream processing
│   ├── industry40/      # Industry 4.0 modules
│   ├── export/          # Data export (CSV, Parquet)
│   └── websocket/       # WebSocket handlers
├── tests/
│   ├── unit/            # Unit tests
│   └── integration/     # Integration tests
├── benches/             # Performance benchmarks
├── docs/                # Documentation
└── examples/            # Example applications
```

## ⚡ Performance

- **Latência**: < 10ms (p99) para ingestão de eventos
- **Throughput**: > 100k eventos/segundo
- **Memória**: Uso eficiente com pooling
- **Escalabilidade**: Suporte para processamento distribuído

## 🛠️ Desenvolvimento

### Build

```bash
# Development build
cargo build

# Release build
cargo build --release
```

### Testes

```bash
# Todos os testes
cargo test

# Testes específicos
cargo test --test integration

# Com coverage
cargo tarpaulin --out Html
```

### Benchmarks

```bash
# Todos os benchmarks
cargo bench

# Benchmark específico
cargo bench --bench event_ingestion
```

## 📊 Roadmap

Veja [BLUEPRINT.md](BLUEPRINT.md) para o plano completo de desenvolvimento (18 meses).

### Próximas Releases

**v0.2.0** (Q1 2024)
- [ ] Suite completa de testes (80%+ coverage)
- [ ] Documentação completa
- [ ] CI/CD pipeline

**v0.3.0** (Q2 2024)
- [ ] Advanced analytics (A/B testing, attribution)
- [ ] Real-time stream processing
- [ ] Recommendation engine

**v0.4.0** (Q3 2024)
- [ ] Deep learning integration
- [ ] AutoML pipeline
- [ ] Time series forecasting

## 🤝 Contribuindo

Contribuições são bem-vindas! Veja [CONTRIBUTING.md](CONTRIBUTING.md) para guidelines.

## 📝 Licença

Este projeto está licenciado sob a licença MIT - veja [LICENSE](LICENSE) para detalhes.

## 🙏 Agradecimentos

Construído com:
- [Tokio](https://tokio.rs) - Async runtime
- [Axum](https://github.com/tokio-rs/axum) - Web framework
- [Serde](https://serde.rs) - Serialization
- [Rayon](https://github.com/rayon-rs/rayon) - Parallelism

## 📧 Contato

- **Repositório**: https://github.com/vizzio/avila-analises
- **Issues**: https://github.com/vizzio/avila-analises/issues
- **Documentação**: https://docs.avila.io/analises

---

Feito com ❤️ pela equipe Vizzio
