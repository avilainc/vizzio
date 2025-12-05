# Architecture Overview - avila-bignum

This document describes the internal architecture and design decisions of the avila-bignum library.

## 🏗️ High-Level Architecture

```
avila-bignum
│
├── Types Layer (U256, U1024, etc.)
│   └── Fixed-size big integers with stack allocation
│
├── Arithmetic Layer
│   ├── Basic Operations (add, sub, mul, div)
│   ├── Bitwise Operations (and, or, xor, shifts)
│   └── Comparison Operations
│
├── Cryptographic Layer
│   ├── Modular Arithmetic
│   ├── Montgomery Multiplication
│   ├── Prime Number Generation
│   └── RSA Primitives
│
├── Traits Layer
│   ├── Operator Overloading (Add, Mul, etc.)
│   ├── Display Formatting
│   └── Type Conversions
│
└── Utilities Layer
    ├── Parsing (string → bigint)
    ├── Formatting (bigint → string)
    └── Serialization (bytes ↔ bigint)
```

## 💾 Memory Layout

### Limb Representation
Big integers are represented as arrays of 64-bit limbs in little-endian order:

```rust
// U256 = 4 limbs × 64 bits = 256 bits
pub struct U256 {
    limbs: [u64; 4]  // [LSB, ..., MSB]
}
// Value = limbs[0] + limbs[1]×2^64 + limbs[2]×2^128 + limbs[3]×2^192
```

**Why 64-bit limbs?**
- Optimal for 64-bit CPUs
- Native u64 × u64 → u128 multiplication
- Efficient carry propagation

**Why little-endian?**
- Natural carry propagation (left to right)
- Simpler loop implementations
- Consistent with CPU architectures

## 🔧 Design Principles

### 1. Zero-Cost Abstractions
Operations compile down to efficient machine code with no runtime overhead.

```rust
// This:
let c = a + b;

// Compiles to efficient inline assembly similar to:
// - ADC (add with carry) instructions
// - No function call overhead
// - SIMD when available
```

### 2. Const Generics (Future)
Generic types parameterized by bit size:

```rust
// Future API:
pub struct UInt<const BITS: usize> {
    limbs: [u64; (BITS + 63) / 64]
}
```

### 3. Constant-Time by Default (Crypto)
All cryptographic operations avoid timing side channels:

```rust
// Constant-time conditional assignment
fn conditional_assign(dest: &mut [u64], src: &[u64], condition: u64) {
    let mask = 0u64.wrapping_sub(condition);
    for (d, &s) in dest.iter_mut().zip(src.iter()) {
        *d ^= mask & (*d ^ s);
    }
}
```

### 4. No_std Compatible
Core functionality works without standard library:

```toml
[dependencies]
# No dependencies for core features
```

## 🚀 Performance Strategies

### Carry Propagation Optimization
Use 128-bit intermediate results for efficient carry handling:

```rust
#[inline]
const fn adc(a: u64, b: u64, carry: u64) -> (u64, u64) {
    let sum = a as u128 + b as u128 + carry as u128;
    (sum as u64, (sum >> 64) as u64)
}
```

### Karatsuba Multiplication
For medium-sized multiplications (threshold ~16 limbs):

```
Traditional: O(n²) operations
Karatsuba:  O(n^1.585) operations
Crossover:  ~1024 bits
```

### Montgomery Reduction
For repeated modular operations:

```
Traditional: mod after each operation
Montgomery:  transform → compute → inverse transform
Speedup:     ~3-5x for RSA operations
```

### SIMD Vectorization
Use AVX2/AVX-512 for parallel limb operations:

```rust
// Conceptual (actual impl uses intrinsics)
#[cfg(target_feature = "avx2")]
fn add_simd(a: &[u64], b: &[u64]) -> Vec<u64> {
    // Process 4 limbs at once with AVX2
}
```

## 🔐 Cryptographic Considerations

### Constant-Time Guarantees
Critical for preventing timing attacks:

- **Comparisons**: Use bitwise operations, no early exits
- **Conditional moves**: Branchless selection
- **Exponentiation**: Always execute same operations regardless of exponent bits

### Side-Channel Resistance
- No secret-dependent branches
- No secret-dependent memory access patterns
- No secret-dependent loop counts

### Random Number Generation
Integrate with platform-specific CSPRNGs:

```rust
#[cfg(feature = "rand")]
fn random(bits: usize) -> UInt {
    // Use getrandom or OS-specific RNG
}
```

## 📦 Module Organization

### `types/`
Fixed-size big integer definitions. Each type is a thin wrapper around a limb array.

### `arithmetic/`
Low-level operations on limb arrays. No type knowledge, pure algorithms.

### `crypto/`
High-level cryptographic primitives using arithmetic operations.

### `traits/`
Rust trait implementations for operator overloading and standard library integration.

### `utils/`
Parsing, formatting, and serialization utilities.

## 🧪 Testing Strategy

### Unit Tests
Test individual functions with known inputs/outputs.

```rust
#[test]
fn test_adc() {
    assert_eq!(adc(u64::MAX, 1, 0), (0, 1));
}
```

### Integration Tests
Test complete workflows (e.g., RSA encrypt/decrypt roundtrip).

### Property-Based Tests
Use `proptest` to verify algebraic properties:

```rust
proptest! {
    fn test_addition_commutative(a: u64, b: u64) {
        assert_eq!(UInt::from(a) + b, UInt::from(b) + a);
    }
}
```

### Benchmark Suite
Track performance across versions using Criterion:

```rust
fn bench_mul(c: &mut Criterion) {
    c.bench_function("U2048 multiplication", |b| {
        b.iter(|| a * b)
    });
}
```

### Fuzzing
Continuous fuzzing to find edge cases:

```bash
cargo fuzz run arithmetic_ops
```

## 🔄 Future Optimizations

1. **Inline Assembly**: Critical paths in x86_64/ARM assembly
2. **GPU Acceleration**: For very large operations (8192+ bits)
3. **Compile-Time Computation**: More const fn support
4. **Profile-Guided Optimization**: CPU-specific tuning

---

**Note**: This is a living document updated as architecture evolves.
