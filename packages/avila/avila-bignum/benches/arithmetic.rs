use criterion::{black_box, criterion_group, criterion_main, Criterion};
use avila_bignum::U1024;

fn bench_addition(c: &mut Criterion) {
    let a = U1024::from(123456789u64);
    let b = U1024::from(987654321u64);

    c.bench_function("U1024 addition", |bencher| {
        bencher.iter(|| {
            let _result = black_box(a) + black_box(b);
        });
    });
}

fn bench_constants(c: &mut Criterion) {
    c.bench_function("U1024 ZERO constant", |bencher| {
        bencher.iter(|| {
            let _zero = black_box(U1024::ZERO);
        });
    });
}

criterion_group!(benches, bench_addition, bench_constants);
criterion_main!(benches);
