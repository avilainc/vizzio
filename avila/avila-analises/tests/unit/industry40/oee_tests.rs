//! Unit tests for Overall Equipment Effectiveness (OEE) module

use avila_analises::industry40::oee::*;
use chrono::{DateTime, Utc, Duration};

#[test]
fn test_oee_calculator_creation() {
    let calculator = OEECalculator::new();
    assert!(calculator.is_ok(), "Should create OEECalculator successfully");
}

#[test]
fn test_calculate_availability() {
    let calculator = OEECalculator::new().unwrap();

    let planned_production_time = 480.0; // 8 hours in minutes
    let downtime = 48.0; // 48 minutes of downtime

    let availability = calculator.calculate_availability(planned_production_time, downtime);

    assert!(availability.is_ok(), "Should calculate availability");
    let av = availability.unwrap();
    assert!((av - 0.9).abs() < 0.001, "Availability should be 90%"); // (480-48)/480 = 0.9
}

#[test]
fn test_calculate_performance() {
    let calculator = OEECalculator::new().unwrap();

    let actual_output = 400.0; // units produced
    let ideal_cycle_time = 1.0; // 1 minute per unit
    let operating_time = 480.0; // 8 hours

    let performance = calculator.calculate_performance(actual_output, ideal_cycle_time, operating_time);

    assert!(performance.is_ok(), "Should calculate performance");
    let perf = performance.unwrap();
    assert!((perf - 0.833).abs() < 0.01, "Performance should be ~83.3%"); // 400/(480/1) = 0.833
}

#[test]
fn test_calculate_quality() {
    let calculator = OEECalculator::new().unwrap();

    let good_units = 380.0;
    let total_units = 400.0;

    let quality = calculator.calculate_quality(good_units, total_units);

    assert!(quality.is_ok(), "Should calculate quality");
    let q = quality.unwrap();
    assert!((q - 0.95).abs() < 0.001, "Quality should be 95%"); // 380/400 = 0.95
}

#[test]
fn test_calculate_oee() {
    let calculator = OEECalculator::new().unwrap();

    let availability = 0.90; // 90%
    let performance = 0.85;  // 85%
    let quality = 0.95;      // 95%

    let oee = calculator.calculate_oee(availability, performance, quality);

    assert!(oee.is_ok(), "Should calculate OEE");
    let oee_value = oee.unwrap();

    let expected = 0.90 * 0.85 * 0.95;
    assert!((oee_value - expected).abs() < 0.001, "OEE should be product of A*P*Q");
}

#[test]
fn test_world_class_oee() {
    let calculator = OEECalculator::new().unwrap();

    // World class OEE targets
    let availability = 0.90;
    let performance = 0.95;
    let quality = 0.99;

    let oee = calculator.calculate_oee(availability, performance, quality).unwrap();

    // World class OEE is typically >= 85%
    assert!(oee >= 0.85, "World class OEE should be >= 85%");
}

#[test]
fn test_poor_oee() {
    let calculator = OEECalculator::new().unwrap();

    // Poor performing equipment
    let availability = 0.60;
    let performance = 0.70;
    let quality = 0.80;

    let oee = calculator.calculate_oee(availability, performance, quality).unwrap();

    // Poor OEE is typically < 65%
    assert!(oee < 0.65, "Poor OEE should be < 65%");
}

#[test]
fn test_record_production_cycle() {
    let mut calculator = OEECalculator::new().unwrap();

    let cycle = ProductionCycle {
        machine_id: "machine_001".to_string(),
        start_time: Utc::now() - Duration::hours(8),
        end_time: Utc::now(),
        planned_time: 480.0,
        downtime: 30.0,
        units_produced: 450.0,
        good_units: 430.0,
        ideal_cycle_time: 1.0,
    };

    let result = calculator.record_cycle(cycle);

    assert!(result.is_ok(), "Should record production cycle");
}

#[test]
fn test_get_machine_oee() {
    let mut calculator = OEECalculator::new().unwrap();

    let cycle = ProductionCycle {
        machine_id: "machine_002".to_string(),
        start_time: Utc::now() - Duration::hours(8),
        end_time: Utc::now(),
        planned_time: 480.0,
        downtime: 48.0,
        units_produced: 400.0,
        good_units: 380.0,
        ideal_cycle_time: 1.0,
    };

    calculator.record_cycle(cycle).unwrap();

    let oee = calculator.get_machine_oee("machine_002");

    assert!(oee.is_ok(), "Should calculate machine OEE");
    let oee_value = oee.unwrap();
    assert!(oee_value > 0.0 && oee_value <= 1.0, "OEE should be between 0 and 1");
}

#[test]
fn test_multiple_cycles_average() {
    let mut calculator = OEECalculator::new().unwrap();

    // Record multiple production cycles for same machine
    for i in 1..=5 {
        let cycle = ProductionCycle {
            machine_id: "machine_003".to_string(),
            start_time: Utc::now() - Duration::hours(8 * i),
            end_time: Utc::now() - Duration::hours(8 * (i - 1)),
            planned_time: 480.0,
            downtime: 20.0 + (i as f64 * 5.0),
            units_produced: 450.0,
            good_units: 430.0,
            ideal_cycle_time: 1.0,
        };
        calculator.record_cycle(cycle).unwrap();
    }

    let avg_oee = calculator.get_average_oee("machine_003");

    assert!(avg_oee.is_ok(), "Should calculate average OEE");
}

#[test]
fn test_oee_trend_analysis() {
    let mut calculator = OEECalculator::new().unwrap();

    let machine_id = "machine_004";

    // Record improving trend
    for i in 1..=10 {
        let improvement_factor = 1.0 - (i as f64 * 0.02); // Decreasing downtime

        let cycle = ProductionCycle {
            machine_id: machine_id.to_string(),
            start_time: Utc::now() - Duration::days(11 - i),
            end_time: Utc::now() - Duration::days(10 - i),
            planned_time: 480.0,
            downtime: 60.0 * improvement_factor,
            units_produced: 450.0,
            good_units: 430.0 + (i as f64),
            ideal_cycle_time: 1.0,
        };
        calculator.record_cycle(cycle).unwrap();
    }

    let trend = calculator.analyze_oee_trend(machine_id);

    assert!(trend.is_ok(), "Should analyze OEE trend");
}

#[test]
fn test_identify_bottleneck() {
    let mut calculator = OEECalculator::new().unwrap();

    // Machine with low availability (bottleneck)
    let cycle_low_availability = ProductionCycle {
        machine_id: "machine_005".to_string(),
        start_time: Utc::now() - Duration::hours(8),
        end_time: Utc::now(),
        planned_time: 480.0,
        downtime: 240.0, // 50% downtime - major bottleneck
        units_produced: 200.0,
        good_units: 195.0,
        ideal_cycle_time: 1.0,
    };

    calculator.record_cycle(cycle_low_availability).unwrap();

    let bottleneck = calculator.identify_bottleneck("machine_005");

    assert!(bottleneck.is_ok(), "Should identify bottleneck");
    let issue = bottleneck.unwrap();
    assert_eq!(issue, BottleneckType::Availability, "Should identify availability as bottleneck");
}

#[test]
fn test_oee_bounds() {
    let calculator = OEECalculator::new().unwrap();

    // Test that OEE is bounded between 0 and 1
    let oee_perfect = calculator.calculate_oee(1.0, 1.0, 1.0).unwrap();
    assert_eq!(oee_perfect, 1.0, "Perfect OEE should be 1.0");

    let oee_zero = calculator.calculate_oee(0.0, 0.0, 0.0).unwrap();
    assert_eq!(oee_zero, 0.0, "Zero OEE should be 0.0");
}

#[test]
fn test_six_big_losses() {
    let calculator = OEECalculator::new().unwrap();

    let cycle = ProductionCycle {
        machine_id: "machine_006".to_string(),
        start_time: Utc::now() - Duration::hours(8),
        end_time: Utc::now(),
        planned_time: 480.0,
        downtime: 60.0,
        units_produced: 350.0,
        good_units: 320.0,
        ideal_cycle_time: 1.0,
    };

    let losses = calculator.analyze_six_big_losses(&cycle);

    assert!(losses.is_ok(), "Should analyze six big losses");
    // Six big losses: Equipment Failure, Setup/Adjustments, Idling/Minor Stops,
    //                 Reduced Speed, Process Defects, Reduced Yield
}

#[test]
fn test_downtime_categorization() {
    let mut calculator = OEECalculator::new().unwrap();

    let cycle = ProductionCycle {
        machine_id: "machine_007".to_string(),
        start_time: Utc::now() - Duration::hours(8),
        end_time: Utc::now(),
        planned_time: 480.0,
        downtime: 90.0,
        units_produced: 380.0,
        good_units: 365.0,
        ideal_cycle_time: 1.0,
    };

    calculator.record_cycle(cycle).unwrap();

    let downtime_analysis = calculator.categorize_downtime("machine_007");

    assert!(downtime_analysis.is_ok(), "Should categorize downtime");
}

#[test]
fn test_production_efficiency_score() {
    let calculator = OEECalculator::new().unwrap();

    let cycle = ProductionCycle {
        machine_id: "machine_008".to_string(),
        start_time: Utc::now() - Duration::hours(8),
        end_time: Utc::now(),
        planned_time: 480.0,
        downtime: 40.0,
        units_produced: 420.0,
        good_units: 410.0,
        ideal_cycle_time: 1.0,
    };

    let efficiency = calculator.calculate_production_efficiency(&cycle);

    assert!(efficiency.is_ok(), "Should calculate production efficiency");
    let score = efficiency.unwrap();
    assert!(score > 0.0 && score <= 1.0, "Efficiency should be between 0 and 1");
}
