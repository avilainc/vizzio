# Guia de Início Rápido

## Instalação

Adicione ao seu `Cargo.toml`:

```toml
[dependencies]
avila-analises = "0.1.0"
tokio = { version = "1.0", features = ["full"] }
serde_json = "1.0"
```

## Configuração Básica

### 1. Inicialização

```rust
use avila_analises::Analytics;

#[tokio::main]
async fn main() -> Result<()> {
    let analytics = Analytics::builder()
        .storage(StorageType::AvilaDB)
        .enable_websocket(true)
        .build()
        .await?;

    Ok(())
}
```

### 2. Rastreamento de Eventos

```rust
// Evento simples
analytics.track("page_view", json!({
    "url": "/dashboard",
    "user_id": "user_123"
})).await?;

// Evento com propriedades customizadas
analytics.track_with_context("purchase", json!({
    "amount": 99.99,
    "currency": "USD",
    "items": ["product_1", "product_2"]
}), context).await?;
```

### 3. Análise de Funil

```rust
use avila_analises::funnel::*;

let funnel = FunnelBuilder::new()
    .add_step("visit_landing")
    .add_step("click_cta")
    .add_step("complete_signup")
    .time_window(Duration::from_secs(3600))
    .build();

let results = analytics.analyze_funnel(&funnel).await?;
```

### 4. Segmentação de Usuários

```rust
use avila_analises::segmentation::*;

let segment = SegmentBuilder::new("high_value_users")
    .add_condition("total_purchases", Operator::GreaterThan, 1000)
    .add_condition("last_active", Operator::Within, "30 days")
    .build();

let users = analytics.get_segment(&segment).await?;
```

## Próximos Passos

- [Arquitetura Detalhada](architecture.md)
- [Tutoriais Avançados](tutorials/README.md)
- [API Reference](api/README.md)
