// cargo run --example 04ex

#[derive(Debug, Clone)]
struct HeavyData {
    name: String,
    value: i32,
    // Imagine: large arrays, nested structures, etc.
}

fn main() {
    let numbers = vec![Some(1), Some(15), Some(25), None, Some(5)];

    // .iter(): yields &Option<i32>
    // .map():  |&opt| pattern matches &Option<i32>, binds opt as Option<i32> (copy of inner value)
    //          opt.filter() receives opt: Option<i32>, returns Option<i32>
    // .filter: |&n| pattern matches &i32 (from Some variant), binds n as i32
    let filtered: Vec<Option<i32>> = numbers.iter().map(|&opt| opt.filter(|&n| n > 10)).collect();
    println!("Raw numbers: {:?}", numbers);
    println!("Filtered   : {:?}", filtered);

    //
    //
    //
    // .iter(): yields &Option<i32>
    // .map():  |opt| binds opt as &Option<i32> (reference, no pattern matching)
    //          opt.filter() receives opt: &Option<i32>, calls Option::filter() on reference
    // .filter: |&n| pattern matches &i32 (from Some variant), binds n as i32
    let filtered: Vec<Option<i32>> = numbers.iter().map(|opt| opt.filter(|&n| n > 10)).collect();
    println!("Raw numbers: {:?}", numbers);
    println!("Filtered   : {:?}", filtered);

    //
    //
    //
    // .iter(): yields &Option<i32>
    // .map():  |opt| binds opt as &Option<i32> (reference, no pattern matching)
    //          opt.filter() receives opt: &Option<i32>, calls Option::filter() on reference
    // .filter: |n| binds n as &i32 (reference, no pattern matching), requires *n to dereference
    let filtered: Vec<Option<i32>> = numbers.iter().map(|opt| opt.filter(|n| *n > 10)).collect();
    println!("Raw numbers: {:?}", numbers);
    println!("Filtered   : {:?}", filtered);

    //
    //
    //
    let my_strings = vec![
        Some("riri".to_string()),
        Some("fifi".to_string()),
        Some("loulou".to_string()),
        None,
        Some("donald".to_string()),
    ];

    // .iter(): yields &Option<String>
    // .map():  |opt| binds opt as &Option<String> (reference, no pattern matching)
    //          opt.clone() creates Option<String> (owned value, String is NOT Copy)
    //          .filter() receives Option<String>, returns Option<String>
    // .filter: |current_str| binds current_str as &String (reference from Some variant)
    //          current_str.len() auto-derefs &String to access len()
    let filtered: Vec<Option<String>> = my_strings
        .iter()
        .map(|opt| opt.clone().filter(|current_str| (*current_str).len() > 4))
        .collect();
    println!();
    println!("Raw Strings: {:?}", my_strings);
    println!("Filtered   : {:?}", filtered);

    //
    //
    //
    // Heavy structures example - filtering without cloning everything
    let heavy_data = vec![
        Some(HeavyData {
            name: "Alice".to_string(),
            value: 10,
        }),
        Some(HeavyData {
            name: "Bob".to_string(),
            value: 50,
        }),
        None,
        Some(HeavyData {
            name: "Charlie".to_string(),
            value: 100,
        }),
        Some(HeavyData {
            name: "Diana".to_string(),
            value: 30,
        }),
    ];

    // INEFFICIENT: clones all structures just to filter
    // .iter(): yields &Option<HeavyData>
    // .map():  opt.clone() creates Option<HeavyData> (clones EVERY structure!)
    // .filter: checks data.value > 42, but already cloned everything
    let filtered_cloned: Vec<Option<HeavyData>> = heavy_data
        .iter()
        .map(|opt| opt.clone().filter(|data| data.value > 42))
        .collect();
    println!();
    println!("Inefficient (clone all): {:?}", filtered_cloned);

    // EFFICIENT: filter first with references, clone only what passes
    // .iter():    yields &Option<HeavyData>
    // .filter():  checks condition on reference, no cloning yet
    // .map():     only clones items that passed the filter
    let filtered_smart: Vec<Option<HeavyData>> = heavy_data
        .iter()
        .filter(|opt| opt.as_ref().map_or(false, |data| data.value > 42))
        .map(|opt| opt.clone())
        .collect();
    println!("Efficient (filter first): {:?}", filtered_smart);

    // ALTERNATIVE: work with references only (no cloning at all)
    // Returns Vec<Option<&HeavyData>> instead of owned data
    let filtered_refs: Vec<Option<&HeavyData>> = heavy_data
        .iter()
        .map(|opt| opt.as_ref().filter(|data| data.value > 42))
        .collect();
    println!("Zero-clone (references): {:?}", filtered_refs);
}
