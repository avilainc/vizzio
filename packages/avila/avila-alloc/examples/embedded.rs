//! Embedded system example (no_std)
//!
//! Compile with: cargo build --example embedded --no-default-features

#![no_std]
#![no_main]

use avila_alloc::{StackVec, StackString, StaticArena};
use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

// Simulated embedded entry point
#[no_mangle]
pub extern "C" fn _start() -> ! {
    main();
    loop {}
}

fn main() {
    // Sensor readings buffer - no heap allocation!
    let mut readings = StackVec::<f32, 100>::new();

    // Simulate sensor readings
    for i in 0..10 {
        let _ = readings.push(20.5 + i as f32 * 0.1);
    }

    // Device name on stack
    let mut device_name = StackString::<32>::new();
    let _ = device_name.push_str("Sensor-Node-01");

    // Temporary data arena
    let mut arena = StaticArena::<512>::new();

    // Process data using arena allocations
    if let Some(buffer) = arena.alloc_slice::<u8>(64) {
        buffer[0] = 0xFF;
        buffer[1] = 0xAA;
    }

    // All memory is on stack or in static storage
    // No heap allocator required!
}
