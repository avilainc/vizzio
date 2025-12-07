//! # avila-cache
//!
//! A distributed cache implementation with multiple features:
//! - Basic cache operations
//! - Eviction policies (LRU, LFU, FIFO)
//! - Statistics collection
//! - Sharding support
//! - Iterator support
//! - Builder pattern for easy creation
//! - TTL (Time-to-Live) support
//! - Batch operations
//! - Shared cache with Arc
//!
//! ## Quick Start
//!
//! ```rust,ignore
//! use avila_cache::builder::CacheBuilder;
//!
//! // Create an LRU cache
//! let mut cache = CacheBuilder::new()
//!     .max_capacity(100)
//!     .with_lru()
//!     .unwrap();
//!
//! cache.insert("key", "value");
//! assert_eq!(cache.get(&"key"), Some(&"value"));
//! ```

extern crate alloc;

// Core modules
pub mod cache;
pub mod operations;
pub mod error;
pub mod config;

// Feature modules
pub mod eviction;
pub mod eviction_advanced;
pub mod iter;
pub mod stats;
pub mod sharding;
pub mod serde;
pub mod builder;
pub mod examples;
pub mod ttl;
pub mod concurrent;
pub mod batch;
pub mod traits;

// Re-exports
pub use cache::{DistributedCache, ManagedCache};
pub use error::{CacheError, CacheResult};
pub use config::CacheConfig;
pub use stats::CacheStats;
pub use eviction::{EvictionPolicy, NoEviction, FifoPolicy, LruPolicy, LfuPolicy};
pub use eviction_advanced::{TtlLruPolicy, TtlLfuPolicy, AdaptivePolicy, SizeBasedPolicy, RandomPolicy};
pub use sharding::ShardedCache;
pub use ttl::{TtlCache, TtlEntry, Timestamp, TimeSource};
pub use concurrent::SharedCache;
pub use batch::BatchResult;
pub use traits::{Metrics, AdvancedMetrics, Histogram};
