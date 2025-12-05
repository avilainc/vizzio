// Emulation engine module
pub mod engine;
pub mod memory;
pub mod context;
pub mod hooks;
pub mod shellcode_analyzer;

pub use engine::EmulationEngine;
pub use context::CpuContext;
pub use shellcode_analyzer::ShellcodeAnalyzer;
