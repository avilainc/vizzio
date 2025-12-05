// Parser ELF nativo 100% Rust - substitui goblin para ELF
use std::io::{Cursor, Read, Seek, SeekFrom};

const ELFMAG: [u8; 4] = [0x7f, 0x45, 0x4c, 0x46]; // \x7fELF

#[derive(Debug)]
pub struct ElfParser {
    pub data: Vec<u8>,
    pub is_64bit: bool,
    pub entry_point: u64,
    pub sections: Vec<ElfSection>,
    pub symbols: Vec<ElfSymbol>,
    pub machine: u16,
}

#[derive(Debug, Clone)]
pub struct ElfSection {
    pub name: String,
    pub address: u64,
    pub offset: u64,
    pub size: u64,
    pub flags: u64,
}

#[derive(Debug, Clone)]
pub struct ElfSymbol {
    pub name: String,
    pub value: u64,
    pub size: u64,
}

impl ElfParser {
    pub fn parse(data: Vec<u8>) -> Result<Self, String> {
        let mut cursor = Cursor::new(data.as_slice());

        // Verificar magic bytes
        let mut magic = [0u8; 4];
        cursor.read_exact(&mut magic).map_err(|e| e.to_string())?;
        if magic != ELFMAG {
            return Err("Assinatura ELF inválida".to_string());
        }

        // Ler classe (32 ou 64 bit)
        let mut class = [0u8; 1];
        cursor.read_exact(&mut class).map_err(|e| e.to_string())?;
        let is_64bit = class[0] == 2;

        // Pular endianness, version, etc
        cursor.seek(SeekFrom::Start(0x10)).map_err(|e| e.to_string())?;

        // Ler type e machine
        let _elf_type = read_u16_le(&mut cursor)?;
        let machine = read_u16_le(&mut cursor)?;

        // Pular version
        cursor.seek(SeekFrom::Current(4)).map_err(|e| e.to_string())?;

        // Entry point
        let entry_point = if is_64bit {
            read_u64_le(&mut cursor)?
        } else {
            read_u32_le(&mut cursor)? as u64
        };

        // Program header e section header offsets
        let phoff = if is_64bit {
            read_u64_le(&mut cursor)?
        } else {
            read_u32_le(&mut cursor)? as u64
        };

        let shoff = if is_64bit {
            read_u64_le(&mut cursor)?
        } else {
            read_u32_le(&mut cursor)? as u64
        };

        // Pular flags
        cursor.seek(SeekFrom::Current(4)).map_err(|e| e.to_string())?;

        // Header sizes
        let _ehsize = read_u16_le(&mut cursor)?;
        let _phentsize = read_u16_le(&mut cursor)?;
        let _phnum = read_u16_le(&mut cursor)?;
        let shentsize = read_u16_le(&mut cursor)?;
        let shnum = read_u16_le(&mut cursor)?;
        let _shstrndx = read_u16_le(&mut cursor)?;

        // Ler seções (simplificado)
        let mut sections = Vec::new();
        for i in 0..shnum {
            cursor.seek(SeekFrom::Start(shoff + i as u64 * shentsize as u64)).map_err(|e| e.to_string())?;

            let name_offset = read_u32_le(&mut cursor)?;
            let _sh_type = read_u32_le(&mut cursor)?;

            let flags = if is_64bit {
                read_u64_le(&mut cursor)?
            } else {
                read_u32_le(&mut cursor)? as u64
            };

            let address = if is_64bit {
                read_u64_le(&mut cursor)?
            } else {
                read_u32_le(&mut cursor)? as u64
            };

            let offset = if is_64bit {
                read_u64_le(&mut cursor)?
            } else {
                read_u32_le(&mut cursor)? as u64
            };

            let size = if is_64bit {
                read_u64_le(&mut cursor)?
            } else {
                read_u32_le(&mut cursor)? as u64
            };

            sections.push(ElfSection {
                name: format!(".section{}", i), // Simplificado
                address,
                offset,
                size,
                flags,
            });
        }

        Ok(ElfParser {
            data,
            is_64bit,
            entry_point,
            sections,
            symbols: Vec::new(), // Simplificado
            machine,
        })
    }

    pub fn read_at(&self, offset: usize, size: usize) -> Option<&[u8]> {
        if offset + size <= self.data.len() {
            Some(&self.data[offset..offset + size])
        } else {
            None
        }
    }
}

fn read_u16_le(cursor: &mut Cursor<&[u8]>) -> Result<u16, String> {
    let mut buf = [0u8; 2];
    cursor.read_exact(&mut buf).map_err(|e| e.to_string())?;
    Ok(u16::from_le_bytes(buf))
}

fn read_u32_le(cursor: &mut Cursor<&[u8]>) -> Result<u32, String> {
    let mut buf = [0u8; 4];
    cursor.read_exact(&mut buf).map_err(|e| e.to_string())?;
    Ok(u32::from_le_bytes(buf))
}

fn read_u64_le(cursor: &mut Cursor<&[u8]>) -> Result<u64, String> {
    let mut buf = [0u8; 8];
    cursor.read_exact(&mut buf).map_err(|e| e.to_string())?;
    Ok(u64::from_le_bytes(buf))
}
