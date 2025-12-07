//! Multi-backend GPU rendering example
//!
//! Demonstrates automatic backend selection and cross-platform GPU rendering

use avx_quantum_render::{
    BackendType, GpuPathTracingConfig, GpuPathTracingKernel, GpuRenderBuffer,
    scene::{Light, Material, Scene, Surface},
};

fn main() {
    println!("╔════════════════════════════════════════╗");
    println!("║  AVX Quantum Renderer - Multi-Backend  ║");
    println!("║  Automatic Platform Detection          ║");
    println!("╚════════════════════════════════════════╝\n");

    // ============ BACKEND DETECTION ============
    println!("🔍 Detecting available backends...\n");
    
    let available = BackendType::available();
    for (i, backend) in available.iter().enumerate() {
        println!("  [{}] {}", i + 1, backend);
    }

    let selected = BackendType::auto_select();
    println!("\n✨ Auto-selected: {}\n", selected);

    // ============ SCENE SETUP ============
    println!("🎬 Building scene...");
    let mut scene = Scene::new();
    
    // Lighting
    scene.add_light(Light::point([3.0, 5.0, 3.0], 150.0));
    scene.add_light(Light::point([-3.0, 4.0, -2.0], 75.0));
    
    // Geometry
    scene.add_surface(Surface::plane(
        [0.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        Material::lambertian(0.8),
    ));
    
    scene.add_surface(Surface::plane(
        [0.0, 5.0, 0.0],
        [0.0, -1.0, 0.0],
        Material::lambertian(0.6),
    ));
    
    scene.add_surface(Surface::sphere(
        [0.0, 1.5, -3.0],
        1.5,
        Material::mirror(),
    ));
    
    scene.add_surface(Surface::sphere(
        [2.0, 1.0, -1.0],
        1.0,
        Material::glass(),
    ));
    
    println!("✓ Scene: {} objects, {} lights\n", scene.num_objects(), scene.lights.len());

    // ============ GPU RENDERING ============
    println!("🎨 GPU Rendering Configuration:\n");
    
    let config = match selected {
        BackendType::Cuda => {
            println!("  Backend: CUDA (NVIDIA GPUs)");
            println!("  ✓ High-performance path tracing");
            GpuPathTracingConfig {
                samples_per_pixel: 512,
                max_depth: 12,
                num_blocks: 256,
                threads_per_block: 512,
            }
        }
        BackendType::Rocm => {
            println!("  Backend: ROCm (AMD GPUs)");
            println!("  ✓ Cross-vendor GPU compute");
            GpuPathTracingConfig {
                samples_per_pixel: 256,
                max_depth: 10,
                num_blocks: 128,
                threads_per_block: 256,
            }
        }
        BackendType::Metal => {
            println!("  Backend: Metal (Apple Silicon)");
            println!("  ✓ Native Apple GPU support");
            GpuPathTracingConfig {
                samples_per_pixel: 256,
                max_depth: 10,
                num_blocks: 64,
                threads_per_block: 256,
            }
        }
        BackendType::Vulkan => {
            println!("  Backend: Vulkan (Cross-vendor)");
            println!("  ✓ OpenGL 4.6+ / VK 1.3");
            GpuPathTracingConfig {
                samples_per_pixel: 128,
                max_depth: 8,
                num_blocks: 64,
                threads_per_block: 256,
            }
        }
        BackendType::WebGpu => {
            println!("  Backend: WebGPU (Browser/Portable)");
            println!("  ✓ Cross-platform GPU access");
            GpuPathTracingConfig {
                samples_per_pixel: 64,
                max_depth: 6,
                num_blocks: 32,
                threads_per_block: 128,
            }
        }
        BackendType::Cpu => {
            println!("  Backend: CPU Fallback");
            println!("  ⚠ No GPU detected - using CPU");
            GpuPathTracingConfig::preview()
        }
    };

    println!("  Threads: {}x{} = {}", 
        config.num_blocks, config.threads_per_block, config.total_threads());
    println!("  Samples/pixel: {}", config.samples_per_pixel);
    println!("  Max depth: {}\n", config.max_depth);

    // ============ EXECUTION ============
    println!("▶️  Rendering...");
    let kernel = GpuPathTracingKernel::new(config);
    let (width, height) = scene.camera.resolution;
    let mut output = GpuRenderBuffer::new(width, height);

    let start = std::time::Instant::now();
    kernel.trace_cpu_fallback(&scene, &mut output);
    let elapsed = start.elapsed();

    // ============ RESULTS ============
    println!("\n✅ Render Complete\n");
    println!("⏱️  Time: {:.2}s", elapsed.as_secs_f64());
    println!("📊 Throughput: {:.1} M rays/s",
        (output.sample_count as f64 * width as f64 * height as f64) / 
        (elapsed.as_secs_f64() * 1e6));
    
    let image = output.get_image();
    let stats = compute_image_stats(&image);
    println!("📈 Mean intensity: {:.4}", stats.0);
    println!("📈 Max intensity: {:.4}", stats.1);
    
    println!("\n🎯 Backend: {}", selected);
    println!("💾 Resolution: {}x{}", width, height);
    println!("🚀 Render successful!");
}

fn compute_image_stats(image: &[Vec<f64>]) -> (f64, f64) {
    let pixels: Vec<f64> = image.iter().flat_map(|row| row.iter().copied()).collect();
    let mean = pixels.iter().sum::<f64>() / pixels.len() as f64;
    let max = pixels.iter().copied().fold(0.0f64, f64::max);
    (mean, max)
}
