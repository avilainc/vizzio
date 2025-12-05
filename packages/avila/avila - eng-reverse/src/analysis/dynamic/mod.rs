// Dynamic analysis submodule
pub mod sandbox;
pub mod hooking;
pub mod monitor;
pub mod network_capture;
pub mod file_monitor;
pub mod registry_monitor;
pub mod tracer;

pub use sandbox::Sandbox;
pub use hooking::HookingEngine;
pub use monitor::BehaviorMonitor;
