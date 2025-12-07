use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use avila_atom::AssociativeArray;

fn map_insert_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("map_insert");

    for size in [100, 1_000, 10_000].iter() {
        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, &size| {
            b.iter(|| {
                let mut map = AssociativeArray::new();
                for i in 0..size {
                    map.insert(black_box(i), black_box(i * 2));
                }
                map
            });
        });
    }

    group.finish();
}

fn map_lookup_benchmark(c: &mut Criterion) {
    let size = 10_000;
    let mut map = AssociativeArray::new();
    for i in 0..size {
        map.insert(i, i * 2);
    }

    c.bench_function("map_lookup", |b| {
        b.iter(|| {
            let mut sum = 0;
            for i in 0..size {
                if let Some(&val) = map.get(&black_box(i)) {
                    sum += val;
                }
            }
            sum
        });
    });
}

fn map_remove_benchmark(c: &mut Criterion) {
    c.bench_function("map_remove", |b| {
        b.iter(|| {
            let mut map = AssociativeArray::new();
            for i in 0..1000 {
                map.insert(i, i * 2);
            }

            for i in 0..500 {
                map.remove(&black_box(i));
            }

            map
        });
    });
}

criterion_group!(
    benches,
    map_insert_benchmark,
    map_lookup_benchmark,
    map_remove_benchmark
);
criterion_main!(benches);
