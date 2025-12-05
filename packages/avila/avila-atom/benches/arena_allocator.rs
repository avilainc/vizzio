use criterion::{black_box, criterion_group, criterion_main, Criterion};
use avila_atom::arena::Arena;

fn arena_allocate_benchmark(c: &mut Criterion) {
    c.bench_function("arena_allocate_1000", |b| {
        b.iter(|| {
            let mut arena = Arena::with_capacity(1024 * 1024);
            for i in 0..1000 {
                let _ = arena.alloc_value(black_box(i));
            }
            arena
        });
    });
}

fn arena_vs_box_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("allocation_comparison");

    group.bench_function("arena", |b| {
        b.iter(|| {
            let mut arena = Arena::with_capacity(8000);
            let mut ptrs = Vec::with_capacity(1000);
            for i in 0..1000 {
                if let Some(ptr) = arena.alloc_value(black_box(i)) {
                    ptrs.push(ptr);
                }
            }
            ptrs
        });
    });

    group.bench_function("box", |b| {
        b.iter(|| {
            let mut ptrs = Vec::with_capacity(1000);
            for i in 0..1000 {
                ptrs.push(Box::new(black_box(i)));
            }
            ptrs
        });
    });

    group.finish();
}

criterion_group!(
    benches,
    arena_allocate_benchmark,
    arena_vs_box_benchmark
);
criterion_main!(benches);
