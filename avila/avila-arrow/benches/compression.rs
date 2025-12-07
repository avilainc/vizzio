//! Compression benchmark

use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn bench_rle_compression(c: &mut Criterion) {
    let data: Vec<i32> = vec![1; 10000];

    c.bench_function("rle_compress_10000", |b| {
        b.iter(|| {
            // Placeholder RLE compression
            black_box(&data);
        });
    });
}

fn bench_delta_encoding(c: &mut Criterion) {
    let data: Vec<i32> = (0..10000).collect();

    c.bench_function("delta_encode_10000", |b| {
        b.iter(|| {
            let deltas: Vec<i32> = data.windows(2)
                .map(|w| w[1] - w[0])
                .collect();
            black_box(deltas);
        });
    });
}

criterion_group!(benches, bench_rle_compression, bench_delta_encoding);
criterion_main!(benches);
