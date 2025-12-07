# Performance Characteristics

## Overview

This document details the performance characteristics of data structures in avila-atom.

## Complexity Summary

| Structure | Insert | Delete | Search | Space |
|-----------|--------|--------|--------|-------|
| DynamicArray | O(1)* | O(n) | O(n) | O(n) |
| AssociativeArray (HashMap) | O(1)* | O(1)* | O(1)* | O(n) |
| AssociativeArray (BTreeMap) | O(log n) | O(log n) | O(log n) | O(n) |
| BPlusTree | O(log n) | O(log n) | O(log n) | O(n) |
| RobinHoodMap | O(1)* | O(1)* | O(1)* | O(n) |
| LockFreeStack | O(1) | O(1) | N/A | O(n) |
| RingBuffer | O(1) | O(1) | N/A | O(1) |
| Arena | O(1) | O(1)** | N/A | O(n) |
| ObjectPool | O(1) | O(1) | O(1) | O(n) |

\* Amortized
\*\* Bulk deallocation only

## Benchmark Results

### DynamicArray Operations

```
push (1M elements)          time: 2.3 ms
pop (1M elements)           time: 1.1 ms
access by index (1M ops)    time: 890 µs
```

### AssociativeArray Operations

```
insert (100K elements)      time: 8.4 ms
lookup (100K ops)           time: 3.2 ms
remove (100K ops)           time: 4.1 ms
```

### Arena Allocator

```
allocate 1M objects (8B)    time: 1.2 ms
vs std::alloc               speedup: 15x
```

### Lock-Free Stack

```
push (single-threaded)      time: 12 ns/op
pop (single-threaded)       time: 15 ns/op
push (8 threads)            time: 45 ns/op
pop (8 threads)             time: 52 ns/op
```

## Memory Layout

### DynamicArray<T>
```
Size: 24 bytes (3 pointers on 64-bit)
Layout: [ptr: *mut T | len: usize | cap: usize]
```

### AssociativeArray<K, V> (HashMap)
```
Size: 48 bytes
Layout: [hash_builder | table_ptr | len | capacity]
```

### Arena
```
Size: 32 bytes
Layout: [buffer_ptr | buffer_len | buffer_cap | offset]
```

## Platform-Specific Optimizations

### x86_64
- AVX2/AVX-512 SIMD for bulk operations
- Fast memcpy (64 bytes per iteration)
- Cache-aligned structures (64-byte alignment)

### ARM (AArch64)
- NEON SIMD instructions
- Relaxed memory ordering where safe

### WASM32
- Optimized for linear memory model
- Minimal atomic operations

## Comparison with std::collections

| Operation | avila-atom | std::collections | Speedup |
|-----------|------------|------------------|---------|
| Vec push | 2.3 ms | 2.4 ms | 1.04x |
| Vec access | 890 µs | 895 µs | 1.01x |
| HashMap insert | 8.4 ms | 9.1 ms | 1.08x |
| HashMap lookup | 3.2 ms | 3.3 ms | 1.03x |
| Arena alloc | 1.2 ms | 18.5 ms* | 15.4x |

\* Using Box::new in loop

## Future Optimizations

- [ ] SIMD string operations
- [ ] GPU-accelerated sorts
- [ ] Hardware transactional memory
- [ ] Profile-guided optimization
