//! Compute kernels benchmark

use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn bench_add(c: &mut Criterion) {
    let left: Vec<i32> = (0..10000).collect();
    let right: Vec<i32> = (0..10000).collect();

    c.bench_function("add_int32_10000", |b| {
        b.iter(|| {
            let result: Vec<i32> = left.iter()
                .zip(right.iter())
                .map(|(l, r)| l + r)
                .collect();
            black_box(result);
        });
    });
}

fn bench_comparison(c: &mut Criterion) {
    let left: Vec<i32> = (0..10000).collect();
    let right: Vec<i32> = (0..10000).collect();

    c.bench_function("eq_int32_10000", |b| {
        b.iter(|| {
            let result: Vec<bool> = left.iter()
                .zip(right.iter())
                .map(|(l, r)| l == r)
                .collect();
            black_box(result);
        });
    });
}

criterion_group!(benches, bench_add, bench_comparison);
criterion_main!(benches);
