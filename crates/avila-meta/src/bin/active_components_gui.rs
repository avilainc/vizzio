use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

use avila::{active_components, platform};

fn main() {
    if let Err(err) = run() {
        eprintln!("[active_components_gui] erro: {err}");
        std::process::exit(1);
    }
}

fn run() -> Result<(), Box<dyn Error>> {
    let components = active_components();
    let html = build_html(&components)?;

    let output_path = workspace_target_file("active_components.html");
    if let Some(parent) = output_path.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(&output_path, html)?;

    println!("Relatório gerado em: {}", output_path.to_string_lossy());

    if !components.is_empty() {
        open_in_browser(&output_path);
    } else {
        println!("Nenhum componente ativo detectado – arquivo não será aberto automaticamente.");
    }

    Ok(())
}

fn workspace_target_file(file_name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("target")
        .join(file_name)
}

fn build_html(components: &[avila::ComponentDescriptor]) -> Result<String, Box<dyn Error>> {
    let generated = human_timestamp();

    let entries = if components.is_empty() {
        String::from(
            "<div class=\"empty\">Nenhuma feature ativa nesta build. Compile com `--features` para popular a lista.</div>",
        )
    } else {
        components
            .iter()
            .map(|component| {
                format!(
                    "<article class=\"component\">\
                     <h2>{name}</h2>\
                     <p class=\"category\">{category}</p>\
                     <p class=\"summary\">{summary}</p>\
                     </article>",
                    name = component.name,
                    category = component.category,
                    summary = component.summary,
                )
            })
            .collect::<Vec<_>>()
            .join("\n")
    };

    let html = format!(
        "<!DOCTYPE html>\
         <html lang=\"pt-BR\">\
         <head>\
             <meta charset=\"utf-8\" />\
             <meta name=\"viewport\" content=\"width=device-width, initial-scale=1\" />\
             <title>Componentes Ativos · {name}</title>\
             <style>\
                 :root {{\
                     color-scheme: dark light;\
                     font-family: 'Segoe UI', system-ui, sans-serif;\
                     background: #0f172a;\
                     color: #e2e8f0;\
                 }}\
                 body {{\
                     margin: 0 auto;\
                     padding: 3rem 1.5rem 4rem;\
                     max-width: 1100px;\
                     line-height: 1.6;\
                 }}\
                 header {{\
                     margin-bottom: 2.5rem;\
                     text-align: center;\
                 }}\
                 header h1 {{\
                     margin: 0;\
                     font-size: clamp(2rem, 5vw, 3rem);\
                     letter-spacing: 0.08em;\
                     text-transform: uppercase;\
                 }}\
                 header p.meta {{\
                     margin-top: 0.5rem;\
                     font-size: 0.95rem;\
                     opacity: 0.75;\
                 }}\
                 .grid {{\
                     display: grid;\
                     grid-template-columns: repeat(auto-fit, minmax(240px, 1fr));\
                     gap: 1.25rem;\
                 }}\
                 .component {{\
                     border-radius: 1rem;\
                     padding: 1.5rem;\
                     background: linear-gradient(145deg, rgba(30, 41, 59, 0.9), rgba(15, 23, 42, 0.9));\
                     border: 1px solid rgba(148, 163, 184, 0.2);\
                     box-shadow: 0 20px 35px -20px rgba(15, 23, 42, 0.75);\
                     transition: transform 160ms ease, border-color 160ms ease;\
                 }}\
                 .component:hover {{\
                     transform: translateY(-6px);\
                     border-color: rgba(56, 189, 248, 0.6);\
                 }}\
                 .component h2 {{\
                     margin: 0;\
                     font-size: 1.4rem;\
                 }}\
                 .component .category {{\
                     margin: 0.25rem 0 1rem;\
                     font-size: 0.85rem;\
                     letter-spacing: 0.04em;\
                     text-transform: uppercase;\
                     color: #38bdf8;\
                 }}\
                 .component .summary {{\
                     margin: 0;\
                     font-size: 0.95rem;\
                     color: #cbd5f5;\
                 }}\
                 .empty {{\
                     margin: 4rem auto;\
                     max-width: 560px;\
                     text-align: center;\
                     padding: 2.5rem 2rem;\
                     border-radius: 1.25rem;\
                     background: rgba(30, 41, 59, 0.75);\
                     border: 1px dashed rgba(148, 163, 184, 0.35);\
                     font-size: 1.05rem;\
                 }}\
                 footer {{\
                     margin-top: 3rem;\
                     text-align: center;\
                     font-size: 0.9rem;\
                     opacity: 0.6;\
                 }}\
                 @media (max-width: 600px) {{\
                     body {{ padding: 2rem 1rem 3rem; }}\
                 }}\
             </style>\
         </head>\
         <body>\
             <header>\
                 <h1>{name}</h1>\
                 <p class=\"meta\">Plataforma gerada em {generated}. Região primária: {region}. Versão: {version}</p>\
             </header>\
             <section class=\"grid\">{entries}</section>\
             <footer>Ávila Cloud · Brasil acima de tudo · Stack soberana</footer>\
         </body>\
         </html>",
        name = platform::NAME,
        generated = generated,
        region = platform::PRIMARY_REGION,
        version = avila::VERSION,
        entries = entries,
    );

    Ok(html)
}

fn human_timestamp() -> String {
    match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(duration) => {
            let seconds = duration.as_secs();
            let hours = seconds / 3600;
            let minutes = (seconds / 60) % 60;
            let secs = seconds % 60;
            format!("{}h:{:02}m:{:02}s desde 1970", hours, minutes, secs)
        }
        Err(_) => "Tempo desconhecido".to_string(),
    }
}

fn open_in_browser(path: &Path) {
    let display_path = match path.canonicalize() {
        Ok(p) => p,
        Err(_) => path.to_path_buf(),
    };
    let display_string = display_path.to_string_lossy().into_owned();

    let open_result = if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(["/C", "start", "", display_string.as_str()])
            .spawn()
    } else if cfg!(target_os = "macos") {
        Command::new("open").arg(&display_path).spawn()
    } else {
        Command::new("xdg-open").arg(&display_path).spawn()
    };

    match open_result {
        Ok(_) => println!("Abrindo navegador padrão..."),
        Err(err) => println!(
            "Não foi possível abrir automaticamente o arquivo ({}). Abra manualmente o caminho informado.",
            err
        ),
    }
}
