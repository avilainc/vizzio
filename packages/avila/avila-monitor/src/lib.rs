//! # avila-monitor v5.0 - Advanced System Monitoring
//!
//! Biblioteca de monitoramento de sistema e coleta de métricas com recursos avançados.
//!
//! ## Características v5.0
//!
//! - **Coleta de Métricas**: Registra e recupera métricas numéricas com timestamps
//! - **Histórico Temporal**: Armazena histórico completo com janelas configuráveis
//! - **Agregações**: Agrupa dados por janelas temporais automáticas
//! - **Percentis**: Calcula P50, P75, P90, P95, P99
//! - **Outliers**: Detecta anomalias usando IQR (Interquartile Range)
//! - **Correlações**: Analisa correlação entre métricas
//! - **Metadados**: Nome, unidade e descrição para cada métrica
//! - **Queries**: Busca por intervalo de tempo
//! - **Benchmark**: Compara com baselines
//! - **Alertas**: Sistema de alertas configuráveis
//! - **No STD Compatible**: Funciona com `alloc` em ambientes embedded
//!
//! ## Aplicações
//!
//! - Monitoramento de performance em produção
//! - Telemetria distribuída
//! - APM (Application Performance Monitoring)
//! - Debugging e profiling avançado
//! - Dashboards em tempo real
//! - Análise de SLOs/SLAs

extern crate alloc;
use alloc::collections::BTreeMap;
use alloc::vec::Vec;

/// Entrada de histórico com timestamp
#[derive(Clone, Copy, Debug)]
pub struct HistoryEntry {
    pub timestamp: u64,
    pub value: f64,
}

/// Alerta de limiar
#[derive(Clone, Copy, Debug)]
pub struct Alert {
    pub metric_id: u64,
    pub threshold: f64,
    pub is_max: bool,
}

/// Estatísticas calculadas
#[derive(Clone, Copy, Debug)]
pub struct Statistics {
    pub min: f64,
    pub max: f64,
    pub mean: f64,
    pub variance: f64,
    pub std_dev: f64,
}

/// Percentis calculados
#[derive(Clone, Copy, Debug)]
pub struct Percentiles {
    pub p50: f64,  // Mediana
    pub p75: f64,
    pub p90: f64,
    pub p95: f64,
    pub p99: f64,
}

/// Agregação temporal
#[derive(Clone, Copy, Debug)]
pub struct TimeWindow {
    pub start: u64,
    pub end: u64,
    pub count: usize,
    pub sum: f64,
    pub min: f64,
    pub max: f64,
}

/// Metadados de métrica
#[derive(Clone, Debug)]
pub struct MetricMetadata {
    pub name: &'static str,
    pub unit: &'static str,
    pub description: &'static str,
}

/// Monitor de sistema v5.0
///
/// Coleta e armazena métricas do sistema identificadas por IDs numéricos.
/// Cada métrica é associada a um valor float de 64 bits.
///
/// Versão 5.0 inclui:
/// - Agregações temporais por janelas
/// - Cálculo de percentis (P50, P75, P90, P95, P99)
/// - Detecção de outliers
/// - Metadados de métricas
/// - Queries por intervalo de tempo
/// - Benchmark e comparações
pub struct Monitor {
    pub metrics: BTreeMap<u64, f64>,
    history: BTreeMap<u64, Vec<HistoryEntry>>,
    alerts: Vec<Alert>,
    history_max_size: usize,
    metadata: BTreeMap<u64, MetricMetadata>,
    aggregations: BTreeMap<u64, Vec<TimeWindow>>,
    enable_aggregation: bool,
    aggregation_window_ms: u64,
}

impl Monitor {
    /// Cria novo monitor v5.0
    ///
    /// Inicializa um monitor vazio pronto para coletar métricas.
    pub fn new() -> Self {
        Self {
            metrics: BTreeMap::new(),
            history: BTreeMap::new(),
            alerts: Vec::new(),
            history_max_size: 100,
            metadata: BTreeMap::new(),
            aggregations: BTreeMap::new(),
            enable_aggregation: false,
            aggregation_window_ms: 60000, // 1 minuto
        }
    }

    /// Cria monitor com tamanho de histórico customizado
    pub fn with_history_size(history_max_size: usize) -> Self {
        Self {
            metrics: BTreeMap::new(),
            history: BTreeMap::new(),
            alerts: Vec::new(),
            history_max_size,
            metadata: BTreeMap::new(),
            aggregations: BTreeMap::new(),
            enable_aggregation: false,
            aggregation_window_ms: 60000,
        }
    }

    /// Cria monitor com agregação temporal habilitada
    pub fn with_aggregation(window_ms: u64) -> Self {
        Self {
            metrics: BTreeMap::new(),
            history: BTreeMap::new(),
            alerts: Vec::new(),
            history_max_size: 1000,
            metadata: BTreeMap::new(),
            aggregations: BTreeMap::new(),
            enable_aggregation: true,
            aggregation_window_ms: window_ms,
        }
    }

    /// Registra uma métrica
    ///
    /// # Argumentos
    ///
    /// * `metric_id` - Identificador único da métrica
    /// * `value` - Valor da métrica a ser registrado
    ///
    /// # Exemplo
    ///
    /// ```rust
    /// # use avila_monitor::Monitor;
    /// let mut monitor = Monitor::new();
    /// monitor.record(100, 42.5); // Registra métrica ID 100 com valor 42.5
    /// ```
    pub fn record(&mut self, metric_id: u64, value: f64) {
        self.metrics.insert(metric_id, value);
    }

    /// Obtém o valor de uma métrica
    ///
    /// # Argumentos
    ///
    /// * `metric_id` - Identificador da métrica
    ///
    /// # Retorna
    ///
    /// `Some(value)` se a métrica existe, `None` caso contrário
    pub fn get(&self, metric_id: u64) -> Option<f64> {
        self.metrics.get(&metric_id).copied()
    }

    /// Remove uma métrica
    ///
    /// # Argumentos
    ///
    /// * `metric_id` - Identificador da métrica a remover
    ///
    /// # Retorna
    ///
    /// O valor anterior da métrica, se existia
    pub fn remove(&mut self, metric_id: u64) -> Option<f64> {
        self.metrics.remove(&metric_id)
    }

    /// Limpa todas as métricas
    pub fn clear(&mut self) {
        self.metrics.clear();
    }

    /// Retorna o número de métricas registradas
    pub fn count(&self) -> usize {
        self.metrics.len()
    }

    /// Verifica se uma métrica existe
    pub fn contains(&self, metric_id: u64) -> bool {
        self.metrics.contains_key(&metric_id)
    }

    /// Retorna todas as métricas como slice de tuplas
    pub fn all_metrics(&self) -> alloc::vec::Vec<(u64, f64)> {
        self.metrics.iter().map(|(&k, &v)| (k, v)).collect()
    }

    /// Calcula a média de todas as métricas
    pub fn average(&self) -> Option<f64> {
        if self.metrics.is_empty() {
            return None;
        }
        let sum: f64 = self.metrics.values().sum();
        Some(sum / self.metrics.len() as f64)
    }

    /// Retorna a métrica com maior valor
    pub fn max_metric(&self) -> Option<(u64, f64)> {
        self.metrics.iter()
            .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap_or(core::cmp::Ordering::Equal))
            .map(|(&id, &val)| (id, val))
    }

    /// Retorna a métrica com menor valor
    pub fn min_metric(&self) -> Option<(u64, f64)> {
        self.metrics.iter()
            .min_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap_or(core::cmp::Ordering::Equal))
            .map(|(&id, &val)| (id, val))
    }

    /// Incrementa uma métrica por um delta
    pub fn increment(&mut self, metric_id: u64, delta: f64) {
        self.metrics.entry(metric_id)
            .and_modify(|v| *v += delta)
            .or_insert(delta);
    }

    /// Decrementa uma métrica por um delta
    pub fn decrement(&mut self, metric_id: u64, delta: f64) {
        self.increment(metric_id, -delta);
    }

    /// Registra métrica com timestamp
    pub fn record_with_timestamp(&mut self, metric_id: u64, value: f64, timestamp: u64) {
        self.metrics.insert(metric_id, value);

        let history = self.history.entry(metric_id).or_insert_with(Vec::new);
        history.push(HistoryEntry { timestamp, value });

        let should_aggregate = self.enable_aggregation && history.len() % 10 == 0;

        if history.len() > self.history_max_size {
            history.remove(0);
        }

        self.check_alerts(metric_id, value);

        // Auto-agregação se habilitada
        if should_aggregate {
            self.aggregate_windows(metric_id);
        }
    }    fn check_alerts(&self, metric_id: u64, value: f64) {
        for alert in &self.alerts {
            if alert.metric_id == metric_id {
                if alert.is_max && value > alert.threshold {
                    // Alerta disparado
                } else if !alert.is_max && value < alert.threshold {
                    // Alerta disparado
                }
            }
        }
    }

    /// Adiciona alerta de máximo
    pub fn add_max_alert(&mut self, metric_id: u64, threshold: f64) {
        self.alerts.push(Alert {
            metric_id,
            threshold,
            is_max: true,
        });
    }

    /// Adiciona alerta de mínimo
    pub fn add_min_alert(&mut self, metric_id: u64, threshold: f64) {
        self.alerts.push(Alert {
            metric_id,
            threshold,
            is_max: false,
        });
    }

    /// Obtém histórico de uma métrica
    pub fn get_history(&self, metric_id: u64) -> Option<&Vec<HistoryEntry>> {
        self.history.get(&metric_id)
    }

    /// Calcula estatísticas de uma métrica
    pub fn calculate_statistics(&self, metric_id: u64) -> Option<Statistics> {
        let history = self.history.get(&metric_id)?;
        if history.is_empty() {
            return None;
        }

        let values: Vec<f64> = history.iter().map(|e| e.value).collect();
        let n = values.len() as f64;

        let min = values.iter().fold(f64::INFINITY, |a, &b| a.min(b));
        let max = values.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b));
        let mean = values.iter().sum::<f64>() / n;

        let variance = values.iter()
            .map(|&v| (v - mean).powi(2))
            .sum::<f64>() / n;
        let std_dev = variance.sqrt();

        Some(Statistics {
            min,
            max,
            mean,
            variance,
            std_dev,
        })
    }

    /// Calcula taxa de mudança (derivative)
    pub fn calculate_rate(&self, metric_id: u64) -> Option<f64> {
        let history = self.history.get(&metric_id)?;
        if history.len() < 2 {
            return None;
        }

        let last = history.last()?;
        let prev = &history[history.len() - 2];

        let dt = (last.timestamp - prev.timestamp) as f64;
        if dt == 0.0 {
            return None;
        }

        Some((last.value - prev.value) / dt)
    }

    /// Calcula média móvel
    pub fn moving_average(&self, metric_id: u64, window: usize) -> Option<f64> {
        let history = self.history.get(&metric_id)?;
        if history.is_empty() {
            return None;
        }

        let start = if history.len() > window {
            history.len() - window
        } else {
            0
        };

        let sum: f64 = history[start..].iter().map(|e| e.value).sum();
        Some(sum / (history.len() - start) as f64)
    }

    /// Adiciona metadados a uma métrica
    pub fn set_metadata(&mut self, metric_id: u64, name: &'static str, unit: &'static str, description: &'static str) {
        self.metadata.insert(metric_id, MetricMetadata {
            name,
            unit,
            description,
        });
    }

    /// Obtém metadados de uma métrica
    pub fn get_metadata(&self, metric_id: u64) -> Option<&MetricMetadata> {
        self.metadata.get(&metric_id)
    }

    /// Calcula percentis do histórico
    pub fn calculate_percentiles(&self, metric_id: u64) -> Option<Percentiles> {
        let history = self.history.get(&metric_id)?;
        if history.is_empty() {
            return None;
        }

        let mut values: Vec<f64> = history.iter().map(|e| e.value).collect();
        values.sort_by(|a, b| a.partial_cmp(b).unwrap());

        let len = values.len();
        let p50 = values[len * 50 / 100];
        let p75 = values[len * 75 / 100];
        let p90 = values[len * 90 / 100];
        let p95 = values[len * 95 / 100];
        let p99 = values[len * 99 / 100];

        Some(Percentiles { p50, p75, p90, p95, p99 })
    }

    /// Detecta outliers usando IQR (Interquartile Range)
    pub fn detect_outliers(&self, metric_id: u64) -> Option<Vec<(u64, f64)>> {
        let history = self.history.get(&metric_id)?;
        if history.len() < 4 {
            return None;
        }

        let mut values: Vec<f64> = history.iter().map(|e| e.value).collect();
        values.sort_by(|a, b| a.partial_cmp(b).unwrap());

        let len = values.len();
        let q1 = values[len / 4];
        let q3 = values[len * 3 / 4];
        let iqr = q3 - q1;
        let lower = q1 - 1.5 * iqr;
        let upper = q3 + 1.5 * iqr;

        let outliers: Vec<(u64, f64)> = history.iter()
            .filter(|e| e.value < lower || e.value > upper)
            .map(|e| (e.timestamp, e.value))
            .collect();

        Some(outliers)
    }

    /// Query por intervalo de tempo
    pub fn query_range(&self, metric_id: u64, start: u64, end: u64) -> Option<Vec<HistoryEntry>> {
        let history = self.history.get(&metric_id)?;
        let entries: Vec<HistoryEntry> = history.iter()
            .filter(|e| e.timestamp >= start && e.timestamp <= end)
            .copied()
            .collect();

        if entries.is_empty() {
            None
        } else {
            Some(entries)
        }
    }

    /// Agrega dados em janelas temporais
    pub fn aggregate_windows(&mut self, metric_id: u64) {
        let history = match self.history.get(&metric_id) {
            Some(h) if !h.is_empty() => h,
            _ => return,
        };

        let window_size = self.aggregation_window_ms;
        let mut windows = Vec::new();
        let mut current_window: Option<TimeWindow> = None;

        for entry in history.iter() {
            match &mut current_window {
                None => {
                    current_window = Some(TimeWindow {
                        start: entry.timestamp,
                        end: entry.timestamp + window_size,
                        count: 1,
                        sum: entry.value,
                        min: entry.value,
                        max: entry.value,
                    });
                }
                Some(window) => {
                    if entry.timestamp < window.end {
                        window.count += 1;
                        window.sum += entry.value;
                        window.min = window.min.min(entry.value);
                        window.max = window.max.max(entry.value);
                    } else {
                        windows.push(*window);
                        current_window = Some(TimeWindow {
                            start: entry.timestamp,
                            end: entry.timestamp + window_size,
                            count: 1,
                            sum: entry.value,
                            min: entry.value,
                            max: entry.value,
                        });
                    }
                }
            }
        }

        if let Some(window) = current_window {
            windows.push(window);
        }

        self.aggregations.insert(metric_id, windows);
    }

    /// Obtém agregações de uma métrica
    pub fn get_aggregations(&self, metric_id: u64) -> Option<&Vec<TimeWindow>> {
        self.aggregations.get(&metric_id)
    }

    /// Calcula correlação entre duas métricas
    pub fn correlation(&self, metric_id1: u64, metric_id2: u64) -> Option<f64> {
        let hist1 = self.history.get(&metric_id1)?;
        let hist2 = self.history.get(&metric_id2)?;

        if hist1.len() != hist2.len() || hist1.is_empty() {
            return None;
        }

        let n = hist1.len() as f64;
        let sum1: f64 = hist1.iter().map(|e| e.value).sum();
        let sum2: f64 = hist2.iter().map(|e| e.value).sum();
        let mean1 = sum1 / n;
        let mean2 = sum2 / n;

        let mut numerator = 0.0;
        let mut denom1 = 0.0;
        let mut denom2 = 0.0;

        for i in 0..hist1.len() {
            let diff1 = hist1[i].value - mean1;
            let diff2 = hist2[i].value - mean2;
            numerator += diff1 * diff2;
            denom1 += diff1 * diff1;
            denom2 += diff2 * diff2;
        }

        let denominator = (denom1 * denom2).sqrt();
        if denominator == 0.0 {
            return None;
        }

        Some(numerator / denominator)
    }

    /// Benchmark: compara métrica com baseline
    pub fn benchmark(&self, metric_id: u64, baseline: f64) -> Option<f64> {
        let current = self.get(metric_id)?;
        Some((current - baseline) / baseline * 100.0)
    }

    /// Exporta métricas em formato CSV-like
    pub fn export_csv(&self, metric_id: u64) -> Option<Vec<(u64, f64)>> {
        let history = self.history.get(&metric_id)?;
        Some(history.iter().map(|e| (e.timestamp, e.value)).collect())
    }

    /// Limpa histórico antigo (antes de timestamp)
    pub fn cleanup_history(&mut self, before_timestamp: u64) {
        for history in self.history.values_mut() {
            history.retain(|e| e.timestamp >= before_timestamp);
        }
    }

    /// Calcula taxa de crescimento (growth rate)
    pub fn growth_rate(&self, metric_id: u64) -> Option<f64> {
        let history = self.history.get(&metric_id)?;
        if history.len() < 2 {
            return None;
        }

        let first = history.first()?.value;
        let last = history.last()?.value;

        if first == 0.0 {
            return None;
        }

        Some((last - first) / first * 100.0)
    }

    /// Obtém resumo de todas as métricas
    pub fn summary(&self) -> Vec<(u64, f64, Option<&MetricMetadata>)> {
        self.metrics.iter()
            .map(|(&id, &val)| (id, val, self.metadata.get(&id)))
            .collect()
    }

    /// Reseta uma métrica específica
    pub fn reset_metric(&mut self, metric_id: u64) {
        self.metrics.remove(&metric_id);
        self.history.remove(&metric_id);
        self.aggregations.remove(&metric_id);
    }
}impl Default for Monitor {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_monitor() {
        let mut mon = Monitor::new();
        mon.record(1, 99.5);
        assert_eq!(mon.get(1), Some(99.5));
    }

    #[test]
    fn test_remove() {
        let mut mon = Monitor::new();
        mon.record(1, 50.0);
        assert_eq!(mon.remove(1), Some(50.0));
        assert_eq!(mon.get(1), None);
    }

    #[test]
    fn test_count_and_clear() {
        let mut mon = Monitor::new();
        mon.record(1, 10.0);
        mon.record(2, 20.0);
        assert_eq!(mon.count(), 2);
        mon.clear();
        assert_eq!(mon.count(), 0);
    }

    #[test]
    fn test_average() {
        let mut mon = Monitor::new();
        mon.record(1, 10.0);
        mon.record(2, 20.0);
        mon.record(3, 30.0);
        assert_eq!(mon.average(), Some(20.0));
    }

    #[test]
    fn test_max_min() {
        let mut mon = Monitor::new();
        mon.record(1, 10.0);
        mon.record(2, 50.0);
        mon.record(3, 5.0);
        assert_eq!(mon.max_metric(), Some((2, 50.0)));
        assert_eq!(mon.min_metric(), Some((3, 5.0)));
    }

    #[test]
    fn test_increment_decrement() {
        let mut mon = Monitor::new();
        mon.record(1, 100.0);
        mon.increment(1, 50.0);
        assert_eq!(mon.get(1), Some(150.0));
        mon.decrement(1, 30.0);
        assert_eq!(mon.get(1), Some(120.0));
    }

    #[test]
    fn test_metadata() {
        let mut mon = Monitor::new();
        mon.set_metadata(1, "cpu_usage", "%", "CPU utilization");
        let meta = mon.get_metadata(1).unwrap();
        assert_eq!(meta.name, "cpu_usage");
        assert_eq!(meta.unit, "%");
    }

    #[test]
    fn test_percentiles() {
        let mut mon = Monitor::new();
        for i in 1..=100 {
            mon.record_with_timestamp(1, i as f64, i);
        }
        let percentiles = mon.calculate_percentiles(1).unwrap();
        assert!(percentiles.p50 >= 40.0 && percentiles.p50 <= 60.0);
        assert!(percentiles.p99 >= 95.0);
    }

    #[test]
    fn test_outliers() {
        let mut mon = Monitor::new();
        // Valores normais
        for i in 0..10 {
            mon.record_with_timestamp(1, 50.0, i);
        }
        // Outlier
        mon.record_with_timestamp(1, 500.0, 10);

        let outliers = mon.detect_outliers(1).unwrap();
        assert!(!outliers.is_empty());
    }

    #[test]
    fn test_query_range() {
        let mut mon = Monitor::new();
        mon.record_with_timestamp(1, 10.0, 1000);
        mon.record_with_timestamp(1, 20.0, 2000);
        mon.record_with_timestamp(1, 30.0, 3000);

        let results = mon.query_range(1, 1500, 2500).unwrap();
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].value, 20.0);
    }

    #[test]
    fn test_aggregation() {
        let mut mon = Monitor::with_aggregation(1000);
        mon.record_with_timestamp(1, 10.0, 500);
        mon.record_with_timestamp(1, 20.0, 800);
        mon.record_with_timestamp(1, 30.0, 1500);

        mon.aggregate_windows(1);
        let aggs = mon.get_aggregations(1).unwrap();
        assert!(aggs.len() >= 1);
    }

    #[test]
    fn test_correlation() {
        let mut mon = Monitor::new();
        for i in 0..10 {
            let val = i as f64 * 10.0;
            mon.record_with_timestamp(1, val, i);
            mon.record_with_timestamp(2, val * 2.0, i); // Correlação perfeita
        }

        let corr = mon.correlation(1, 2).unwrap();
        assert!(corr > 0.99); // Correlação quase perfeita
    }

    #[test]
    fn test_benchmark() {
        let mut mon = Monitor::new();
        mon.record(1, 110.0);
        let diff = mon.benchmark(1, 100.0).unwrap();
        assert_eq!(diff, 10.0); // 10% acima do baseline
    }

    #[test]
    fn test_growth_rate() {
        let mut mon = Monitor::new();
        mon.record_with_timestamp(1, 100.0, 1000);
        mon.record_with_timestamp(1, 150.0, 2000);

        let growth = mon.growth_rate(1).unwrap();
        assert_eq!(growth, 50.0); // 50% de crescimento
    }

    #[test]
    fn test_export_csv() {
        let mut mon = Monitor::new();
        mon.record_with_timestamp(1, 10.0, 1000);
        mon.record_with_timestamp(1, 20.0, 2000);

        let csv = mon.export_csv(1).unwrap();
        assert_eq!(csv.len(), 2);
        assert_eq!(csv[0], (1000, 10.0));
    }

    #[test]
    fn test_cleanup_history() {
        let mut mon = Monitor::new();
        mon.record_with_timestamp(1, 10.0, 1000);
        mon.record_with_timestamp(1, 20.0, 2000);
        mon.record_with_timestamp(1, 30.0, 3000);

        mon.cleanup_history(2500);
        let history = mon.get_history(1).unwrap();
        assert_eq!(history.len(), 1); // Apenas última entrada
    }

    #[test]
    fn test_summary() {
        let mut mon = Monitor::new();
        mon.set_metadata(1, "cpu", "%", "CPU");
        mon.record(1, 50.0);
        mon.record(2, 100.0);

        let summary = mon.summary();
        assert_eq!(summary.len(), 2);
    }

    #[test]
    fn test_reset_metric() {
        let mut mon = Monitor::new();
        mon.record_with_timestamp(1, 10.0, 1000);
        assert!(mon.get(1).is_some());

        mon.reset_metric(1);
        assert!(mon.get(1).is_none());
    }
}
