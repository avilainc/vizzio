//! String processing example without heap allocation

use avila_alloc::StackString;

fn main() {
    println!("=== String Processing Example ===\n");

    // Build a formatted message on stack
    let mut message = StackString::<256>::new();
    message.push_str("Status: ").unwrap();
    message.push_str("OK").unwrap();
    message.push_str(" | Temperature: ").unwrap();
    message.push_str("25.3°C").unwrap();
    message.push_str(" | Humidity: ").unwrap();
    message.push_str("65%").unwrap();

    println!("Message: {}", message.as_str());
    println!("Length: {} bytes", message.len());
    println!("Capacity: {} bytes\n", message.capacity());

    // Unicode support
    let mut greeting = StackString::<128>::new();
    greeting.push_str("Hello: ").unwrap();
    greeting.push_str("こんにちは, ").unwrap();
    greeting.push_str("Привет, ").unwrap();
    greeting.push_str("مرحبا").unwrap();

    println!("Unicode greeting: {}", greeting.as_str());
    println!("Length: {} bytes (UTF-8 encoded)", greeting.len());
}
