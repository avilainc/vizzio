//! Array operations benchmark

use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn bench_array_creation(c: &mut Criterion) {
    c.bench_function("create_int32_array_1000", |b| {
        b.iter(|| {
            let data: Vec<i32> = (0..1000).collect();
            black_box(data);
        });
    });
}

fn bench_array_sum(c: &mut Criterion) {
    let data: Vec<i32> = (0..10000).collect();

    c.bench_function("sum_int32_array_10000", |b| {
        b.iter(|| {
            let sum: i32 = data.iter().sum();
            black_box(sum);
        });
    });
}

criterion_group!(benches, bench_array_creation, bench_array_sum);
criterion_main!(benches);
