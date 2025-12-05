// cargo run --example 10ex

fn main() {
    // i32 implements Copy, so Option<i32> also implements Copy
    let opt = Some(42);

    // Pattern matching copies opt instead of moving it
    if let Some(n) = opt {
        println!("{n}"); // n is copied from the Option
    }
    println!("{:?}", opt); // OK: opt was copied, not moved

    println!();
    // String does NOT implement Copy, so Option<String> does not implement Copy either
    let opt = Some(String::from("hello"));
    // Pattern matching moves opt and its inner String
    if let Some(s) = opt {
        // s is moved out of opt
        println!("Length: {}", s.len());
    }
    // println!("{:?}", opt); // ERROR: opt was moved and cannot be used here

    // Borrowing with as_ref => Option<T> remains usable afterwards
    println!();
    let opt = Some(String::from("hello"));
    if let Some(s) = opt.as_ref() {
        println!("Length: {}", s.len());
    }
    println!("{:?}", opt); // Ah, ha, ha, ha, stayin' alive, stayin' alive

    println!();
    // this express the same intention as `as_ref`
    if let Some(my_str) = &opt {
        println!("Length: {}", my_str.len());
    }
    println!("{:?}", opt); // Ah, ha, ha, ha, stayin' alive, stayin' alive

    println!();
    let mut path = Some(std::env::current_dir().expect("Cannot read current dir"));

    // as_ref() is useful with map - read without consuming
    let len = path.as_ref().map(|p| p.as_os_str().len());
    println!("The path {:?} has a length of {:?}", path, len);

    // as_mut is useful with map - modify in place
    path.as_mut().map(|p| p.push("documents"));
    path.as_mut().map(|p| p.push("top_secret"));

    println!("The path is now: {:?}", path);
}
