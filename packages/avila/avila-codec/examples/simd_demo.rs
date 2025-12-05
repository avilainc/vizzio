//! SIMD encoding demonstration
//!
//! Demonstrates the use of hardware-accelerated SIMD encoding
//! for hex and base64 operations.

use avila_codec::simd;

fn main() {
    println!("=== SIMD Encoding Demo ===\n");

    // Check SIMD availability
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    {
        let has_avx2 = simd::has_avx2();
        println!("AVX2 support: {}", if has_avx2 { "✓ Available" } else { "✗ Not available" });

        if has_avx2 {
            demo_avx2();
        } else {
            println!("\nAVX2 not available on this CPU. Skipping AVX2 demo.\n");
        }
    }

    #[cfg(target_arch = "aarch64")]
    {
        let has_neon = simd::has_neon();
        println!("NEON support: {}", if has_neon { "✓ Available" } else { "✗ Not available" });

        if has_neon {
            demo_neon();
        }
    }

    #[cfg(not(any(target_arch = "x86", target_arch = "x86_64", target_arch = "aarch64")))]
    {
        println!("SIMD not supported on this architecture.");
    }
}

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
fn demo_avx2() {
    use avila_codec::simd::avx2;

    println!("\n--- AVX2 Demo ---");

    // Test data
    let data = b"Hello, World! This is a SIMD encoding test with AVX2 instructions.";
    println!("\nOriginal data ({} bytes):", data.len());
    println!("{}", String::from_utf8_lossy(data));

    // Hex encoding
    let mut hex_output = vec![0u8; data.len() * 2];
    unsafe {
        let written = avx2::hex_encode_avx2(data, &mut hex_output);
        println!("\nHex encoded ({} bytes):", written);
        println!("{}", String::from_utf8_lossy(&hex_output[..written]));
    }

    // Base64 encoding
    let mut base64_output = vec![0u8; (data.len() + 2) / 3 * 4];
    unsafe {
        let written = avx2::base64_encode_avx2(data, &mut base64_output);
        println!("\nBase64 encoded ({} bytes):", written);
        println!("{}", String::from_utf8_lossy(&base64_output[..written]));
    }

    // Performance comparison hint
    println!("\n💡 AVX2 processes 16 bytes per iteration for hex encoding");
    println!("   This provides significant speedup for large data (>1KB)");
}

#[cfg(target_arch = "aarch64")]
fn demo_neon() {
    use avila_codec::simd::neon;

    println!("\n--- NEON Demo ---");

    // Test data
    let data = b"Hello, World! This is a SIMD encoding test with NEON instructions.";
    println!("\nOriginal data ({} bytes):", data.len());
    println!("{}", String::from_utf8_lossy(data));

    // Hex encoding
    let mut hex_output = vec![0u8; data.len() * 2];
    unsafe {
        let written = neon::hex_encode_neon(data, &mut hex_output);
        println!("\nHex encoded ({} bytes):", written);
        println!("{}", String::from_utf8_lossy(&hex_output[..written]));
    }

    // Base64 encoding
    let mut base64_output = vec![0u8; (data.len() + 2) / 3 * 4];
    unsafe {
        let written = neon::base64_encode_neon(data, &mut base64_output);
        println!("\nBase64 encoded ({} bytes):", written);
        println!("{}", String::from_utf8_lossy(&base64_output[..written]));
    }

    // Performance comparison hint
    println!("\n💡 NEON processes 16 bytes per iteration for hex encoding");
    println!("   Optimized for ARM64 architecture with 128-bit SIMD registers");
}
