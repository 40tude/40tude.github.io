// cargo run --example 06ex

// Simulates a costly function (time/CPU/network) to generate a default value
// Prints a trace to show when it is executed
fn expensive_computation() -> String {
    println!("\tComputing a default value for 10 seconds...");
    "DEFAULT_NAME".to_string()
}

fn main() {
    println!("\n--- PART 1: Where default is NOT needed");
    let some_name: Option<String> = Some("Zoubida".into());

    // 1.1: .unwrap_or_else() (LAZY Evaluation)
    // The closure '|| expensive_computation()' is called ONLY IF 'some_name' is None (not the case here)
    // This avoids the expensive operation.
    // NO [Computing...] message is printed.
    // This is the correct, efficient approach when dealing with Some.
    println!("About to call .unwrap_or_else():");
    let _name4 = some_name.clone().unwrap_or_else(|| expensive_computation());
    println!("\tResult after .unwrap_or_else() on Some: {_name4}");

    // 1.2: .unwrap_or() (EAGER Evaluation)
    // The argument 'expensive_computation()'  is calculated first, regardless of whether 'some_name' is None or Some.
    // The [Computing...] message is printed, the returned value is thrown away
    // This is a wasted computation
    println!("About to call .unwrap_or():");
    let _name3 = some_name.unwrap_or(expensive_computation());
    println!("\tResult after .unwrap_or() on Some: {_name3}");

    println!("\n\n--- PART 2: Where default is NEEDED");
    let none_name: Option<String> = None;

    // 2.1: .unwrap_or_else() (LAZY Evaluation)
    // The closure '|| expensive_computation()' is called ONLY IF 'none_name' is None.
    // Since it is the case here, the [Computing...] message is printed and the DEFAULT_NAME is used
    println!("About to call .unwrap_or_else():");
    let _name2 = none_name.clone().unwrap_or_else(|| expensive_computation());
    println!("\tResult after .unwrap_or_else() on None: {_name2}");
    // Output: [COMPUTATION] ... AND Result: DEFAULT_NAME

    // 2.2: .unwrap_or() (EAGER Evaluation)
    // The argument 'expensive_computation()' is calculated first, regardless of whether 'none_name' is None or Some.
    // The [Computing...] message is printed.
    println!("About to call .unwrap_or():");
    let _name1 = none_name.unwrap_or(expensive_computation());
    println!("\tResult after .unwrap_or() on None: {_name1}");
}
