use criterion::{black_box, criterion_group, criterion_main, Criterion};
use avila_atom::lockfree::{LockFreeStack, RingBuffer};

fn lockfree_stack_push_pop(c: &mut Criterion) {
    c.bench_function("lockfree_stack_push_pop", |b| {
        let stack = LockFreeStack::new();

        b.iter(|| {
            for i in 0..1000 {
                stack.push(black_box(i));
            }

            for _ in 0..1000 {
                black_box(stack.pop());
            }
        });
    });
}

fn ring_buffer_operations(c: &mut Criterion) {
    c.bench_function("ring_buffer_push_pop", |b| {
        let buffer: RingBuffer<i32, 1024> = RingBuffer::new();

        b.iter(|| {
            for i in 0..512 {
                let _ = buffer.push(black_box(i));
            }

            for _ in 0..512 {
                black_box(buffer.pop());
            }
        });
    });
}

criterion_group!(
    benches,
    lockfree_stack_push_pop,
    ring_buffer_operations
);
criterion_main!(benches);
