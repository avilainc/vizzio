# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Core cache implementation with `DistributedCache`
- `ManagedCache` with automatic eviction policies
- Eviction policies: LRU, LFU, FIFO, NoEviction
- `CacheStats` for hit/miss tracking and metrics
- `CacheConfig` with builder pattern
- `CacheBuilder` for convenient cache creation
- Iterator support (keys, values, iter, iter_mut)
- Extended operations (remove, contains_key, clear, len, is_empty)
- `ShardedCache` for hash-based distribution
- `TtlCache` with Time-to-Live support
- `SharedCache` with Arc-based sharing (RefCell)
- Batch operations (get_batch, insert_batch, remove_batch)
- Comprehensive error handling with `CacheError`
- Full `no_std` support (alloc only)
- Extensive test coverage for all modules
- Usage examples in `examples.rs`
- README with documentation
- TODO roadmap

### TODO
- Real serde integration (currently placeholder)
- Actual timestamp implementation for TTL
- Thread-safe concurrent cache (std::sync or spin)
- Metrics trait implementation
- Hybrid TTL + LRU/LFU eviction
- Cache warming strategies
- Persistence layer
- Compression support
- Benchmarks
- CI/CD pipeline

## [0.1.0] - 2025-12-05 (Initial Development)

### Added
- Initial project structure
- Basic cache operations
- Module organization
- Documentation foundation

---

**Note**: This project is currently in alpha development. APIs may change.
