use crate::core::{Binary, Disassembler, Architecture};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RopGadget {
    pub address: u64,
    pub instructions: Vec<String>,
    pub category: GadgetCategory,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GadgetCategory {
    PopReg,      // pop reg; ret
    MovReg,      // mov reg, reg; ret
    LoadMem,     // mov reg, [mem]; ret
    StoreMem,    // mov [mem], reg; ret
    Arithmetic,  // add/sub/xor reg, reg; ret
    Syscall,     // syscall; ret / int 0x80; ret
    Jmp,         // jmp reg
    Call,        // call reg
    Other,
}

pub struct RopGadgetFinder {
    arch: Architecture,
}

impl RopGadgetFinder {
    pub fn new(arch: Architecture) -> Self {
        Self { arch }
    }

    pub fn find_gadgets(&self, binary: &Binary, max_instructions: usize) -> Vec<RopGadget> {
        let mut gadgets = Vec::new();

        // Procurar por instruções RET em seções executáveis
        for section in &binary.sections {
            if (section.characteristics & 0x20000000) == 0 {
                continue; // Não é executável
            }

            let section_start = section.raw_address as usize;
            let section_end = section_start + section.raw_size as usize;

            if section_end > binary.data.len() {
                continue;
            }

            let section_data = &binary.data[section_start..section_end];

            // Procurar bytes de RET (0xC3, 0xC2)
            for (offset, window) in section_data.windows(1).enumerate() {
                if window[0] == 0xC3 || window[0] == 0xC2 {
                    // Encontrou RET, tentar desassemblar para trás
                    let ret_addr = section.virtual_address + offset as u64;

                    if let Some(gadget) = self.extract_gadget(
                        section_data,
                        offset,
                        ret_addr,
                        max_instructions,
                    ) {
                        gadgets.push(gadget);
                    }
                }
            }
        }

        // Remover duplicatas e ordenar por utilidade
        Self::deduplicate_gadgets(gadgets)
    }

    fn extract_gadget(
        &self,
        data: &[u8],
        ret_offset: usize,
        ret_addr: u64,
        max_instructions: usize,
    ) -> Option<RopGadget> {
        // Tentar desassemblar de 1 a max_instructions antes do RET
        for lookback in 1..=15 {
            if lookback > ret_offset {
                break;
            }

            let start_offset = ret_offset - lookback;
            let gadget_data = &data[start_offset..=ret_offset];

            let disasm = Disassembler::new(self.arch);
            let start_addr = ret_addr - lookback as u64;

            if let Ok(instructions) = disasm.disassemble(gadget_data, start_addr, max_instructions + 1) {
                if instructions.len() <= max_instructions &&
                   instructions.len() > 0 &&
                   Self::is_valid_gadget(&instructions) {

                    let instr_strings: Vec<String> = instructions.iter()
                        .map(|i| format!("{} {}", i.mnemonic, i.operands))
                        .collect();

                    let category = Self::categorize_gadget(&instructions);

                    return Some(RopGadget {
                        address: start_addr,
                        instructions: instr_strings,
                        category,
                    });
                }
            }
        }

        None
    }

    fn is_valid_gadget(instructions: &[crate::core::disasm::DisasmInstruction]) -> bool {
        if instructions.is_empty() {
            return false;
        }

        // Última instrução deve ser RET
        let last = &instructions[instructions.len() - 1];
        if !last.mnemonic.to_lowercase().starts_with("ret") {
            return false;
        }

        // Não deve conter instruções inválidas
        for instr in instructions {
            let mnem = instr.mnemonic.to_lowercase();
            if mnem.contains("invalid") || mnem.is_empty() {
                return false;
            }
        }

        true
    }

    fn categorize_gadget(instructions: &[crate::core::disasm::DisasmInstruction]) -> GadgetCategory {
        if instructions.is_empty() {
            return GadgetCategory::Other;
        }

        let first = &instructions[0].mnemonic.to_lowercase();

        if first.starts_with("pop") {
            GadgetCategory::PopReg
        } else if first.starts_with("mov") {
            if instructions[0].operands.contains('[') {
                GadgetCategory::LoadMem
            } else {
                GadgetCategory::MovReg
            }
        } else if first == "syscall" || first == "int" {
            GadgetCategory::Syscall
        } else if first.starts_with("jmp") {
            GadgetCategory::Jmp
        } else if first.starts_with("call") {
            GadgetCategory::Call
        } else if first.starts_with("add") || first.starts_with("sub") ||
                  first.starts_with("xor") || first.starts_with("and") {
            GadgetCategory::Arithmetic
        } else {
            GadgetCategory::Other
        }
    }

    fn deduplicate_gadgets(mut gadgets: Vec<RopGadget>) -> Vec<RopGadget> {
        let mut seen = HashMap::new();
        let mut unique = Vec::new();

        for gadget in gadgets {
            let key = gadget.instructions.join(";");
            if !seen.contains_key(&key) {
                seen.insert(key, true);
                unique.push(gadget);
            }
        }

        unique
    }

    pub fn find_specific_gadget(&self, binary: &Binary, pattern: &str) -> Vec<RopGadget> {
        let all_gadgets = self.find_gadgets(binary, 5);

        all_gadgets.into_iter()
            .filter(|g| g.instructions.iter().any(|i| i.contains(pattern)))
            .collect()
    }

    pub fn build_rop_chain(&self, gadgets: &[RopGadget], goal: &str) -> Vec<u64> {
        // Implementação básica de construção de ROP chain
        // Em produção, seria muito mais sofisticado
        let mut chain = Vec::new();

        match goal {
            "pop_rdi" => {
                // Procurar gadget pop rdi; ret
                if let Some(gadget) = gadgets.iter()
                    .find(|g| g.instructions[0].contains("pop") && g.instructions[0].contains("rdi")) {
                    chain.push(gadget.address);
                }
            },
            _ => {}
        }

        chain
    }
}
