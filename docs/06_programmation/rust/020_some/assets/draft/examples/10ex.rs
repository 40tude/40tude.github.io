// use std::path::PathBuf;

fn main() {
    // Moving consumes the Option
    let opt = Some(String::from("hello"));
    if let Some(s) = opt {
        println!("Length: {}", s.len());
    }
    // println!("{:?}", opt); // ERROR: opt was moved

    // Borrowing with as_ref => Option<T> remains usable
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
