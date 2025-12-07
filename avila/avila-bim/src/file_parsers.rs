//! Base parsers para formatos de arquivos CAD/BIM (Rust puro)

use std::collections::HashMap;

/// Tipos de arquivos suportados
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FileFormat {
    IFC,
    DWG,
    DXF,
    OBJ,
    STL,
    PLY,
    FBX,
    GLTF,
    GLB,
    SKP,
    RVT,
    NWD,
    Unknown,
}

/// Detectar formato baseado na extensão
pub fn detect_format(filename: &str) -> FileFormat {
    let filename = filename.to_lowercase();

    if filename.ends_with(".ifc") {
        FileFormat::IFC
    } else if filename.ends_with(".dwg") {
        FileFormat::DWG
    } else if filename.ends_with(".dxf") {
        FileFormat::DXF
    } else if filename.ends_with(".obj") {
        FileFormat::OBJ
    } else if filename.ends_with(".stl") {
        FileFormat::STL
    } else if filename.ends_with(".ply") {
        FileFormat::PLY
    } else if filename.ends_with(".fbx") {
        FileFormat::FBX
    } else if filename.ends_with(".gltf") {
        FileFormat::GLTF
    } else if filename.ends_with(".glb") {
        FileFormat::GLB
    } else if filename.ends_with(".skp") {
        FileFormat::SKP
    } else if filename.ends_with(".rvt") {
        FileFormat::RVT
    } else if filename.ends_with(".nwd") {
        FileFormat::NWD
    } else {
        FileFormat::Unknown
    }
}

/// Estrutura base para dados de modelo carregado
#[derive(Debug, Clone)]
pub struct LoadedModel {
    pub format: FileFormat,
    pub elements: Vec<ModelElement>,
    pub materials: Vec<ModelMaterial>,
    pub metadata: HashMap<String, String>,
}

/// Elemento de modelo genérico
#[derive(Debug, Clone)]
pub struct ModelElement {
    pub id: String,
    pub name: Option<String>,
    pub element_type: String,
    pub geometry: ElementGeometry,
    pub properties: HashMap<String, PropertyValue>,
    pub transform: Option<TransformMatrix>,
}

/// Geometria de elemento
#[derive(Debug, Clone)]
pub enum ElementGeometry {
    Mesh {
        vertices: Vec<[f64; 3]>,
        indices: Vec<u32>,
        normals: Option<Vec<[f64; 3]>>,
        uvs: Option<Vec<[f64; 2]>>,
    },
    Points(Vec<[f64; 3]>),
    Lines(Vec<([f64; 3], [f64; 3])>),
    Empty,
}

/// Material de modelo
#[derive(Debug, Clone)]
pub struct ModelMaterial {
    pub id: String,
    pub name: Option<String>,
    pub diffuse_color: Option<[f32; 4]>,
    pub specular_color: Option<[f32; 3]>,
    pub shininess: Option<f32>,
    pub texture_path: Option<String>,
}

/// Valor de propriedade
#[derive(Debug, Clone)]
pub enum PropertyValue {
    String(String),
    Integer(i64),
    Float(f64),
    Boolean(bool),
    Vector2([f64; 2]),
    Vector3([f64; 3]),
    Array(Vec<PropertyValue>),
}

/// Matriz de transformação 4x4
#[derive(Debug, Clone)]
pub struct TransformMatrix {
    pub matrix: [f64; 16],
}

/// Resultado de parsing
pub type ParseResult<T> = Result<T, ParseError>;

/// Erro de parsing
#[derive(Debug, Clone)]
pub enum ParseError {
    InvalidFormat(String),
    UnsupportedVersion(String),
    CorruptedFile(String),
    MissingData(String),
    IoError(String),
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ParseError::InvalidFormat(msg) => write!(f, "Invalid format: {}", msg),
            ParseError::UnsupportedVersion(msg) => write!(f, "Unsupported version: {}", msg),
            ParseError::CorruptedFile(msg) => write!(f, "Corrupted file: {}", msg),
            ParseError::MissingData(msg) => write!(f, "Missing data: {}", msg),
            ParseError::IoError(msg) => write!(f, "IO error: {}", msg),
        }
    }
}

impl std::error::Error for ParseError {}

/// Trait base para parsers
pub trait FileParser {
    fn can_parse(&self, format: FileFormat) -> bool;
    fn parse(&self, data: &[u8], filename: &str) -> ParseResult<LoadedModel>;
}

/// Gerenciador de parsers
pub struct ParserManager {
    parsers: Vec<Box<dyn FileParser>>,
}

impl ParserManager {
    pub fn new() -> Self {
        Self {
            parsers: Vec::new(),
        }
    }

    pub fn register_parser(&mut self, parser: Box<dyn FileParser>) {
        self.parsers.push(parser);
    }

    pub fn parse_file(&self, data: &[u8], filename: &str) -> ParseResult<LoadedModel> {
        let format = detect_format(filename);

        for parser in &self.parsers {
            if parser.can_parse(format) {
                return parser.parse(data, filename);
            }
        }

        Err(ParseError::InvalidFormat(format!("No parser available for format {:?}", format)))
    }
}

impl Default for ParserManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_format() {
        assert_eq!(detect_format("model.ifc"), FileFormat::IFC);
        assert_eq!(detect_format("drawing.dwg"), FileFormat::DWG);
        assert_eq!(detect_format("export.dxf"), FileFormat::DXF);
        assert_eq!(detect_format("mesh.obj"), FileFormat::OBJ);
        assert_eq!(detect_format("print.stl"), FileFormat::STL);
        assert_eq!(detect_format("model.ply"), FileFormat::PLY);
        assert_eq!(detect_format("scene.fbx"), FileFormat::FBX);
        assert_eq!(detect_format("model.gltf"), FileFormat::GLTF);
        assert_eq!(detect_format("model.glb"), FileFormat::GLB);
        assert_eq!(detect_format("building.skp"), FileFormat::SKP);
        assert_eq!(detect_format("project.rvt"), FileFormat::RVT);
        assert_eq!(detect_format("coordination.nwd"), FileFormat::NWD);
        assert_eq!(detect_format("unknown.xyz"), FileFormat::Unknown);
    }
}
