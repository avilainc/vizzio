//! GPU-accelerated quantum rendering example
//!
//! Demonstrates path tracing on GPU with fallback CPU execution

use avx_quantum_render::scene::{Camera, Light, Material, Scene, Surface};
use avx_quantum_render::{GpuPathTracingConfig, GpuPathTracingKernel, GpuRenderBuffer};

fn main() {
    println!("=== AVX Quantum Renderer - GPU Acceleration Demo ===\n");

    // Create simple scene
    let mut scene = Scene::new();

    // Add light
    scene.add_light(Light::point([2.0, 4.0, 2.0], 100.0));

    // Floor
    scene.add_surface(Surface::plane(
        [0.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        Material::lambertian(0.7),
    ));

    // Back wall
    scene.add_surface(Surface::plane(
        [0.0, 0.0, -5.0],
        [0.0, 0.0, 1.0],
        Material::lambertian(0.8),
    ));

    // Sphere
    scene.add_surface(Surface::sphere(
        [0.0, 1.0, -2.0],
        1.0,
        Material::glass(),
    ));

    println!("✓ Scene created with {} objects", scene.num_objects());

    // GPU configuration
    let gpu_config = GpuPathTracingConfig::preview();
    println!("\n🚀 GPU Configuration:");
    println!("  - Total threads: {}", gpu_config.total_threads());
    println!("  - Samples/pixel: {}", gpu_config.samples_per_pixel);
    println!("  - Max depth: {}", gpu_config.max_depth);
    println!("  - Total work items: {}", gpu_config.total_work());

    // Create GPU kernel
    let kernel = GpuPathTracingKernel::new(gpu_config);
    println!("\n✓ GPU kernel created: {}", kernel.name);

    // Create output buffer
    let (width, height) = scene.camera.resolution;
    let mut output = GpuRenderBuffer::new(width, height);
    println!("✓ GPU output buffer: {}x{}", width, height);

    // Trace (CPU fallback for demo)
    println!("\n🎨 Tracing with GPU kernel (CPU fallback)...");
    let start = std::time::Instant::now();
    kernel.trace_cpu_fallback(&scene, &mut output);
    let elapsed = start.elapsed();

    println!("✓ Tracing complete in {:.2}s", elapsed.as_secs_f64());
    println!("  - Throughput: {:.1} M rays/s",
             (output.sample_count as f64 * width as f64 * height as f64) /
             (elapsed.as_secs_f64() * 1e6));

    // Get final image
    let image = output.get_image();
    let mean = image.iter()
        .flat_map(|row| row.iter())
        .sum::<f64>() / (width * height) as f64;

    println!("\n📊 Render Statistics:");
    println!("  - Mean intensity: {:.4}", mean);
    println!("  - Samples accumulated: {}", output.sample_count);

    println!("\n✅ GPU acceleration demo complete!");
}
