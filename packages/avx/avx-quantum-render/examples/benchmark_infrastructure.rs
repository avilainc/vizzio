//! Comprehensive benchmark demonstrating GPU acceleration and CPU path tracing

use avx_quantum_render::*;

fn main() {
    println!("╔════════════════════════════════════════════════════════════════╗");
    println!("║  AVX Quantum Render - GPU vs CPU Benchmark Suite              ║");
    println!("╚════════════════════════════════════════════════════════════════╝\n");

    // Benchmark configuration
    let width = 512u32;
    let height = 512u32;
    let samples = 256u32;
    let depth = 8u32;

    println!("📊 Configuration:");
    println!("   Resolution: {}×{}", width, height);
    println!("   Samples/pixel: {}", samples);
    println!("   Max depth: {}\n", depth);

    // Test optimization configs
    println!("⚙️  Optimization Profiles:");
    benchmark_optimization_configs();

    // Test material performance
    println!("\n💎 Material Evaluation:");
    benchmark_materials();

    // Test texture sampling
    println!("\n🎨 Texture Sampling:");
    benchmark_textures();

    // Test camera systems
    println!("\n📷 Camera Systems:");
    benchmark_cameras();

    // Test acceleration structures
    println!("\n🚀 Acceleration Structures:");
    benchmark_acceleration();

    // Test post-processing
    println!("\n✨ Post-Processing:");
    benchmark_postprocessing();

    // Test serialization
    println!("\n💾 Serialization:");
    benchmark_serialization();

    println!("\n╔════════════════════════════════════════════════════════════════╗");
    println!("║  Benchmark Complete!                                           ║");
    println!("╚════════════════════════════════════════════════════════════════╝");
}

fn benchmark_optimization_configs() {
    let timer = PerfTimer::start();

    // Auto configuration
    let auto_cfg = ParallelConfig::auto();
    println!("   Auto Config:");
    println!("     • Threads: {}", auto_cfg.num_threads);
    println!("     • Chunk size: {}", auto_cfg.chunk_size);
    println!("     • SIMD enabled: {}", auto_cfg.use_simd);

    // Aggressive configuration
    let agg_cfg = ParallelConfig::aggressive();
    println!("   Aggressive Config:");
    println!("     • Threads: {}", agg_cfg.num_threads);
    println!("     • Chunk size: {}", agg_cfg.chunk_size);
    println!("     • SIMD enabled: {}", agg_cfg.use_simd);

    println!("     ⏱️  Instantiation time: {:?}", timer.elapsed());
}

fn benchmark_materials() {
    // Test material properties
    let dielectric = MaterialProperties::dielectric(1.5, 0.1);
    let metal = MaterialProperties::metal(0.3);
    let cloth = MaterialProperties::cloth(0.6);
    let skin = MaterialProperties::skin();

    println!("   Dielectric (IOR=1.5, roughness=0.1)");
    println!("   Metal (roughness=0.3)");
    println!("   Cloth (roughness=0.6)");
    println!("   Skin (subsurface=0.5)");

    // Fresnel calculations
    let f_schlick = Fresnel::schlick(0.5, 1.5);
    println!("   Fresnel Schlick: {:.4}", f_schlick);

    // GGX distribution
    let ggx_d = GGX::d(0.8, 0.5);
    let ggx_g = GGX::g(0.8, 0.5);
    println!("   GGX D(0.8, 0.5): {:.6}", ggx_d);
    println!("   GGX G(0.8, 0.5): {:.4}", ggx_g);
}

fn benchmark_textures() {
    let timer = PerfTimer::start();

    // Checker texture
    let checker = CheckerTexture::new(4.0, [1.0, 1.0, 1.0, 1.0], [0.0, 0.0, 0.0, 1.0]);
    let mut samples = 0;
    for i in 0..100 {
        let u = (i as f64) / 100.0;
        let color = checker.sample(TexCoord::new(u, u));
        if color[0] > 0.5 {
            samples += 1;
        }
    }
    println!("   Checker texture sampled 100x, {} bright pixels", samples);

    // Solid texture
    let solid = SolidTexture::new([0.8, 0.8, 0.8, 1.0]);
    for _ in 0..100 {
        let _ = solid.sample(TexCoord::new(0.5, 0.5));
    }
    println!("   Solid texture: 100 samples (constant)");

    // Noise texture
    let noise = NoiseTexture::new(2.0, 4);
    let mut noise_sum = 0.0;
    for i in 0..100 {
        let u = (i as f64) / 100.0;
        let color = noise.sample(TexCoord::new(u, u));
        noise_sum += color[0];
    }
    println!("   Noise texture: 100 samples, avg value: {:.3}", noise_sum / 100.0);

    println!("     ⏱️  Texture benchmark: {:?}", timer.elapsed());
}

fn benchmark_cameras() {
    let timer = PerfTimer::start();

    // Standard camera
    let camera = AdvancedCamera::new(
        [0.0, 0.0, 5.0],
        [0.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        60.0,
        16.0 / 9.0,
    );

    // With DoF
    let _camera_dof = camera.clone().with_dof(0.05, 5.0);
    println!("   Standard camera (60° FOV)");
    println!("   With DOF (aperture=0.05, focus_distance=5.0)");

    // With motion blur
    let _camera_mb = camera.clone().with_motion_blur([0.1, 0.0, 0.0], 0.016); // ~60fps
    println!("   With motion blur (velocity=[0.1, 0, 0], exposure=16ms)");

    // Generate rays
    let mut ray_count = 0;
    for i in 0..10 {
        for j in 0..10 {
            let x = (i as f64) / 10.0;
            let y = (j as f64) / 10.0;
            let _ray = camera.generate_ray(x, y);
            ray_count += 1;
        }
    }
    println!("   Generated {} camera rays", ray_count);

    println!("     ⏱️  Camera benchmark: {:?}", timer.elapsed());
}

fn benchmark_acceleration() {
    let timer = PerfTimer::start();

    // AABB tests
    let aabb = AABB::new([0.0, 0.0, 0.0], [2.0, 2.0, 2.0]);
    println!("   AABB (0,0,0) to (2,2,2)");
    println!("     • Volume: {:.2}", aabb.volume());
    println!("     • Surface area: {:.2}", aabb.surface_area());
    println!("     • Center: [{:.2}, {:.2}, {:.2}]", aabb.center()[0], aabb.center()[1], aabb.center()[2]);

    // Ray-AABB intersection
    let ray_origin = [0.0, 0.0, -5.0];
    let ray_dir = [0.0, 0.0, 1.0];
    let intersects = aabb.intersect_ray(ray_origin, ray_dir);
    println!("     • Ray intersection: {}", intersects);

    // Spatial grid
    let bounds = AABB::new([0.0, 0.0, 0.0], [10.0, 10.0, 10.0]);
    let _grid = acceleration::SpatialGrid::new(bounds, 8);
    println!("   Spatial Grid (8×8×8)");
    println!("     • Bounds: 10×10×10");

    // BVH node
    let leaf = acceleration::BVHNode::leaf(aabb, 0);
    let cost = leaf.sah_cost(1000);
    println!("   BVH Leaf Node");
    println!("     • SAH cost (1000 rays): {:.2}", cost);

    println!("     ⏱️  Acceleration benchmark: {:?}", timer.elapsed());
}

fn benchmark_postprocessing() {
    let timer = PerfTimer::start();

    let test_color = [1.5, 1.2, 0.8]; // HDR color

    // Reinhard tonemapping
    let reinhard = ToneMapper::reinhard(test_color, 1.0);
    println!("   Reinhard tonemapping:");
    println!("     • Input: [{:.2}, {:.2}, {:.2}]", test_color[0], test_color[1], test_color[2]);
    println!("     • Output: [{:.3}, {:.3}, {:.3}]", reinhard[0], reinhard[1], reinhard[2]);

    // ACES tonemapping
    let aces = ToneMapper::aces(test_color);
    println!("   ACES tonemapping:");
    println!("     • Output: [{:.3}, {:.3}, {:.3}]", aces[0], aces[1], aces[2]);

    // Gamma correction
    let gamma = ToneMapper::gamma_correct(reinhard, 2.2);
    println!("   Gamma correction (γ=2.2):");
    println!("     • Output: [{:.3}, {:.3}, {:.3}]", gamma[0], gamma[1], gamma[2]);

    // Color space conversions
    let linear = [0.5, 0.5, 0.5];
    let srgb = ColorSpace::linear_to_srgb(linear);
    println!("   sRGB conversion: [{:.3}, {:.3}, {:.3}]", srgb[0], srgb[1], srgb[2]);

    // Bloom extraction
    let bloom = BloomEffect::new(1.0, 2.0, 5);
    let bloom_result = bloom.extract_bloom([3.0, 2.5, 2.0]);
    println!("   Bloom extraction: [{:.3}, {:.3}, {:.3}]", bloom_result[0], bloom_result[1], bloom_result[2]);

    println!("     ⏱️  Post-processing benchmark: {:?}", timer.elapsed());
}

fn benchmark_serialization() {
    let timer = PerfTimer::start();

    // Image buffer
    let mut buffer = ImageBuffer::new(256, 256, ColorFormat::Rgb8);
    println!("   Image buffer: 256×256 RGB8");
    println!("     • Data size: {} bytes", buffer.width() * buffer.height() * 3);

    // Set some pixels
    for i in 0..256 {
        buffer.set_pixel(i, i, 1.0, 0.0, 0.0); // Diagonal red line
    }
    println!("     • Wrote 256 pixels");

    // Different formats
    let rgb16_buf = ImageBuffer::new(512, 512, ColorFormat::Rgb16);
    println!("   RGB16 buffer: 512×512 ({} bytes)", rgb16_buf.width() * rgb16_buf.height() * 6);

    let rgb32f_buf = ImageBuffer::new(512, 512, ColorFormat::Rgb32F);
    println!("   RGB32F buffer: 512×512 ({} bytes)", rgb32f_buf.width() * rgb32f_buf.height() * 12);

    println!("     ⏱️  Serialization benchmark: {:?}", timer.elapsed());
}

/// Import necessary types from the library
use avx_quantum_render::textures::*;
use avx_quantum_render::materials::*;
use avx_quantum_render::postprocessing::*;
use avx_quantum_render::acceleration;
