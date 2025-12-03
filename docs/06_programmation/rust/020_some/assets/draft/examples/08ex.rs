// cargo run --example 08ex

fn parse_positive(s: &str) -> Option<i32> {
    s.parse::<i32>().ok().filter(|&n| n > 0)
}

fn main() {
    let input = Some("42");

    // map creates nested Option<Option<i32>>
    let bad = input.map(|s| parse_positive(s));
    println!("{:?}", bad); // Some(Some(42)) - awkward!

    // and_then flattens automatically
    let good = input.and_then(|s| parse_positive(s));
    println!("{:?}", good); // Some(42) - clean!

    // Chaining multiple and_then
    let chain_result = Some("100")
        .and_then(|s| parse_positive(s)) // Some(100)
        .and_then(|n| if n < 50 { Some(n * 2) } else { None });

    println!("{:?}", chain_result); // None (100 >= 50)
}
