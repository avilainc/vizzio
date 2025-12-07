//! Unit tests for cohort analysis module

use avila_analises::cohort::*;
use avila_analises::models::*;
use chrono::{DateTime, Utc, Duration};

#[test]
fn test_cohort_analyzer_creation() {
    let analyzer = CohortAnalyzer::new();
    assert!(analyzer.is_ok(), "Should create CohortAnalyzer successfully");
}

#[test]
fn test_add_user_to_cohort() {
    let mut analyzer = CohortAnalyzer::new().unwrap();

    let user_id = "user123".to_string();
    let cohort_date = Utc::now();

    analyzer.add_user_to_cohort(user_id.clone(), cohort_date);

    // Verify user was added
    let cohorts = analyzer.get_all_cohorts();
    assert!(!cohorts.is_empty(), "Cohort should be created");
}

#[test]
fn test_record_user_activity() {
    let mut analyzer = CohortAnalyzer::new().unwrap();

    let user_id = "user456".to_string();
    let cohort_date = Utc::now();
    let activity_date = cohort_date + Duration::days(5);

    analyzer.add_user_to_cohort(user_id.clone(), cohort_date);
    analyzer.record_user_activity(user_id.clone(), activity_date);

    // Verify activity was recorded
    let cohorts = analyzer.get_all_cohorts();
    assert!(!cohorts.is_empty(), "Should have at least one cohort");
}

#[test]
fn test_calculate_retention() {
    let mut analyzer = CohortAnalyzer::new().unwrap();

    let cohort_date = Utc::now();

    // Add multiple users to the same cohort
    for i in 0..10 {
        let user_id = format!("user{}", i);
        analyzer.add_user_to_cohort(user_id, cohort_date);
    }

    // Record activity for some users
    for i in 0..5 {
        let user_id = format!("user{}", i);
        let activity_date = cohort_date + Duration::days(7);
        analyzer.record_user_activity(user_id, activity_date);
    }

    let retention = analyzer.calculate_retention(cohort_date, Duration::days(7));

    match retention {
        Ok(rate) => {
            assert!(rate >= 0.0 && rate <= 1.0, "Retention rate should be between 0 and 1");
            assert!((rate - 0.5).abs() < 0.01, "Expected ~50% retention");
        }
        Err(e) => panic!("Failed to calculate retention: {:?}", e),
    }
}

#[test]
fn test_cohort_analysis_report() {
    let mut analyzer = CohortAnalyzer::new().unwrap();

    let base_date = Utc::now();

    // Create multiple cohorts
    for cohort_offset in 0..3 {
        let cohort_date = base_date - Duration::days(cohort_offset * 7);

        for user_id in 0..10 {
            let user = format!("user_{}_{}", cohort_offset, user_id);
            analyzer.add_user_to_cohort(user.clone(), cohort_date);

            // Add activity for some periods
            for period in 1..=3 {
                if user_id % 2 == 0 {  // 50% retention pattern
                    let activity_date = cohort_date + Duration::days(period * 7);
                    analyzer.record_user_activity(user.clone(), activity_date);
                }
            }
        }
    }

    let report = analyzer.generate_cohort_report();

    assert!(report.is_ok(), "Should generate cohort report successfully");
    let report = report.unwrap();
    assert!(!report.is_empty(), "Report should contain cohort data");
}

#[test]
fn test_empty_cohort_retention() {
    let analyzer = CohortAnalyzer::new().unwrap();

    let result = analyzer.calculate_retention(Utc::now(), Duration::days(7));

    // Should handle empty cohort gracefully
    assert!(result.is_err() || result.unwrap() == 0.0, "Empty cohort should return 0 or error");
}

#[test]
fn test_cohort_size() {
    let mut analyzer = CohortAnalyzer::new().unwrap();

    let cohort_date = Utc::now();
    let expected_size = 15;

    for i in 0..expected_size {
        let user_id = format!("user{}", i);
        analyzer.add_user_to_cohort(user_id, cohort_date);
    }

    let size = analyzer.get_cohort_size(cohort_date);
    assert_eq!(size, expected_size, "Cohort size should match number of users added");
}

#[test]
fn test_multiple_cohorts_isolation() {
    let mut analyzer = CohortAnalyzer::new().unwrap();

    let cohort1_date = Utc::now();
    let cohort2_date = cohort1_date + Duration::days(7);

    // Add users to different cohorts
    for i in 0..5 {
        analyzer.add_user_to_cohort(format!("user_cohort1_{}", i), cohort1_date);
    }

    for i in 0..3 {
        analyzer.add_user_to_cohort(format!("user_cohort2_{}", i), cohort2_date);
    }

    let size1 = analyzer.get_cohort_size(cohort1_date);
    let size2 = analyzer.get_cohort_size(cohort2_date);

    assert_eq!(size1, 5, "Cohort 1 should have 5 users");
    assert_eq!(size2, 3, "Cohort 2 should have 3 users");
}

#[test]
fn test_retention_over_multiple_periods() {
    let mut analyzer = CohortAnalyzer::new().unwrap();

    let cohort_date = Utc::now();

    // Add users
    for i in 0..20 {
        let user_id = format!("user{}", i);
        analyzer.add_user_to_cohort(user_id.clone(), cohort_date);

        // Simulate declining retention over time
        if i < 15 {  // 75% at day 7
            analyzer.record_user_activity(user_id.clone(), cohort_date + Duration::days(7));
        }
        if i < 10 {  // 50% at day 14
            analyzer.record_user_activity(user_id.clone(), cohort_date + Duration::days(14));
        }
        if i < 5 {   // 25% at day 21
            analyzer.record_user_activity(user_id.clone(), cohort_date + Duration::days(21));
        }
    }

    let retention_day7 = analyzer.calculate_retention(cohort_date, Duration::days(7)).unwrap();
    let retention_day14 = analyzer.calculate_retention(cohort_date, Duration::days(14)).unwrap();
    let retention_day21 = analyzer.calculate_retention(cohort_date, Duration::days(21)).unwrap();

    assert!(retention_day7 > retention_day14, "Retention should decline over time");
    assert!(retention_day14 > retention_day21, "Retention should continue declining");
}
