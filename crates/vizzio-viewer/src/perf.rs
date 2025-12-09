//! Sistema de métricas de performance para Vizzio

use std::sync::atomic::{AtomicU64, Ordering};
use std::time::Instant;

/// Counter simples thread-safe
pub struct Counter {
    value: AtomicU64,
}

impl Counter {
    pub const fn new() -> Self {
        Self {
            value: AtomicU64::new(0),
        }
    }

    pub fn inc(&self) {
        self.value.fetch_add(1, Ordering::Relaxed);
    }

    pub fn add(&self, n: u64) {
        self.value.fetch_add(n, Ordering::Relaxed);
    }

    pub fn value(&self) -> u64 {
        self.value.load(Ordering::Relaxed)
    }
}

/// Histogram simples
pub struct Histogram {
    sum: AtomicU64,
    count: AtomicU64,
}

impl Histogram {
    pub fn new() -> Self {
        Self {
            sum: AtomicU64::new(0),
            count: AtomicU64::new(0),
        }
    }

    pub fn observe(&self, value: f64) {
        let value_u64 = (value * 1000.0) as u64; // ms -> us
        self.sum.fetch_add(value_u64, Ordering::Relaxed);
        self.count.fetch_add(1, Ordering::Relaxed);
    }

    pub fn mean(&self) -> f64 {
        let sum = self.sum.load(Ordering::Relaxed);
        let count = self.count.load(Ordering::Relaxed);
        if count > 0 {
            (sum as f64 / count as f64) / 1000.0 // us -> ms
        } else {
            0.0
        }
    }

    pub fn percentile(&self, _p: f64) -> f64 {
        // Simplificado - retorna média
        self.mean()
    }
}

/// Métricas globais do viewer
pub struct ViewerMetrics {
    /// Frames renderizados
    pub frames_rendered: Counter,
    /// Tempo de render por frame (ms)
    pub render_time: Histogram,
    /// Entidades IFC carregadas
    pub entities_loaded: Counter,
    /// Geometrias extraídas
    pub geometries_extracted: Counter,
    /// Cache hits
    pub cache_hits: Counter,
    /// Cache misses
    pub cache_misses: Counter,
}

impl ViewerMetrics {
    /// Cria novas métricas
    pub fn new() -> Self {
        Self {
            frames_rendered: Counter::new(),
            render_time: Histogram::new(),
            entities_loaded: Counter::new(),
            geometries_extracted: Counter::new(),
            cache_hits: Counter::new(),
            cache_misses: Counter::new(),
        }
    }

    /// Registra frame renderizado
    pub fn record_frame(&self, duration_ms: f64) {
        self.frames_rendered.inc();
        self.render_time.observe(duration_ms);
    }

    /// Registra carregamento IFC
    pub fn record_ifc_load(&self, entity_count: u64, geometry_count: u64) {
        self.entities_loaded.add(entity_count);
        self.geometries_extracted.add(geometry_count);
    }

    /// Imprime estatísticas
    pub fn print_stats(&self) {
        println!("\n📊 Estatísticas de Performance:");
        println!("  Frames renderizados: {}", self.frames_rendered.value());
        println!("  Render médio: {:.2}ms", self.render_time.mean());
        println!("  Render p95: {:.2}ms", self.render_time.percentile(0.95));
        println!("  Entidades IFC: {}", self.entities_loaded.value());
        println!("  Geometrias: {}", self.geometries_extracted.value());
        println!("  Cache hit rate: {:.1}%", self.cache_hit_rate());
    }

    /// Taxa de acerto do cache
    fn cache_hit_rate(&self) -> f64 {
        let hits = self.cache_hits.value() as f64;
        let total = hits + self.cache_misses.value() as f64;
        if total > 0.0 {
            (hits / total) * 100.0
        } else {
            0.0
        }
    }
}

/// Timer para medir duração de operações
pub struct PerfTimer {
    start: Instant,
    name: String,
}

impl PerfTimer {
    /// Inicia timer
    pub fn start(name: &str) -> Self {
        Self {
            start: Instant::now(),
            name: name.to_string(),
        }
    }

    /// Para timer e retorna duração em ms
    pub fn stop(self) -> f64 {
        let duration = self.start.elapsed();
        let ms = duration.as_secs_f64() * 1000.0;
        println!("⏱️  {} levou {:.2}ms", self.name, ms);
        ms
    }
}
