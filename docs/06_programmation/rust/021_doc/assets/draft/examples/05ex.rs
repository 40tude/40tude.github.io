// cargo run --example 05ex

#[derive(Debug, Clone)]
struct HeavyData {
    name: String,
    value: i32,
}

fn main() {
    let heavy_data = [
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
