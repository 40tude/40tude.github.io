// cargo run --example 12_2ex

fn main() {
    let numbers = vec![Some(1), Some(15), Some(25), None, Some(5)];

    // Filter keeps only Some(v) where the predicate is true
    let filtered: Vec<Option<i32>> = numbers.iter().map(|&opt| opt.filter(|&n| n > 10)).collect();
    println!("Raw numbers: {:?}", numbers); // [Some(1), Some(15), Some(25), None, Some(5)]
    println!("Filtered   : {:?}", filtered); // [None,   Some(15), Some(25), None, None]

    // Filter keeps only Some(v) where the predicate is true
    let filtered: Vec<Option<i32>> = numbers.iter().map(|opt| opt.filter(|&n| n > 10)).collect();
    println!("Raw numbers: {:?}", numbers); // [Some(1), Some(15), Some(25), None, Some(5)]
    println!("Filtered   : {:?}", filtered); // [None,   Some(15), Some(25), None, None]

    // Filter keeps only Some(v) where the predicate is true
    let filtered: Vec<Option<i32>> = numbers.iter().map(|opt| opt.filter(|n| *n > 10)).collect();
    println!("Raw numbers: {:?}", numbers); // [Some(1), Some(15), Some(25), None, Some(5)]
    println!("Filtered   : {:?}", filtered); // [None,   Some(15), Some(25), None, None]
}
