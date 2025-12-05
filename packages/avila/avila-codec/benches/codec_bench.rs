//! Benchmark suite for avila-codec

use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};

fn hex_benchmark(c: &mut Criterion) {
    let data = b"The quick brown fox jumps over the lazy dog";

    c.bench_function("hex_encode", |b| {
        b.iter(|| avila_codec::hex::encode(black_box(data)))
    });

    let encoded = avila_codec::hex::encode(data);
    c.bench_function("hex_decode", |b| {
        b.iter(|| avila_codec::hex::decode(black_box(&encoded)))
    });
}

fn base64_benchmark(c: &mut Criterion) {
    let data = b"The quick brown fox jumps over the lazy dog";

    c.bench_function("base64_encode", |b| {
        b.iter(|| avila_codec::base64::encode(black_box(data)))
    });

    let encoded = avila_codec::base64::encode(data);
    c.bench_function("base64_decode", |b| {
        b.iter(|| avila_codec::base64::decode(black_box(&encoded)))
    });
}

fn checksum_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("checksums");

    for size in [64, 256, 1024, 4096].iter() {
        let data = vec![0u8; *size];

        group.bench_with_input(BenchmarkId::new("crc32", size), &data, |b, d| {
            b.iter(|| avila_codec::checksum::crc::crc32(black_box(d)))
        });

        group.bench_with_input(BenchmarkId::new("xxhash32", size), &data, |b, d| {
            b.iter(|| avila_codec::checksum::xxhash::xxhash32(black_box(d)))
        });
    }

    group.finish();
}

criterion_group!(benches, hex_benchmark, base64_benchmark, checksum_benchmark);
criterion_main!(benches);
