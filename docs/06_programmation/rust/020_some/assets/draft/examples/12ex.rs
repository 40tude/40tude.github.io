// cargo run --example 12ex

fn main() {
    let numbers = vec![Some(1), Some(15), Some(25), None, Some(5)];

    // Filter keeps only Some(v) where the predicate is true
    let filtered: Vec<Option<i32>> = numbers.iter().map(|opt| opt.filter(|n| *n > 10)).collect();
    println!("Raw numbers: {:?}", numbers); // [Some(1), Some(15), Some(25), None, Some(5)]
    println!("Filtered   : {:?}", filtered); // [None, Some(15), Some(25), None, None]

    // Combining with map
    let name = Some("  Zoubida  ");
    let result = name
        .map(|n| n.trim())
        .filter(|n| !n.is_empty()) // Keep only if not empty after trim
        .map(|n| n.to_uppercase());
    println!("{:?}", result); // Some("ZOUBIDA")

    // Filter out invalid values
    let maybe_age = Some(-5);
    let valid_age = maybe_age.filter(|&age| age >= 0 && age <= 150);
    println!("{:?}", valid_age); // None (negative age rejected)
}
