# Arquitetura

## Visão Geral

A arquitetura do Avila Analytics é modular e desacoplada, permitindo extensibilidade e alta performance.

```
┌─────────────────────────────────────────────────┐
│              API Layer (HTTP/WS)                │
│  ┌──────────┐  ┌──────────┐  ┌──────────┐     │
│  │  REST    │  │WebSocket │  │  gRPC    │     │
│  └──────────┘  └──────────┘  └──────────┘     │
└────────────────────┬────────────────────────────┘
                     │
┌────────────────────▼────────────────────────────┐
│           Analytics Core Engine                 │
│  ┌──────────┐  ┌──────────┐  ┌──────────┐     │
│  │ Tracker  │  │  Funnel  │  │Segmentaçãol     │
│  └──────────┘  └──────────┘  └──────────┘     │
│  ┌──────────┐  ┌──────────┐  ┌──────────┐     │
│  │ Cohort   │  │Dashboard │  │Prediction│     │
│  └──────────┘  └──────────┘  └──────────┘     │
└────────────────────┬────────────────────────────┘
                     │
┌────────────────────▼────────────────────────────┐
│              ML/Streaming Layer                 │
│  ┌──────────┐  ┌──────────┐  ┌──────────┐     │
│  │ML Models │  │ Streaming│  │Real-time │     │
│  └──────────┘  └──────────┘  └──────────┘     │
└────────────────────┬────────────────────────────┘
                     │
┌────────────────────▼────────────────────────────┐
│              Storage Layer                      │
│  ┌──────────┐  ┌──────────┐  ┌──────────┐     │
│  │ AvilaDB  │  │  Memory  │  │EventStore│     │
│  └──────────┘  └──────────┘  └──────────┘     │
└─────────────────────────────────────────────────┘
```

## Componentes Principais

### 1. API Layer
- **REST API**: Endpoints HTTP para operações CRUD
- **WebSocket**: Comunicação real-time bidirecional
- **gRPC**: Alta performance para comunicação serviço-a-serviço

### 2. Analytics Core
- **Tracker**: Captura e validação de eventos
- **Funnel**: Análise de conversão
- **Segmentation**: Agrupamento de usuários
- **Cohort**: Análise de coortes
- **Prediction**: Modelos preditivos

### 3. ML/Streaming
- **ML Models**: Modelos de machine learning
- **Streaming**: Processamento de streams em tempo real
- **Real-time**: Análises em tempo real

### 4. Storage
- **AvilaDB**: Storage principal
- **Memory**: Cache em memória
- **EventStore**: Log de eventos

## Padrões de Projeto

### Event Sourcing
Todos os eventos são imutáveis e armazenados em ordem cronológica.

### CQRS (Command Query Responsibility Segregation)
Separação entre operações de escrita (commands) e leitura (queries).

### Actor Model
Uso de atores para processamento concorrente e isolamento.

## Performance

- **Latência**: < 10ms (p99) para ingestão de eventos
- **Throughput**: > 100k eventos/segundo
- **Memória**: Uso eficiente com pooling
- **CPU**: Paralelização com Rayon
