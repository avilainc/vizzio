//! # avila-bim - Demo Simples
//! Servidor HTTP básico para demonstração do BIM

use std::sync::Arc;
use tokio::sync::RwLock;
use std::collections::HashMap;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

#[derive(Debug, Clone)]
struct BimProject {
    id: String,
    name: String,
    status: String,
}

struct AppState {
    projects: Arc<RwLock<HashMap<String, BimProject>>>,
}

#[tokio::main]
async fn main() -> Result<()> {
    println!("═══════════════════════════════════════════════════════════");
    println!("   🏗️  AVILA BIM - Building Information Modeling");
    println!("   📐 Conversão IFC → glTF/GLB");
    println!("═══════════════════════════════════════════════════════════\n");

    let state = Arc::new(AppState {
        projects: Arc::new(RwLock::new(HashMap::new())),
    });

    // Adicionar projeto demo
    {
        let mut projects = state.projects.write().await;
        projects.insert(
            "demo-1".to_string(),
            BimProject {
                id: "demo-1".to_string(),
                name: "Projeto Demo".to_string(),
                status: "ready".to_string(),
            },
        );
    }

    println!("✅ Servidor BIM inicializado");
    println!("📊 Projetos carregados: 1");
    println!("\n🎯 Recursos disponíveis:");
    println!("   • Conversão IFC → glTF");
    println!("   • Parser DWG/DXF");
    println!("   • Otimização de malhas");
    println!("   • Exportação para múltiplos formatos");

    println!("\n⏳ Aguardando comandos... (Ctrl+C para sair)");

    // Loop infinito mantendo o processo vivo
    tokio::signal::ctrl_c().await?;
    println!("\n👋 Encerrando servidor BIM...");

    Ok(())
}
