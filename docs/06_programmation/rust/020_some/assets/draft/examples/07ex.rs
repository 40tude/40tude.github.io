// cargo run --example 07ex

fn main() {
    let name: Option<String> = Some("  Zoubida  ".to_string());

    // Chain transformations - only applied if Some
    let result = name
        .map(|n| n.trim().to_string()) // Some("Zoubida")
        .map(|n| n.to_uppercase()) // Some("ZOUBIDA")
        .unwrap_or_else(|| "ANONYMOUS".to_string());
    println!("{}", result); // "ZOUBIDA"

    // With None - transformations skipped, default used
    let no_name: Option<String> = None;
    let result2 = no_name
        .map(|n| n.trim().to_string())
        .map(|n| n.to_uppercase())
        .unwrap_or_else(|| "ANONYMOUS".to_string());
    println!("{}", result2); // "ANONYMOUS"
}
