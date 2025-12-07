use std::path::PathBuf;
use std::process;
use env_logger;

fn main() {
    env_logger::init();

    println!("🏗️  Avila BIM Server");
    println!("====================");
    println!("Version: 0.1.0");
    println!("Runtime: Single-threaded demo mode");
    println!();

    // Parse CLI args
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        print_usage();
        process::exit(1);
    }

    match args[1].as_str() {
        "info" => {
            println!("📦 Avila BIM - Building Information Modeling Engine");
            println!();
            println!("Supported formats:");
            println!("  - IFC (Industry Foundation Classes)");
            println!("  - DWG (AutoCAD Drawing)");
            println!("  - DXF (Drawing Exchange Format)");
            println!("  - glTF (GL Transmission Format)");
            println!("  - OBJ, STL, PLY, FBX");
            println!("  - SKP (SketchUp)");
            println!("  - RVT (Revit)");
            println!("  - NWD (Navisworks)");
            println!();
            println!("Features:");
            println!("  - Spatial indexing (BVH, Octree)");
            println!("  - Collision detection");
            println!("  - Geometry validation");
            println!("  - Material caching");
            println!();
        }
        "convert" => {
            if args.len() < 4 {
                eprintln!("❌ Usage: avila-bim-server convert <input> <output>");
                process::exit(1);
            }
            let input = PathBuf::from(&args[2]);
            let output = PathBuf::from(&args[3]);

            println!("🔄 Converting file...");
            println!("   Input:  {}", input.display());
            println!("   Output: {}", output.display());
            println!();
            println!("⚠️  Conversion not yet implemented (requires lib features)");
        }
        "server" => {
            println!("🚀 Starting HTTP server...");
            println!("   Address: http://0.0.0.0:8080");
            println!();
            println!("⚠️  Server mode requires 'server' feature");
            println!("   Compile with: cargo build --features server");
        }
        _ => {
            print_usage();
            process::exit(1);
        }
    }
}

fn print_usage() {
    println!("Usage: avila-bim-server <command> [args]");
    println!();
    println!("Commands:");
    println!("  info              Show supported formats and features");
    println!("  convert <in> <out>  Convert BIM file");
    println!("  server            Start HTTP API server");
}
