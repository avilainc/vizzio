//! Exemplo de uso da infraestrutura de parsers de arquivos CAD/BIM

use std::path::Path;
use avila_bim::{ParserManager, FileFormat, LoadedModel};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Criar um gerenciador de parsers
    let mut parser_manager = ParserManager::new();

    // Registrar todos os parsers disponíveis
    parser_manager.register_parser(FileFormat::IFC, Box::new(avila_bim::ifc_parser::IfcFileParser::new()));
    parser_manager.register_parser(FileFormat::DWG, Box::new(avila_bim::dwg_parser::DwgFileParser::new()));
    parser_manager.register_parser(FileFormat::DXF, Box::new(avila_bim::dxf_parser::DxfFileParser::new()));
    parser_manager.register_parser(FileFormat::OBJ, Box::new(avila_bim::obj_parser::ObjFileParser::new()));
    parser_manager.register_parser(FileFormat::STL, Box::new(avila_bim::stl_parser::StlFileParser::new()));
    parser_manager.register_parser(FileFormat::PLY, Box::new(avila_bim::ply_parser::PlyFileParser::new()));

    // Exemplo: Carregar um arquivo IFC
    let ifc_path = Path::new("exemplo.ifc");
    match parser_manager.parse_file(ifc_path) {
        Ok(model) => {
            println!("Arquivo IFC carregado com sucesso!");
            println!("Número de elementos: {}", model.elements.len());

            for element in &model.elements {
                println!("Elemento: {} - Tipo: {:?}", element.name, element.element_type);
                if let Some(geom) = &element.geometry {
                    println!("  Geometria: {} vértices, {} índices", geom.vertices.len(), geom.indices.len());
                }
            }
        }
        Err(e) => {
            println!("Erro ao carregar arquivo IFC: {}", e);
        }
    }

    // Exemplo: Carregar um arquivo DXF
    let dxf_path = Path::new("desenho.dxf");
    match parser_manager.parse_file(dxf_path) {
        Ok(model) => {
            println!("Arquivo DXF carregado com sucesso!");
            println!("Número de elementos: {}", model.elements.len());
        }
        Err(e) => {
            println!("Erro ao carregar arquivo DXF: {}", e);
        }
    }

    // Exemplo: Carregar um arquivo DWG
    let dwg_path = Path::new("projeto.dwg");
    match parser_manager.parse_file(dwg_path) {
        Ok(model) => {
            println!("Arquivo DWG carregado com sucesso!");
            println!("Versão: {}", model.metadata.get("version").unwrap_or(&"Unknown".to_string()));
            println!("Tamanho: {} bytes", model.metadata.get("file_size").unwrap_or(&"Unknown".to_string()));
            println!("Número de elementos: {}", model.elements.len());
        }
        Err(e) => {
            println!("Erro ao carregar arquivo DWG: {}", e);
        }
    }

    Ok(())
}
