# Avila Analytics - Documentação

## Visão Geral

Avila Analytics é uma biblioteca Rust de alta performance para analytics em tempo real, com foco em:

- 📊 **Event Tracking** - Captura e processamento de eventos
- 🔍 **Análise de Funil** - Análise de conversão e drop-off
- 👥 **Segmentação** - Agrupamento dinâmico de usuários
- 🏭 **Industry 4.0** - Análises específicas para manufatura
- 🤖 **Machine Learning** - Predições e recomendações

## Índice

- [Guia de Início Rápido](getting-started.md)
- [Arquitetura](architecture.md)
- [API Reference](api/README.md)
- [Tutoriais](tutorials/README.md)
- [Exemplos](examples/README.md)
- [Performance](performance.md)
- [Deployment](deployment.md)

## Quick Start

```rust
use avila_analises::*;

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

## Links Úteis

- [GitHub Repository](https://github.com/vizzio/avila-analises)
- [Changelog](CHANGELOG.md)
- [Contributing](CONTRIBUTING.md)
