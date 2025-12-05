//! Event Tracker - Implementação pura sem dependências externas
//!
//! Sistema de rastreamento de eventos de comportamento do usuário

use std::collections::{HashMap, VecDeque, HashSet};
use crate::models::BehaviorEvent;

/// Sistema de rastreamento de eventos
pub struct Tracker {
    events: VecDeque<BehaviorEvent>,
    max_buffer_size: usize,
    event_counts: HashMap<String, usize>,
    user_event_counts: HashMap<String, usize>,
    session_counts: HashMap<String, usize>,
}

impl Tracker {
    /// Cria um novo tracker com capacidade padrão
    pub fn new() -> Self {
        Self::with_capacity(10000)
    }

    /// Cria um novo tracker com capacidade específica
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            events: VecDeque::with_capacity(capacity),
            max_buffer_size: capacity,
            event_counts: HashMap::new(),
            user_event_counts: HashMap::new(),
            session_counts: HashMap::new(),
        }
    }

    /// Rastreia um novo evento
    pub fn track(&mut self, event: BehaviorEvent) {
        // Atualiza contadores
        let event_name = self.get_event_name(&event);
        *self.event_counts.entry(event_name).or_insert(0) += 1;
        *self.user_event_counts.entry(event.user_id.clone()).or_insert(0) += 1;
        *self.session_counts.entry(event.session_id.clone()).or_insert(0) += 1;

        // Adiciona evento ao buffer
        self.events.push_back(event);

        // Remove eventos antigos se exceder capacidade
        if self.events.len() > self.max_buffer_size {
            if let Some(old_event) = self.events.pop_front() {
                // Decrementa contadores do evento removido
                let old_name = self.get_event_name(&old_event);
                if let Some(count) = self.event_counts.get_mut(&old_name) {
                    *count = count.saturating_sub(1);
                }
            }
        }
    }

    /// Rastreia múltiplos eventos em batch
    pub fn track_batch(&mut self, events: Vec<BehaviorEvent>) {
        for event in events {
            self.track(event);
        }
    }

    /// Obtém todos os eventos
    pub fn get_events(&self) -> Vec<BehaviorEvent> {
        self.events.iter().cloned().collect()
    }

    /// Obtém eventos de um usuário específico
    pub fn get_user_events(&self, user_id: &str) -> Vec<BehaviorEvent> {
        self.events
            .iter()
            .filter(|e| e.user_id == user_id)
            .cloned()
            .collect()
    }

    /// Obtém eventos de uma sessão específica
    pub fn get_session_events(&self, session_id: &str) -> Vec<BehaviorEvent> {
        self.events
            .iter()
            .filter(|e| e.session_id == session_id)
            .cloned()
            .collect()
    }

    /// Filtra eventos por tipo
    pub fn filter_by_type(&self, type_name: &str) -> Vec<BehaviorEvent> {
        self.events
            .iter()
            .filter(|e| self.get_event_name(e) == type_name)
            .cloned()
            .collect()
    }

    /// Filtra eventos por período
    pub fn filter_by_time_range(
        &self,
        start: &chrono::DateTime<chrono::Utc>,
        end: &chrono::DateTime<chrono::Utc>,
    ) -> Vec<BehaviorEvent> {
        self.events
            .iter()
            .filter(|e| e.timestamp >= *start && e.timestamp <= *end)
            .cloned()
            .collect()
    }

    /// Obtém contagem de eventos por tipo
    pub fn get_event_counts(&self) -> &HashMap<String, usize> {
        &self.event_counts
    }

    /// Obtém contagem de eventos por usuário
    pub fn get_user_event_counts(&self) -> &HashMap<String, usize> {
        &self.user_event_counts
    }

    /// Retorna o total de eventos rastreados
    pub fn total_events(&self) -> usize {
        self.events.len()
    }

    /// Retorna quantos usuários únicos foram rastreados
    pub fn unique_users(&self) -> usize {
        self.user_event_counts.keys().filter(|k| {
            self.user_event_counts.get(*k).map_or(false, |&count| count > 0)
        }).count()
    }

    /// Retorna quantas sessões únicas foram rastreadas
    pub fn unique_sessions(&self) -> usize {
        let mut sessions: HashSet<&String> = HashSet::new();
        for event in &self.events {
            sessions.insert(&event.session_id);
        }
        sessions.len()
    }

    /// Limpa todos os eventos
    pub fn clear(&mut self) {
        self.events.clear();
        self.event_counts.clear();
        self.user_event_counts.clear();
        self.session_counts.clear();
    }

    /// Obtém nome legível do tipo de evento
    fn get_event_name(&self, event: &BehaviorEvent) -> String {
        use crate::models::EventType;
        match &event.event_type {
            EventType::PageView { .. } => "page_view".to_string(),
            EventType::Click { .. } => "click".to_string(),
            EventType::Scroll { .. } => "scroll".to_string(),
            EventType::Search { .. } => "search".to_string(),
            EventType::Purchase { .. } => "purchase".to_string(),
            EventType::AddToCart { .. } => "add_to_cart".to_string(),
            EventType::RemoveFromCart { .. } => "remove_from_cart".to_string(),
            EventType::FormSubmit { .. } => "form_submit".to_string(),
            EventType::VideoPlay { .. } => "video_play".to_string(),
            EventType::VideoComplete { .. } => "video_complete".to_string(),
            EventType::Download { .. } => "download".to_string(),
            EventType::Share { .. } => "share".to_string(),
            EventType::Custom { name, .. } => name.clone(),
        }
    }

    /// Obtém estatísticas rápidas
    pub fn get_stats(&self) -> TrackerStats {
        TrackerStats {
            total_events: self.total_events(),
            unique_users: self.unique_users(),
            unique_sessions: self.unique_sessions(),
            buffer_usage_percent: (self.events.len() as f64 / self.max_buffer_size as f64) * 100.0,
            events_by_type: self.event_counts.clone(),
        }
    }

    /// Calcula taxa de conversão geral
    pub fn conversion_rate(&self) -> f64 {
        let total = self.total_events() as f64;
        if total == 0.0 {
            return 0.0;
        }

        let purchases = self.event_counts.get("purchase").copied().unwrap_or(0) as f64;
        (purchases / total) * 100.0
    }

    /// Calcula engajamento médio por usuário
    pub fn avg_events_per_user(&self) -> f64 {
        let users = self.unique_users() as f64;
        if users == 0.0 {
            return 0.0;
        }

        self.total_events() as f64 / users
    }
}

impl Default for Tracker {
    fn default() -> Self {
        Self::new()
    }
}

/// Estatísticas do tracker
#[derive(Debug, Clone)]
pub struct TrackerStats {
    pub total_events: usize,
    pub unique_users: usize,
    pub unique_sessions: usize,
    pub buffer_usage_percent: f64,
    pub events_by_type: HashMap<String, usize>,
}

impl TrackerStats {
    pub fn print_summary(&self) {
        println!("=== Tracker Statistics ===");
        println!("Total Events: {}", self.total_events);
        println!("Unique Users: {}", self.unique_users);
        println!("Unique Sessions: {}", self.unique_sessions);
        println!("Buffer Usage: {:.2}%", self.buffer_usage_percent);
        println!("\nEvents by Type:");
        for (event_type, count) in &self.events_by_type {
            println!("  {}: {}", event_type, count);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::*;
    use chrono::Utc;
    use std::collections::HashMap;

    fn create_test_event(user_id: &str, session_id: &str, event_type: EventType) -> BehaviorEvent {
        BehaviorEvent {
            event_id: format!("evt_{}", user_id),
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
                    city: Some("São Paulo".to_string()),
                    timezone: "America/Sao_Paulo".to_string(),
                    ip_address: "127.0.0.1".to_string(),
                },
                referrer: None,
                user_agent: "Mozilla/5.0".to_string(),
                viewport: Viewport {
                    width: 1920,
                    height: 1080,
                },
            },
        }
    }

    #[test]
    fn test_tracker_basic() {
        let mut tracker = Tracker::new();

        let event = create_test_event(
            "user1",
            "session1",
            EventType::PageView {
                url: "/home".to_string(),
                title: "Home".to_string(),
                duration_ms: 5000,
            },
        );

        tracker.track(event);

        assert_eq!(tracker.total_events(), 1);
        assert_eq!(tracker.unique_users(), 1);
    }

    #[test]
    fn test_tracker_multiple_users() {
        let mut tracker = Tracker::new();

        for i in 0..5 {
            let event = create_test_event(
                &format!("user{}", i),
                "session1",
                EventType::Click {
                    element_id: "btn".to_string(),
                    element_class: "button".to_string(),
                    x: 100,
                    y: 200,
                },
            );
            tracker.track(event);
        }

        assert_eq!(tracker.total_events(), 5);
        assert_eq!(tracker.unique_users(), 5);
    }

    #[test]
    fn test_tracker_buffer_overflow() {
        let mut tracker = Tracker::with_capacity(3);

        for i in 0..5 {
            let event = create_test_event(
                &format!("user{}", i),
                "session1",
                EventType::PageView {
                    url: format!("/page{}", i),
                    title: "Page".to_string(),
                    duration_ms: 1000,
                },
            );
            tracker.track(event);
        }

        // Deve ter apenas os últimos 3 eventos
        assert_eq!(tracker.total_events(), 3);
    }
}
