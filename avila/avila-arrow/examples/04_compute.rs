//! Compute operations on arrays

fn main() {
    println!("=== Compute Operations Example ===\n");

    let array1: Vec<i32> = vec![1, 2, 3, 4, 5];
    let array2: Vec<i32> = vec![5, 4, 3, 2, 1];

    // Addition
    let sum: Vec<i32> = array1.iter()
        .zip(array2.iter())
        .map(|(a, b)| a + b)
        .collect();
    println!("Addition: {:?} + {:?} = {:?}", array1, array2, sum);

    // Multiplication
    let product: Vec<i32> = array1.iter()
        .zip(array2.iter())
        .map(|(a, b)| a * b)
        .collect();
    println!("Multiplication: {:?} * {:?} = {:?}", array1, array2, product);

    // Comparison
    let comparison: Vec<bool> = array1.iter()
        .zip(array2.iter())
        .map(|(a, b)| a > b)
        .collect();
    println!("Comparison (>): {:?} > {:?} = {:?}", array1, array2, comparison);

    // Aggregation
    let total: i32 = array1.iter().sum();
    let max: i32 = *array1.iter().max().unwrap();
    println!("\nAggregation:");
    println!("  Sum: {}", total);
    println!("  Max: {}", max);

    println!("\n✓ Successfully performed compute operations");
}
