//! Dashboard Metrics - Implementação pura sem dependências externas
//!
//! Cálculo de métricas agregadas para dashboards em tempo real

use std::collections::HashMap;
use chrono::{DateTime, Utc, Duration};
use crate::models::{BehaviorEvent, EventType};

/// Métricas gerais do dashboard
#[derive(Debug, Clone)]
pub struct DashboardMetrics {
    pub total_users: usize,
    pub total_sessions: usize,
    pub total_events: usize,
    pub total_pageviews: usize,
    pub total_purchases: usize,
    pub total_revenue: f64,
    pub avg_session_duration_seconds: f64,
    pub avg_events_per_session: f64,
    pub avg_events_per_user: f64,
    pub conversion_rate: f64,
    pub bounce_rate: f64,
}

/// Métrica de série temporal
#[derive(Debug, Clone)]
pub struct TimeSeriesPoint {
    pub timestamp: DateTime<Utc>,
    pub value: f64,
}

/// Agregação de métricas ao longo do tempo
#[derive(Debug, Clone)]
pub struct TimeSeriesMetrics {
    pub metric_name: String,
    pub points: Vec<TimeSeriesPoint>,
}

impl TimeSeriesMetrics {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            metric_name: name.into(),
            points: Vec::new(),
        }
    }

    pub fn add_point(&mut self, timestamp: DateTime<Utc>, value: f64) {
        self.points.push(TimeSeriesPoint { timestamp, value });
    }

    /// Calcula média móvel
    pub fn moving_average(&self, window_size: usize) -> Vec<TimeSeriesPoint> {
        if window_size == 0 || self.points.is_empty() {
            return Vec::new();
        }

        let mut result = Vec::new();

        for i in 0..self.points.len() {
            let start = if i >= window_size { i - window_size + 1 } else { 0 };
            let window = &self.points[start..=i];

            let sum: f64 = window.iter().map(|p| p.value).sum();
            let avg = sum / window.len() as f64;

            result.push(TimeSeriesPoint {
                timestamp: self.points[i].timestamp,
                value: avg,
            });
        }

        result
    }
}

/// Sistema de métricas de dashboard
pub struct Dashboard {
    events: Vec<BehaviorEvent>,
}

impl Dashboard {
    pub fn new() -> Self {
        Self {
            events: Vec::new(),
        }
    }

    pub fn from_events(events: Vec<BehaviorEvent>) -> Self {
        Self { events }
    }

    /// Adiciona novos eventos
    pub fn add_events(&mut self, events: Vec<BehaviorEvent>) {
        self.events.extend(events);
    }

    /// Calcula métricas gerais
    pub fn calculate_metrics(&self) -> DashboardMetrics {
        let unique_users = self.count_unique_users();
        let unique_sessions = self.count_unique_sessions();
        let total_events = self.events.len();

        let pageviews = self.count_by_type("page_view");
        let purchases = self.count_by_type("purchase");
        let revenue = self.calculate_total_revenue();

        let session_durations = self.calculate_session_durations();
        let avg_session_duration = if !session_durations.is_empty() {
            session_durations.iter().sum::<f64>() / session_durations.len() as f64
        } else {
            0.0
        };

        let avg_events_per_session = if unique_sessions > 0 {
            total_events as f64 / unique_sessions as f64
        } else {
            0.0
        };

        let avg_events_per_user = if unique_users > 0 {
            total_events as f64 / unique_users as f64
        } else {
            0.0
        };

        let conversion_rate = if unique_sessions > 0 {
            (purchases as f64 / unique_sessions as f64) * 100.0
        } else {
            0.0
        };

        let bounce_rate = self.calculate_bounce_rate();

        DashboardMetrics {
            total_users: unique_users,
            total_sessions: unique_sessions,
            total_events,
            total_pageviews: pageviews,
            total_purchases: purchases,
            total_revenue: revenue,
            avg_session_duration_seconds: avg_session_duration,
            avg_events_per_session,
            avg_events_per_user,
            conversion_rate,
            bounce_rate,
        }
    }

    /// Calcula métricas para um período específico
    pub fn calculate_metrics_for_period(
        &self,
        start: &DateTime<Utc>,
        end: &DateTime<Utc>,
    ) -> DashboardMetrics {
        let filtered_events: Vec<_> = self.events
            .iter()
            .filter(|e| e.timestamp >= *start && e.timestamp <= *end)
            .cloned()
            .collect();

        Dashboard::from_events(filtered_events).calculate_metrics()
    }

    /// Gera série temporal de eventos por hora
    pub fn events_per_hour(&self, hours: usize) -> TimeSeriesMetrics {
        let mut metrics = TimeSeriesMetrics::new("Events per Hour");
        let now = Utc::now();

        for hour in (0..hours).rev() {
            let period_end = now - Duration::hours(hour as i64);
            let period_start = period_end - Duration::hours(1);

            let count = self.events
                .iter()
                .filter(|e| e.timestamp >= period_start && e.timestamp < period_end)
                .count();

            metrics.add_point(period_end, count as f64);
        }

        metrics
    }

    /// Gera série temporal de usuários ativos por dia
    pub fn active_users_per_day(&self, days: usize) -> TimeSeriesMetrics {
        let mut metrics = TimeSeriesMetrics::new("Active Users per Day");
        let now = Utc::now();

        for day in (0..days).rev() {
            let period_end = now - Duration::days(day as i64);
            let period_start = period_end - Duration::days(1);

            let mut unique_users = std::collections::HashSet::new();
            for event in &self.events {
                if event.timestamp >= period_start && event.timestamp < period_end {
                    unique_users.insert(&event.user_id);
                }
            }

            metrics.add_point(period_end, unique_users.len() as f64);
        }

        metrics
    }

    /// Top eventos por volume
    pub fn top_events(&self, limit: usize) -> Vec<(String, usize)> {
        let mut counts: HashMap<String, usize> = HashMap::new();

        for event in &self.events {
            let event_name = self.get_event_name(event);
            *counts.entry(event_name).or_insert(0) += 1;
        }

        let mut sorted: Vec<_> = counts.into_iter().collect();
        sorted.sort_by(|a, b| b.1.cmp(&a.1));
        sorted.truncate(limit);
        sorted
    }

    /// Top páginas visitadas
    pub fn top_pages(&self, limit: usize) -> Vec<(String, usize)> {
        let mut counts: HashMap<String, usize> = HashMap::new();

        for event in &self.events {
            if let EventType::PageView { url, .. } = &event.event_type {
                *counts.entry(url.clone()).or_insert(0) += 1;
            }
        }

        let mut sorted: Vec<_> = counts.into_iter().collect();
        sorted.sort_by(|a, b| b.1.cmp(&a.1));
        sorted.truncate(limit);
        sorted
    }

    /// Taxa de conversão por dia
    pub fn conversion_rate_per_day(&self, days: usize) -> TimeSeriesMetrics {
        let mut metrics = TimeSeriesMetrics::new("Conversion Rate per Day");
        let now = Utc::now();

        for day in (0..days).rev() {
            let period_end = now - Duration::days(day as i64);
            let period_start = period_end - Duration::days(1);

            let period_events: Vec<_> = self.events
                .iter()
                .filter(|e| e.timestamp >= period_start && e.timestamp < period_end)
                .collect();

            let sessions: std::collections::HashSet<_> = period_events
                .iter()
                .map(|e| &e.session_id)
                .collect();

            let purchases = period_events
                .iter()
                .filter(|e| matches!(e.event_type, EventType::Purchase { .. }))
                .count();

            let conversion = if !sessions.is_empty() {
                (purchases as f64 / sessions.len() as f64) * 100.0
            } else {
                0.0
            };

            metrics.add_point(period_end, conversion);
        }

        metrics
    }

    // Métodos auxiliares privados

    fn count_unique_users(&self) -> usize {
        let mut users = std::collections::HashSet::new();
        for event in &self.events {
            users.insert(&event.user_id);
        }
        users.len()
    }

    fn count_unique_sessions(&self) -> usize {
        let mut sessions = std::collections::HashSet::new();
        for event in &self.events {
            sessions.insert(&event.session_id);
        }
        sessions.len()
    }

    fn count_by_type(&self, type_name: &str) -> usize {
        self.events
            .iter()
            .filter(|e| self.get_event_name(e) == type_name)
            .count()
    }

    fn calculate_total_revenue(&self) -> f64 {
        self.events
            .iter()
            .filter_map(|e| {
                if let EventType::Purchase { amount, .. } = e.event_type {
                    Some(amount)
                } else {
                    None
                }
            })
            .sum()
    }

    fn calculate_session_durations(&self) -> Vec<f64> {
        let mut session_times: HashMap<String, (DateTime<Utc>, DateTime<Utc>)> = HashMap::new();

        for event in &self.events {
            session_times
                .entry(event.session_id.clone())
                .and_modify(|(first, last)| {
                    if event.timestamp < *first {
                        *first = event.timestamp;
                    }
                    if event.timestamp > *last {
                        *last = event.timestamp;
                    }
                })
                .or_insert((event.timestamp, event.timestamp));
        }

        session_times
            .values()
            .map(|(first, last)| {
                last.signed_duration_since(*first).num_seconds() as f64
            })
            .collect()
    }

    fn calculate_bounce_rate(&self) -> f64 {
        let mut session_event_counts: HashMap<String, usize> = HashMap::new();

        for event in &self.events {
            *session_event_counts.entry(event.session_id.clone()).or_insert(0) += 1;
        }

        let bounced = session_event_counts.values().filter(|&&count| count == 1).count();
        let total = session_event_counts.len();

        if total > 0 {
            (bounced as f64 / total as f64) * 100.0
        } else {
            0.0
        }
    }

    fn get_event_name(&self, event: &BehaviorEvent) -> String {
        match &event.event_type {
            EventType::PageView { .. } => "page_view".to_string(),
            EventType::Click { .. } => "click".to_string(),
            EventType::Purchase { .. } => "purchase".to_string(),
            EventType::AddToCart { .. } => "add_to_cart".to_string(),
            EventType::FormSubmit { .. } => "form_submit".to_string(),
            EventType::Custom { name, .. } => name.clone(),
            _ => "other".to_string(),
        }
    }
}

impl Default for Dashboard {
    fn default() -> Self {
        Self::new()
    }
}

impl DashboardMetrics {
    pub fn print_report(&self) {
        println!("\n=== Dashboard Metrics ===");
        println!("\nOverview:");
        println!("  Total Users: {}", self.total_users);
        println!("  Total Sessions: {}", self.total_sessions);
        println!("  Total Events: {}", self.total_events);

        println!("\nEngagement:");
        println!("  Total Pageviews: {}", self.total_pageviews);
        println!("  Avg Events/Session: {:.2}", self.avg_events_per_session);
        println!("  Avg Events/User: {:.2}", self.avg_events_per_user);
        println!("  Avg Session Duration: {:.0}s", self.avg_session_duration_seconds);
        println!("  Bounce Rate: {:.2}%", self.bounce_rate);

        println!("\nRevenue:");
        println!("  Total Purchases: {}", self.total_purchases);
        println!("  Total Revenue: ${:.2}", self.total_revenue);
        println!("  Conversion Rate: {:.2}%", self.conversion_rate);

        if self.total_purchases > 0 {
            let avg_order = self.total_revenue / self.total_purchases as f64;
            println!("  Avg Order Value: ${:.2}", avg_order);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::*;
    use std::collections::HashMap;

    fn create_test_event(user_id: &str, session_id: &str, event_type: EventType) -> BehaviorEvent {
        BehaviorEvent {
            event_id: format!("evt_{}_{}", user_id, session_id),
            user_id: user_id.to_string(),
            session_id: session_id.to_string(),
            timestamp: Utc::now(),
            event_type,
            metadata: HashMap::new(),
            context: EventContext {
                device: DeviceInfo {
                    device_type: DeviceType::Desktop,
                    os: "Linux".to_string(),
                    browser: "Firefox".to_string(),
                    screen_resolution: (1920, 1080),
                },
                location: LocationInfo {
                    country: "BR".to_string(),
                    city: None,
                    timezone: "America/Sao_Paulo".to_string(),
                    ip_address: "127.0.0.1".to_string(),
                },
                referrer: None,
                user_agent: "Mozilla/5.0".to_string(),
                viewport: Viewport { width: 1920, height: 1080 },
            },
        }
    }

    #[test]
    fn test_dashboard_metrics() {
        let events = vec![
            create_test_event("user1", "session1", EventType::PageView {
                url: "/".to_string(),
                title: "Home".to_string(),
                duration_ms: 1000,
            }),
            create_test_event("user1", "session1", EventType::Purchase {
                product_id: "prod1".to_string(),
                amount: 99.99,
                currency: "USD".to_string(),
            }),
            create_test_event("user2", "session2", EventType::PageView {
                url: "/about".to_string(),
                title: "About".to_string(),
                duration_ms: 2000,
            }),
        ];

        let dashboard = Dashboard::from_events(events);
        let metrics = dashboard.calculate_metrics();

        assert_eq!(metrics.total_users, 2);
        assert_eq!(metrics.total_sessions, 2);
        assert_eq!(metrics.total_events, 3);
        assert_eq!(metrics.total_purchases, 1);
        assert!((metrics.total_revenue - 99.99).abs() < 0.01);
    }
}
