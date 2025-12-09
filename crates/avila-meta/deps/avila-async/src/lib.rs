//! Placeholder crate for avila-async.

/// Returns the version string of this placeholder crate.
pub fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}
