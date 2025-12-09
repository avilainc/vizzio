use avila_ifc::{IfcFile, IfcQuery, SpatialQuery};
use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <ifc_file>", args[0]);
        process::exit(1);
    }

    let file_path = &args[1];

    println!("Opening IFC file: {}", file_path);
    println!();

    // Open and parse the IFC file
    let ifc_file = match IfcFile::open(file_path) {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Error opening IFC file: {}", e);
            process::exit(1);
        }
    };

    // Print file information
    println!("File: {}", ifc_file.file_name());
    println!("Schema: {}", ifc_file.schema().unwrap_or("Unknown"));
    println!("System: {}", ifc_file.originating_system());
    println!();

    // Print statistics
    let stats = ifc_file.statistics();
    println!("{}", stats);
    println!();

    // Get project information
    if let Some(project) = ifc_file.get_project() {
        if let Ok(Some(name)) = project.get_string_attribute(2) {
            println!("Project: {}", name);
        }
    }

    // List buildings
    println!("\nBuildings:");
    for building in ifc_file.get_buildings() {
        if let Ok(Some(name)) = building.get_string_attribute(2) {
            println!("  - {}", name);
        }
    }

    // List building storeys
    println!("\nBuilding Storeys:");
    for storey in ifc_file.get_building_storeys() {
        if let Ok(Some(name)) = storey.get_string_attribute(2) {
            let elevation = storey
                .get_real_attribute(9)
                .ok()
                .flatten()
                .unwrap_or(0.0);
            println!("  - {} (Elevation: {:.2})", name, elevation);
        }
    }

    // Query examples
    println!("\nQuery Examples:");

    // Find walls containing "P" in name
    let walls_with_p = IfcQuery::new(&ifc_file)
        .entity_type("IFCWALL")
        .name_contains("P")
        .execute();
    println!("Walls containing 'P': {}", walls_with_p.len());

    // Count all structural elements
    let spatial_query = SpatialQuery::new(&ifc_file);
    let structural = spatial_query.all_structural_elements();
    println!("Total structural elements: {}", structural.len());

    // List all entity types
    println!("\nAll Entity Types:");
    let mut types = ifc_file.get_entity_types();
    types.sort();
    for entity_type in types {
        let count = ifc_file.count_entities_by_type(&entity_type);
        if count > 0 {
            println!("  {} ({})", entity_type, count);
        }
    }
}
