// Parser PE nativo 100% Rust - substitui goblin para PE
use std::io::{Cursor, Read, Seek, SeekFrom};

const IMAGE_DOS_SIGNATURE: u16 = 0x5A4D; // MZ
const IMAGE_NT_SIGNATURE: u32 = 0x00004550; // PE\0\0

#[derive(Debug)]
pub struct PeParser {
    pub data: Vec<u8>,
    pub is_64bit: bool,
    pub entry_point: u64,
    pub image_base: u64,
    pub sections: Vec<PeSection>,
    pub imports: Vec<PeImport>,
    pub exports: Vec<PeExport>,
}

#[derive(Debug, Clone)]
pub struct PeSection {
    pub name: String,
    pub virtual_address: u32,
    pub virtual_size: u32,
    pub raw_address: u32,
    pub raw_size: u32,
    pub characteristics: u32,
}

#[derive(Debug, Clone)]
pub struct PeImport {
    pub dll: String,
    pub function: String,
    pub rva: u32,
}

#[derive(Debug, Clone)]
pub struct PeExport {
    pub name: String,
    pub rva: u32,
    pub ordinal: u16,
}

impl PeParser {
    pub fn parse(data: Vec<u8>) -> Result<Self, String> {
        let mut cursor = Cursor::new(data.as_slice());

        // Verificar assinatura DOS
        let dos_sig = read_u16(&mut cursor)?;
        if dos_sig != IMAGE_DOS_SIGNATURE {
            return Err("Assinatura DOS inválida".to_string());
        }

        // Ler offset do PE header
        cursor.seek(SeekFrom::Start(0x3C)).map_err(|e| e.to_string())?;
        let pe_offset = read_u32(&mut cursor)? as u64;

        // Verificar assinatura PE
        cursor.seek(SeekFrom::Start(pe_offset)).map_err(|e| e.to_string())?;
        let pe_sig = read_u32(&mut cursor)?;
        if pe_sig != IMAGE_NT_SIGNATURE {
            return Err("Assinatura PE inválida".to_string());
        }

        // Ler COFF header
        let machine = read_u16(&mut cursor)?;
        let number_of_sections = read_u16(&mut cursor)?;
        cursor.seek(SeekFrom::Current(12)).map_err(|e| e.to_string())?; // Pular timestamp, etc
        let size_of_optional_header = read_u16(&mut cursor)?;
        cursor.seek(SeekFrom::Current(2)).map_err(|e| e.to_string())?; // Characteristics

        let is_64bit = machine == 0x8664;

        // Ler Optional Header
        let magic = read_u16(&mut cursor)?;
        cursor.seek(SeekFrom::Current(2)).map_err(|e| e.to_string())?; // Major/Minor linker version
        cursor.seek(SeekFrom::Current(12)).map_err(|e| e.to_string())?; // Code/Data sizes

        let entry_point = read_u32(&mut cursor)? as u64;
        cursor.seek(SeekFrom::Current(8)).map_err(|e| e.to_string())?; // Base of code/data

        let image_base = if is_64bit {
            read_u64(&mut cursor)?
        } else {
            read_u32(&mut cursor)? as u64
        };

        // Pular o resto do optional header
        let current_pos = cursor.position();
        let sections_offset = pe_offset + 24 + size_of_optional_header as u64;
        cursor.seek(SeekFrom::Start(sections_offset)).map_err(|e| e.to_string())?;

        // Ler seções
        let mut sections = Vec::new();
        for _ in 0..number_of_sections {
            let mut name_bytes = [0u8; 8];
            cursor.read_exact(&mut name_bytes).map_err(|e| e.to_string())?;
            let name = String::from_utf8_lossy(&name_bytes)
                .trim_end_matches('\0')
                .to_string();

            let virtual_size = read_u32(&mut cursor)?;
            let virtual_address = read_u32(&mut cursor)?;
            let raw_size = read_u32(&mut cursor)?;
            let raw_address = read_u32(&mut cursor)?;
            cursor.seek(SeekFrom::Current(12))?; // Relocations, etc
            let characteristics = read_u32(&mut cursor)?;

            sections.push(PeSection {
                name,
                virtual_address,
                virtual_size,
                raw_address,
                raw_size,
                characteristics,
            });
        }

        Ok(PeParser {
            data,
            is_64bit,
            entry_point,
            image_base,
            sections,
            imports: Vec::new(), // Simplified - full implementation would parse import table
            exports: Vec::new(), // Simplified
        })
    }

    pub fn read_at(&self, offset: usize, size: usize) -> Option<&[u8]> {
        if offset + size <= self.data.len() {
            Some(&self.data[offset..offset + size])
        } else {
            None
        }
    }

    pub fn rva_to_offset(&self, rva: u32) -> Option<usize> {
        for section in &self.sections {
            if rva >= section.virtual_address
                && rva < section.virtual_address + section.virtual_size {
                let offset = rva - section.virtual_address;
                return Some((section.raw_address + offset) as usize);
            }
        }
        None
    }
}

// Funções auxiliares para leitura
fn read_u16(cursor: &mut Cursor<&[u8]>) -> Result<u16, String> {
    let mut buf = [0u8; 2];
    cursor.read_exact(&mut buf).map_err(|e| e.to_string())?;
    Ok(u16::from_le_bytes(buf))
}

fn read_u32(cursor: &mut Cursor<&[u8]>) -> Result<u32, String> {
    let mut buf = [0u8; 4];
    cursor.read_exact(&mut buf).map_err(|e| e.to_string())?;
    Ok(u32::from_le_bytes(buf))
}

fn read_u64(cursor: &mut Cursor<&[u8]>) -> Result<u64, String> {
    let mut buf = [0u8; 8];
    cursor.read_exact(&mut buf).map_err(|e| e.to_string())?;
    Ok(u64::from_le_bytes(buf))
}
