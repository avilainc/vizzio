//! Basic array creation and manipulation

fn main() {
    println!("=== Basic Arrays Example ===\n");

    // Create an Int32 array
    let int_array: Vec<i32> = vec![1, 2, 3, 4, 5];
    println!("Int32 Array: {:?}", int_array);

    // Create a Float64 array
    let float_array: Vec<f64> = vec![1.1, 2.2, 3.3, 4.4, 5.5];
    println!("Float64 Array: {:?}", float_array);

    // Create a String array
    let string_array: Vec<String> = vec!["hello".to_string(), "world".to_string()];
    println!("String Array: {:?}", string_array);

    // Create a Boolean array
    let bool_array: Vec<bool> = vec![true, false, true, false];
    println!("Boolean Array: {:?}", bool_array);

    println!("\n✓ Successfully created basic arrays");
}
