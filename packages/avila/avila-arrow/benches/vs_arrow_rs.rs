//! Benchmark avila-arrow vs arrow-rs

use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn bench_avila_array_ops(c: &mut Criterion) {
    let data: Vec<i32> = (0..10000).collect();

    c.bench_function("avila_sum_10000", |b| {
        b.iter(|| {
            let sum: i32 = data.iter().sum();
            black_box(sum);
        });
    });
}

// Placeholder for arrow-rs comparison
// fn bench_arrow_rs_array_ops(c: &mut Criterion) { ... }

criterion_group!(benches, bench_avila_array_ops);
criterion_main!(benches);
