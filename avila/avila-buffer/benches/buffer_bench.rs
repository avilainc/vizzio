use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use avila_buffer::ByteBuffer;

fn bench_write(c: &mut Criterion) {
    let mut group = c.benchmark_group("write");

    for size in [64, 256, 1024, 4096].iter() {
        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, &size| {
            let data = vec![0u8; size];
            b.iter(|| {
                let mut buffer = ByteBuffer::with_capacity(size);
                buffer.write(black_box(&data)).unwrap();
            });
        });
    }

    group.finish();
}

fn bench_read(c: &mut Criterion) {
    let mut group = c.benchmark_group("read");

    for size in [64, 256, 1024, 4096].iter() {
        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, &size| {
            let data = vec![0u8; size];
            let buffer = ByteBuffer::from_vec(data);

            b.iter(|| {
                let mut buf = buffer.clone();
                let mut output = vec![0u8; size];
                buf.read(black_box(&mut output)).unwrap();
            });
        });
    }

    group.finish();
}

fn bench_write_read(c: &mut Criterion) {
    c.bench_function("write_read_1kb", |b| {
        let data = vec![42u8; 1024];
        b.iter(|| {
            let mut buffer = ByteBuffer::with_capacity(1024);
            buffer.write(black_box(&data)).unwrap();

            let mut output = vec![0u8; 1024];
            buffer.read(black_box(&mut output)).unwrap();
        });
    });
}

fn bench_compact(c: &mut Criterion) {
    c.bench_function("compact_4kb", |b| {
        b.iter(|| {
            let mut buffer = ByteBuffer::from_vec(vec![0u8; 4096]);
            let mut tmp = vec![0u8; 1024];
            buffer.read(&mut tmp).unwrap();
            buffer.compact();
        });
    });
}

// Note: This won't compile until Clone is implemented for ByteBuffer
// Keeping it here as a template for future implementation
impl Clone for ByteBuffer {
    fn clone(&self) -> Self {
        Self {
            data: self.data.clone(),
            read_pos: self.read_pos,
            write_pos: self.write_pos,
        }
    }
}

criterion_group!(benches, bench_write, bench_read, bench_write_read, bench_compact);
criterion_main!(benches);
