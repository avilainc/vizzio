use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use avila_atom::DynamicArray;

fn vec_push_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("vec_push");

    for size in [100, 1_000, 10_000, 100_000].iter() {
        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, &size| {
            b.iter(|| {
                let mut v = DynamicArray::new();
                for i in 0..size {
                    v.push(black_box(i));
                }
                v
            });
        });
    }

    group.finish();
}

fn vec_push_with_capacity(c: &mut Criterion) {
    let mut group = c.benchmark_group("vec_push_with_capacity");

    for size in [100, 1_000, 10_000, 100_000].iter() {
        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, &size| {
            b.iter(|| {
                let mut v = DynamicArray::with_capacity(size);
                for i in 0..size {
                    v.push(black_box(i));
                }
                v
            });
        });
    }

    group.finish();
}

fn vec_access_benchmark(c: &mut Criterion) {
    let size = 10_000;
    let v: DynamicArray<i32> = (0..size).collect();

    c.bench_function("vec_access", |b| {
        b.iter(|| {
            let mut sum = 0;
            for i in 0..size {
                sum += v[black_box(i as usize)];
            }
            sum
        });
    });
}

fn vec_iteration_benchmark(c: &mut Criterion) {
    let size = 10_000;
    let v: DynamicArray<i32> = (0..size).collect();

    c.bench_function("vec_iteration", |b| {
        b.iter(|| {
            let mut sum = 0;
            for &item in &v {
                sum += black_box(item);
            }
            sum
        });
    });
}

criterion_group!(
    benches,
    vec_push_benchmark,
    vec_push_with_capacity,
    vec_access_benchmark,
    vec_iteration_benchmark
);
criterion_main!(benches);
