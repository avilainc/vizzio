//! Vizzio Viewer - Servidor HTTP simples
//! Usa apenas Avila Stack (avila-bim + avila-vision)

use std::fs;

mod server;
mod cache;
mod perf;

use cache::IfcCache;
use perf::{ViewerMetrics, PerfTimer};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🏗️  Vizzio Viewer MVP v0.1.0");
    println!("=========================================");
    println!("🚀 Powered by Avila Stack");
    println!();

    let metrics = ViewerMetrics::new();
    let mut ifc_cache = IfcCache::new();

    let timer = PerfTimer::start("Busca de arquivos IFC");
    let ifc_files = find_ifc_files(".")?;
    timer.stop();

    if ifc_files.is_empty() {
        println!("❌ Nenhum arquivo IFC encontrado");
        return Ok(());
    }

    println!("📁 Arquivos IFC encontrados:");
    for (i, file) in ifc_files.iter().enumerate() {
        println!("  {}. {}", i + 1, file);
    }

    let ifc_path = &ifc_files[0];
    println!("\n📥 Carregando: {}", ifc_path);

    if let Some(cached) = ifc_cache.get(ifc_path) {
        println!("💾 Cache hit!");
        metrics.cache_hits.inc();
    } else {
        metrics.cache_misses.inc();

        let timer = PerfTimer::start("Parse IFC");
        let content = fs::read_to_string(ifc_path)?;
        let file_size = content.len();
        let model = avila_bim::IfcModel::from_step(&content)?;
        timer.stop();

        println!("✅ IFC parseado!");
        println!("   Entidades: {}", model.entities.len());
        println!("   Schema: {}", model.header.schema);
        println!("   Tamanho: {:.2} MB", file_size as f64 / (1024.0 * 1024.0));

        let timer = PerfTimer::start("Extração de geometria");
        let geometries = model.extract_geometry()?;
        timer.stop();

        println!("✅ Geometrias: {} objetos", geometries.len());

        metrics.record_ifc_load(model.entities.len() as u64, geometries.len() as u64);
        ifc_cache.insert(ifc_path.clone(), model, file_size);
    }

    println!();
    ifc_cache.stats().print();
    metrics.print_stats();

    println!("\n🌐 Iniciando servidor...");
    server::start(8080)?;

    Ok(())
}

fn find_ifc_files(dir: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let mut files = Vec::new();
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() {
            if let Some(ext) = path.extension() {
                if ext.eq_ignore_ascii_case("ifc") {
                    if let Some(name) = path.file_name() {
                        files.push(name.to_string_lossy().to_string());
                    }
                }
            }
        }
    }
    files.sort();
    Ok(files)
}
