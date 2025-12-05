# avila-monitor v5.0 🚀

Sistema avançado de monitoramento e análise de métricas em tempo real.

## 🌟 Novidades v5.0

### Recursos Principais

- **📊 Percentis Automáticos**: P50, P75, P90, P95, P99
- **🔍 Detecção de Outliers**: Algoritmo IQR (Interquartile Range)
- **📦 Agregações Temporais**: Janelas configuráveis com auto-agregação
- **🔗 Análise de Correlação**: Correlação de Pearson entre métricas
- **⏱️ Queries Temporais**: Busca por intervalo de timestamps
- **🎯 Benchmarks**: Comparação com baselines
- **📈 Crescimento**: Taxa de crescimento automática
- **📉 Média Móvel**: Janela deslizante configurável
- **⚡ Taxa de Mudança**: Derivative em tempo real
- **🏷️ Metadados**: Nome, unidade e descrição por métrica
- **🚨 Sistema de Alertas**: Limiares máx/mín configuráveis
- **💾 Exportação**: Formato CSV pronto para análise
- **🧹 Cleanup Automático**: Gerenciamento de histórico

## 📦 Instalação

```rust
// Em seu projeto, inclua:
extern crate alloc;
use alloc::collections::BTreeMap;
use alloc::vec::Vec;

// Copie avila-monitor/src/lib.rs para seu projeto
```

## 🚀 Quick Start

### Exemplo Básico

```rust
use avila_monitor::Monitor;

let mut monitor = Monitor::new();

// Configurar métrica
monitor.set_metadata(1, "response_time", "ms", "API response time");

// Coletar dados
monitor.record_with_timestamp(1, 125.5, 1000);
monitor.record_with_timestamp(1, 98.3, 2000);

// Analisar
let stats = monitor.calculate_statistics(1);
println!("Mean: {:.2} ms", stats.unwrap().mean);
```

### Exemplo Avançado

```rust
use avila_monitor::Monitor;

// Monitor com agregação automática (janelas de 60s)
let mut monitor = Monitor::with_aggregation(60000);

// Metadados
monitor.set_metadata(1, "latency", "ms", "Request latency");
monitor.set_metadata(2, "throughput", "req/s", "Requests per second");

// Alertas
monitor.add_max_alert(1, 1000.0); // Latency > 1s
monitor.add_min_alert(2, 10.0);   // Throughput < 10 req/s

// Coleta contínua
for i in 0..1000 {
    let timestamp = i * 1000;
    monitor.record_with_timestamp(1, calc_latency(), timestamp);
    monitor.record_with_timestamp(2, calc_throughput(), timestamp);
}

// Análise completa
let percentiles = monitor.calculate_percentiles(1).unwrap();
println!("P95 latency: {:.2} ms", percentiles.p95);

let outliers = monitor.detect_outliers(1).unwrap();
println!("Detected {} anomalies", outliers.len());

let correlation = monitor.correlation(1, 2).unwrap();
println!("Latency vs Throughput correlation: {:.3}", correlation);

// Agregações
monitor.aggregate_windows(1);
let windows = monitor.get_aggregations(1).unwrap();
for win in windows {
    println!("Window: {} samples, avg={:.2}", win.count, win.sum / win.count as f64);
}
```

## 📚 API Completa

### Construção

```rust
Monitor::new()                      // Padrão (histórico=100)
Monitor::with_history_size(1000)    // Histórico customizado
Monitor::with_aggregation(60000)    // Com agregação (janelas de 60s)
```

### Coleta

```rust
record(metric_id, value)                          // Valor simples
record_with_timestamp(metric_id, value, ts)       // Com timestamp
increment(metric_id, delta)                        // Incrementar
decrement(metric_id, delta)                        // Decrementar
```

### Consulta

```rust
get(metric_id)                                    // Valor atual
get_history(metric_id)                            // Histórico completo
query_range(metric_id, start_ts, end_ts)         // Range temporal
```

### Estatísticas

```rust
calculate_statistics(metric_id)       // min, max, mean, variance, std_dev
calculate_percentiles(metric_id)      // P50, P75, P90, P95, P99
average()                             // Média de todas métricas
max_metric()                          // Métrica com maior valor
min_metric()                          // Métrica com menor valor
```

### Análise Avançada

```rust
detect_outliers(metric_id)            // Detecção de anomalias (IQR)
correlation(metric_id1, metric_id2)   // Correlação de Pearson
moving_average(metric_id, window)     // Média móvel
calculate_rate(metric_id)             // Taxa de mudança (derivative)
growth_rate(metric_id)                // Crescimento percentual
benchmark(metric_id, baseline)        // Comparação vs baseline
```

### Agregações

```rust
aggregate_windows(metric_id)          // Criar janelas temporais
get_aggregations(metric_id)           // Obter agregações
```

### Metadados e Alertas

```rust
set_metadata(id, name, unit, desc)    // Definir metadados
get_metadata(metric_id)               // Consultar metadados
add_max_alert(metric_id, threshold)   // Alerta de máximo
add_min_alert(metric_id, threshold)   // Alerta de mínimo
```

### Utilitários

```rust
summary()                             // Resumo de todas métricas
export_csv(metric_id)                 // Exportar para CSV
cleanup_history(before_timestamp)     // Limpar histórico antigo
reset_metric(metric_id)               // Resetar métrica específica
clear()                               // Limpar todas métricas
count()                               // Número de métricas ativas
```

## 🎯 Casos de Uso

### 1. Monitoramento de API

```rust
let mut mon = Monitor::with_aggregation(60000);
mon.set_metadata(1, "response_time", "ms", "API latency");
mon.add_max_alert(1, 1000.0);

// Loop de coleta
loop {
    let latency = measure_request();
    mon.record_with_timestamp(1, latency, timestamp());

    if let Some(p95) = mon.calculate_percentiles(1).map(|p| p.p95) {
        if p95 > 500.0 {
            alert("P95 latency above 500ms");
        }
    }
}
```

### 2. Monitoramento de Recursos

```rust
let mut mon = Monitor::new();
mon.set_metadata(1, "cpu", "%", "CPU usage");
mon.set_metadata(2, "memory", "MB", "Memory usage");
mon.set_metadata(3, "disk_io", "MB/s", "Disk I/O");

// Verificar correlação
let corr = mon.correlation(1, 2).unwrap();
if corr > 0.8 {
    println!("Strong correlation between CPU and Memory");
}
```

### 3. Detecção de Anomalias

```rust
let mut mon = Monitor::new();

// Coletar métricas normais
for _ in 0..100 {
    mon.record_with_timestamp(1, normal_value(), timestamp());
}

// Detectar outliers
let outliers = mon.detect_outliers(1).unwrap();
for (ts, value) in outliers {
    alert(format!("Anomaly at {}: value={}", ts, value));
}
```

### 4. SLO/SLA Tracking

```rust
let mut mon = Monitor::new();
mon.set_metadata(1, "availability", "%", "Service uptime");

let percentiles = mon.calculate_percentiles(1).unwrap();
let slo_target = 99.9; // 99.9% uptime

if percentiles.p99 < slo_target {
    alert("SLO breach: P99 availability below target");
}
```

## 📊 Estruturas de Dados

### Statistics

```rust
pub struct Statistics {
    pub min: f64,
    pub max: f64,
    pub mean: f64,
    pub variance: f64,
    pub std_dev: f64,
}
```

### Percentiles

```rust
pub struct Percentiles {
    pub p50: f64,  // Mediana
    pub p75: f64,
    pub p90: f64,
    pub p95: f64,
    pub p99: f64,
}
```

### TimeWindow

```rust
pub struct TimeWindow {
    pub start: u64,
    pub end: u64,
    pub count: usize,
    pub sum: f64,
    pub min: f64,
    pub max: f64,
}
```

### MetricMetadata

```rust
pub struct MetricMetadata {
    pub name: &'static str,
    pub unit: &'static str,
    pub description: &'static str,
}
```

## 🧪 Testes

A v5.0 inclui 17 testes abrangentes:

```bash
rustc --test src/lib.rs -o tests && ./tests
```

Testes incluem:
- ✅ Operações básicas (record, get, remove)
- ✅ Estatísticas (avg, max, min, variance)
- ✅ Metadados
- ✅ Percentis
- ✅ Detecção de outliers
- ✅ Queries por range
- ✅ Agregações temporais
- ✅ Correlação
- ✅ Benchmark
- ✅ Taxa de crescimento
- ✅ Exportação CSV
- ✅ Cleanup de histórico
- ✅ Reset de métricas

## 🎨 Exemplo Completo

Execute o demo:

```bash
rustc examples/v5_demo.rs --edition 2021 -o demo && ./demo
```

Output esperado:
```
=== avila-monitor v5.0 Demo ===

📊 Coletando métricas...

📈 Estatísticas Básicas:
   Total de métricas: 3
   Média geral: 1789.50

📊 Response Time (métrica 1):
   Min: 100.00 ms
   Max: 1490.00 ms
   Mean: 224.50 ms
   Std Dev: 187.32 ms

📈 Percentis do Response Time:
   P50 (mediana): 190.00 ms
   P75: 200.00 ms
   P90: 590.00 ms
   P95: 690.00 ms
   P99: 1390.00 ms

🔍 Detecção de Outliers:
   5 outliers detectados

... (e muito mais!)
```

## 🏆 Performance

- **Coleta**: O(log n) - BTreeMap insert
- **Query**: O(n) - Linear scan com filtro
- **Percentis**: O(n log n) - Sort + index
- **Outliers**: O(n log n) - IQR calculation
- **Agregações**: O(n) - Single pass
- **Correlação**: O(n) - Linear correlation

## 🔒 Segurança

- `no_std` compatible com `alloc`
- Type-safe IDs (u64)
- Bounds checking automático
- Sem panics em operações normais
- Gerenciamento de memória controlado

## 📈 Roadmap v6.0

- [ ] Streaming analytics
- [ ] Distributed tracing integration
- [ ] Custom aggregation functions
- [ ] Prometheus/OpenTelemetry export
- [ ] Time-series forecasting
- [ ] Anomaly detection ML models

## 🤝 Contribuindo

Melhorias são bem-vindas! Áreas de interesse:
- Otimizações de performance
- Novos algoritmos estatísticos
- Exportadores para diferentes formatos
- Exemplos de integração

## 📄 Licença

Parte do projeto Avila - Internal use

## ✨ Changelog v5.0

### Added
- Percentis (P50, P75, P90, P95, P99)
- Detecção de outliers com IQR
- Agregações temporais
- Análise de correlação
- Queries por intervalo
- Metadados de métricas
- Benchmark vs baseline
- Taxa de crescimento
- Exportação CSV
- Cleanup de histórico
- Auto-agregação
- 11 novos testes

### Enhanced
- Histórico com timestamps
- Sistema de alertas
- Estatísticas avançadas
- Documentação completa
- Exemplos práticos

---

**avila-monitor v5.0** - Production-ready system monitoring 🎉
