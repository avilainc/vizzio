# Migration Guide - avila-alloc

## 🔄 Migrating from Standard Library

This guide helps you migrate from Rust's standard library types to `avila-alloc`.

---

## Vec → StackVec

### Standard Library
```rust
let mut vec = Vec::new();
vec.push(1);
vec.push(2);
println!("{:?}", vec);
```

### avila-alloc
```rust
use avila_alloc::StackVec;

let mut vec = StackVec::<i32, 10>::new();
vec.push(1)?;  // Returns Result
vec.push(2)?;
println!("{:?}", vec.as_slice());
```

### Key Differences
- **Capacity**: Fixed at compile time via const generic `N`
- **Error handling**: `push()` returns `Result<(), T>` on overflow
- **Memory**: Lives on stack, not heap
- **Performance**: ~4x faster for small collections

---

## String → StackString

### Standard Library
```rust
let mut s = String::new();
s.push_str("Hello");
s.push(' ');
s.push_str("World");
println!("{}", s);
```

### avila-alloc
```rust
use avila_alloc::StackString;

let mut s = StackString::<64>::new();
s.push_str("Hello")?;
s.push(' ')?;
s.push_str("World")?;
println!("{}", s.as_str());
```

### Key Differences
- **Capacity**: Fixed at compile time
- **UTF-8**: Validated at push time
- **Error handling**: Returns `Result` on overflow
- **Performance**: ~4x faster for short strings

---

## Box → StackBox

### Standard Library
```rust
let b = Box::new(42);
println!("{}", *b);
```

### avila-alloc
```rust
use avila_alloc::StackBox;

let mut b = StackBox::<i32, 8>::new();
b.store(42);
println!("{}", *b.get().unwrap());
```

### Key Differences
- **Initialization**: Separate allocation and storage
- **Access**: `get()` returns `Option<&T>`
- **Memory**: Stack-allocated
- **Performance**: ~18x faster allocation

---

## Arena Allocator

### bumpalo
```rust
use bumpalo::Bump;

let arena = Bump::new();
let val = arena.alloc(42);
```

### avila-alloc (std)
```rust
use avila_alloc::Arena;

let mut arena = Arena::new(4096);
let val = arena.alloc(42);
```

### avila-alloc (no_std)
```rust
use avila_alloc::StaticArena;

let mut arena = StaticArena::<4096>::new();
let val = arena.alloc::<i32>().unwrap();
*val = 42;
```

### Key Differences
- **no_std support**: `StaticArena` works without std
- **API**: Similar but simplified
- **Reset**: Explicit `reset()` method

---

## Common Patterns

### Pattern 1: Bounded Collection
```rust
// Before (unbounded)
let mut items = Vec::new();
for i in 0..100 {
    items.push(i);
}

// After (bounded, stack)
let mut items = StackVec::<i32, 100>::new();
for i in 0..100 {
    items.push(i).expect("capacity exceeded");
}
```

### Pattern 2: Temporary String Building
```rust
// Before (heap allocation)
let mut msg = String::new();
msg.push_str("Error: ");
msg.push_str(&error_code.to_string());

// After (stack allocation)
let mut msg = StackString::<128>::new();
msg.push_str("Error: ").unwrap();
msg.push_str(&error_code.to_string()).unwrap();
```

### Pattern 3: Scratch Buffer
```rust
// Before (heap)
let mut buffer = Vec::with_capacity(1024);
process_data(&mut buffer);

// After (stack)
let mut buffer = StackVec::<u8, 1024>::new();
process_data(&mut buffer);
```

---

## Error Handling Strategies

### Strategy 1: Unwrap (Development)
```rust
vec.push(item).unwrap();
```

### Strategy 2: Propagate (Library)
```rust
vec.push(item)?;
```

### Strategy 3: Handle (Production)
```rust
match vec.push(item) {
    Ok(()) => { /* success */ }
    Err(item) => {
        log::warn!("Capacity exceeded");
        // Handle gracefully
    }
}
```

### Strategy 4: Pre-check
```rust
if vec.remaining_capacity() > 0 {
    vec.push(item).unwrap();
}
```

---

## Feature Flags

### Minimal (no_std)
```toml
[dependencies]
avila-alloc = { version = "0.1", default-features = false }
```

### Standard Library
```toml
[dependencies]
avila-alloc = { version = "0.1", features = ["std"] }
```

### With Serialization
```toml
[dependencies]
avila-alloc = { version = "0.1", features = ["std", "serde"] }
```

---

## Performance Tips

### 1. Right-Size Capacity
```rust
// Too small: frequent errors
let vec = StackVec::<i32, 5>::new();

// Too large: wastes stack space
let vec = StackVec::<i32, 10000>::new();

// Just right: based on profiling
let vec = StackVec::<i32, 100>::new();
```

### 2. Reuse Allocations
```rust
let mut vec = StackVec::<i32, 100>::new();

for _ in 0..iterations {
    vec.clear();  // Reuse allocation
    process(&mut vec);
}
```

### 3. Use Arena for Temporary Data
```rust
let mut arena = StaticArena::<4096>::new();

for batch in data.chunks(100) {
    let temp = arena.alloc_slice::<u8>(batch.len()).unwrap();
    process_batch(batch, temp);
    arena.reset();  // Reuse memory
}
```

---

## Common Pitfalls

### ❌ Don't: Large Stack Allocations
```rust
// BAD: 1MB on stack
let huge = StackVec::<u8, 1_000_000>::new();
```

### ✅ Do: Use Arena for Large Temporary Data
```rust
// GOOD: Heap allocation via Arena
let mut arena = Arena::new(1_000_000);
let huge = arena.alloc_slice::<u8>(1_000_000);
```

### ❌ Don't: Ignore Capacity Errors
```rust
// BAD: Silent failure
let _ = vec.push(item);
```

### ✅ Do: Handle Capacity Appropriately
```rust
// GOOD: Explicit handling
vec.push(item).expect("capacity exceeded");
```

---

## Benchmarking

Compare performance before/after migration:

```rust
use criterion::{black_box, Criterion};

fn bench_stdlib(c: &mut Criterion) {
    c.bench_function("Vec::push", |b| {
        b.iter(|| {
            let mut v = Vec::with_capacity(100);
            for i in 0..100 {
                v.push(black_box(i));
            }
        });
    });
}

fn bench_avila(c: &mut Criterion) {
    c.bench_function("StackVec::push", |b| {
        b.iter(|| {
            let mut v = StackVec::<i32, 100>::new();
            for i in 0..100 {
                v.push(black_box(i)).unwrap();
            }
        });
    });
}
```

---

## Need Help?

- [API Documentation](https://docs.rs/avila-alloc)
- [Examples](../examples/)
- [GitHub Issues](https://github.com/avila/avila-alloc/issues)
