# Avila Arrow

**High-performance columnar data format in pure Rust - No external dependencies**

[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-1.70+-orange.svg)](https://www.rust-lang.org)

Avila Arrow é uma implementação nativa e genuína do formato Arrow, otimizada para computação científica brasileira e integração com AvilaDB.

## 🎯 Filosofia

- **Zero dependências externas**: Código Rust puro e genuíno
- **Performance**: Otimizações SIMD nativas
- **Científico**: Suporte nativo para quaternions, tensores, números complexos e spinors
- **Brasileiro**: Documentação e exemplos em português

## ✨ Características

### Tipos de Dados

#### Tipos Primitivos
- **Inteiros**: Int8, Int16, Int32, Int64
- **Sem Sinal**: UInt8, UInt16, UInt32, UInt64
- **Ponto Flutuante**: Float32, Float64
- **Boolean**: true/false
- **String**: Utf8, Binary

#### Tipos Científicos
- **Quaternion**: Rotações 4D (w, x, y, z)
- **Complex64**: Números complexos (real, imaginário)
- **Tensor4D**: Tensores 4x4 para física
- **Spinor**: Spinors para física de partículas

### Operações de Compute

#### Agregações
```rust
use avila_arrow::compute::*;

let values = vec![1, 2, 3, 4, 5];
let sum = sum_i32(&values);           // 15
let mean = mean_i32(&values);         // 3.0
let min = min_i32(&values);           // Some(1)
let max = max_i32(&values);           // Some(5)
```

#### Aritméticas
```rust
let a = vec![1, 2, 3];
let b = vec![4, 5, 6];

let sum = add_i32(&a, &b)?;           // [5, 7, 9]
let diff = sub_i32(&a, &b)?;          // [-3, -3, -3]
let prod = mul_i32(&a, &b)?;          // [4, 10, 18]
```

#### Comparações
```rust
let a = vec![1, 2, 3];
let b = vec![2, 2, 2];

let eq = eq_i32(&a, &b)?;             // [false, true, false]
let lt = lt_i32(&a, &b)?;             // [true, false, false]
let gt = gt_i32(&a, &b)?;             // [false, false, true]
```

#### Filtros e Ordenação
```rust
// Filter
let values = vec![1, 2, 3, 4, 5];
let mask = vec![true, false, true, false, true];
let filtered = filter_i32(&values, &mask)?;  // [1, 3, 5]

// Sort
let unsorted = vec![5, 2, 8, 1, 9];
let sorted = sort_i32(&unsorted);            // [1, 2, 5, 8, 9]

// Argsort
let indices = argsort_i32(&unsorted);        // [3, 1, 0, 2, 4]
```

#### Window Functions
```rust
// Rolling sum
let values = vec![1, 2, 3, 4, 5];
let rolling = rolling_sum_i32(&values, 3)?;  // [6, 9, 12]

// Cumulative sum
let cumsum = cumsum_i32(&values);            // [1, 3, 6, 10, 15]

// Exponential moving average
let values = vec![1.0, 2.0, 3.0, 4.0, 5.0];
let ema = ema_f64(&values, 0.5);
```

#### Estatísticas
```rust
use avila_arrow::compute::statistics::*;

let values = vec![1.0, 2.0, 3.0, 4.0, 5.0];

// Medidas básicas
let mean = mean_f64(&values);              // 3.0
let variance = variance_f64(&values);
let std_dev = std_dev_f64(&values);

// Quartis
let (q1, median, q3) = quartiles_f64(&values);
let iqr = iqr_f64(&values);

// Correlação
let x = vec![1.0, 2.0, 3.0, 4.0, 5.0];
let y = vec![2.0, 4.0, 6.0, 8.0, 10.0];
let corr = correlation_f64(&x, &y)?;       // 1.0 (perfeita)

// Z-scores
let z_scores = z_scores_f64(&values);

// Skewness e Kurtosis
let skew = skewness_f64(&values);
let kurt = kurtosis_f64(&values);
```

#### Hash e Unique
```rust
use avila_arrow::compute::hash::*;

let values = vec![1, 2, 2, 3, 3, 3];

// Valores únicos
let unique = unique_i32(&values);          // [1, 2, 3]
let count = unique_count_i32(&values);     // 3

// Contagem de valores
let counts = value_counts_i32(&values);    // {1: 1, 2: 2, 3: 3}

// Moda
let mode = mode_i32(&values);              // Some(3)
```

### Arrays

```rust
use avila_arrow::array::*;

// Int32 array
let int_array = Int32Array::new(vec![1, 2, 3, 4, 5]);
assert_eq!(int_array.value(0), Some(1));
assert_eq!(int_array.len(), 5);

// Array com nulls
let array_with_nulls = Int32Array::with_nulls(
    vec![1, 2, 3],
    vec![true, false, true]
)?;
assert_eq!(array_with_nulls.value(1), None);

// Float64 array
let float_array = Float64Array::new(vec![1.1, 2.2, 3.3]);
```

### Schemas e RecordBatches

```rust
use avila_arrow::*;

// Define schema
let schema = Schema::new(vec![
    Field::new("id", DataType::Int64),
    Field::new("value", DataType::Float64),
    Field::new("label", DataType::Utf8),
]);

// Create arrays
let ids = Int64Array::new(vec![1, 2, 3]);
let values = Float64Array::new(vec![1.1, 2.2, 3.3]);
let labels = StringArray::new(vec!["a".to_string(), "b".to_string(), "c".to_string()]);

// Create RecordBatch
let batch = RecordBatch::try_new(
    schema,
    vec![
        Box::new(ids),
        Box::new(values),
        Box::new(labels),
    ]
)?;

assert_eq!(batch.num_rows(), 3);
assert_eq!(batch.num_columns(), 3);
```

### Tipos Científicos

#### Quaternions
```rust
use avila_arrow::scientific::Quaternion;

let q1 = Quaternion::new(1.0, 0.0, 0.0, 0.0);  // identidade
let q2 = Quaternion::new(0.0, 1.0, 0.0, 0.0);

let conjugate = q1.conjugate();
let magnitude = q1.magnitude();
let normalized = q1.normalize();

// Multiplicação de quaternions
let product = quat_mul(&q1, &q2);
```

#### Números Complexos
```rust
use avila_arrow::scientific::Complex64;

let c1 = Complex64::new(3.0, 4.0);
let c2 = Complex64::new(1.0, 2.0);

let magnitude = c1.magnitude();  // 5.0
let phase = c1.phase();
let conjugate = c1.conjugate();

// Operações
let sum = complex_add(&c1, &c2);
let product = complex_mul(&c1, &c2);
```

#### Tensores
```rust
use avila_arrow::scientific::Tensor4D;

let mut tensor = Tensor4D::<f64>::new([2, 3, 4, 5], 0.0);

tensor.set(0, 1, 2, 3, 42.0);
let value = tensor.get(0, 1, 2, 3);  // Some(&42.0)

let shape = tensor.shape();  // [2, 3, 4, 5]
```

#### Spinors
```rust
use avila_arrow::scientific::Spinor;

let spin_up = Spinor::spin_up();
let spin_down = Spinor::spin_down();

let custom = Spinor::new(1.0, 0.0, 0.0, 1.0);
let normalized = custom.normalize();
```

## 🚀 Performance

Todas as operações são implementadas em Rust puro, sem dependências externas:

- **Zero-copy**: Operações sem alocação desnecessária
- **SIMD-ready**: Estruturas otimizadas para vetorização futura
- **Cache-friendly**: Layout de memória otimizado
- **Parallelizable**: Preparado para paralelização

## 📊 Benchmarks

Execute os benchmarks:

```bash
cargo bench --bench array_ops
cargo bench --bench compute
cargo bench --bench simd
cargo bench --bench compression
```

## 🧪 Testes

Execute a suite de testes:

```bash
# Todos os testes
cargo test

# Testes específicos
cargo test --test integration
cargo test --lib

# Com output verbose
cargo test -- --nocapture
```

## 📚 Exemplos

### 01. Arrays Básicos
```bash
cargo run --example 01_basic_arrays
```

### 02. RecordBatches
```bash
cargo run --example 02_record_batches
```

### 03. Schemas
```bash
cargo run --example 03_schemas
```

### 04. Compute Operations
```bash
cargo run --example 04_compute
```

### 08. Tipos Científicos
```bash
cargo run --example 08_scientific
```

## 🏗️ Arquitetura

```
src/
├── datatypes/      # Sistema de tipos
│   ├── primitive.rs
│   ├── temporal.rs
│   ├── decimal.rs
│   ├── nested.rs
│   └── fixed.rs
├── array/          # Implementações de arrays
│   ├── primitive.rs
│   ├── boolean.rs
│   ├── string.rs
│   └── builder/
├── buffer/         # Gerenciamento de memória
│   ├── buffer.rs
│   ├── bitmap.rs
│   └── pool.rs
├── compute/        # Kernels de computação
│   ├── aggregate.rs
│   ├── arithmetic.rs
│   ├── comparison.rs
│   ├── boolean.rs
│   ├── cast.rs
│   ├── filter.rs
│   ├── sort.rs
│   ├── window.rs
│   ├── hash.rs
│   └── statistics.rs
├── scientific/     # Tipos científicos
│   ├── quaternion.rs
│   ├── complex.rs
│   ├── tensor.rs
│   ├── spinor.rs
│   ├── ops.rs
│   ├── units.rs
│   └── constants.rs
└── compression/    # Codecs de compressão
    ├── rle.rs
    ├── delta.rs
    ├── dictionary.rs
    └── bitpack.rs
```

## 🤝 Contribuindo

Contribuições são bem-vindas! Este projeto segue a filosofia de **código genuíno e puro**:

- ✅ Implementações em Rust puro
- ✅ Zero dependências externas (exceto std)
- ✅ Documentação em português
- ✅ Testes completos
- ✅ Benchmarks

## 📄 Licença

MIT License - veja [LICENSE](LICENSE) para detalhes.

## 🌟 Roadmap

- [x] Tipos primitivos completos
- [x] Operações de compute básicas
- [x] Tipos científicos (Quaternion, Complex, Tensor, Spinor)
- [x] Estatísticas avançadas
- [x] Window functions
- [x] Hash e unique operations
- [ ] SIMD otimizations (AVX2, AVX-512, NEON)
- [ ] Compressão avançada
- [ ] IPC format completo
- [ ] Python bindings (PyO3)
- [ ] WebAssembly support
- [ ] C API (FFI)

## 📞 Contato

Projeto Avila - Computação Científica Brasileira

---

**Made with ❤️ in Brazil 🇧🇷**
