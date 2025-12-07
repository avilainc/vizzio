use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use avila_alloc::{StackVec, StackString, StaticArena, Pool};

fn bench_stack_vec(c: &mut Criterion) {
    let mut group = c.benchmark_group("StackVec");

    group.bench_function("push_10", |b| {
        b.iter(|| {
            let mut vec = StackVec::<u64, 10>::new();
            for i in 0..10 {
                vec.push(black_box(i)).unwrap();
            }
            vec
        });
    });

    group.bench_function("push_pop", |b| {
        b.iter(|| {
            let mut vec = StackVec::<u64, 100>::new();
            for i in 0..50 {
                vec.push(black_box(i)).unwrap();
            }
            for _ in 0..50 {
                black_box(vec.pop());
            }
        });
    });

    group.finish();
}

fn bench_stack_string(c: &mut Criterion) {
    let mut group = c.benchmark_group("StackString");

    group.bench_function("push_str", |b| {
        b.iter(|| {
            let mut s = StackString::<128>::new();
            s.push_str(black_box("Hello")).unwrap();
            s.push_str(black_box(", ")).unwrap();
            s.push_str(black_box("World!")).unwrap();
            s
        });
    });

    group.bench_function("push_char", |b| {
        b.iter(|| {
            let mut s = StackString::<64>::new();
            for c in "Hello".chars() {
                s.push(black_box(c)).unwrap();
            }
            s
        });
    });

    group.finish();
}

fn bench_static_arena(c: &mut Criterion) {
    let mut group = c.benchmark_group("StaticArena");

    group.bench_function("alloc_u64", |b| {
        let mut arena = StaticArena::<4096>::new();
        b.iter(|| {
            arena.reset();
            for _ in 0..100 {
                let val = arena.alloc::<u64>().unwrap();
                *val = black_box(42);
            }
        });
    });

    group.bench_function("alloc_slice", |b| {
        let mut arena = StaticArena::<4096>::new();
        b.iter(|| {
            arena.reset();
            let slice = arena.alloc_slice::<u32>(black_box(10)).unwrap();
            slice[0] = 1;
        });
    });

    group.finish();
}

#[cfg(feature = "std")]
fn bench_std_arena(c: &mut Criterion) {
    use avila_alloc::Arena;

    let mut group = c.benchmark_group("Arena");

    group.bench_function("alloc_u64", |b| {
        let mut arena = Arena::new(4096);
        b.iter(|| {
            arena.reset();
            for _ in 0..100 {
                let val = arena.alloc::<u64>();
                *val = black_box(42);
            }
        });
    });

    group.finish();
}

fn bench_comparisons(c: &mut Criterion) {
    let mut group = c.benchmark_group("Comparisons");

    // StackVec vs Vec
    #[cfg(feature = "std")]
    {
        group.bench_function("StackVec_push", |b| {
            b.iter(|| {
                let mut vec = StackVec::<u64, 100>::new();
                for i in 0..100 {
                    vec.push(black_box(i)).unwrap();
                }
            });
        });

        group.bench_function("Vec_push", |b| {
            b.iter(|| {
                let mut vec = Vec::with_capacity(100);
                for i in 0..100 {
                    vec.push(black_box(i));
                }
            });
        });
    }

    group.finish();
}

criterion_group!(
    benches,
    bench_stack_vec,
    bench_stack_string,
    bench_static_arena,
    bench_comparisons
);

#[cfg(feature = "std")]
criterion_group!(std_benches, bench_std_arena);

#[cfg(feature = "std")]
criterion_main!(benches, std_benches);

#[cfg(not(feature = "std"))]
criterion_main!(benches);
