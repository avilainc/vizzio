use std::fmt;
use std::io;

#[derive(Debug, Clone)]
pub enum DeriaxError {
    ParseError(String),
    FileNotFound(String),
    InvalidFormat(String),
    AnalysisError(String),
    DisassemblyError(String),
    NotSupported(String),
    IoError(String),
}

impl fmt::Display for DeriaxError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DeriaxError::ParseError(msg) => write!(f, "Erro de parsing: {}", msg),
            DeriaxError::FileNotFound(msg) => write!(f, "Arquivo não encontrado: {}", msg),
            DeriaxError::InvalidFormat(msg) => write!(f, "Formato inválido: {}", msg),
            DeriaxError::AnalysisError(msg) => write!(f, "Erro de análise: {}", msg),
            DeriaxError::DisassemblyError(msg) => write!(f, "Erro de desassembler: {}", msg),
            DeriaxError::NotSupported(msg) => write!(f, "Não suportado: {}", msg),
            DeriaxError::IoError(msg) => write!(f, "Erro IO: {}", msg),
        }
    }
}

impl std::error::Error for DeriaxError {}

impl From<io::Error> for DeriaxError {
    fn from(err: io::Error) -> Self {
        DeriaxError::IoError(err.to_string())
    }
}

pub type Result<T> = std::result::Result<T, DeriaxError>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BinaryFormat {
    PE32,
    PE64,
    ELF32,
    ELF64,
    MachO32,
    MachO64,
    Raw,
    Unknown,
}

impl fmt::Display for BinaryFormat {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            BinaryFormat::PE32 => write!(f, "PE32"),
            BinaryFormat::PE64 => write!(f, "PE64"),
            BinaryFormat::ELF32 => write!(f, "ELF32"),
            BinaryFormat::ELF64 => write!(f, "ELF64"),
            BinaryFormat::MachO32 => write!(f, "Mach-O 32-bit"),
            BinaryFormat::MachO64 => write!(f, "Mach-O 64-bit"),
            BinaryFormat::Raw => write!(f, "Raw Binary"),
            BinaryFormat::Unknown => write!(f, "Unknown"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct FileHash {
    pub md5: String,
    pub sha1: String,
    pub sha256: String,
}

#[derive(Debug, Clone)]
pub struct Section {
    pub name: String,
    pub virtual_address: u64,
    pub virtual_size: u64,
    pub raw_address: u64,
    pub raw_size: u64,
    pub characteristics: u32,
    pub entropy: f64,
}

#[derive(Debug, Clone)]
pub struct Import {
    pub library: String,
    pub function: String,
    pub address: Option<u64>,
}

#[derive(Debug, Clone)]
pub struct Export {
    pub name: String,
    pub address: u64,
    pub ordinal: u32,
}

#[derive(Debug, Clone)]
pub struct Symbol {
    pub name: String,
    pub address: u64,
    pub size: u64,
    pub symbol_type: String,
}
