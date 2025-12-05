use crate::core::{Architecture, types::Result};
use crate::internal::X86Disassembler;

pub struct Disassembler {
    arch: Architecture,
}

impl Disassembler {
    pub fn new(arch: Architecture) -> Self {
        Self { arch }
    }

    pub fn disassemble(&self, code: &[u8], address: u64, count: usize) -> Result<Vec<DisasmInstruction>> {
        match self.arch {
            Architecture::X86 | Architecture::X86_64 => {
                self.disassemble_x86(code, address, count)
            }
            _ => Err(crate::core::types::DeriaxError::DisassemblyError(
                format!("Arquitetura {} não suportada ainda", self.arch.name())
            )),
        }
    }

    fn disassemble_x86(&self, code: &[u8], address: u64, count: usize) -> Result<Vec<DisasmInstruction>> {
        let is_64bit = self.arch == Architecture::X86_64;
        let disasm = X86Disassembler::new(is_64bit);
        let internal_instructions = disasm.disassemble(code, address, count);

        let instructions = internal_instructions.into_iter().map(|instr| {
            DisasmInstruction {
                address: instr.address,
                bytes: instr.bytes,
                mnemonic: instr.mnemonic,
                operands: instr.operands,
                size: instr.length,
            }
        }).collect();

        Ok(instructions)
    }

    pub fn disassemble_function(&self, code: &[u8], start: u64) -> Result<Vec<DisasmInstruction>> {
        // Desassemblar até encontrar RET
        self.disassemble(code, start, 1000)
    }
}

#[derive(Debug, Clone)]
pub struct DisasmInstruction {
    pub address: u64,
    pub bytes: Vec<u8>,
    pub mnemonic: String,
    pub operands: String,
    pub size: usize,
}

impl DisasmInstruction {
    pub fn to_string(&self) -> String {
        let bytes_hex = self.bytes.iter()
            .map(|b| format!("{:02x}", b))
            .collect::<Vec<_>>()
            .join(" ");

        format!("0x{:016x}: {:20} {} {}",
                self.address,
                bytes_hex,
                self.mnemonic,
                self.operands)
    }
}
