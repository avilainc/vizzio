//! Funnel Analysis - Implementação pura sem dependências externas
//!
//! Análise de funil de conversão para identificar drop-offs

use std::collections::HashMap;
use crate::models::BehaviorEvent;

/// Representa um passo no funil de conversão
#[derive(Debug, Clone)]
pub struct FunnelStep {
    pub name: String,
    pub event_type: String,
    pub condition: Option<StepCondition>,
}

/// Condição para um passo do funil
#[derive(Debug, Clone)]
pub enum StepCondition {
    UrlContains(String),
    UrlEquals(String),
    PropertyEquals(String, String),
    CustomPredicate(fn(&BehaviorEvent) -> bool),
}

impl FunnelStep {
    pub fn new(name: impl Into<String>, event_type: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            event_type: event_type.into(),
            condition: None,
        }
    }

    pub fn with_condition(mut self, condition: StepCondition) -> Self {
        self.condition = Some(condition);
        self
    }

    /// Verifica se um evento satisfaz este passo
    pub fn matches(&self, event: &BehaviorEvent) -> bool {
        use crate::models::EventType;

        // Verifica tipo de evento
        let event_name = match &event.event_type {
            EventType::PageView { .. } => "page_view",
            EventType::Click { .. } => "click",
            EventType::Purchase { .. } => "purchase",
            EventType::AddToCart { .. } => "add_to_cart",
            EventType::FormSubmit { .. } => "form_submit",
            EventType::Custom { name, .. } => name.as_str(),
            _ => return false,
        };

        if event_name != self.event_type {
            return false;
        }

        // Verifica condição adicional se existir
        if let Some(ref condition) = self.condition {
            match condition {
                StepCondition::UrlContains(url_part) => {
                    if let EventType::PageView { url, .. } = &event.event_type {
                        url.contains(url_part)
                    } else {
                        false
                    }
                }
                StepCondition::UrlEquals(expected_url) => {
                    if let EventType::PageView { url, .. } = &event.event_type {
                        url == expected_url
                    } else {
                        false
                    }
                }
                StepCondition::PropertyEquals(key, value) => {
                    event.metadata.get(key).map_or(false, |v| v == value)
                }
                StepCondition::CustomPredicate(predicate) => predicate(event),
            }
        } else {
            true
        }
    }
}

/// Configuração do funil
pub struct Funnel {
    pub name: String,
    pub steps: Vec<FunnelStep>,
    pub time_window_seconds: Option<u64>,
}

impl Funnel {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            steps: Vec::new(),
            time_window_seconds: None,
        }
    }

    pub fn add_step(mut self, step: FunnelStep) -> Self {
        self.steps.push(step);
        self
    }

    pub fn with_time_window(mut self, seconds: u64) -> Self {
        self.time_window_seconds = Some(seconds);
        self
    }

    /// Analisa o funil para um conjunto de eventos
    pub fn analyze(&self, events: &[BehaviorEvent]) -> FunnelAnalysis {
        let mut user_journeys: HashMap<String, Vec<&BehaviorEvent>> = HashMap::new();

        // Agrupa eventos por usuário
        for event in events {
            user_journeys
                .entry(event.user_id.clone())
                .or_insert_with(Vec::new)
                .push(event);
        }

        // Ordena eventos de cada usuário por timestamp
        for events in user_journeys.values_mut() {
            events.sort_by_key(|e| e.timestamp);
        }

        // Analisa cada jornada de usuário
        let mut step_counts = vec![0; self.steps.len()];
        let mut completed_users = Vec::new();
        let mut dropped_at_step: HashMap<usize, Vec<String>> = HashMap::new();

        for (user_id, user_events) in &user_journeys {
            let completion = self.check_user_completion(user_events);

            // Atualiza contadores
            for (step_idx, reached) in completion.steps_reached.iter().enumerate() {
                if *reached {
                    step_counts[step_idx] += 1;
                }
            }

            if completion.completed {
                completed_users.push(user_id.clone());
            } else if let Some(drop_idx) = completion.dropped_at_step {
                dropped_at_step
                    .entry(drop_idx)
                    .or_insert_with(Vec::new)
                    .push(user_id.clone());
            }
        }

        // Calcula métricas
        let total_users = user_journeys.len();
        let mut step_results = Vec::new();

        for (idx, step) in self.steps.iter().enumerate() {
            let count = step_counts[idx];
            let conversion_rate = if total_users > 0 {
                (count as f64 / total_users as f64) * 100.0
            } else {
                0.0
            };

            let drop_off_rate = if idx > 0 && step_counts[idx - 1] > 0 {
                let previous = step_counts[idx - 1];
                let dropped = previous - count;
                (dropped as f64 / previous as f64) * 100.0
            } else {
                0.0
            };

            step_results.push(FunnelStepResult {
                step_name: step.name.clone(),
                users_reached: count,
                conversion_rate,
                drop_off_rate,
            });
        }

        FunnelAnalysis {
            funnel_name: self.name.clone(),
            total_users,
            completed_users: completed_users.len(),
            overall_conversion_rate: if total_users > 0 {
                (completed_users.len() as f64 / total_users as f64) * 100.0
            } else {
                0.0
            },
            steps: step_results,
            dropped_users: dropped_at_step,
        }
    }

    /// Verifica se um usuário completou o funil
    fn check_user_completion(&self, events: &[&BehaviorEvent]) -> UserFunnelCompletion {
        let mut steps_reached = vec![false; self.steps.len()];
        let mut current_step = 0;
        let mut last_step_time = None;

        for event in events {
            if current_step >= self.steps.len() {
                break;
            }

            let step = &self.steps[current_step];

            if step.matches(event) {
                // Verifica time window se configurado
                if let Some(window) = self.time_window_seconds {
                    if let Some(last_time) = last_step_time {
                        let elapsed = event.timestamp.signed_duration_since(last_time);
                        if elapsed.num_seconds() > window as i64 {
                            // Excedeu o time window, reseta
                            steps_reached = vec![false; self.steps.len()];
                            current_step = 0;
                            last_step_time = None;
                            continue;
                        }
                    }
                }

                steps_reached[current_step] = true;
                last_step_time = Some(event.timestamp);
                current_step += 1;
            }
        }

        let completed = current_step >= self.steps.len();
        let dropped_at_step = if !completed && current_step > 0 {
            Some(current_step)
        } else {
            None
        };

        UserFunnelCompletion {
            steps_reached,
            completed,
            dropped_at_step,
        }
    }
}

/// Resultado da análise de um passo do funil
#[derive(Debug, Clone)]
pub struct FunnelStepResult {
    pub step_name: String,
    pub users_reached: usize,
    pub conversion_rate: f64,
    pub drop_off_rate: f64,
}

/// Resultado completo da análise do funil
#[derive(Debug, Clone)]
pub struct FunnelAnalysis {
    pub funnel_name: String,
    pub total_users: usize,
    pub completed_users: usize,
    pub overall_conversion_rate: f64,
    pub steps: Vec<FunnelStepResult>,
    pub dropped_users: HashMap<usize, Vec<String>>,
}

impl FunnelAnalysis {
    pub fn print_report(&self) {
        println!("\n=== Funnel Analysis: {} ===", self.funnel_name);
        println!("Total Users: {}", self.total_users);
        println!("Completed: {} ({:.2}%)", self.completed_users, self.overall_conversion_rate);
        println!("\nStep-by-Step Breakdown:");

        for (idx, step) in self.steps.iter().enumerate() {
            println!("\n  Step {}: {}", idx + 1, step.step_name);
            println!("    Users Reached: {}", step.users_reached);
            println!("    Conversion Rate: {:.2}%", step.conversion_rate);
            if step.drop_off_rate > 0.0 {
                println!("    Drop-off Rate: {:.2}%", step.drop_off_rate);
            }
        }

        println!("\nDrop-off Analysis:");
        for (step_idx, users) in &self.dropped_users {
            if !users.is_empty() {
                println!("  {} users dropped at step {}", users.len(), step_idx + 1);
            }
        }
    }

    /// Identifica o passo com maior drop-off
    pub fn biggest_drop_off(&self) -> Option<&FunnelStepResult> {
        self.steps
            .iter()
            .max_by(|a, b| a.drop_off_rate.partial_cmp(&b.drop_off_rate).unwrap())
    }
}

/// Completude do funil para um usuário
#[derive(Debug)]
struct UserFunnelCompletion {
    steps_reached: Vec<bool>,
    completed: bool,
    dropped_at_step: Option<usize>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::*;
    use chrono::Utc;
    use std::collections::HashMap;

    fn create_test_event(user_id: &str, event_type: EventType) -> BehaviorEvent {
        BehaviorEvent {
            event_id: format!("evt_{}", user_id),
            user_id: user_id.to_string(),
            session_id: "session1".to_string(),
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
    fn test_simple_funnel() {
        let funnel = Funnel::new("E-commerce")
            .add_step(FunnelStep::new("Landing", "page_view"))
            .add_step(FunnelStep::new("Product View", "page_view"))
            .add_step(FunnelStep::new("Add to Cart", "add_to_cart"))
            .add_step(FunnelStep::new("Purchase", "purchase"));

        let events = vec![
            create_test_event("user1", EventType::PageView {
                url: "/".to_string(),
                title: "Home".to_string(),
                duration_ms: 1000,
            }),
            create_test_event("user1", EventType::PageView {
                url: "/product".to_string(),
                title: "Product".to_string(),
                duration_ms: 2000,
            }),
            create_test_event("user1", EventType::AddToCart {
                product_id: "prod1".to_string(),
                quantity: 1,
            }),
            create_test_event("user1", EventType::Purchase {
                product_id: "prod1".to_string(),
                amount: 99.99,
                currency: "USD".to_string(),
            }),
        ];

        let analysis = funnel.analyze(&events);
        assert_eq!(analysis.completed_users, 1);
        assert_eq!(analysis.overall_conversion_rate, 100.0);
    }
}
