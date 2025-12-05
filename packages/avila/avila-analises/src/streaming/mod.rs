//! Real-time streaming module
//!
//! Provides streaming capabilities:
//! - Stream processing
//! - Window operations
//! - Aggregations
//! - Event time processing

pub mod processor;
pub mod window;
pub mod aggregation;
pub mod kafka_connector;
pub mod kinesis_connector;

pub use processor::*;
pub use window::*;
pub use aggregation::*;
pub use kafka_connector::*;
pub use kinesis_connector::*;
