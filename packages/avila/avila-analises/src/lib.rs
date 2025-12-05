//! # Avila Analytics Engine
//!
//! High-performance analytics library for behavioral analysis and Industry 4.0.
//!
//! ## Features
//!
//! - **Event Tracking**: Real-time event capture and processing
//! - **Funnel Analysis**: Conversion analysis and drop-off identification
//! - **User Segmentation**: Dynamic behavioral segmentation
//! - **Cohort Analysis**: Cohort and retention analysis
//! - **Machine Learning**: Predictive analytics (RFM, CLV, churn)
//! - **Industry 4.0**: IoT, OEE, predictive maintenance
//!
//! ## Quick Example
//!
//! ```no_run
//! use avila_analises::models::*;
//! use avila_analises::tracker::*;
//! # async {
//! let store = EventStore::new();
//! // Track events...
//! # };
//! ```

pub mod error;
pub mod models;
pub mod tracker;
pub mod funnel;
pub mod cohort;
pub mod segmentation;
pub mod prediction;
pub mod dashboard;
pub mod storage;
pub mod api;
pub mod websocket;
pub mod export;
pub mod industry40;

// Re-export commonly used types
pub use error::{AvilaError, AvilaResult};
pub use models::{BehaviorEvent, EventType, EventContext, DeviceType};
