//! GPU acceleration support

pub mod backends;

#[cfg(feature = "gpu")]
pub mod cuda;

#[cfg(feature = "gpu-wgpu")]
pub mod rocm;
