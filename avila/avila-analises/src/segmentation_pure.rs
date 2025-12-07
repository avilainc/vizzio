//! User Segmentation - Implementação pura sem dependências externas
//!
//! Sistema de segmentação dinâmica de usuários baseada em comportamento

use std::collections::{HashMap, HashSet};
use crate::models::{BehaviorEvent, UserProfile};

/// Operador de comparação para condições
#[derive(Debug, Clone, PartialEq)]
pub enum Operator {
    Equals,
    NotEquals,
    GreaterThan,
    LessThan,
    GreaterThanOrEqual,
    LessThanOrEqual,
    Contains,
    NotContains,
    In,
    NotIn,
}

/// Valor para comparação
#[derive(Debug, Clone)]
pub enum ConditionValue {
    String(String),
    Number(f64),
    Boolean(bool),
    StringList(Vec<String>),
}

/// Condição de segmentação
#[derive(Debug, Clone)]
pub struct SegmentCondition {
    pub field: String,
    pub operator: Operator,
    pub value: ConditionValue,
}

impl SegmentCondition {
    pub fn new(field: impl Into<String>, operator: Operator, value: ConditionValue) -> Self {
        Self {
            field: field.into(),
            operator,
            value,
        }
    }

    /// Avalia se um perfil de usuário satisfaz esta condição
    pub fn evaluate(&self, profile: &UserProfile) -> bool {
        match self.field.as_str() {
            "total_sessions" => self.compare_number(profile.total_sessions as f64),
            "total_events" => self.compare_number(profile.total_events as f64),
            "engagement_score" => self.compare_number(profile.engagement_score),
            "loyalty_score" => self.compare_number(profile.loyalty_score),
            "churn_risk" => self.compare_number(profile.churn_risk),
            "total_purchases" => self.compare_number(profile.behaviors.total_purchases as f64),
            "total_spent" => self.compare_number(profile.behaviors.total_spent),
            "avg_order_value" => self.compare_number(profile.behaviors.avg_order_value),
            "bounce_rate" => self.compare_number(profile.behaviors.bounce_rate),
            "conversion_rate" => self.compare_number(profile.behaviors.conversion_rate),
            _ => false,
        }
    }

    fn compare_number(&self, actual: f64) -> bool {
        if let ConditionValue::Number(expected) = self.value {
            match self.operator {
                Operator::Equals => (actual - expected).abs() < f64::EPSILON,
                Operator::NotEquals => (actual - expected).abs() >= f64::EPSILON,
                Operator::GreaterThan => actual > expected,
                Operator::LessThan => actual < expected,
                Operator::GreaterThanOrEqual => actual >= expected,
                Operator::LessThanOrEqual => actual <= expected,
                _ => false,
            }
        } else {
            false
        }
    }

    fn compare_string(&self, actual: &str) -> bool {
        match &self.value {
            ConditionValue::String(expected) => match self.operator {
                Operator::Equals => actual == expected,
                Operator::NotEquals => actual != expected,
                Operator::Contains => actual.contains(expected),
                Operator::NotContains => !actual.contains(expected),
                _ => false,
            },
            ConditionValue::StringList(list) => match self.operator {
                Operator::In => list.contains(&actual.to_string()),
                Operator::NotIn => !list.contains(&actual.to_string()),
                _ => false,
            },
            _ => false,
        }
    }
}

/// Lógica de combinação de condições
#[derive(Debug, Clone, PartialEq)]
pub enum LogicOperator {
    And,
    Or,
}

/// Definição de um segmento
pub struct Segment {
    pub name: String,
    pub description: String,
    pub conditions: Vec<SegmentCondition>,
    pub logic: LogicOperator,
}

impl Segment {
    pub fn new(name: impl Into<String>, description: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            description: description.into(),
            conditions: Vec::new(),
            logic: LogicOperator::And,
        }
    }

    pub fn add_condition(mut self, condition: SegmentCondition) -> Self {
        self.conditions.push(condition);
        self
    }

    pub fn with_logic(mut self, logic: LogicOperator) -> Self {
        self.logic = logic;
        self
    }

    /// Verifica se um usuário pertence a este segmento
    pub fn matches(&self, profile: &UserProfile) -> bool {
        if self.conditions.is_empty() {
            return true;
        }

        match self.logic {
            LogicOperator::And => self.conditions.iter().all(|c| c.evaluate(profile)),
            LogicOperator::Or => self.conditions.iter().any(|c| c.evaluate(profile)),
        }
    }
}

/// Sistema de segmentação
pub struct Segmentation {
    segments: Vec<Segment>,
    user_segments: HashMap<String, Vec<String>>,
}

impl Segmentation {
    pub fn new() -> Self {
        Self {
            segments: Vec::new(),
            user_segments: HashMap::new(),
        }
    }

    /// Adiciona um novo segmento
    pub fn add_segment(&mut self, segment: Segment) {
        self.segments.push(segment);
    }

    /// Segmenta um conjunto de usuários
    pub fn segment_users(&mut self, profiles: &[UserProfile]) -> SegmentationResult {
        self.user_segments.clear();
        let mut segment_counts: HashMap<String, usize> = HashMap::new();

        for profile in profiles {
            let mut user_segment_names = Vec::new();

            for segment in &self.segments {
                if segment.matches(profile) {
                    user_segment_names.push(segment.name.clone());
                    *segment_counts.entry(segment.name.clone()).or_insert(0) += 1;
                }
            }

            if !user_segment_names.is_empty() {
                self.user_segments.insert(profile.user_id.clone(), user_segment_names);
            }
        }

        let total_users = profiles.len();
        let segmented_users = self.user_segments.len();
        let unsegmented_users = total_users - segmented_users;

        SegmentationResult {
            total_users,
            segmented_users,
            unsegmented_users,
            segment_counts,
        }
    }

    /// Obtém os segmentos de um usuário
    pub fn get_user_segments(&self, user_id: &str) -> Option<&Vec<String>> {
        self.user_segments.get(user_id)
    }

    /// Obtém todos os usuários de um segmento
    pub fn get_segment_users(&self, segment_name: &str) -> Vec<String> {
        self.user_segments
            .iter()
            .filter(|(_, segments)| segments.contains(&segment_name.to_string()))
            .map(|(user_id, _)| user_id.clone())
            .collect()
    }

    /// Calcula overlap entre segmentos
    pub fn calculate_overlap(&self, segment1: &str, segment2: &str) -> SegmentOverlap {
        let users1: HashSet<_> = self.get_segment_users(segment1).into_iter().collect();
        let users2: HashSet<_> = self.get_segment_users(segment2).into_iter().collect();

        let intersection: HashSet<_> = users1.intersection(&users2).cloned().collect();
        let union: HashSet<_> = users1.union(&users2).cloned().collect();

        let jaccard_index = if !union.is_empty() {
            intersection.len() as f64 / union.len() as f64
        } else {
            0.0
        };

        SegmentOverlap {
            segment1: segment1.to_string(),
            segment2: segment2.to_string(),
            overlap_count: intersection.len(),
            only_in_segment1: users1.difference(&users2).count(),
            only_in_segment2: users2.difference(&users1).count(),
            jaccard_index,
        }
    }

    /// Obtém estatísticas de todos os segmentos
    pub fn get_all_segments_info(&self) -> Vec<SegmentInfo> {
        self.segments
            .iter()
            .map(|segment| {
                let user_count = self.get_segment_users(&segment.name).len();
                SegmentInfo {
                    name: segment.name.clone(),
                    description: segment.description.clone(),
                    user_count,
                    condition_count: segment.conditions.len(),
                }
            })
            .collect()
    }
}

impl Default for Segmentation {
    fn default() -> Self {
        Self::new()
    }
}

/// Resultado da segmentação
#[derive(Debug, Clone)]
pub struct SegmentationResult {
    pub total_users: usize,
    pub segmented_users: usize,
    pub unsegmented_users: usize,
    pub segment_counts: HashMap<String, usize>,
}

impl SegmentationResult {
    pub fn print_report(&self) {
        println!("\n=== Segmentation Results ===");
        println!("Total Users: {}", self.total_users);
        println!("Segmented Users: {} ({:.2}%)",
            self.segmented_users,
            (self.segmented_users as f64 / self.total_users as f64) * 100.0
        );
        println!("Unsegmented Users: {}", self.unsegmented_users);
        println!("\nSegment Distribution:");

        let mut sorted_segments: Vec<_> = self.segment_counts.iter().collect();
        sorted_segments.sort_by(|a, b| b.1.cmp(a.1));

        for (name, count) in sorted_segments {
            let percentage = (*count as f64 / self.total_users as f64) * 100.0;
            println!("  {}: {} users ({:.2}%)", name, count, percentage);
        }
    }
}

/// Informações de overlap entre segmentos
#[derive(Debug, Clone)]
pub struct SegmentOverlap {
    pub segment1: String,
    pub segment2: String,
    pub overlap_count: usize,
    pub only_in_segment1: usize,
    pub only_in_segment2: usize,
    pub jaccard_index: f64,
}

impl SegmentOverlap {
    pub fn print_report(&self) {
        println!("\n=== Segment Overlap Analysis ===");
        println!("Segment 1: {}", self.segment1);
        println!("Segment 2: {}", self.segment2);
        println!("Overlap: {} users", self.overlap_count);
        println!("Only in {}: {} users", self.segment1, self.only_in_segment1);
        println!("Only in {}: {} users", self.segment2, self.only_in_segment2);
        println!("Jaccard Index: {:.4}", self.jaccard_index);
    }
}

/// Informações de um segmento
#[derive(Debug, Clone)]
pub struct SegmentInfo {
    pub name: String,
    pub description: String,
    pub user_count: usize,
    pub condition_count: usize,
}

/// Builder para criar segmentos pré-definidos comuns
pub struct SegmentBuilder;

impl SegmentBuilder {
    /// Segmento de usuários de alto valor
    pub fn high_value_users(min_ltv: f64) -> Segment {
        Segment::new(
            "High Value Users",
            format!("Users with lifetime value > ${}", min_ltv),
        )
        .add_condition(SegmentCondition::new(
            "total_spent",
            Operator::GreaterThan,
            ConditionValue::Number(min_ltv),
        ))
    }

    /// Segmento de usuários ativos
    pub fn active_users(min_sessions: usize) -> Segment {
        Segment::new(
            "Active Users",
            format!("Users with {} or more sessions", min_sessions),
        )
        .add_condition(SegmentCondition::new(
            "total_sessions",
            Operator::GreaterThanOrEqual,
            ConditionValue::Number(min_sessions as f64),
        ))
    }

    /// Segmento de usuários em risco de churn
    pub fn churn_risk_users(min_risk: f64) -> Segment {
        Segment::new(
            "Churn Risk",
            format!("Users with churn risk > {}", min_risk),
        )
        .add_condition(SegmentCondition::new(
            "churn_risk",
            Operator::GreaterThan,
            ConditionValue::Number(min_risk),
        ))
    }

    /// Segmento de power users
    pub fn power_users(min_engagement: f64) -> Segment {
        Segment::new(
            "Power Users",
            format!("Highly engaged users (score > {})", min_engagement),
        )
        .add_condition(SegmentCondition::new(
            "engagement_score",
            Operator::GreaterThan,
            ConditionValue::Number(min_engagement),
        ))
    }

    /// Segmento de compradores frequentes
    pub fn frequent_buyers(min_purchases: usize) -> Segment {
        Segment::new(
            "Frequent Buyers",
            format!("Users with {} or more purchases", min_purchases),
        )
        .add_condition(SegmentCondition::new(
            "total_purchases",
            Operator::GreaterThanOrEqual,
            ConditionValue::Number(min_purchases as f64),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::*;
    use chrono::Utc;

    fn create_test_profile(user_id: &str, total_spent: f64, engagement: f64) -> UserProfile {
        UserProfile {
            user_id: user_id.to_string(),
            first_seen: Utc::now(),
            last_seen: Utc::now(),
            total_sessions: 10,
            total_events: 100,
            behaviors: UserBehaviors {
                total_spent,
                ..Default::default()
            },
            segments: Vec::new(),
            engagement_score: engagement,
            loyalty_score: 0.5,
            conversion_probability: 0.3,
            churn_risk: 0.2,
            interests: Vec::new(),
            preferred_categories: HashMap::new(),
            browsing_patterns: Default::default(),
        }
    }

    #[test]
    fn test_segment_condition() {
        let condition = SegmentCondition::new(
            "total_spent",
            Operator::GreaterThan,
            ConditionValue::Number(1000.0),
        );

        let profile = create_test_profile("user1", 1500.0, 0.8);
        assert!(condition.evaluate(&profile));

        let profile2 = create_test_profile("user2", 500.0, 0.5);
        assert!(!condition.evaluate(&profile2));
    }

    #[test]
    fn test_segmentation() {
        let mut segmentation = Segmentation::new();

        let high_value = SegmentBuilder::high_value_users(1000.0);
        segmentation.add_segment(high_value);

        let profiles = vec![
            create_test_profile("user1", 1500.0, 0.8),
            create_test_profile("user2", 500.0, 0.5),
            create_test_profile("user3", 2000.0, 0.9),
        ];

        let result = segmentation.segment_users(&profiles);
        assert_eq!(result.segmented_users, 2);
    }
}
