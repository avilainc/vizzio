//! Scientific types example

fn main() {
    println!("=== Scientific Types Example ===\n");

    // Quaternion
    println!("Quaternion (for 3D rotations):");
    let q = (1.0, 0.0, 0.0, 0.0); // w, x, y, z
    println!("  q = {} + {}i + {}j + {}k", q.0, q.1, q.2, q.3);

    // Complex numbers
    println!("\nComplex Number:");
    let c = (3.0, 4.0); // real, imaginary
    let magnitude = (c.0 * c.0 + c.1 * c.1).sqrt();
    println!("  c = {} + {}i", c.0, c.1);
    println!("  |c| = {}", magnitude);

    // Tensor4D
    println!("\nTensor4D (4-dimensional array):");
    let shape = [2, 3, 4, 5];
    println!("  Shape: {:?}", shape);
    println!("  Total elements: {}", shape.iter().product::<usize>());

    // Physical constants
    println!("\nPhysical Constants:");
    println!("  Speed of light: 2.998e8 m/s");
    println!("  Planck constant: 6.626e-34 J⋅s");
    println!("  Electron mass: 9.109e-31 kg");

    println!("\n✓ Successfully demonstrated scientific types");
}
