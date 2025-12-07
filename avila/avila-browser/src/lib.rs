//! # Avila Browser
//!
//! High-assurance web browser implementing multi-layer onion routing architecture
//!
//! ## Adversarial Model
//!
//! - **Passive Adversary**: Observes network traffic without modification capabilities
//! - **Active Adversary**: Possesses packet manipulation, injection, and dropping capabilities
//! - **Global Adversary**: Exhibits omniscient network monitoring capabilities (nation-state level)
//!
//! ## Cryptographic Security Properties
//!
//! - **Sender-Receiver Anonymity**: Computational unlinkability of communicating parties
//! - **Session Unlinkability**: Infeasibility of correlating distinct protocol sessions
//! - **Communication Unobservability**: Statistical indistinguishability from random noise
//! - **Perfect Forward Secrecy**: Retroactive security guarantee under key compromise
//! - **Traffic Analysis Resistance**: Countermeasures against temporal and volumetric side-channels

// Core modules
pub mod core;
pub mod layers;
pub mod protocols;
pub mod rendering;

// New expansion modules
pub mod crypto;
pub mod network;
pub mod privacy;
pub mod storage;
pub mod api;
pub mod cli;

// Re-exports
pub use core::{Browser, BrowserConfig, Request, Response, BrowserError};
pub use layers::{LayerStack, ProtectionLayer, LayerType};
pub use protocols::{HttpProtocol, QuicProtocol, DohProtocol};
pub use rendering::{Dom, CssParser, LayoutEngine};

// New module re-exports
pub use crypto::{Cipher, Signer, Hasher, KeyExchange, SecurityLevel};
pub use network::{TcpStream, UdpSocket, TlsConnector, ConnectionPool};
pub use privacy::{FingerprintProtection, TrackerBlocker, CookieJar, ReferrerPolicy};
pub use storage::{Database, FileStorage, Keychain};
pub use api::{RestApi, GrpcServer, WebDriverSession};

/// Library version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
