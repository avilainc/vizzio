# Migration Guide

## Migrating from std::collections

This guide helps you migrate from Rust's standard library collections to avila-atom.

## Type Mappings

| std::collections | avila-atom | Notes |
|------------------|------------|-------|
| `Vec<T>` | `DynamicArray<T>` | Drop-in replacement |
| `HashMap<K, V>` | `AssociativeArray<K, V>` | std mode |
| `BTreeMap<K, V>` | `AssociativeArray<K, V>` | no_std mode |
| `String` | `StringBuffer` | Type alias |
| `VecDeque<T>` | `RingBuffer<T, N>` | Fixed capacity |

## Basic Migrations

### Vec to DynamicArray

**Before:**
```rust
use std::vec::Vec;

let mut v = Vec::new();
v.push(1);
v.push(2);
```

**After:**
```rust
use avila_atom::DynamicArray;

let mut v = DynamicArray::new();
v.push(1);
v.push(2);
```

### HashMap to AssociativeArray

**Before:**
```rust
use std::collections::HashMap;

let mut map = HashMap::new();
map.insert("key", "value");
```

**After:**
```rust
use avila_atom::AssociativeArray;

let mut map = AssociativeArray::new();
map.insert("key", "value");
```

Or use the macro:
```rust
use avila_atom::map;

let map = map! {
    "key" => "value",
};
```

## Advanced Features

### Extension Traits

avila-atom provides additional methods:

```rust
use avila_atom::{DynamicArray, DynamicArrayExt};

let mut v = DynamicArray::new();
v.reserve_exact_fast(100);  // Optimized reservation
v.clear_and_resize(50);     // Clear and set capacity
```

### no_std Migration

**Before (std):**
```rust
use std::vec::Vec;
use std::collections::HashMap;
```

**After (no_std):**
```rust
#![no_std]
extern crate alloc;

use avila_atom::{DynamicArray, AssociativeArray};
// DynamicArray = alloc::vec::Vec
// AssociativeArray = alloc::collections::BTreeMap
```

### Performance Optimizations

#### Arena Allocation

Replace many small allocations:

**Before:**
```rust
let mut objects = Vec::new();
for i in 0..1000 {
    objects.push(Box::new(MyStruct { id: i }));
}
```

**After:**
```rust
use avila_atom::arena::Arena;

let mut arena = Arena::with_capacity(1000 * size_of::<MyStruct>());
let mut objects = Vec::new();
for i in 0..1000 {
    let obj = arena.alloc_value(MyStruct { id: i }).unwrap();
    objects.push(obj);
}
// All deallocated at once when arena drops
```

#### Object Pool

Reuse objects instead of allocating:

**Before:**
```rust
for _ in 0..1000 {
    let obj = Box::new(ExpensiveObject::new());
    // use obj
    drop(obj);
}
```

**After:**
```rust
use avila_atom::pool::ObjectPool;

let mut pool = ObjectPool::with_capacity(10);
for _ in 0..1000 {
    let id = pool.acquire(|| ExpensiveObject::new());
    // use pool.get_mut(id)
    pool.release(id);
}
```

## Common Patterns

### Iteration

Works the same:
```rust
let v = DynamicArray::from([1, 2, 3]);
for item in &v {
    println!("{}", item);
}
```

### Collecting

Works the same:
```rust
let v: DynamicArray<_> = (0..10).collect();
```

### Macros

Use convenient macros:
```rust
use avila_atom::{list, map, array};

let v = list![1, 2, 3];
let m = map!{ "a" => 1, "b" => 2 };
let a = array![1, 2, 3, 4];  // Stack-allocated
```

## Breaking Changes

### Capacity Behavior

`DynamicArray` uses geometric growth (2x) like Vec, but provides more control:

```rust
let mut v = DynamicArray::new();
v.reserve_exact_fast(100);  // No over-allocation
```

### no_std Differences

In no_std mode, `AssociativeArray` uses BTreeMap (O(log n)) instead of HashMap (O(1)):

```rust
#[cfg(feature = "std")]
type Map<K, V> = HashMap<K, V>;  // O(1)

#[cfg(not(feature = "std"))]
type Map<K, V> = BTreeMap<K, V>; // O(log n)
```

## Performance Tips

1. **Pre-allocate when size is known:**
   ```rust
   let mut v = DynamicArray::with_capacity(1000);
   ```

2. **Use arenas for temporary allocations:**
   ```rust
   let mut arena = Arena::with_capacity(1024 * 1024);
   ```

3. **Prefer stack allocation for small arrays:**
   ```rust
   let arr = array![1, 2, 3, 4];  // No heap allocation
   ```

4. **Use object pools for reusable objects:**
   ```rust
   let mut pool = ObjectPool::with_capacity(100);
   ```

## Compatibility

avila-atom maintains API compatibility with std::collections where possible:

- ✅ Same method names
- ✅ Same iterator types
- ✅ Same trait implementations
- ✅ Drop-in replacement in most cases
- ⚠️ Additional features via extension traits
- ⚠️ Some optimizations change capacity behavior

## Next Steps

- Read the [API documentation](https://docs.rs/avila-atom)
- See [examples/](examples/) for more usage patterns
- Check [PERFORMANCE.md](PERFORMANCE.md) for benchmarks
