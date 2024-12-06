fn main() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    let first = vec[0];
    println!("First element: {}", first);
    //this will cause an error because we try to access a non-existent index
    let invalid_index = vec[10];
    println!("Invalid index: {}", invalid_index);
}