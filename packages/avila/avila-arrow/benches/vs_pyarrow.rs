//! Benchmark avila-arrow vs PyArrow

use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn bench_avila_ipc_write(c: &mut Criterion) {
    c.bench_function("avila_ipc_write", |b| {
        b.iter(|| {
            // Placeholder
            black_box(());
        });
    });
}

criterion_group!(benches, bench_avila_ipc_write);
criterion_main!(benches);
