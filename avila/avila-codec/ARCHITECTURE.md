# Avila Codec - Arquitetura e Estrutura

## 📐 Visão Geral

O **avila-codec** é uma biblioteca Rust para encoding/decoding de múltiplos formatos, otimizada para performance e segurança, com suporte para ambientes `no_std`.

## 🏗️ Arquitetura

```
avila-codec/
├── src/
│   ├── lib.rs              # Módulo principal e exports
│   ├── error.rs            # Sistema de erros interno
│   ├── traits.rs           # Traits genéricos (Encode, Decode, Checksum)
│   │
│   ├── Encoding Modules
│   ├── hex.rs              # Hexadecimal (48656c6c6f)
│   ├── base64.rs           # Base64 (SGVsbG8=)
│   ├── base58.rs           # Bitcoin-style (9Ajdvzr)
│   ├── base32.rs           # RFC 4648 (JBSWY3DP)
│   ├── base85.rs           # ASCII85 + Z85
│   ├── url.rs              # Percent encoding
│   ├── multibase.rs        # IPFS-style auto-detect
│   │
│   ├── checksum/           # Checksums e hashing
│   │   ├── mod.rs
│   │   ├── crc.rs          # CRC32, CRC16
│   │   └── xxhash.rs       # XXHash32
│   │
│   ├── binary/             # Formatos binários
│   │   ├── mod.rs
│   │   └── varint.rs       # LEB128, ZigZag
│   │
│   ├── compression/        # Compressão
│   │   ├── mod.rs
│   │   └── lz4.rs          # LZ4 implementado
│   │
│   ├── network/            # Formatos de rede
│   │   ├── mod.rs
│   │   └── quoted_printable.rs
│   │
│   └── simd/               # Aceleração SIMD
│       ├── mod.rs
│       ├── avx2.rs         # Intel/AMD
│       └── neon.rs         # ARM
│
├── examples/
│   ├── basic.rs            # Exemplo geral
│   ├── checksums.rs        # CRC e XXHash
│   ├── varint.rs           # Inteiros variáveis
│   └── multibase.rs        # Multibase variants
│
├── benches/
│   └── codec_bench.rs      # Benchmarks (Criterion)
│
├── tests/                  # Testes de integração (futuro)
│
├── Cargo.toml
├── README.md
├── CHANGELOG.md
├── LICENSE-MIT
└── LICENSE-APACHE
```

## 🔧 Componentes Principais

### 1. Sistema de Erros (`error.rs`)
```rust
pub enum ErrorKind {
    InvalidInput,
    Unsupported,
    BufferTooSmall,
    EncodingError,
    DecodingError,
}

pub struct Error { kind, message }
pub type Result<T> = core::result::Result<T, Error>
```

### 2. Traits Genéricos (`traits.rs`)
```rust
trait Encode { fn encode(&self) -> Result<Vec<u8>>; }
trait Decode { fn decode(data: &[u8]) -> Result<Self>; }
trait Checksum { fn digest(data: &[u8]) -> Self::Output; }
trait StreamingEncoder { fn update/finalize }
```

### 3. Módulos de Encoding

#### Hex
- Lowercase/uppercase
- Slice-based (zero-copy quando possível)
- Constant-time operations

#### Base64
- RFC 4648 standard
- Padding correto
- ~1.8 GB/s encoding

#### Base58
- Bitcoin alphabet (sem 0OIl)
- Leading zeros preservation
- Usado em crypto/blockchain

#### Base32
- Standard + Hex variants
- Case-insensitive decode
- Bom para URLs case-insensitive

#### Base85
- ASCII85 (Adobe)
- Z85 (ZeroMQ)
- Maior densidade que Base64

#### URL Encoding
- Percent encoding (RFC 3986)
- Path/query variants
- Form URL encoding (+ para espaços)

#### Multibase
- Auto-detecting com prefixos
- Suporta todos os encodings
- Ideal para IPFS/distributed systems

### 4. Checksums

#### CRC32/16
- Lookup table para performance
- Incremental calculation
- ~8 GB/s throughput

#### XXHash32
- Non-cryptographic hash
- Seed support
- ~15 GB/s throughput
- Ideal para hash tables

### 5. Binary Formats

#### VarInt
- **LEB128**: Little-Endian Base 128
  - Unsigned: 0-127 em 1 byte
  - Signed: sign extension correto
- **ZigZag**: Signed para unsigned mapping
  - -1 → 1, 1 → 2, -2 → 3, etc.
- **Space savings**: até 87.5% para valores pequenos

### 6. Network Formats

#### Quoted-Printable
- Email-safe (RFC 2045)
- Soft line breaks
- Printable ASCII preservation

## 📊 Performance

### Benchmarks (hardware moderno)
```
Hex encoding:     ~2.5 GB/s
Base64 encoding:  ~1.8 GB/s
CRC32:            ~8 GB/s
XXHash32:         ~15 GB/s
```

### Otimizações
1. **Lookup tables** para CRC e encoding
2. **Constant-time** para operações sensíveis
3. **Zero-copy** quando possível
4. **SIMD-ready** (infraestrutura preparada)

## 🎯 no_std Support

Todos os módulos funcionam sem std:
```toml
[dependencies]
avila-codec = { version = "0.1", default-features = false }
```

Usa apenas:
- `core::` - primitivos
- `alloc::` - String, Vec

## 🧪 Testes

### Cobertura
- **45 testes unitários** (100% passing)
- Roundtrip tests para todos encodings
- Edge cases (empty, leading zeros, etc.)
- Incremental operations (CRC)

### Executar
```bash
cargo test --lib              # Testes unitários
cargo test --all              # Todos os testes
cargo run --example basic     # Exemplo básico
```

## 🚀 Roadmap

### Phase 1: ✅ COMPLETO
- [x] Core encodings (hex, base64, base58, base32, base85)
- [x] URL encoding
- [x] Multibase
- [x] Checksums (CRC, XXHash)
- [x] VarInt (LEB128, ZigZag)
- [x] Network formats (quoted-printable)

### Phase 2: 🚧 Em Progresso
- [ ] SIMD acceleration (AVX2, NEON)
- [ ] Streaming API
- [ ] Async support

### Phase 3: 🚧 Em Desenvolvimento
- [x] Compression - LZ4
- [ ] Compression - Deflate, Brotli
- [ ] Binary formats (MessagePack, CBOR)
- [ ] Extended network (Punycode, ASCII Armor)

## 💡 Princípios de Design

1. **Zero Dependencies**: Apenas `alloc`, sem deps externas
2. **no_std First**: Funciona em embedded
3. **Performance**: Otimizado mas legível
4. **Safety**: Sem unsafe (exceto SIMD futuro)
5. **Ergonomia**: API simples e consistente
6. **Testing**: Cobertura extensiva

## 📚 Uso

### Básico
```rust
use avila_codec::prelude::*;

let data = b"Hello";
let hex = hex::encode(data);           // "48656c6c6f"
let b64 = base64::encode(data);        // "SGVsbG8="
let crc = crc::crc32(data);            // 0xF7D18982
```

### Multibase
```rust
let mb = multibase::encode_base58btc(data);  // "z9Ajdvzr"
let decoded = multibase::decode(&mb)?;       // Auto-detect
```

### VarInt
```rust
let encoded = varint::encode_varint_u64(12345);  // [185, 96]
let (value, len) = varint::decode_varint_u64(&encoded)?;
```

## 🎓 Recursos de Aprendizado

- [RFC 4648](https://tools.ietf.org/html/rfc4648) - Base encodings
- [RFC 3986](https://tools.ietf.org/html/rfc3986) - URL encoding
- [Multibase spec](https://github.com/multiformats/multibase)
- [LEB128](https://en.wikipedia.org/wiki/LEB128)
- [XXHash](https://github.com/Cyan4973/xxHash)

## 📄 Licença

MIT OR Apache-2.0
