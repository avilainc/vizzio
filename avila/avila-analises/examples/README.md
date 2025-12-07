# Avila Analytics - Examples

Exemplos práticos demonstrando as capacidades da biblioteca.

## Exemplos Básicos

### 1. Event Tracking
```bash
cargo run --example basic_tracking
```
Demonstra rastreamento básico de eventos (page views, signups, purchases).

### 2. Funnel Analysis
```bash
cargo run --example funnel_analysis
```
Análise de funil de conversão e-commerce com cálculo de drop-off.

### 3. User Segmentation
```bash
cargo run --example user_segmentation
```
Segmentação dinâmica de usuários baseada em comportamento e valor.

## Exemplos Avançados

### 4. Real-time Dashboard
```bash
cargo run --example realtime_dashboard
```
Dashboard em tempo real com métricas atualizadas continuamente.

### 5. ML Predictions
```bash
cargo run --example ml_predictions
```
Predições com Machine Learning (churn, LTV, recomendações).

### 6. Streaming Analytics
```bash
cargo run --example streaming_analytics
```
Processamento de streams com agregações em janelas.

## Exemplos Industry 4.0

### 7. OEE Monitoring
```bash
cargo run --example industry40_oee
```
Cálculo de Overall Equipment Effectiveness para manufatura.

## Executar Todos

```bash
# Listar todos os exemplos
cargo run --example

# Executar com logging debug
RUST_LOG=debug cargo run --example basic_tracking
```

## Estrutura dos Exemplos

Cada exemplo segue o padrão:
1. Inicialização do sistema
2. Configuração de componentes
3. Geração/simulação de dados
4. Processamento e análise
5. Exibição de resultados

## Personalizando

Você pode modificar os exemplos para testar diferentes cenários:
- Alterar tamanhos de dataset
- Ajustar parâmetros de janela
- Modificar critérios de segmentação
- Experimentar diferentes modelos ML
