# Architecture Overview - avila-alloc

## 🏗️ High-Level Architecture

```
avila-alloc
├── stack/          # Stack-based allocators (no heap)
│   ├── vec         # StackVec<T, N>
│   ├── string      # StackString<N>
│   ├── box         # StackBox<T, N>
│   └── queue       # StackQueue<T, N>
│
├── arena/          # Bump allocators
│   ├── std_arena   # Arena (requires std)
│   └── static      # StaticArena<N> (no_std)
│
├── pool/           # Object pools
│   └── fixed       # Pool<T, N>
│
└── utils/          # Utilities
    ├── error       # AllocError
    └── alignment   # Memory alignment helpers
```

## 📦 Module Design

### Stack Allocators (`stack/`)

Stack-based allocators that use compile-time fixed-size arrays:

- **No heap allocation**: Everything lives on the stack
- **Compile-time sizing**: Size known at compile time via const generics
- **Zero runtime overhead**: No dynamic allocation costs
- **Deterministic**: Predictable memory usage and performance

**Key Types:**
- `StackVec<T, N>`: Fixed-capacity vector
- `StackString<N>`: UTF-8 validated string
- `StackBox<T, N>`: Single-value container
- `StackQueue<T, N>`: Circular FIFO queue

### Arena Allocators (`arena/`)

Bump allocators for temporary allocations:

- **Fast allocation**: O(1) bump pointer increment
- **No individual deallocation**: Free all at once
- **Sequential memory**: Better cache locality
- **Two variants**: `Arena` (std) and `StaticArena<N>` (no_std)

**Allocation Strategy:**
```
[─────────────allocated─────────────][────────free────────]
^                                    ^
base                                 bump pointer
```

### Pool Allocators (`pool/`)

Fixed-size object pools for reusable allocations:

- **Object recycling**: Reuse memory slots
- **O(n) allocation**: Linear search for free slot
- **Fragmentation-free**: All objects same size
- **Predictable capacity**: Fixed number of slots

**Free List Strategy:**
```
Storage: [obj1][obj2][obj3][obj4][obj5]
Free:     true  false true  true  false
           ^              ^    ^
           free slots to allocate
```

## 🔧 Design Principles

### 1. no_std First
```rust
#![cfg_attr(not(feature = "std"), no_std)]
```
- Core functionality works without standard library
- Optional `std` feature for enhanced types (Arena)
- Embedded-friendly by default

### 2. Const Generics
```rust
pub struct StackVec<T, const N: usize> { ... }
```
- Compile-time size specification
- Zero-runtime overhead
- Type-level capacity constraints

### 3. Zero Dependencies
```toml
[dependencies]
# Core has no dependencies
serde = { optional = true }  # Optional features only
```

### 4. Explicit Error Handling
```rust
pub fn push(&mut self, value: T) -> Result<(), T>
```
- No panics in production code
- Return values for overflow/OOM
- User decides error handling strategy

## 🧩 Type Relationships

```
┌─────────────────────────────────────┐
│         Stack Allocators            │
│  (StackVec, StackString, etc.)      │
│  • Fixed size at compile time       │
│  • Lives entirely on stack          │
└─────────────────────────────────────┘
                 │
                 │ no heap allocation
                 ▼
┌─────────────────────────────────────┐
│         Arena Allocators            │
│  (Arena, StaticArena)               │
│  • Bump allocation                  │
│  • Batch deallocation               │
└─────────────────────────────────────┘
                 │
                 │ temporary allocations
                 ▼
┌─────────────────────────────────────┐
│         Pool Allocators             │
│  (Pool, SlabAllocator)              │
│  • Fixed-size objects               │
│  • Object recycling                 │
└─────────────────────────────────────┘
```

## 🔐 Safety Guarantees

### Memory Safety
- **No raw pointer dereference** in safe code
- **MaybeUninit** for uninitialized memory
- **Drop implementation** ensures cleanup
- **Bounds checking** on all accesses

### Type Safety
- **Const generics** enforce capacity at compile time
- **Rust's ownership** prevents double-free
- **Lifetime tracking** prevents dangling references

## ⚡ Performance Characteristics

| Operation | StackVec | Arena | Pool |
|-----------|----------|-------|------|
| Allocation | O(1) | O(1) | O(n) |
| Deallocation | O(1) | O(1)* | O(1) |
| Access | O(1) | O(1) | O(1) |
| Memory | Stack | Heap/Stack | Stack |

*Arena deallocation is O(1) for all objects (reset)

## 🧪 Testing Strategy

### Unit Tests
- Per-module tests in `src/`
- Test basic operations
- Edge cases and overflow

### Integration Tests
- Cross-module tests in `tests/`
- Real-world scenarios
- Performance validation

### Property Tests
- Using `proptest` crate
- Fuzz testing
- Invariant checking

### Miri
- Undefined behavior detection
- Memory safety validation
- Concurrency testing

## 🚀 Future Architecture

### Phase 2: Smart Pointers
```
┌─────────────────────────────────────┐
│      Smart Pointers Layer           │
│  StackRc, StackArc, StackRefCell    │
└─────────────────────────────────────┘
              │
              ▼
        (existing allocators)
```

### Phase 3: Allocator Traits
```rust
pub trait Allocator {
    fn allocate(&mut self, layout: Layout) -> Result<NonNull<u8>, AllocError>;
    fn deallocate(&mut self, ptr: NonNull<u8>, layout: Layout);
}
```

### Phase 4: Global Allocator
```rust
#[global_allocator]
static GLOBAL: AvilaAllocator = AvilaAllocator::new();
```

## 📚 Further Reading

- [Cargo.toml](../Cargo.toml) - Project configuration
- [README.md](../README.md) - User documentation
- [ROADMAP.md](ROADMAP.md) - Development plan
- [Rust nomicon](https://doc.rust-lang.org/nomicon/) - Unsafe Rust
