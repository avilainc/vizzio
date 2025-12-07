//! Demonstração do avila-monitor v5.0
//!
//! Este exemplo mostra todas as funcionalidades da versão 5.0:
//! - Percentis, outliers, correlações
//! - Agregações temporais
//! - Metadados e alertas
//! - Queries e benchmarks

#![allow(dead_code)]

extern crate alloc;

// Simulação para exemplo standalone
#[path = "../src/lib.rs"]
mod monitor_lib;

use monitor_lib::*;

fn main() {
    println!("=== avila-monitor v5.0 Demo ===\n");

    // 1. Criar monitor com agregação automática
    let mut mon = Monitor::with_aggregation(5000); // janelas de 5s

    // 2. Configurar metadados
    mon.set_metadata(1, "response_time", "ms", "API response time");
    mon.set_metadata(2, "cpu_usage", "%", "CPU utilization");
    mon.set_metadata(3, "memory_mb", "MB", "Memory usage");

    // 3. Configurar alertas
    mon.add_max_alert(1, 1000.0); // Response time > 1s
    mon.add_max_alert(2, 90.0);   // CPU > 90%

    println!("📊 Coletando métricas...\n");

    // 4. Simular coleta de métricas ao longo do tempo
    for i in 0..100 {
        let timestamp = i * 100;

        // Response time: normalmente 100-200ms, com alguns picos
        let response_time = if i % 20 == 0 {
            500.0 + (i as f64 * 10.0) // Picos ocasionais
        } else {
            100.0 + (i as f64 % 10 as f64) * 10.0
        };

        // CPU: oscila entre 50-80%
        let cpu = 50.0 + (i as f64 * 0.3) % 30.0;

        // Memory: cresce gradualmente
        let memory = 1000.0 + i as f64 * 10.0;

        mon.record_with_timestamp(1, response_time, timestamp);
        mon.record_with_timestamp(2, cpu, timestamp);
        mon.record_with_timestamp(3, memory, timestamp);
    }

    // 5. Estatísticas básicas
    println!("📈 Estatísticas Básicas:");
    println!("   Total de métricas: {}", mon.count());
    println!("   Média geral: {:.2}", mon.average().unwrap_or(0.0));

    if let Some((id, val)) = mon.max_metric() {
        if let Some(meta) = mon.get_metadata(id) {
            println!("   Métrica máxima: {} = {:.2} {}", meta.name, val, meta.unit);
        }
    }

    // 6. Estatísticas avançadas por métrica
    println!("\n📊 Response Time (métrica 1):");
    if let Some(stats) = mon.calculate_statistics(1) {
        println!("   Min: {:.2} ms", stats.min);
        println!("   Max: {:.2} ms", stats.max);
        println!("   Mean: {:.2} ms", stats.mean);
        println!("   Std Dev: {:.2} ms", stats.std_dev);
    }

    // 7. Percentis
    if let Some(perc) = mon.calculate_percentiles(1) {
        println!("\n📈 Percentis do Response Time:");
        println!("   P50 (mediana): {:.2} ms", perc.p50);
        println!("   P75: {:.2} ms", perc.p75);
        println!("   P90: {:.2} ms", perc.p90);
        println!("   P95: {:.2} ms", perc.p95);
        println!("   P99: {:.2} ms", perc.p99);
    }

    // 8. Detecção de outliers
    println!("\n🔍 Detecção de Outliers:");
    if let Some(outliers) = mon.detect_outliers(1) {
        println!("   {} outliers detectados", outliers.len());
        for (ts, val) in outliers.iter().take(3) {
            println!("   - Timestamp {}: {:.2} ms (anômalo)", ts, val);
        }
    }

    // 9. Correlação entre métricas
    println!("\n🔗 Análise de Correlação:");
    if let Some(corr) = mon.correlation(2, 3) {
        println!("   Correlação CPU vs Memory: {:.3}", corr);
        if corr > 0.7 {
            println!("   ✓ Forte correlação positiva");
        } else if corr < -0.7 {
            println!("   ✓ Forte correlação negativa");
        } else {
            println!("   ○ Correlação fraca");
        }
    }

    // 10. Query por range temporal
    println!("\n⏱️  Query por Intervalo (timestamps 2000-5000):");
    if let Some(entries) = mon.query_range(1, 2000, 5000) {
        println!("   {} entradas encontradas", entries.len());
        let avg: f64 = entries.iter().map(|e| e.value).sum::<f64>() / entries.len() as f64;
        println!("   Média do período: {:.2} ms", avg);
    }

    // 11. Agregações temporais
    println!("\n📦 Agregações Temporais (janelas de 5s):");
    mon.aggregate_windows(1);
    if let Some(windows) = mon.get_aggregations(1) {
        println!("   {} janelas temporais criadas", windows.len());
        for (i, win) in windows.iter().take(3).enumerate() {
            println!("   Janela {}: {} medições, avg={:.2} ms",
                i + 1, win.count, win.sum / win.count as f64);
        }
    }

    // 12. Taxa de crescimento
    println!("\n📈 Crescimento:");
    if let Some(growth) = mon.growth_rate(3) {
        println!("   Memory growth rate: {:.2}%", growth);
    }

    // 13. Benchmark contra baseline
    println!("\n🎯 Benchmark vs Baseline:");
    if let Some(diff) = mon.benchmark(1, 150.0) {
        println!("   Response time vs baseline (150ms): {:+.2}%", diff);
    }

    // 14. Média móvel
    println!("\n📉 Média Móvel (últimas 10 medições):");
    if let Some(ma) = mon.moving_average(1, 10) {
        println!("   MA(10) response time: {:.2} ms", ma);
    }

    // 15. Taxa de mudança
    println!("\n⚡ Taxa de Mudança:");
    if let Some(rate) = mon.calculate_rate(2) {
        println!("   CPU rate of change: {:.2} %/timestamp", rate);
    }

    // 16. Resumo geral
    println!("\n📋 Resumo de Todas as Métricas:");
    let summary = mon.summary();
    for (id, value, meta) in summary {
        if let Some(m) = meta {
            println!("   [{}] {} = {:.2} {} - {}",
                id, m.name, value, m.unit, m.description);
        }
    }

    // 17. Exportar dados
    println!("\n💾 Exportação CSV:");
    if let Some(csv) = mon.export_csv(1) {
        println!("   {} registros prontos para export", csv.len());
        println!("   Primeiros 3 registros:");
        for (ts, val) in csv.iter().take(3) {
            println!("   {},{:.2}", ts, val);
        }
    }

    // 18. Limpeza de histórico antigo
    println!("\n🧹 Limpeza de Histórico:");
    let history_before = mon.get_history(1).map(|h| h.len()).unwrap_or(0);
    mon.cleanup_history(5000);
    let history_after = mon.get_history(1).map(|h| h.len()).unwrap_or(0);
    println!("   Antes: {} entradas", history_before);
    println!("   Depois: {} entradas (removidas {} antigas)",
        history_after, history_before - history_after);

    println!("\n✅ Demo completo da v5.0!");
    println!("\n🎉 avila-monitor v5.0 - Production Ready!");
}
