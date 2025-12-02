fn expensive_computation() -> String {
    println!("Computing default value...");
    "DEFAULT_NAME".to_string()
}

fn main() {
    // ----------------------------------------------------
    // ----------------------------------------------------
    let none_name: Option<String> = None;

    // unwrap_or: value is ALWAYS computed (eager evaluation)
    let name1 = none_name.clone().unwrap_or(expensive_computation());
    // Output: "Computing default value..."

    // unwrap_or_else: closure called ONLY if None (lazy evaluation)
    let name2 = none_name.unwrap_or_else(|| expensive_computation());
    // let name2 = expensive_computation();
    // Output: "Computing default value..."

    // ----------------------------------------------------
    // ----------------------------------------------------
    // Compare with Some
    let some_name: Option<String> = Some("Alice".to_string());

    // unwrap_or: value is ALWAYS computed (eager evaluation)
    let name3 = some_name.clone().unwrap_or(expensive_computation());
    // Still prints "Computing default value..." => Waste of time

    // unwrap_or_else: closure called ONLY if None (lazy evaluation)
    let name4 = some_name.unwrap_or_else(|| expensive_computation());
    // let name4 = "Alice".to_string();
    // The closure not called

    println!("Results: {name1}, {name2}, {name3}, {name4}");
}
