use crate::core::types::*;
use crate::core::Architecture;
use crate::core::hash::SimpleHash;
use crate::internal::{PeParser, ElfParser};
use std::path::Path;
use std::fs;

pub struct Binary {
    pub path: String,
    pub data: Vec<u8>,
    pub format: BinaryFormat,
    pub arch: Architecture,
    pub entry_point: u64,
    pub image_base: u64,
    pub sections: Vec<Section>,
    pub imports: Vec<Import>,
    pub exports: Vec<Export>,
    pub symbols: Vec<Symbol>,
    pub hashes: FileHash,
}

impl Binary {
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        let path_str = path.as_ref().to_string_lossy().to_string();
        let data = fs::read(&path).map_err(|e| {
            DeriaxError::FileNotFound(format!("{}: {}", path_str, e))
        })?;

        Self::from_bytes(data, path_str)
    }

    pub fn from_bytes(data: Vec<u8>, path: String) -> Result<Self> {
        let hashes = Self::calculate_hashes(&data);

        // Detectar formato do arquivo
        if data.len() < 4 {
            return Err(DeriaxError::InvalidFormat("Arquivo muito pequeno".to_string()));
        }

        // Verificar PE (MZ)
        if data[0] == 0x4D && data[1] == 0x5A {
            return Self::parse_pe_internal(data, path, hashes);
        }

        // Verificar ELF (\x7fELF)
        if data[0] == 0x7F && data[1] == 0x45 && data[2] == 0x4C && data[3] == 0x46 {
            return Self::parse_elf_internal(data, path, hashes);
        }

        // Formato desconhecido
        Ok(Binary {
            path,
            data,
            format: BinaryFormat::Unknown,
            arch: Architecture::Unknown,
            entry_point: 0,
            image_base: 0,
            sections: vec![],
            imports: vec![],
            exports: vec![],
            symbols: vec![],
            hashes,
        })
    }

    fn calculate_hashes(data: &[u8]) -> FileHash {
        let md5 = SimpleHash::md5_simple(data);
        let sha1 = SimpleHash::sha1_simple(data);
        let sha256 = SimpleHash::sha256_simple(data);

        FileHash { md5, sha1, sha256 }
    }

    fn parse_pe_internal(data: Vec<u8>, path: String, hashes: FileHash) -> Result<Self> {
        let pe = PeParser::parse(data.clone())
            .map_err(|e| DeriaxError::ParseError(format!("Erro ao parsear PE: {}", e)))?;

        let format = if pe.is_64bit { BinaryFormat::PE64 } else { BinaryFormat::PE32 };
        let arch = if pe.is_64bit { Architecture::X86_64 } else { Architecture::X86 };

        // Converter seções
        let sections = pe.sections.iter().map(|s| {
            Section {
                name: s.name.clone(),
                virtual_address: s.virtual_address as u64,
                virtual_size: s.virtual_size as u64,
                raw_address: s.raw_address as u64,
                raw_size: s.raw_size as u64,
                characteristics: s.characteristics,
                entropy: Self::calculate_entropy(&data, s.raw_address as usize, s.raw_size as usize),
            }
        }).collect();

        // Converter imports
        let imports = pe.imports.iter().map(|imp| {
            Import {
                library: imp.dll.clone(),
                function: imp.function.clone(),
                address: Some(imp.rva as u64),
            }
        }).collect();

        // Converter exports
        let exports = pe.exports.iter().map(|exp| {
            Export {
                name: exp.name.clone(),
                address: exp.rva as u64,
                ordinal: exp.ordinal as u32,
            }
        }).collect();

        Ok(Binary {
            path,
            data,
            format,
            arch,
            entry_point: pe.entry_point,
            image_base: pe.image_base,
            sections,
            imports,
            exports,
            symbols: vec![],
            hashes,
        })
    }

    fn parse_elf_internal(data: Vec<u8>, path: String, hashes: FileHash) -> Result<Self> {
        let elf = ElfParser::parse(data.clone())
            .map_err(|e| DeriaxError::ParseError(format!("Erro ao parsear ELF: {}", e)))?;

        let format = if elf.is_64bit { BinaryFormat::ELF64 } else { BinaryFormat::ELF32 };

        let arch = match elf.machine {
            3 => Architecture::X86,
            62 => Architecture::X86_64,
            40 => Architecture::ARM,
            183 => Architecture::ARM64,
            8 => Architecture::MIPS,
            _ => Architecture::Unknown,
        };

        // Converter seções
        let sections = elf.sections.iter().map(|s| {
            Section {
                name: s.name.clone(),
                virtual_address: s.address,
                virtual_size: s.size,
                raw_address: s.offset,
                raw_size: s.size,
                characteristics: s.flags as u32,
                entropy: Self::calculate_entropy(&data, s.offset as usize, s.size as usize),
            }
        }).collect();

        // Converter símbolos como imports
        let imports = elf.symbols.iter().map(|sym| {
            Import {
                library: String::from("unknown"),
                function: sym.name.clone(),
                address: Some(sym.value),
            }
        }).collect();

        // Converter símbolos
        let symbols = elf.symbols.iter().map(|sym| {
            Symbol {
                name: sym.name.clone(),
                address: sym.value,
                size: sym.size,
                symbol_type: "FUNC".to_string(),
            }
        }).collect();

        let entry_point = elf.entry_point;
        let image_base = 0x400000;

        Ok(Binary {
            path,
            data,
            format,
            arch,
            entry_point,
            image_base,
            sections,
            imports,
            exports: vec![],
            symbols,
            hashes,
        })
    }

    fn parse_mach(_data: Vec<u8>, _path: String, _hashes: FileHash) -> Result<Self> {
        // TODO: Implementação básica para Mach-O
        Err(DeriaxError::NotSupported("Mach-O format not yet supported".to_string()))
    }

    fn calculate_entropy(data: &[u8], offset: usize, size: usize) -> f64 {
        if size == 0 || offset + size > data.len() {
            return 0.0;
        }

        let section_data = &data[offset..offset + size];
        let mut counts = [0u32; 256];

        for &byte in section_data {
            counts[byte as usize] += 1;
        }

        let len = section_data.len() as f64;
        counts.iter()
            .filter(|&&count| count > 0)
            .map(|&count| {
                let p = count as f64 / len;
                -p * p.log2()
            })
            .sum()
    }

    pub fn find_strings(&self, min_length: usize) -> Vec<String> {
        let mut strings = Vec::new();
        let mut current = String::new();

        for &byte in &self.data {
            if byte >= 32 && byte <= 126 {
                current.push(byte as char);
            } else {
                if current.len() >= min_length {
                    strings.push(current.clone());
                }
                current.clear();
            }
        }

        if current.len() >= min_length {
            strings.push(current);
        }

        strings
    }
}
