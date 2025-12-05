// Disassembler x86/x64 nativo 100% Rust - sem dependências externas
// Implementação simplificada com os opcodes mais comuns

#[derive(Debug, Clone)]
pub struct X86Instruction {
    pub address: u64,
    pub bytes: Vec<u8>,
    pub mnemonic: String,
    pub operands: String,
    pub length: usize,
}

pub struct X86Disassembler {
    is_64bit: bool,
}

impl X86Disassembler {
    pub fn new(is_64bit: bool) -> Self {
        Self { is_64bit }
    }

    pub fn disassemble(&self, code: &[u8], address: u64, max_instructions: usize) -> Vec<X86Instruction> {
        let mut instructions = Vec::new();
        let mut offset = 0;

        while offset < code.len() && instructions.len() < max_instructions {
            if let Some(instr) = self.decode_instruction(&code[offset..], address + offset as u64) {
                offset += instr.length;
                instructions.push(instr);
            } else {
                // Byte inválido, criar instrução DB
                instructions.push(X86Instruction {
                    address: address + offset as u64,
                    bytes: vec![code[offset]],
                    mnemonic: "db".to_string(),
                    operands: format!("0x{:02x}", code[offset]),
                    length: 1,
                });
                offset += 1;
            }
        }

        instructions
    }

    fn decode_instruction(&self, code: &[u8], address: u64) -> Option<X86Instruction> {
        if code.is_empty() {
            return None;
        }

        let opcode = code[0];
        let (mnemonic, operands, length) = match opcode {
            // NOP
            0x90 => ("nop", "", 1),

            // RET
            0xC3 => ("ret", "", 1),
            0xC2 => {
                if code.len() < 3 {
                    return None;
                }
                let imm = u16::from_le_bytes([code[1], code[2]]);
                ("ret", &format!("0x{:x}", imm), 3)
            }

            // PUSH
            0x50..=0x57 => {
                let reg = Self::get_register(opcode - 0x50, self.is_64bit);
                ("push", &reg, 1)
            }
            0x68 => {
                if code.len() < 5 {
                    return None;
                }
                let imm = i32::from_le_bytes([code[1], code[2], code[3], code[4]]);
                ("push", &format!("0x{:x}", imm), 5)
            }
            0x6A => {
                if code.len() < 2 {
                    return None;
                }
                let imm = code[1] as i8;
                ("push", &format!("0x{:x}", imm), 2)
            }

            // POP
            0x58..=0x5F => {
                let reg = Self::get_register(opcode - 0x58, self.is_64bit);
                ("pop", &reg, 1)
            }

            // MOV
            0x88 => ("mov", "byte ptr [...]", 2),
            0x89 => ("mov", "dword ptr [...]", 2),
            0x8A => ("mov", "..., byte ptr [...]", 2),
            0x8B => ("mov", "..., dword ptr [...]", 2),
            0xA0 => ("mov", "al, [...]", if self.is_64bit { 9 } else { 5 }),
            0xA1 => ("mov", "eax, [...]", if self.is_64bit { 9 } else { 5 }),
            0xA2 => ("mov", "[...], al", if self.is_64bit { 9 } else { 5 }),
            0xA3 => ("mov", "[...], eax", if self.is_64bit { 9 } else { 5 }),
            0xB0..=0xB7 => {
                if code.len() < 2 {
                    return None;
                }
                let reg = Self::get_register_8bit(opcode - 0xB0);
                let imm = code[1];
                ("mov", &format!("{}, 0x{:x}", reg, imm), 2)
            }
            0xB8..=0xBF => {
                if code.len() < 5 {
                    return None;
                }
                let reg = Self::get_register(opcode - 0xB8, self.is_64bit);
                let imm = u32::from_le_bytes([code[1], code[2], code[3], code[4]]);
                ("mov", &format!("{}, 0x{:x}", reg, imm), 5)
            }
            0xC6 => ("mov", "byte ptr [...], imm8", 3),
            0xC7 => ("mov", "dword ptr [...], imm32", 6),

            // CALL
            0xE8 => {
                if code.len() < 5 {
                    return None;
                }
                let offset = i32::from_le_bytes([code[1], code[2], code[3], code[4]]);
                let target = (address as i64 + 5 + offset as i64) as u64;
                ("call", &format!("0x{:x}", target), 5)
            }
            0xFF if code.len() > 1 => {
                let modrm = code[1];
                let reg = (modrm >> 3) & 0x07;
                match reg {
                    2 => ("call", "qword ptr [...]", 2),
                    4 => ("jmp", "qword ptr [...]", 2),
                    6 => ("push", "qword ptr [...]", 2),
                    _ => ("???", "", 2),
                }
            }

            // JMP
            0xE9 => {
                if code.len() < 5 {
                    return None;
                }
                let offset = i32::from_le_bytes([code[1], code[2], code[3], code[4]]);
                let target = (address as i64 + 5 + offset as i64) as u64;
                ("jmp", &format!("0x{:x}", target), 5)
            }
            0xEB => {
                if code.len() < 2 {
                    return None;
                }
                let offset = code[1] as i8;
                let target = (address as i64 + 2 + offset as i64) as u64;
                ("jmp", &format!("0x{:x}", target), 2)
            }

            // Jumps condicionais (Jcc)
            0x70..=0x7F => {
                if code.len() < 2 {
                    return None;
                }
                let condition = Self::get_condition(opcode - 0x70);
                let offset = code[1] as i8;
                let target = (address as i64 + 2 + offset as i64) as u64;
                (&format!("j{}", condition), &format!("0x{:x}", target), 2)
            }
            0x0F if code.len() > 1 && (0x80..=0x8F).contains(&code[1]) => {
                if code.len() < 6 {
                    return None;
                }
                let condition = Self::get_condition(code[1] - 0x80);
                let offset = i32::from_le_bytes([code[2], code[3], code[4], code[5]]);
                let target = (address as i64 + 6 + offset as i64) as u64;
                (&format!("j{}", condition), &format!("0x{:x}", target), 6)
            }

            // Operações aritméticas
            0x00 => ("add", "byte ptr [...], r8", 2),
            0x01 => ("add", "dword ptr [...], r32", 2),
            0x28 => ("sub", "byte ptr [...], r8", 2),
            0x29 => ("sub", "dword ptr [...], r32", 2),
            0x30 => ("xor", "byte ptr [...], r8", 2),
            0x31 => ("xor", "dword ptr [...], r32", 2),
            0x38 => ("cmp", "byte ptr [...], r8", 2),
            0x39 => ("cmp", "dword ptr [...], r32", 2),
            0x80 => ("arith", "byte ptr [...], imm8", 3),
            0x81 => ("arith", "dword ptr [...], imm32", 6),
            0x83 => ("arith", "dword ptr [...], imm8", 3),

            // TEST
            0x84 => ("test", "r/m8, r8", 2),
            0x85 => ("test", "r/m32, r32", 2),
            0xA8 => {
                if code.len() < 2 {
                    return None;
                }
                ("test", &format!("al, 0x{:x}", code[1]), 2)
            }
            0xA9 => {
                if code.len() < 5 {
                    return None;
                }
                let imm = u32::from_le_bytes([code[1], code[2], code[3], code[4]]);
                ("test", &format!("eax, 0x{:x}", imm), 5)
            }

            // LEA
            0x8D => ("lea", "r32, [...]", 2),

            // XCHG
            0x86 => ("xchg", "r/m8, r8", 2),
            0x87 => ("xchg", "r/m32, r32", 2),
            0x91..=0x97 => {
                let reg = Self::get_register(opcode - 0x90, self.is_64bit);
                ("xchg", &format!("eax, {}", reg), 1)
            }

            // INC/DEC
            0x40..=0x47 if !self.is_64bit => {
                let reg = Self::get_register(opcode - 0x40, false);
                ("inc", &reg, 1)
            }
            0x48..=0x4F if !self.is_64bit => {
                let reg = Self::get_register(opcode - 0x48, false);
                ("dec", &reg, 1)
            }
            0xFE => ("inc/dec", "byte ptr [...]", 2),
            0xFF => ("inc/dec", "dword ptr [...]", 2),

            // INT
            0xCC => ("int3", "", 1),
            0xCD => {
                if code.len() < 2 {
                    return None;
                }
                ("int", &format!("0x{:x}", code[1]), 2)
            }

            // Prefixos e instruções multi-byte
            0x0F => {
                if code.len() < 2 {
                    return None;
                }
                return self.decode_two_byte(code, address);
            }

            // REX prefixes (64-bit)
            0x40..=0x4F if self.is_64bit => {
                if code.len() < 2 {
                    return None;
                }
                // Por simplicidade, apenas reportar presença do REX
                return self.decode_instruction(&code[1..], address).map(|mut instr| {
                    instr.bytes.insert(0, opcode);
                    instr.length += 1;
                    instr.address = address;
                    instr
                });
            }

            _ => {
                // Opcode desconhecido
                return None;
            }
        };

        Some(X86Instruction {
            address,
            bytes: code[..length].to_vec(),
            mnemonic: mnemonic.to_string(),
            operands: operands.to_string(),
            length,
        })
    }

    fn decode_two_byte(&self, code: &[u8], address: u64) -> Option<X86Instruction> {
        if code.len() < 2 {
            return None;
        }

        let second = code[1];
        let (mnemonic, operands, length) = match second {
            // MOVZX / MOVSX
            0xB6 => ("movzx", "r32, r/m8", 3),
            0xB7 => ("movzx", "r32, r/m16", 3),
            0xBE => ("movsx", "r32, r/m8", 3),
            0xBF => ("movsx", "r32, r/m16", 3),

            // CMOV
            0x40..=0x4F => {
                let condition = Self::get_condition(second - 0x40);
                (&format!("cmov{}", condition), "r32, r/m32", 3)
            }

            // IMUL
            0xAF => ("imul", "r32, r/m32", 3),

            _ => ("???", "", 2),
        };

        Some(X86Instruction {
            address,
            bytes: code[..length].to_vec(),
            mnemonic: mnemonic.to_string(),
            operands: operands.to_string(),
            length,
        })
    }

    fn get_register(index: u8, is_64bit: bool) -> String {
        if is_64bit {
            match index {
                0 => "rax", 1 => "rcx", 2 => "rdx", 3 => "rbx",
                4 => "rsp", 5 => "rbp", 6 => "rsi", 7 => "rdi",
                _ => "r?",
            }
        } else {
            match index {
                0 => "eax", 1 => "ecx", 2 => "edx", 3 => "ebx",
                4 => "esp", 5 => "ebp", 6 => "esi", 7 => "edi",
                _ => "r?",
            }
        }.to_string()
    }

    fn get_register_8bit(index: u8) -> String {
        match index {
            0 => "al", 1 => "cl", 2 => "dl", 3 => "bl",
            4 => "ah", 5 => "ch", 6 => "dh", 7 => "bh",
            _ => "?l",
        }.to_string()
    }

    fn get_condition(code: u8) -> &'static str {
        match code {
            0 => "o", 1 => "no", 2 => "b", 3 => "nb",
            4 => "z", 5 => "nz", 6 => "be", 7 => "a",
            8 => "s", 9 => "ns", 10 => "p", 11 => "np",
            12 => "l", 13 => "ge", 14 => "le", 15 => "g",
            _ => "?",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_disassemble_nop() {
        let disasm = X86Disassembler::new(false);
        let code = vec![0x90]; // NOP
        let instrs = disasm.disassemble(&code, 0x1000, 1);

        assert_eq!(instrs.len(), 1);
        assert_eq!(instrs[0].mnemonic, "nop");
        assert_eq!(instrs[0].length, 1);
    }

    #[test]
    fn test_disassemble_ret() {
        let disasm = X86Disassembler::new(false);
        let code = vec![0xC3]; // RET
        let instrs = disasm.disassemble(&code, 0x1000, 1);

        assert_eq!(instrs.len(), 1);
        assert_eq!(instrs[0].mnemonic, "ret");
    }

    #[test]
    fn test_disassemble_push_pop() {
        let disasm = X86Disassembler::new(false);
        let code = vec![0x50, 0x58]; // PUSH EAX, POP EAX
        let instrs = disasm.disassemble(&code, 0x1000, 2);

        assert_eq!(instrs.len(), 2);
        assert_eq!(instrs[0].mnemonic, "push");
        assert_eq!(instrs[1].mnemonic, "pop");
    }
}
            0x8A => ("mov", "r8, r/m8", 2),
            0x8B => ("mov", "r32, r/m32", 2),
            0xB0..=0xB7 => ("mov", "al/cl, imm8", 2),
            0xB8..=0xBF => ("mov", "eax/ecx, imm32", 5),

            // PUSH/POP
            0x50..=0x57 => ("push", &format!("r{}", opcode - 0x50), 1),
            0x58..=0x5F => ("pop", &format!("r{}", opcode - 0x58), 1),
            0x68 => ("push", "imm32", 5),
            0x6A => ("push", "imm8", 2),

            // CALL/RET
            0xE8 => ("call", "rel32", 5),
            0xC2 => ("ret", "imm16", 3),
            0xC3 => ("ret", "", 1),

            // JMP
            0xE9 => ("jmp", "rel32", 5),
            0xEB => ("jmp", "rel8", 2),
            0xFF if code.len() > 1 => {
                let modrm = code[1];
                match (modrm >> 3) & 0x7 {
                    4 => ("jmp", "r/m32", 2),
                    _ => ("unknown", "", 2),
                }
            }

            // Conditional jumps
            0x70..=0x7F => {
                let condition = match opcode & 0x0F {
                    0x0 => "o", 0x1 => "no", 0x2 => "b", 0x3 => "ae",
                    0x4 => "e", 0x5 => "ne", 0x6 => "be", 0x7 => "a",
                    0x8 => "s", 0x9 => "ns", 0xA => "p", 0xB => "np",
                    0xC => "l", 0xD => "ge", 0xE => "le", 0xF => "g",
                    _ => "?",
                };
                (&format!("j{}", condition), "rel8", 2)
            }

            // TEST
            0x84 => ("test", "r/m8, r8", 2),
            0x85 => ("test", "r/m32, r32", 2),

            // CMP
            0x38 => ("cmp", "r/m8, r8", 2),
            0x39 => ("cmp", "r/m32, r32", 2),
            0x3A => ("cmp", "r8, r/m8", 2),
            0x3B => ("cmp", "r32, r/m32", 2),
            0x3C => ("cmp", "al, imm8", 2),
            0x3D => ("cmp", "eax, imm32", 5),

            // ADD/SUB
            0x00 => ("add", "r/m8, r8", 2),
            0x01 => ("add", "r/m32, r32", 2),
            0x28 => ("sub", "r/m8, r8", 2),
            0x29 => ("sub", "r/m32, r32", 2),
            0x83 if code.len() > 1 => {
                let modrm = code[1];
                let op = match (modrm >> 3) & 0x7 {
                    0 => "add",
                    5 => "sub",
                    4 => "and",
                    1 => "or",
                    6 => "xor",
                    7 => "cmp",
                    _ => "unknown",
                };
                (op, "r/m32, imm8", 3)
            }

            // XOR
            0x30 => ("xor", "r/m8, r8", 2),
            0x31 => ("xor", "r/m32, r32", 2),
            0x32 => ("xor", "r8, r/m8", 2),
            0x33 => ("xor", "r32, r/m32", 2),

            // NOP
            0x90 => ("nop", "", 1),

            // LEA
            0x8D => ("lea", "r32, m", 2),

            // INT
            0xCD => ("int", "imm8", 2),

            // Prefixos REX (x64)
            0x40..=0x4F if self.is_64bit => {
                // REX prefix, continuar para próximo byte
                if code.len() > 1 {
                    return self.decode_instruction(&code[1..], address);
                }
                return None;
            }

            _ => ("db", "??", 1),
        };

        Some(X86Instruction {
            address,
            bytes: code[..length.min(code.len())].to_vec(),
            mnemonic: mnemonic.to_string(),
            operands: operands.to_string(),
            length,
        })
    }
}

impl X86Instruction {
    pub fn format(&self) -> String {
        let bytes_str = self.bytes.iter()
            .map(|b| format!("{:02x}", b))
            .collect::<Vec<_>>()
            .join(" ");

        if self.operands.is_empty() {
            format!("0x{:016x}: {:20} {}", self.address, bytes_str, self.mnemonic)
        } else {
            format!("0x{:016x}: {:20} {} {}",
                self.address, bytes_str, self.mnemonic, self.operands)
        }
    }
}
