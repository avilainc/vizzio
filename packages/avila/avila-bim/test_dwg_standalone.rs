//! Teste simples do parser DWG (versão standalone)

use std::fs;
use std::convert::TryInto;

// Copiando as estruturas necessárias do file_parsers.rs
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FileFormat {
    DWG,
    // Outros formatos omitidos para simplicidade
}

#[derive(Debug)]
pub enum ParseError {
    IoError(String),
    ParseError(String),
    UnsupportedFormat,
    InvalidData(String),
}

pub trait FileParser {
    fn can_parse(&self, format: FileFormat) -> bool;
    fn parse(&self, data: &[u8]) -> Result<LoadedModel, ParseError>;
}

#[derive(Debug)]
pub struct LoadedModel {
    pub elements: Vec<ModelElement>,
    pub materials: Vec<Material>,
    pub metadata: std::collections::HashMap<String, String>,
}

#[derive(Debug)]
pub struct ModelElement {
    pub name: String,
    pub element_type: ElementType,
    pub geometry: Option<ElementGeometry>,
    pub properties: std::collections::HashMap<String, PropertyValue>,
}

#[derive(Debug)]
pub enum ElementType {
    Wall,
    Slab,
    Beam,
    Column,
    Other(String),
}

#[derive(Debug)]
pub struct ElementGeometry {
    pub vertices: Vec<[f32; 3]>,
    pub normals: Vec<[f32; 3]>,
    pub indices: Vec<u32>,
    pub primitive_type: PrimitiveType,
}

#[derive(Debug)]
pub enum PrimitiveType {
    Triangles,
    Lines,
    Points,
}

#[derive(Debug)]
pub struct Material {
    pub name: String,
    pub diffuse_color: [f32; 4],
}

#[derive(Debug)]
pub enum PropertyValue {
    String(String),
    Float(f32),
    Int(i32),
    Bool(bool),
}

// Copiando o DWG parser
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DwgVersion {
    R13, // AC1012
    R14, // AC1014
    R2000, // AC1015
    R2004, // AC1018
    R2007, // AC1021
    R2010, // AC1024
    R2013, // AC1027
    R2018, // AC1032
    Unknown,
}

impl DwgVersion {
    pub fn from_signature(sig: &[u8; 6]) -> Self {
        match sig {
            b"AC1012" => DwgVersion::R13,
            b"AC1014" => DwgVersion::R14,
            b"AC1015" => DwgVersion::R2000,
            b"AC1018" => DwgVersion::R2004,
            b"AC1021" => DwgVersion::R2007,
            b"AC1024" => DwgVersion::R2010,
            b"AC1027" => DwgVersion::R2013,
            b"AC1032" => DwgVersion::R2018,
            _ => DwgVersion::Unknown,
        }
    }
}

#[derive(Debug)]
pub struct DwgHeader {
    pub version: DwgVersion,
    pub file_size: u32,
    pub maintenance_release: u8,
}

pub struct DwgFileParser;

impl DwgFileParser {
    pub fn new() -> Self {
        DwgFileParser
    }

    fn parse_modern_header(&self, data: &[u8]) -> Result<DwgHeader, ParseError> {
        if data.len() < 0x80 {
            return Err(ParseError::InvalidData("Arquivo DWG muito pequeno".to_string()));
        }

        let file_size = if data.len() >= 0x2C {
            let size_bytes = &data[0x28..0x2C];
            u32::from_le_bytes([size_bytes[0], size_bytes[1], size_bytes[2], size_bytes[3]])
        } else {
            data.len() as u32
        };

        let version_sig = &data[0..6];
        let version_array: [u8; 6] = version_sig.try_into().unwrap();
        let version = DwgVersion::from_signature(&version_array);

        let maintenance_release = if data.len() > 0x0B {
            data[0x0B]
        } else {
            0
        };

        Ok(DwgHeader {
            version,
            file_size,
            maintenance_release,
        })
    }
}

impl FileParser for DwgFileParser {
    fn can_parse(&self, format: FileFormat) -> bool {
        format == FileFormat::DWG
    }

    fn parse(&self, data: &[u8]) -> Result<LoadedModel, ParseError> {
        if data.len() < 6 {
            return Err(ParseError::InvalidData("Arquivo muito pequeno para ser DWG".to_string()));
        }

        let mut version_sig = [0u8; 6];
        version_sig.copy_from_slice(&data[0..6]);

        let version = DwgVersion::from_signature(&version_sig);
        if version == DwgVersion::Unknown {
            return Err(ParseError::InvalidData(format!(
                "Assinatura DWG inválida: {:?}",
                String::from_utf8_lossy(&version_sig)
            )));
        }

        let header = match version {
            DwgVersion::R2000 | DwgVersion::R2004 | DwgVersion::R2007 |
            DwgVersion::R2010 | DwgVersion::R2013 | DwgVersion::R2018 => {
                self.parse_modern_header(data)?
            }
            _ => {
                DwgHeader {
                    version,
                    file_size: data.len() as u32,
                    maintenance_release: 0,
                }
            }
        };

        let mut model = LoadedModel {
            elements: Vec::new(),
            materials: Vec::new(),
            metadata: std::collections::HashMap::new(),
        };

        model.metadata.insert("format".to_string(), "DWG".to_string());
        model.metadata.insert("version".to_string(), format!("{:?}", header.version));
        model.metadata.insert("file_size".to_string(), header.file_size.to_string());
        model.metadata.insert("maintenance_release".to_string(), header.maintenance_release.to_string());

        let placeholder_element = ModelElement {
            name: "DWG Drawing".to_string(),
            element_type: ElementType::Other("Drawing".to_string()),
            geometry: None,
            properties: std::collections::HashMap::new(),
        };

        model.elements.push(placeholder_element);

        Ok(model)
    }
}

fn main() {
    println!("Testando parser DWG...");

    let parser = DwgFileParser::new();

    assert!(parser.can_parse(FileFormat::DWG));
    println!("✓ Parser pode parsear DWG");

    match fs::read("test.dwg") {
        Ok(data) => {
            println!("✓ Arquivo test.dwg lido ({} bytes)", data.len());

            match parser.parse(&data) {
                Ok(model) => {
                    println!("✓ DWG parseado com sucesso!");
                    println!("  Metadados:");
                    for (key, value) in &model.metadata {
                        println!("    {}: {}", key, value);
                    }
                    println!("  Elementos: {}", model.elements.len());
                }
                Err(e) => {
                    println!("✗ Erro ao parsear DWG: {:?}", e);
                }
            }
        }
        Err(e) => {
            println!("✗ Erro ao ler arquivo test.dwg: {}", e);
        }
    }
}
