//! Basic collections usage examples

use avila_atom::{DynamicArray, AssociativeArray, StringBuffer, map, list};

fn main() {
    println!("=== avila-atom Basic Collections Examples ===\n");

    // DynamicArray (Vec) example
    dynamic_array_example();

    // AssociativeArray (HashMap/BTreeMap) example
    associative_array_example();

    // StringBuffer example
    string_buffer_example();

    // Macros example
    macros_example();
}

fn dynamic_array_example() {
    println!("--- DynamicArray ---");

    let mut vec = DynamicArray::new();
    vec.push(1);
    vec.push(2);
    vec.push(3);

    println!("Vector: {:?}", vec);
    println!("Length: {}", vec.len());
    println!("Capacity: {}", vec.capacity());

    // Iteration
    print!("Elements: ");
    for item in &vec {
        print!("{} ", item);
    }
    println!("\n");
}

fn associative_array_example() {
    println!("--- AssociativeArray ---");

    let mut map = AssociativeArray::new();
    map.insert("name", "Alice");
    map.insert("age", "30");
    map.insert("city", "NYC");

    println!("Map: {:?}", map);
    println!("Name: {:?}", map.get("name"));
    println!("Age: {:?}", map.get("age"));
    println!("Length: {}", map.len());
    println!();
}

fn string_buffer_example() {
    println!("--- StringBuffer ---");

    let mut s = StringBuffer::from("Hello");
    s.push_str(", ");
    s.push_str("World!");

    println!("String: {}", s);
    println!("Length: {}", s.len());
    println!("Uppercase: {}", s.to_uppercase());
    println!();
}

fn macros_example() {
    println!("--- Convenient Macros ---");

    // map! macro
    let m = map! {
        "one" => 1,
        "two" => 2,
        "three" => 3,
    };
    println!("Map from macro: {:?}", m);

    // list! macro
    let v = list![10, 20, 30, 40, 50];
    println!("List from macro: {:?}", v);
    println!();
}
