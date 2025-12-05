// CPU context management
use std::collections::HashMap;

/// CPU context for emulation
pub struct CpuContext {
    registers: HashMap<String, u64>,
    flags: u64,
    pc: u64,
    sp: u64,
}

impl CpuContext {
    pub fn new() -> Self {
        Self {
            registers: HashMap::new(),
            flags: 0,
            pc: 0,
            sp: 0,
        }
    }

    /// Set register value
    pub fn set_register(&mut self, name: &str, value: u64) {
        self.registers.insert(name.to_string(), value);
    }

    /// Get register value
    pub fn get_register(&self, name: &str) -> Option<u64> {
        self.registers.get(name).copied()
    }

    /// Set program counter
    pub fn set_pc(&mut self, pc: u64) {
        self.pc = pc;
    }

    /// Get program counter
    pub fn get_pc(&self) -> u64 {
        self.pc
    }

    /// Set stack pointer
    pub fn set_sp(&mut self, sp: u64) {
        self.sp = sp;
    }

    /// Get stack pointer
    pub fn get_sp(&self) -> u64 {
        self.sp
    }

    /// Set flags
    pub fn set_flags(&mut self, flags: u64) {
        self.flags = flags;
    }

    /// Get flags
    pub fn get_flags(&self) -> u64 {
        self.flags
    }

    /// Save context snapshot
    pub fn snapshot(&self) -> ContextSnapshot {
        ContextSnapshot {
            registers: self.registers.clone(),
            flags: self.flags,
            pc: self.pc,
            sp: self.sp,
        }
    }

    /// Restore from snapshot
    pub fn restore(&mut self, snapshot: &ContextSnapshot) {
        self.registers = snapshot.registers.clone();
        self.flags = snapshot.flags;
        self.pc = snapshot.pc;
        self.sp = snapshot.sp;
    }
}

#[derive(Debug, Clone)]
pub struct ContextSnapshot {
    registers: HashMap<String, u64>,
    flags: u64,
    pc: u64,
    sp: u64,
}
