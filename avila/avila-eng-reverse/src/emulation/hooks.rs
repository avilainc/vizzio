// Emulation hooks for instrumentation
use std::collections::HashMap;

/// Hook manager for emulation instrumentation
pub struct HookManager {
    code_hooks: HashMap<u64, CodeHook>,
    memory_hooks: Vec<MemoryHook>,
    interrupt_hooks: HashMap<u32, InterruptHook>,
}

#[derive(Debug, Clone)]
pub struct CodeHook {
    pub address: u64,
    pub callback: String, // Function pointer in real implementation
}

#[derive(Debug, Clone)]
pub struct MemoryHook {
    pub address_range: (u64, u64),
    pub hook_type: MemoryHookType,
    pub callback: String,
}

#[derive(Debug, Clone, PartialEq)]
pub enum MemoryHookType {
    Read,
    Write,
    ReadWrite,
}

#[derive(Debug, Clone)]
pub struct InterruptHook {
    pub interrupt_number: u32,
    pub callback: String,
}

impl HookManager {
    pub fn new() -> Self {
        Self {
            code_hooks: HashMap::new(),
            memory_hooks: Vec::new(),
            interrupt_hooks: HashMap::new(),
        }
    }

    /// Add code execution hook
    pub fn add_code_hook(&mut self, address: u64, callback: String) {
        self.code_hooks.insert(address, CodeHook { address, callback });
    }

    /// Add memory access hook
    pub fn add_memory_hook(&mut self, start: u64, end: u64, hook_type: MemoryHookType, callback: String) {
        self.memory_hooks.push(MemoryHook {
            address_range: (start, end),
            hook_type,
            callback,
        });
    }

    /// Add interrupt hook
    pub fn add_interrupt_hook(&mut self, interrupt: u32, callback: String) {
        self.interrupt_hooks.insert(interrupt, InterruptHook {
            interrupt_number: interrupt,
            callback,
        });
    }

    /// Remove code hook
    pub fn remove_code_hook(&mut self, address: u64) {
        self.code_hooks.remove(&address);
    }

    /// Clear all hooks
    pub fn clear_all(&mut self) {
        self.code_hooks.clear();
        self.memory_hooks.clear();
        self.interrupt_hooks.clear();
    }
}
