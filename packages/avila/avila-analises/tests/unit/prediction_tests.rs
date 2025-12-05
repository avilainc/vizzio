//! Unit tests for prediction and machine learning module

use avila_analises::prediction::*;
use avila_analises::models::*;
use chrono::{DateTime, Utc, Duration};

#[test]
fn test_predictor_creation() {
    let predictor = Predictor::new();
    assert!(predictor.is_ok(), "Should create Predictor successfully");
}

#[test]
fn test_rfm_score_calculation() {
    let predictor = Predictor::new().unwrap();

    let mut user_profile = UserProfile {
        user_id: "user123".to_string(),
        first_seen: Utc::now() - Duration::days(100),
        last_seen: Utc::now() - Duration::days(5),
        total_sessions: 10,
        total_events: 150,
        properties: std::collections::HashMap::new(),
        segments: vec![],
        score: None,
    };

    let result = predictor.calculate_rfm_score(&mut user_profile);

    assert!(result.is_ok(), "Should calculate RFM score");
    assert!(user_profile.score.is_some(), "User should have RFM score");

    let score = user_profile.score.unwrap();
    assert!(score >= 1.0 && score <= 5.0, "RFM score should be between 1 and 5");
}

#[test]
fn test_rfm_score_components() {
    let predictor = Predictor::new().unwrap();

    // High value customer - recent, frequent, high monetary
    let mut high_value = UserProfile {
        user_id: "high_value".to_string(),
        first_seen: Utc::now() - Duration::days(10),
        last_seen: Utc::now() - Duration::days(1),  // Very recent
        total_sessions: 50,  // Frequent
        total_events: 500,   // High engagement
        properties: std::collections::HashMap::new(),
        segments: vec![],
        score: None,
    };

    // Low value customer - not recent, infrequent
    let mut low_value = UserProfile {
        user_id: "low_value".to_string(),
        first_seen: Utc::now() - Duration::days(365),
        last_seen: Utc::now() - Duration::days(180),  // Long time ago
        total_sessions: 2,   // Infrequent
        total_events: 5,     // Low engagement
        properties: std::collections::HashMap::new(),
        segments: vec![],
        score: None,
    };

    predictor.calculate_rfm_score(&mut high_value).unwrap();
    predictor.calculate_rfm_score(&mut low_value).unwrap();

    assert!(high_value.score.unwrap() > low_value.score.unwrap(),
            "High value customer should have higher RFM score");
}

#[test]
fn test_clv_prediction() {
    let predictor = Predictor::new().unwrap();

    let user_profile = UserProfile {
        user_id: "user456".to_string(),
        first_seen: Utc::now() - Duration::days(180),
        last_seen: Utc::now() - Duration::days(2),
        total_sessions: 25,
        total_events: 300,
        properties: std::collections::HashMap::new(),
        segments: vec![],
        score: Some(4.5),
    };

    let clv = predictor.predict_customer_lifetime_value(&user_profile);

    assert!(clv.is_ok(), "Should predict CLV");
    let clv_value = clv.unwrap();
    assert!(clv_value > 0.0, "CLV should be positive");
}

#[test]
fn test_clv_based_on_activity() {
    let predictor = Predictor::new().unwrap();

    let active_user = UserProfile {
        user_id: "active".to_string(),
        first_seen: Utc::now() - Duration::days(100),
        last_seen: Utc::now() - Duration::days(1),
        total_sessions: 100,
        total_events: 1000,
        properties: std::collections::HashMap::new(),
        segments: vec![],
        score: Some(5.0),
    };

    let inactive_user = UserProfile {
        user_id: "inactive".to_string(),
        first_seen: Utc::now() - Duration::days(100),
        last_seen: Utc::now() - Duration::days(90),
        total_sessions: 5,
        total_events: 20,
        properties: std::collections::HashMap::new(),
        segments: vec![],
        score: Some(2.0),
    };

    let clv_active = predictor.predict_customer_lifetime_value(&active_user).unwrap();
    let clv_inactive = predictor.predict_customer_lifetime_value(&inactive_user).unwrap();

    assert!(clv_active > clv_inactive, "Active user should have higher CLV");
}

#[test]
fn test_churn_risk_prediction() {
    let predictor = Predictor::new().unwrap();

    let user_profile = UserProfile {
        user_id: "user789".to_string(),
        first_seen: Utc::now() - Duration::days(200),
        last_seen: Utc::now() - Duration::days(60),  // Hasn't been active recently
        total_sessions: 30,
        total_events: 250,
        properties: std::collections::HashMap::new(),
        segments: vec![],
        score: Some(3.5),
    };

    let churn_risk = predictor.predict_churn_risk(&user_profile);

    assert!(churn_risk.is_ok(), "Should predict churn risk");
    let risk = churn_risk.unwrap();
    assert!(risk >= 0.0 && risk <= 1.0, "Churn risk should be between 0 and 1");
}

#[test]
fn test_churn_risk_high_for_inactive() {
    let predictor = Predictor::new().unwrap();

    let inactive_profile = UserProfile {
        user_id: "inactive_user".to_string(),
        first_seen: Utc::now() - Duration::days(365),
        last_seen: Utc::now() - Duration::days(180),  // Very inactive
        total_sessions: 5,
        total_events: 20,
        properties: std::collections::HashMap::new(),
        segments: vec![],
        score: Some(1.5),
    };

    let active_profile = UserProfile {
        user_id: "active_user".to_string(),
        first_seen: Utc::now() - Duration::days(365),
        last_seen: Utc::now() - Duration::days(1),  // Very active
        total_sessions: 200,
        total_events: 2000,
        properties: std::collections::HashMap::new(),
        segments: vec![],
        score: Some(5.0),
    };

    let risk_inactive = predictor.predict_churn_risk(&inactive_profile).unwrap();
    let risk_active = predictor.predict_churn_risk(&active_profile).unwrap();

    assert!(risk_inactive > risk_active, "Inactive user should have higher churn risk");
    assert!(risk_inactive > 0.5, "Inactive user should have high churn risk (> 0.5)");
}

#[test]
fn test_next_action_prediction() {
    let predictor = Predictor::new().unwrap();

    let events = vec![
        BehaviorEvent {
            event_id: "e1".to_string(),
            user_id: "user123".to_string(),
            session_id: "session1".to_string(),
            event_type: "page_view".to_string(),
            timestamp: Utc::now() - Duration::minutes(10),
            properties: std::collections::HashMap::new(),
            context: EventContext::default(),
        },
        BehaviorEvent {
            event_id: "e2".to_string(),
            user_id: "user123".to_string(),
            session_id: "session1".to_string(),
            event_type: "add_to_cart".to_string(),
            timestamp: Utc::now() - Duration::minutes(5),
            properties: std::collections::HashMap::new(),
            context: EventContext::default(),
        },
    ];

    let prediction = predictor.predict_next_action(&events);

    assert!(prediction.is_ok(), "Should predict next action");
    let next_action = prediction.unwrap();
    assert!(!next_action.is_empty(), "Should suggest a next action");
}

#[test]
fn test_segment_prediction() {
    let predictor = Predictor::new().unwrap();

    let user_profile = UserProfile {
        user_id: "user999".to_string(),
        first_seen: Utc::now() - Duration::days(50),
        last_seen: Utc::now() - Duration::days(1),
        total_sessions: 40,
        total_events: 600,
        properties: std::collections::HashMap::new(),
        segments: vec![],
        score: Some(4.8),
    };

    let segment = predictor.predict_user_segment(&user_profile);

    assert!(segment.is_ok(), "Should predict user segment");
    let seg = segment.unwrap();
    assert!(!seg.is_empty(), "Should return a segment name");
}

#[test]
fn test_batch_rfm_calculation() {
    let predictor = Predictor::new().unwrap();

    let mut profiles = vec![
        UserProfile {
            user_id: "user1".to_string(),
            first_seen: Utc::now() - Duration::days(30),
            last_seen: Utc::now() - Duration::days(1),
            total_sessions: 20,
            total_events: 200,
            properties: std::collections::HashMap::new(),
            segments: vec![],
            score: None,
        },
        UserProfile {
            user_id: "user2".to_string(),
            first_seen: Utc::now() - Duration::days(60),
            last_seen: Utc::now() - Duration::days(30),
            total_sessions: 10,
            total_events: 50,
            properties: std::collections::HashMap::new(),
            segments: vec![],
            score: None,
        },
    ];

    for profile in &mut profiles {
        predictor.calculate_rfm_score(profile).unwrap();
    }

    assert!(profiles[0].score.is_some(), "First profile should have score");
    assert!(profiles[1].score.is_some(), "Second profile should have score");
}

#[test]
fn test_prediction_consistency() {
    let predictor = Predictor::new().unwrap();

    let user_profile = UserProfile {
        user_id: "consistent_user".to_string(),
        first_seen: Utc::now() - Duration::days(100),
        last_seen: Utc::now() - Duration::days(5),
        total_sessions: 30,
        total_events: 400,
        properties: std::collections::HashMap::new(),
        segments: vec![],
        score: Some(4.0),
    };

    // Run predictions multiple times - should be consistent
    let clv1 = predictor.predict_customer_lifetime_value(&user_profile).unwrap();
    let clv2 = predictor.predict_customer_lifetime_value(&user_profile).unwrap();
    let churn1 = predictor.predict_churn_risk(&user_profile).unwrap();
    let churn2 = predictor.predict_churn_risk(&user_profile).unwrap();

    assert_eq!(clv1, clv2, "CLV predictions should be consistent");
    assert_eq!(churn1, churn2, "Churn predictions should be consistent");
}
