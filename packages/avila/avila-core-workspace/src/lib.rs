//! avila-core-workspace - Avila workspace module

#![doc = include_str!("../README.md")]

pub fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}
