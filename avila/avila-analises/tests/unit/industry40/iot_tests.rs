//! Unit tests for IoT device management module

use avila_analises::industry40::iot::*;
use avila_analises::models::*;
use std::collections::HashMap;

#[test]
fn test_iot_manager_creation() {
    let manager = IoTManager::new();
    assert!(manager.is_ok(), "Should create IoTManager successfully");
}

#[test]
fn test_register_device() {
    let mut manager = IoTManager::new().unwrap();

    let device = IoTDevice {
        device_id: "sensor001".to_string(),
        device_type: "temperature".to_string(),
        location: "factory_floor_a".to_string(),
        status: DeviceStatus::Active,
        last_reading: None,
        metadata: HashMap::new(),
    };

    let result = manager.register_device(device);

    assert!(result.is_ok(), "Should register device successfully");
}

#[test]
fn test_get_device() {
    let mut manager = IoTManager::new().unwrap();

    let device = IoTDevice {
        device_id: "sensor002".to_string(),
        device_type: "pressure".to_string(),
        location: "line_1".to_string(),
        status: DeviceStatus::Active,
        last_reading: None,
        metadata: HashMap::new(),
    };

    manager.register_device(device.clone()).unwrap();

    let retrieved = manager.get_device("sensor002");

    assert!(retrieved.is_some(), "Should retrieve registered device");
    let retrieved_device = retrieved.unwrap();
    assert_eq!(retrieved_device.device_id, "sensor002");
    assert_eq!(retrieved_device.device_type, "pressure");
}

#[test]
fn test_update_device_status() {
    let mut manager = IoTManager::new().unwrap();

    let device = IoTDevice {
        device_id: "sensor003".to_string(),
        device_type: "vibration".to_string(),
        location: "machine_5".to_string(),
        status: DeviceStatus::Active,
        last_reading: None,
        metadata: HashMap::new(),
    };

    manager.register_device(device).unwrap();

    let result = manager.update_device_status("sensor003", DeviceStatus::Maintenance);

    assert!(result.is_ok(), "Should update device status");

    let updated = manager.get_device("sensor003").unwrap();
    assert_eq!(updated.status, DeviceStatus::Maintenance);
}

#[test]
fn test_record_sensor_reading() {
    let mut manager = IoTManager::new().unwrap();

    let device = IoTDevice {
        device_id: "sensor004".to_string(),
        device_type: "temperature".to_string(),
        location: "warehouse".to_string(),
        status: DeviceStatus::Active,
        last_reading: None,
        metadata: HashMap::new(),
    };

    manager.register_device(device).unwrap();

    let reading = SensorReading {
        device_id: "sensor004".to_string(),
        timestamp: chrono::Utc::now(),
        value: 25.5,
        unit: "celsius".to_string(),
        quality: ReadingQuality::Good,
    };

    let result = manager.record_reading(reading);

    assert!(result.is_ok(), "Should record sensor reading");

    let device = manager.get_device("sensor004").unwrap();
    assert!(device.last_reading.is_some(), "Device should have last reading");
}

#[test]
fn test_get_devices_by_type() {
    let mut manager = IoTManager::new().unwrap();

    // Register multiple temperature sensors
    for i in 1..=5 {
        let device = IoTDevice {
            device_id: format!("temp{}", i),
            device_type: "temperature".to_string(),
            location: format!("zone_{}", i),
            status: DeviceStatus::Active,
            last_reading: None,
            metadata: HashMap::new(),
        };
        manager.register_device(device).unwrap();
    }

    // Register a pressure sensor
    let pressure_device = IoTDevice {
        device_id: "pressure1".to_string(),
        device_type: "pressure".to_string(),
        location: "zone_1".to_string(),
        status: DeviceStatus::Active,
        last_reading: None,
        metadata: HashMap::new(),
    };
    manager.register_device(pressure_device).unwrap();

    let temp_devices = manager.get_devices_by_type("temperature");

    assert_eq!(temp_devices.len(), 5, "Should have 5 temperature sensors");
}

#[test]
fn test_get_devices_by_location() {
    let mut manager = IoTManager::new().unwrap();

    let location = "production_line_a";

    for i in 1..=3 {
        let device = IoTDevice {
            device_id: format!("sensor{}", i),
            device_type: "various".to_string(),
            location: location.to_string(),
            status: DeviceStatus::Active,
            last_reading: None,
            metadata: HashMap::new(),
        };
        manager.register_device(device).unwrap();
    }

    let devices = manager.get_devices_by_location(location);

    assert_eq!(devices.len(), 3, "Should have 3 devices in location");
}

#[test]
fn test_device_health_check() {
    let mut manager = IoTManager::new().unwrap();

    let device = IoTDevice {
        device_id: "sensor005".to_string(),
        device_type: "humidity".to_string(),
        location: "storage".to_string(),
        status: DeviceStatus::Active,
        last_reading: None,
        metadata: HashMap::new(),
    };

    manager.register_device(device).unwrap();

    let health = manager.check_device_health("sensor005");

    assert!(health.is_ok(), "Should check device health");
}

#[test]
fn test_multiple_readings_for_device() {
    let mut manager = IoTManager::new().unwrap();

    let device = IoTDevice {
        device_id: "sensor006".to_string(),
        device_type: "temperature".to_string(),
        location: "lab".to_string(),
        status: DeviceStatus::Active,
        last_reading: None,
        metadata: HashMap::new(),
    };

    manager.register_device(device).unwrap();

    // Record multiple readings
    for i in 1..=10 {
        let reading = SensorReading {
            device_id: "sensor006".to_string(),
            timestamp: chrono::Utc::now(),
            value: 20.0 + i as f64,
            unit: "celsius".to_string(),
            quality: ReadingQuality::Good,
        };
        manager.record_reading(reading).unwrap();
    }

    let device = manager.get_device("sensor006").unwrap();
    assert!(device.last_reading.is_some(), "Should have last reading after multiple updates");
}

#[test]
fn test_deactivate_device() {
    let mut manager = IoTManager::new().unwrap();

    let device = IoTDevice {
        device_id: "sensor007".to_string(),
        device_type: "flow".to_string(),
        location: "pipeline_1".to_string(),
        status: DeviceStatus::Active,
        last_reading: None,
        metadata: HashMap::new(),
    };

    manager.register_device(device).unwrap();

    let result = manager.update_device_status("sensor007", DeviceStatus::Inactive);

    assert!(result.is_ok(), "Should deactivate device");

    let device = manager.get_device("sensor007").unwrap();
    assert_eq!(device.status, DeviceStatus::Inactive);
}

#[test]
fn test_device_metadata() {
    let mut manager = IoTManager::new().unwrap();

    let mut metadata = HashMap::new();
    metadata.insert("manufacturer".to_string(), "Siemens".to_string());
    metadata.insert("model".to_string(), "S7-1200".to_string());
    metadata.insert("firmware".to_string(), "v2.5".to_string());

    let device = IoTDevice {
        device_id: "plc001".to_string(),
        device_type: "plc".to_string(),
        location: "control_room".to_string(),
        status: DeviceStatus::Active,
        last_reading: None,
        metadata: metadata.clone(),
    };

    manager.register_device(device).unwrap();

    let retrieved = manager.get_device("plc001").unwrap();
    assert_eq!(retrieved.metadata.get("manufacturer"), Some(&"Siemens".to_string()));
    assert_eq!(retrieved.metadata.get("model"), Some(&"S7-1200".to_string()));
}

#[test]
fn test_get_all_devices() {
    let mut manager = IoTManager::new().unwrap();

    for i in 1..=7 {
        let device = IoTDevice {
            device_id: format!("device{}", i),
            device_type: "sensor".to_string(),
            location: "facility".to_string(),
            status: DeviceStatus::Active,
            last_reading: None,
            metadata: HashMap::new(),
        };
        manager.register_device(device).unwrap();
    }

    let all_devices = manager.get_all_devices();

    assert_eq!(all_devices.len(), 7, "Should have 7 devices registered");
}

#[test]
fn test_reading_quality_levels() {
    let mut manager = IoTManager::new().unwrap();

    let device = IoTDevice {
        device_id: "sensor008".to_string(),
        device_type: "quality_test".to_string(),
        location: "test_area".to_string(),
        status: DeviceStatus::Active,
        last_reading: None,
        metadata: HashMap::new(),
    };

    manager.register_device(device).unwrap();

    // Test different quality levels
    let qualities = vec![
        ReadingQuality::Good,
        ReadingQuality::Uncertain,
        ReadingQuality::Bad,
    ];

    for quality in qualities {
        let reading = SensorReading {
            device_id: "sensor008".to_string(),
            timestamp: chrono::Utc::now(),
            value: 100.0,
            unit: "test".to_string(),
            quality: quality.clone(),
        };

        let result = manager.record_reading(reading);
        assert!(result.is_ok(), "Should accept all quality levels");
    }
}
