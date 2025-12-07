//! Unit tests for Predictive Maintenance module

use avila_analises::industry40::predictive_maintenance::*;
use chrono::{DateTime, Utc, Duration};
use std::collections::HashMap;

#[test]
fn test_maintenance_predictor_creation() {
    let predictor = MaintenancePredictor::new();
    assert!(predictor.is_ok(), "Should create MaintenancePredictor successfully");
}

#[test]
fn test_add_equipment() {
    let mut predictor = MaintenancePredictor::new().unwrap();
    
    let equipment = Equipment {
        equipment_id: "pump_001".to_string(),
        equipment_type: "centrifugal_pump".to_string(),
        location: "building_a".to_string(),
        installation_date: Utc::now() - Duration::days(365),
        last_maintenance: Utc::now() - Duration::days(90),
        operating_hours: 8000.0,
        metadata: HashMap::new(),
    };
    
    let result = predictor.add_equipment(equipment);
    
    assert!(result.is_ok(), "Should add equipment successfully");
}

#[test]
fn test_predict_failure() {
    let mut predictor = MaintenancePredictor::new().unwrap();
    
    let equipment = Equipment {
        equipment_id: "motor_001".to_string(),
        equipment_type: "electric_motor".to_string(),
        location: "production_line_1".to_string(),
        installation_date: Utc::now() - Duration::days(730),
        last_maintenance: Utc::now() - Duration::days(180),
        operating_hours: 15000.0,
        metadata: HashMap::new(),
    };
    
    predictor.add_equipment(equipment).unwrap();
    
    // Add sensor readings indicating wear
    for i in 1..=10 {
        let reading = MaintenanceReading {
            equipment_id: "motor_001".to_string(),
            timestamp: Utc::now() - Duration::days(10 - i),
            vibration: 5.0 + (i as f64 * 0.5), // Increasing vibration
            temperature: 70.0 + (i as f64 * 2.0), // Increasing temperature
            current: 15.0,
            noise_level: 85.0,
        };
        predictor.record_reading(reading).unwrap();
    }
    
    let prediction = predictor.predict_failure("motor_001");
    
    assert!(prediction.is_ok(), "Should predict failure");
    let result = prediction.unwrap();
    assert!(result.risk_score >= 0.0 && result.risk_score <= 1.0, "Risk score should be between 0 and 1");
}

#[test]
fn test_calculate_remaining_useful_life() {
    let mut predictor = MaintenancePredictor::new().unwrap();
    
    let equipment = Equipment {
        equipment_id: "gearbox_001".to_string(),
        equipment_type: "gearbox".to_string(),
        location: "machine_hall".to_string(),
        installation_date: Utc::now() - Duration::days(1825), // 5 years
        last_maintenance: Utc::now() - Duration::days(365),
        operating_hours: 40000.0,
        metadata: HashMap::new(),
    };
    
    predictor.add_equipment(equipment).unwrap();
    
    let rul = predictor.calculate_remaining_useful_life("gearbox_001");
    
    assert!(rul.is_ok(), "Should calculate RUL");
    let days = rul.unwrap();
    assert!(days > 0, "RUL should be positive");
}

#[test]
fn test_schedule_maintenance() {
    let mut predictor = MaintenancePredictor::new().unwrap();
    
    let equipment = Equipment {
        equipment_id: "compressor_001".to_string(),
        equipment_type: "air_compressor".to_string(),
        location: "utility_room".to_string(),
        installation_date: Utc::now() - Duration::days(500),
        last_maintenance: Utc::now() - Duration::days(120),
        operating_hours: 10000.0,
        metadata: HashMap::new(),
    };
    
    predictor.add_equipment(equipment).unwrap();
    
    let scheduled_date = Utc::now() + Duration::days(30);
    let result = predictor.schedule_maintenance("compressor_001", scheduled_date, MaintenanceType::Preventive);
    
    assert!(result.is_ok(), "Should schedule maintenance");
}

#[test]
fn test_maintenance_history() {
    let mut predictor = MaintenancePredictor::new().unwrap();
    
    let equipment = Equipment {
        equipment_id: "conveyor_001".to_string(),
        equipment_type: "conveyor_belt".to_string(),
        location: "warehouse".to_string(),
        installation_date: Utc::now() - Duration::days(1000),
        last_maintenance: Utc::now() - Duration::days(60),
        operating_hours: 20000.0,
        metadata: HashMap::new(),
    };
    
    predictor.add_equipment(equipment).unwrap();
    
    // Schedule multiple maintenance activities
    for i in 1..=5 {
        let date = Utc::now() + Duration::days(30 * i);
        predictor.schedule_maintenance(
            "conveyor_001", 
            date, 
            if i % 2 == 0 { MaintenanceType::Preventive } else { MaintenanceType::Corrective }
        ).unwrap();
    }
    
    let history = predictor.get_maintenance_history("conveyor_001");
    
    assert!(history.is_ok(), "Should retrieve maintenance history");
    let records = history.unwrap();
    assert_eq!(records.len(), 5, "Should have 5 maintenance records");
}

#[test]
fn test_anomaly_detection() {
    let mut predictor = MaintenancePredictor::new().unwrap();
    
    let equipment = Equipment {
        equipment_id: "turbine_001".to_string(),
        equipment_type: "steam_turbine".to_string(),
        location: "power_plant".to_string(),
        installation_date: Utc::now() - Duration::days(3650),
        last_maintenance: Utc::now() - Duration::days(90),
        operating_hours: 80000.0,
        metadata: HashMap::new(),
    };
    
    predictor.add_equipment(equipment).unwrap();
    
    // Normal readings
    for i in 1..=20 {
        let reading = MaintenanceReading {
            equipment_id: "turbine_001".to_string(),
            timestamp: Utc::now() - Duration::hours(20 - i),
            vibration: 3.0,
            temperature: 85.0,
            current: 20.0,
            noise_level: 80.0,
        };
        predictor.record_reading(reading).unwrap();
    }
    
    // Anomalous reading
    let anomaly_reading = MaintenanceReading {
        equipment_id: "turbine_001".to_string(),
        timestamp: Utc::now(),
        vibration: 15.0, // Significantly higher
        temperature: 120.0,
        current: 30.0,
        noise_level: 100.0,
    };
    
    let is_anomaly = predictor.detect_anomaly(&anomaly_reading);
    
    assert!(is_anomaly.is_ok(), "Should detect anomaly");
    assert!(is_anomaly.unwrap(), "Should identify reading as anomalous");
}

#[test]
fn test_health_score_calculation() {
    let mut predictor = MaintenancePredictor::new().unwrap();
    
    let equipment = Equipment {
        equipment_id: "robot_001".to_string(),
        equipment_type: "industrial_robot".to_string(),
        location: "assembly_line".to_string(),
        installation_date: Utc::now() - Duration::days(365),
        last_maintenance: Utc::now() - Duration::days(30),
        operating_hours: 8000.0,
        metadata: HashMap::new(),
    };
    
    predictor.add_equipment(equipment).unwrap();
    
    // Add recent readings
    for i in 1..=5 {
        let reading = MaintenanceReading {
            equipment_id: "robot_001".to_string(),
            timestamp: Utc::now() - Duration::days(5 - i),
            vibration: 4.0,
            temperature: 75.0,
            current: 18.0,
            noise_level: 78.0,
        };
        predictor.record_reading(reading).unwrap();
    }
    
    let health_score = predictor.calculate_health_score("robot_001");
    
    assert!(health_score.is_ok(), "Should calculate health score");
    let score = health_score.unwrap();
    assert!(score >= 0.0 && score <= 100.0, "Health score should be between 0 and 100");
}

#[test]
fn test_prioritize_maintenance() {
    let mut predictor = MaintenancePredictor::new().unwrap();
    
    // Add multiple equipment with varying conditions
    let equipment_ids = vec!["eq_001", "eq_002", "eq_003"];
    
    for (idx, id) in equipment_ids.iter().enumerate() {
        let equipment = Equipment {
            equipment_id: id.to_string(),
            equipment_type: "various".to_string(),
            location: "facility".to_string(),
            installation_date: Utc::now() - Duration::days(365 * (idx + 1) as i64),
            last_maintenance: Utc::now() - Duration::days(180 + (idx * 30) as i64),
            operating_hours: 10000.0 * (idx + 1) as f64,
            metadata: HashMap::new(),
        };
        predictor.add_equipment(equipment).unwrap();
    }
    
    let priority_list = predictor.prioritize_maintenance_tasks();
    
    assert!(priority_list.is_ok(), "Should prioritize maintenance");
    let list = priority_list.unwrap();
    assert!(!list.is_empty(), "Should have prioritized tasks");
}

#[test]
fn test_maintenance_cost_estimation() {
    let mut predictor = MaintenancePredictor::new().unwrap();
    
    let equipment = Equipment {
        equipment_id: "crane_001".to_string(),
        equipment_type: "overhead_crane".to_string(),
        location: "workshop".to_string(),
        installation_date: Utc::now() - Duration::days(2000),
        last_maintenance: Utc::now() - Duration::days(200),
        operating_hours: 30000.0,
        metadata: HashMap::new(),
    };
    
    predictor.add_equipment(equipment).unwrap();
    
    let cost_preventive = predictor.estimate_maintenance_cost("crane_001", MaintenanceType::Preventive);
    let cost_corrective = predictor.estimate_maintenance_cost("crane_001", MaintenanceType::Corrective);
    
    assert!(cost_preventive.is_ok(), "Should estimate preventive maintenance cost");
    assert!(cost_corrective.is_ok(), "Should estimate corrective maintenance cost");
    
    // Corrective maintenance is typically more expensive
    assert!(cost_corrective.unwrap() > cost_preventive.unwrap(), 
            "Corrective maintenance should be more expensive");
}

#[test]
fn test_failure_mode_analysis() {
    let mut predictor = MaintenancePredictor::new().unwrap();
    
    let equipment = Equipment {
        equipment_id: "valve_001".to_string(),
        equipment_type: "control_valve".to_string(),
        location: "pipeline_section_a".to_string(),
        installation_date: Utc::now() - Duration::days(1500),
        last_maintenance: Utc::now() - Duration::days(150),
        operating_hours: 25000.0,
        metadata: HashMap::new(),
    };
    
    predictor.add_equipment(equipment).unwrap();
    
    let failure_modes = predictor.analyze_failure_modes("valve_001");
    
    assert!(failure_modes.is_ok(), "Should analyze failure modes");
}

#[test]
fn test_mtbf_calculation() {
    let mut predictor = MaintenancePredictor::new().unwrap();
    
    let equipment = Equipment {
        equipment_id: "bearing_001".to_string(),
        equipment_type: "roller_bearing".to_string(),
        location: "machine_001".to_string(),
        installation_date: Utc::now() - Duration::days(730),
        last_maintenance: Utc::now() - Duration::days(180),
        operating_hours: 15000.0,
        metadata: HashMap::new(),
    };
    
    predictor.add_equipment(equipment).unwrap();
    
    // Mean Time Between Failures
    let mtbf = predictor.calculate_mtbf("bearing_001");
    
    assert!(mtbf.is_ok(), "Should calculate MTBF");
}

#[test]
fn test_sensor_reading_validation() {
    let predictor = MaintenancePredictor::new().unwrap();
    
    let valid_reading = MaintenanceReading {
        equipment_id: "test_001".to_string(),
        timestamp: Utc::now(),
        vibration: 5.0,
        temperature: 80.0,
        current: 15.0,
        noise_level: 85.0,
    };
    
    let invalid_reading = MaintenanceReading {
        equipment_id: "test_001".to_string(),
        timestamp: Utc::now(),
        vibration: -1.0, // Invalid negative value
        temperature: 300.0, // Unrealistically high
        current: -5.0,
        noise_level: 200.0,
    };
    
    assert!(predictor.validate_reading(&valid_reading), "Should validate correct reading");
    assert!(!predictor.validate_reading(&invalid_reading), "Should reject invalid reading");
}

#[test]
fn test_maintenance_window_optimization() {
    let mut predictor = MaintenancePredictor::new().unwrap();
    
    let equipment = Equipment {
        equipment_id: "press_001".to_string(),
        equipment_type: "hydraulic_press".to_string(),
        location: "stamping_area".to_string(),
        installation_date: Utc::now() - Duration::days(1000),
        last_maintenance: Utc::now() - Duration::days(90),
        operating_hours: 18000.0,
        metadata: HashMap::new(),
    };
    
    predictor.add_equipment(equipment).unwrap();
    
    let optimal_window = predictor.find_optimal_maintenance_window("press_001");
    
    assert!(optimal_window.is_ok(), "Should find optimal maintenance window");
}
