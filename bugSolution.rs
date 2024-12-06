fn main() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    let first = vec[0];
    println!("First element: {}", first);
    // Check for valid index before access
    if vec.len() > 10 {
        let valid_index = vec[10];
        println!("Valid index: {}", valid_index);
    } else {
        println!("Index out of bounds");
    }
} 