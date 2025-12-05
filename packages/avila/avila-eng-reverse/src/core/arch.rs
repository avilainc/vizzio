#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Architecture {
    X86,
    X86_64,
    ARM,
    ARM64,
    MIPS,
    MIPS64,
    PowerPC,
    SPARC,
    Unknown,
}

impl Architecture {
    pub fn bits(&self) -> usize {
        match self {
            Architecture::X86 | Architecture::ARM | Architecture::MIPS => 32,
            Architecture::X86_64 | Architecture::ARM64 | Architecture::MIPS64 => 64,
            Architecture::PowerPC => 32,
            Architecture::SPARC => 32,
            Architecture::Unknown => 0,
        }
    }

    pub fn name(&self) -> &str {
        match self {
            Architecture::X86 => "x86",
            Architecture::X86_64 => "x86_64",
            Architecture::ARM => "ARM",
            Architecture::ARM64 => "ARM64",
            Architecture::MIPS => "MIPS",
            Architecture::MIPS64 => "MIPS64",
            Architecture::PowerPC => "PowerPC",
            Architecture::SPARC => "SPARC",
            Architecture::Unknown => "Unknown",
        }
    }

    pub fn is_64bit(&self) -> bool {
        self.bits() == 64
    }
}
