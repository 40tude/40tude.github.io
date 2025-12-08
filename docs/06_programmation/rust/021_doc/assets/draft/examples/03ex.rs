// cargo run --example 02ex
fn main() {
    let words = vec!["hello", "world", "rust"];

    let result: Vec<String> = words
        // .into_iter()
        .iter()
        .map(|word| word.to_uppercase())
        .collect();

    println!("Uppercase: {:?}", result);

    // Now let's try to use the original vector
    for word in words {
        println!("Original: {}", word);
    }
}
