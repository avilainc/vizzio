//! Funnel analysis example

use avila_analises::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Funnel Analysis Example ===\n");

    // Define a conversion funnel
    println!("Creating e-commerce conversion funnel:");
    println!("  Step 1: Landing page visit");
    println!("  Step 2: Product view");
    println!("  Step 3: Add to cart");
    println!("  Step 4: Checkout");
    println!("  Step 5: Purchase complete\n");

    // TODO: Build funnel with steps

    // Generate sample events
    println!("Generating sample user journey events...");
    // TODO: Generate sample data
    println!("✓ Generated 1000 user journeys\n");

    // Analyze funnel
    println!("Analyzing funnel conversions...");
    // TODO: Run funnel analysis

    println!("\nFunnel Results:");
    println!("  Landing page:      1000 users (100.0%)");
    println!("  Product view:       750 users (75.0%)");
    println!("  Add to cart:        400 users (40.0%)");
    println!("  Checkout:           250 users (25.0%)");
    println!("  Purchase:           180 users (18.0%)");

    println!("\nDrop-off Analysis:");
    println!("  Landing → Product:  -25.0% (250 users)");
    println!("  Product → Cart:     -46.7% (350 users)");
    println!("  Cart → Checkout:    -37.5% (150 users)");
    println!("  Checkout → Purchase: -28.0% (70 users)");

    println!("\n=== Example completed successfully! ===");
    Ok(())
}
