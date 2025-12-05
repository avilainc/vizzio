//! Unified backend selection for AVX GPU
//!
//! Automatically selects the best available backend for the current system

use std::fmt;

/// Available GPU backends
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BackendType {
    /// NVIDIA CUDA (highest performance for NVIDIA GPUs)
    Cuda,
    /// AMD ROCm/HIP
    Rocm,
    /// Apple Metal (Apple Silicon & Intel Macs)
    Metal,
    /// Khronos Vulkan (cross-vendor)
    Vulkan,
    /// WebGPU via wgpu (browser & portable)
    WebGpu,
    /// CPU fallback
    Cpu,
}

impl fmt::Display for BackendType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BackendType::Cuda => write!(f, "CUDA"),
            BackendType::Rocm => write!(f, "ROCm"),
            BackendType::Metal => write!(f, "Metal"),
            BackendType::Vulkan => write!(f, "Vulkan"),
            BackendType::WebGpu => write!(f, "WebGPU"),
            BackendType::Cpu => write!(f, "CPU"),
        }
    }
}

impl BackendType {
    /// Get all available backends for current platform
    pub fn available() -> Vec<BackendType> {
        let mut backends = Vec::new();

        // Check CUDA availability
        #[cfg(feature = "cuda")]
        if Self::detect_cuda() {
            backends.push(BackendType::Cuda);
        }

        // Check ROCm availability
        #[cfg(feature = "rocm")]
        if Self::detect_rocm() {
            backends.push(BackendType::Rocm);
        }

        // Check Metal (macOS/iOS only)
        #[cfg(target_os = "macos")]
        if Self::detect_metal() {
            backends.push(BackendType::Metal);
        }

        // Check Vulkan (most platforms)
        #[cfg(feature = "vulkan")]
        if Self::detect_vulkan() {
            backends.push(BackendType::Vulkan);
        }

        // WebGPU always available (with fallback)
        #[cfg(feature = "webgpu")]
        backends.push(BackendType::WebGpu);

        // CPU fallback always available
        backends.push(BackendType::Cpu);

        backends
    }

    /// Auto-select best backend for current system
    pub fn auto_select() -> BackendType {
        let available = Self::available();

        // Priority order
        for backend in &[
            BackendType::Cuda,
            BackendType::Rocm,
            BackendType::Metal,
            BackendType::Vulkan,
            BackendType::WebGpu,
            BackendType::Cpu,
        ] {
            if available.contains(backend) {
                return *backend;
            }
        }

        BackendType::Cpu
    }

    fn detect_cuda() -> bool {
        // Check for CUDA runtime
        #[allow(unreachable_code)]
        {
            std::env::var("CUDA_PATH").is_ok() || std::env::var("CUDA_HOME").is_ok()
        }
    }

    fn detect_rocm() -> bool {
        // Check for ROCm installation
        #[allow(unreachable_code)]
        {
            std::env::var("ROCM_HOME").is_ok() || std::env::var("HIP_PATH").is_ok()
        }
    }

    fn detect_metal() -> bool {
        // Metal is always available on macOS
        #[allow(unreachable_code)]
        {
            cfg!(target_os = "macos")
        }
    }

    fn detect_vulkan() -> bool {
        // Check for Vulkan loader
        #[allow(unreachable_code)]
        {
            std::env::var("VK_ICD_FILENAMES").is_ok() || cfg!(windows)
        }
    }
}

/// Backend information and statistics
#[derive(Debug, Clone)]
pub struct BackendInfo {
    /// Backend type
    pub backend: BackendType,
    /// Backend version
    pub version: String,
    /// GPU devices available
    pub devices: usize,
    /// Feature support level
    pub features: String,
}

impl fmt::Display for BackendInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} v{} ({} devices) [{}]",
            self.backend, self.version, self.devices, self.features
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_backend_type_display() {
        assert_eq!(BackendType::Cuda.to_string(), "CUDA");
        assert_eq!(BackendType::Vulkan.to_string(), "Vulkan");
        assert_eq!(BackendType::Cpu.to_string(), "CPU");
    }

    #[test]
    fn test_available_backends() {
        let backends = BackendType::available();
        assert!(!backends.is_empty());
        assert!(backends.contains(&BackendType::Cpu)); // CPU fallback always available
    }

    #[test]
    fn test_auto_select() {
        let selected = BackendType::auto_select();
        // CPU is always available as fallback
        assert_eq!(selected, BackendType::Cpu);
    }

    #[test]
    fn test_backend_info_display() {
        let info = BackendInfo {
            backend: BackendType::Cuda,
            version: "12.0".to_string(),
            devices: 2,
            features: "compute_capability:8.0".to_string(),
        };
        let display = info.to_string();
        assert!(display.contains("CUDA"));
        assert!(display.contains("v12.0"));
    }
}
