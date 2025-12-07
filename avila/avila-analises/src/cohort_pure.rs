//! Cohort Analysis - Implementação pura sem dependências externas
//!
//! Análise de coortes para medir retenção e comportamento ao longo do tempo

use std::collections::HashMap;
use chrono::{DateTime, Utc, Datelike, Duration};
use crate::models::BehaviorEvent;

/// Período de agrupamento de coortes
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CohortPeriod {
    Daily,
    Weekly,
    Monthly,
}

/// Identificador de coorte (baseado em data)
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CohortId {
    pub year: i32,
    pub period: u32,
    pub period_type: CohortPeriod,
}

impl CohortId {
    /// Cria um CohortId a partir de uma data
    pub fn from_date(date: &DateTime<Utc>, period: CohortPeriod) -> Self {
        match period {
            CohortPeriod::Daily => Self {
                year: date.year(),
                period: date.ordinal(),
                period_type: period,
            },
            CohortPeriod::Weekly => {
                let week = date.iso_week().week();
                Self {
                    year: date.year(),
                    period: week,
                    period_type: period,
                }
            }
            CohortPeriod::Monthly => Self {
                year: date.year(),
                period: date.month(),
                period_type: period,
            },
        }
    }

    /// Retorna uma representação em string
    pub fn to_string(&self) -> String {
        match self.period_type {
            CohortPeriod::Daily => format!("{}-D{:03}", self.year, self.period),
            CohortPeriod::Weekly => format!("{}-W{:02}", self.year, self.period),
            CohortPeriod::Monthly => format!("{}-M{:02}", self.year, self.period),
        }
    }
}

/// Dados de uma coorte
#[derive(Debug, Clone)]
pub struct Cohort {
    pub id: CohortId,
    pub users: Vec<String>,
    pub first_seen: DateTime<Utc>,
}

impl Cohort {
    pub fn new(id: CohortId, first_seen: DateTime<Utc>) -> Self {
        Self {
            id,
            users: Vec::new(),
            first_seen,
        }
    }

    pub fn add_user(&mut self, user_id: String) {
        if !self.users.contains(&user_id) {
            self.users.push(user_id);
        }
    }

    pub fn size(&self) -> usize {
        self.users.len()
    }
}

/// Análise de coorte
pub struct CohortAnalysis {
    period: CohortPeriod,
    cohorts: HashMap<CohortId, Cohort>,
    user_cohorts: HashMap<String, CohortId>,
}

impl CohortAnalysis {
    pub fn new(period: CohortPeriod) -> Self {
        Self {
            period,
            cohorts: HashMap::new(),
            user_cohorts: HashMap::new(),
        }
    }

    /// Processa eventos para construir coortes
    pub fn process_events(&mut self, events: &[BehaviorEvent]) {
        // Agrupa eventos por usuário e ordena por timestamp
        let mut user_events: HashMap<String, Vec<&BehaviorEvent>> = HashMap::new();

        for event in events {
            user_events
                .entry(event.user_id.clone())
                .or_insert_with(Vec::new)
                .push(event);
        }

        // Para cada usuário, encontra o primeiro evento (aquisição)
        for (user_id, events) in user_events {
            if let Some(first_event) = events.iter().min_by_key(|e| e.timestamp) {
                let cohort_id = CohortId::from_date(&first_event.timestamp, self.period);

                // Cria ou obtém a coorte
                let cohort = self.cohorts
                    .entry(cohort_id.clone())
                    .or_insert_with(|| Cohort::new(cohort_id.clone(), first_event.timestamp));

                cohort.add_user(user_id.clone());
                self.user_cohorts.insert(user_id, cohort_id);
            }
        }
    }

    /// Calcula retenção para cada coorte
    pub fn calculate_retention(&self, events: &[BehaviorEvent]) -> CohortRetentionReport {
        let mut retention_data: HashMap<CohortId, Vec<RetentionPeriod>> = HashMap::new();

        for (cohort_id, cohort) in &self.cohorts {
            let mut period_retention = Vec::new();

            // Calcula retenção para cada período após aquisição
            for period_offset in 0..12 {
                let period_start = self.add_periods(&cohort.first_seen, period_offset);
                let period_end = self.add_periods(&cohort.first_seen, period_offset + 1);

                let active_users = self.count_active_users_in_period(
                    &cohort.users,
                    events,
                    &period_start,
                    &period_end,
                );

                let retention_rate = if cohort.size() > 0 {
                    (active_users as f64 / cohort.size() as f64) * 100.0
                } else {
                    0.0
                };

                period_retention.push(RetentionPeriod {
                    period: period_offset,
                    active_users,
                    total_users: cohort.size(),
                    retention_rate,
                });

                // Para se ninguém está mais ativo
                if active_users == 0 && period_offset > 0 {
                    break;
                }
            }

            retention_data.insert(cohort_id.clone(), period_retention);
        }

        CohortRetentionReport {
            period_type: self.period,
            cohorts: self.get_cohort_summaries(),
            retention_data,
        }
    }

    /// Conta usuários ativos em um período
    fn count_active_users_in_period(
        &self,
        users: &[String],
        events: &[BehaviorEvent],
        start: &DateTime<Utc>,
        end: &DateTime<Utc>,
    ) -> usize {
        let mut active: std::collections::HashSet<&String> = std::collections::HashSet::new();

        for event in events {
            if event.timestamp >= *start && event.timestamp < *end {
                if users.contains(&event.user_id) {
                    active.insert(&event.user_id);
                }
            }
        }

        active.len()
    }

    /// Adiciona períodos a uma data
    fn add_periods(&self, date: &DateTime<Utc>, periods: i32) -> DateTime<Utc> {
        match self.period {
            CohortPeriod::Daily => *date + Duration::days(periods as i64),
            CohortPeriod::Weekly => *date + Duration::weeks(periods as i64),
            CohortPeriod::Monthly => {
                // Aproximação: 30 dias por mês
                *date + Duration::days((periods * 30) as i64)
            }
        }
    }

    /// Obtém sumário das coortes
    fn get_cohort_summaries(&self) -> Vec<CohortSummary> {
        let mut summaries: Vec<_> = self.cohorts
            .iter()
            .map(|(id, cohort)| CohortSummary {
                cohort_id: id.to_string(),
                user_count: cohort.size(),
                first_seen: cohort.first_seen,
            })
            .collect();

        summaries.sort_by(|a, b| a.first_seen.cmp(&b.first_seen));
        summaries
    }

    /// Obtém a coorte de um usuário
    pub fn get_user_cohort(&self, user_id: &str) -> Option<&CohortId> {
        self.user_cohorts.get(user_id)
    }

    /// Obtém todas as coortes
    pub fn get_all_cohorts(&self) -> Vec<&Cohort> {
        self.cohorts.values().collect()
    }
}

/// Período de retenção
#[derive(Debug, Clone)]
pub struct RetentionPeriod {
    pub period: i32,
    pub active_users: usize,
    pub total_users: usize,
    pub retention_rate: f64,
}

/// Sumário de uma coorte
#[derive(Debug, Clone)]
pub struct CohortSummary {
    pub cohort_id: String,
    pub user_count: usize,
    pub first_seen: DateTime<Utc>,
}

/// Relatório de retenção de coortes
#[derive(Debug, Clone)]
pub struct CohortRetentionReport {
    pub period_type: CohortPeriod,
    pub cohorts: Vec<CohortSummary>,
    pub retention_data: HashMap<CohortId, Vec<RetentionPeriod>>,
}

impl CohortRetentionReport {
    /// Imprime relatório formatado
    pub fn print_report(&self) {
        println!("\n=== Cohort Retention Analysis ({:?}) ===", self.period_type);
        println!("\nCohorts Overview:");

        for cohort in &self.cohorts {
            println!("  {} - {} users (First seen: {})",
                cohort.cohort_id,
                cohort.user_count,
                cohort.first_seen.format("%Y-%m-%d")
            );
        }

        println!("\nRetention Rates (%):");
        println!("{:<15} {:>6} {:>6} {:>6} {:>6} {:>6} {:>6}",
            "Cohort", "P0", "P1", "P2", "P3", "P4", "P5");
        println!("{:-<63}", "");

        for cohort in &self.cohorts {
            let cohort_id = CohortId::from_date(&cohort.first_seen, self.period_type);

            if let Some(retention) = self.retention_data.get(&cohort_id) {
                print!("{:<15}", cohort.cohort_id);
                for period in retention.iter().take(6) {
                    print!(" {:>5.1}", period.retention_rate);
                }
                println!();
            }
        }
    }

    /// Calcula retenção média por período
    pub fn avg_retention_by_period(&self) -> Vec<(i32, f64)> {
        let mut period_sums: HashMap<i32, (f64, usize)> = HashMap::new();

        for retention_periods in self.retention_data.values() {
            for period in retention_periods {
                let entry = period_sums.entry(period.period).or_insert((0.0, 0));
                entry.0 += period.retention_rate;
                entry.1 += 1;
            }
        }

        let mut result: Vec<_> = period_sums
            .into_iter()
            .map(|(period, (sum, count))| (period, sum / count as f64))
            .collect();

        result.sort_by_key(|(period, _)| *period);
        result
    }

    /// Encontra coorte com melhor retenção
    pub fn best_cohort(&self) -> Option<(String, f64)> {
        self.retention_data
            .iter()
            .filter_map(|(id, periods)| {
                // Calcula retenção média dos primeiros 3 períodos
                let avg = periods.iter()
                    .take(3)
                    .map(|p| p.retention_rate)
                    .sum::<f64>() / periods.len().min(3) as f64;

                Some((id.to_string(), avg))
            })
            .max_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
    }

    /// Encontra coorte com pior retenção
    pub fn worst_cohort(&self) -> Option<(String, f64)> {
        self.retention_data
            .iter()
            .filter_map(|(id, periods)| {
                let avg = periods.iter()
                    .take(3)
                    .map(|p| p.retention_rate)
                    .sum::<f64>() / periods.len().min(3) as f64;

                Some((id.to_string(), avg))
            })
            .min_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::*;
    use std::collections::HashMap;

    fn create_test_event(user_id: &str, days_ago: i64) -> BehaviorEvent {
        BehaviorEvent {
            event_id: format!("evt_{}", user_id),
            user_id: user_id.to_string(),
            session_id: "session1".to_string(),
            timestamp: Utc::now() - Duration::days(days_ago),
            event_type: EventType::PageView {
                url: "/".to_string(),
                title: "Home".to_string(),
                duration_ms: 1000,
            },
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
    fn test_cohort_creation() {
        let mut analysis = CohortAnalysis::new(CohortPeriod::Weekly);

        let events = vec![
            create_test_event("user1", 0),
            create_test_event("user2", 7),
            create_test_event("user3", 14),
        ];

        analysis.process_events(&events);

        assert!(analysis.cohorts.len() > 0);
    }

    #[test]
    fn test_cohort_id_from_date() {
        let date = Utc::now();
        let id = CohortId::from_date(&date, CohortPeriod::Monthly);

        assert_eq!(id.year, date.year());
        assert_eq!(id.period, date.month());
        assert_eq!(id.period_type, CohortPeriod::Monthly);
    }
}
