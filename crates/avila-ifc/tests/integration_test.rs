use avila_ifc::{IfcFile, Result};

#[test]
fn test_parse_real_ifc_file() -> Result<()> {
    // Test with the actual IFC file in the workspace
    let test_file = "../../VZZ086_25 Magnussão - Res. Heitor - Estrutural Executivo - Rev08.ifc";

    // Try to open the file
    let ifc_file = IfcFile::open(test_file)?;

    // Verify basic file properties
    assert!(ifc_file.total_entities() > 0);
    assert_eq!(ifc_file.schema(), Some("IFC2X3"));

    // Check for expected entities
    let projects = ifc_file.get_entities_by_type("IFCPROJECT");
    assert!(!projects.is_empty(), "Should have at least one project");

    let buildings = ifc_file.get_entities_by_type("IFCBUILDING");
    assert!(!buildings.is_empty(), "Should have at least one building");

    // Test statistics
    let stats = ifc_file.statistics();
    assert!(stats.total_entities > 0);
    assert_eq!(stats.schema, "IFC2X3");

    println!("Successfully parsed IFC file!");
    println!("{}", stats);

    Ok(())
}

#[test]
fn test_query_building_storeys() -> Result<()> {
    let test_file = "../../VZZ086_25 Magnussão - Res. Heitor - Estrutural Executivo - Rev08.ifc";
    let ifc_file = IfcFile::open(test_file)?;

    let storeys = ifc_file.get_building_storeys();

    println!("\nBuilding Storeys found:");
    for storey in &storeys {
        if let Ok(Some(name)) = storey.get_string_attribute(2) {
            let elevation = storey
                .get_real_attribute(9)
                .ok()
                .flatten()
                .unwrap_or(0.0);
            println!("  - {} (Elevation: {:.2} mm)", name, elevation);
        }
    }

    assert!(
        !storeys.is_empty(),
        "Expected to find building storeys in the file"
    );

    Ok(())
}

#[test]
fn test_structural_elements() -> Result<()> {
    let test_file = "../../VZZ086_25 Magnussão - Res. Heitor - Estrutural Executivo - Rev08.ifc";
    let ifc_file = IfcFile::open(test_file)?;

    use avila_ifc::SpatialQuery;
    let spatial = SpatialQuery::new(&ifc_file);

    let walls = spatial.all_walls();
    let beams = spatial.all_beams();
    let columns = spatial.all_columns();
    let slabs = spatial.all_slabs();

    println!("\nStructural Elements:");
    println!("  Walls: {}", walls.len());
    println!("  Beams: {}", beams.len());
    println!("  Columns: {}", columns.len());
    println!("  Slabs: {}", slabs.len());

    Ok(())
}

#[test]
fn test_geometry_extraction() -> Result<()> {
    let test_file = "../../VZZ086_25 Magnussão - Res. Heitor - Estrutural Executivo - Rev08.ifc";
    let ifc_file = IfcFile::open(test_file)?;

    use avila_ifc::GeometryProcessor;
    let geom = GeometryProcessor::new(&ifc_file);

    // Get all cartesian points
    let points = ifc_file.get_entities_by_type("IFCCARTESIANPOINT");
    assert!(!points.is_empty(), "Should have cartesian points");

    // Try to extract coordinates from first point
    if let Some(point) = points.first() {
        let coords = geom.get_cartesian_point(point)?;
        println!("\nFirst point coordinates: ({:.2}, {:.2}, {:.2})",
                 coords.x, coords.y, coords.z);
    }

    Ok(())
}

#[test]
fn test_query_by_name() -> Result<()> {
    let test_file = "../../VZZ086_25 Magnussão - Res. Heitor - Estrutural Executivo - Rev08.ifc";
    let ifc_file = IfcFile::open(test_file)?;

    use avila_ifc::IfcQuery;

    // Search for entities with "Piscina" in name
    let results = IfcQuery::new(&ifc_file)
        .name_contains("Piscina")
        .execute();

    println!("\nEntities with 'Piscina' in name:");
    for entity in &results {
        if let Ok(Some(name)) = entity.get_string_attribute(2) {
            println!("  - {} ({})", entity.entity_type, name);
        }
    }

    Ok(())
}

#[test]
fn test_file_header_info() -> Result<()> {
    let test_file = "../../VZZ086_25 Magnussão - Res. Heitor - Estrutural Executivo - Rev08.ifc";
    let ifc_file = IfcFile::open(test_file)?;

    println!("\nFile Header Information:");
    println!("  File Name: {}", ifc_file.file_name());
    println!("  Schema: {}", ifc_file.schema().unwrap_or("Unknown"));
    println!("  Originating System: {}", ifc_file.originating_system());

    assert!(ifc_file.file_name().contains("VZZ086"));
    assert!(ifc_file.originating_system().contains("Revit"));

    Ok(())
}
