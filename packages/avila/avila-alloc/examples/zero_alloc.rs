//! Performance-critical code example showing zero-allocation patterns

use avila_alloc::{StackVec, StackString, StackRing, StaticArena};

fn main() {
    println!("=== Zero-Allocation Patterns ===\n");

    // Pattern 1: Fixed-size buffer processing
    process_sensor_data();

    // Pattern 2: String formatting without heap
    format_messages();

    // Pattern 3: Sliding window computation
    sliding_window_average();

    // Pattern 4: Temporary computation with arena
    matrix_operations();
}

fn process_sensor_data() {
    println!("1. Sensor Data Processing (no heap)");

    let mut readings = StackVec::<f32, 100>::new();

    // Simulate sensor readings
    for i in 0..50 {
        let value = 20.0 + (i as f32 * 0.1);
        readings.push(value).unwrap();
    }

    // Compute statistics without allocation
    let sum: f32 = readings.iter().sum();
    let avg = sum / readings.len() as f32;
    let max = readings.iter().fold(f32::MIN, |a, &b| a.max(b));
    let min = readings.iter().fold(f32::MAX, |a, &b| a.min(b));

    println!("   Readings: {} samples", readings.len());
    println!("   Average: {:.2}°C", avg);
    println!("   Range: {:.2}°C - {:.2}°C\n", min, max);
}

fn format_messages() {
    println!("2. Message Formatting (stack only)");

    let mut message = StackString::<256>::new();

    // Build JSON-like message without heap allocation
    message.push_str("{\"status\":\"").unwrap();
    message.push_str("online").unwrap();
    message.push_str("\",\"temp\":").unwrap();
    message.push_str("25.3").unwrap();
    message.push_str(",\"humidity\":").unwrap();
    message.push_str("65").unwrap();
    message.push_str("}").unwrap();

    println!("   Message: {}", message.as_str());
    println!("   Length: {} bytes (no heap!)\n", message.len());
}

fn sliding_window_average() {
    println!("3. Sliding Window Average (ring buffer)");

    let mut window = StackRing::<f32, 10>::new();
    let data = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0,
                11.0, 12.0, 13.0, 14.0, 15.0];

    for (i, &value) in data.iter().enumerate() {
        window.push(value);

        // Compute moving average
        let sum: f32 = window.iter().sum();
        let avg = sum / window.len() as f32;

        if i >= 9 {
            println!("   Window[{}]: avg = {:.2}", i - 9, avg);
        }
    }
    println!();
}

fn matrix_operations() {
    println!("4. Matrix Operations (arena)");

    let mut arena = StaticArena::<4096>::new();

    // Allocate matrices
    let matrix_a = arena.alloc_slice::<f32>(16).unwrap();
    let matrix_b = arena.alloc_slice::<f32>(16).unwrap();
    let result = arena.alloc_slice::<f32>(16).unwrap();

    // Initialize matrices (4x4)
    for i in 0..16 {
        matrix_a[i] = i as f32;
        matrix_b[i] = 1.0;
        result[i] = 0.0;
    }

    // Matrix addition (simplified)
    for i in 0..16 {
        result[i] = matrix_a[i] + matrix_b[i];
    }

    println!("   Matrix A + Matrix B (first 4 elements):");
    println!("   {:?}", &result[0..4]);
    println!("   Arena used: {} bytes\n", arena.used());

    // Arena memory automatically reclaimed
}

// Bonus: Real-time filtering
fn apply_filter(input: &[f32], output: &mut StackVec<f32, 1024>, cutoff: f32) {
    for &sample in input {
        if sample >= cutoff {
            let _ = output.push(sample);
        }
    }
}
