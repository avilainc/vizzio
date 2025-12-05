//! Teste simples do parser DWG

use std::fs;
use avila_bim::dwg_parser::DwgFileParser;
use avila_bim::file_parsers::FileFormat;

fn main() {
    println!("Testando parser DWG...");

    // Criar parser
    let parser = DwgFileParser::new();

    // Verificar se pode parsear DWG
    assert!(parser.can_parse(FileFormat::DWG));
    println!("✓ Parser pode parsear DWG");

    // Tentar ler arquivo de teste
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
                    println!("✗ Erro ao parsear DWG: {}", e);
                }
            }
        }
        Err(e) => {
            println!("✗ Erro ao ler arquivo test.dwg: {}", e);
        }
    }
}
