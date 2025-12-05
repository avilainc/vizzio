//! SIMD operations benchmark

use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn bench_scalar_sum(c: &mut Criterion) {
    let data: Vec<f32> = vec![1.0; 10000];

    c.bench_function("scalar_sum_f32_10000", |b| {
        b.iter(|| {
            let sum: f32 = data.iter().sum();
            black_box(sum);
        });
    });
}

fn bench_scalar_add(c: &mut Criterion) {
    let left: Vec<f32> = vec![1.0; 10000];
    let right: Vec<f32> = vec![2.0; 10000];

    c.bench_function("scalar_add_f32_10000", |b| {
        b.iter(|| {
            let result: Vec<f32> = left.iter()
                .zip(right.iter())
                .map(|(l, r)| l + r)
                .collect();
            black_box(result);
        });
    });
}

criterion_group!(benches, bench_scalar_sum, bench_scalar_add);
criterion_main!(benches);
