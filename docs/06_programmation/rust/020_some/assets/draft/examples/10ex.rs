use std::path::PathBuf;

fn main() {
    // Moving consumes the Option
    let opt = Some(String::from("hello"));
    if let Some(s) = opt {
        println!("Length: {}", s.len());
    }
    // println!("{:?}", opt); // ERROR: opt was moved

    // Borrowing with as_ref - Option remains usable
    let opt = Some(String::from("hello"));

    if let Some(s) = opt.as_ref() {
        // s is &String
        println!("Length: {}", s.len());
    }
    println!("{:?}", opt); // Ah, ha, ha, ha, opt stayin' alive, stayin' alive (Some("hello")

    if let Some(my_str) = &opt {
        // my_str is &String
        println!("Length: {}", my_str.len());
    }
    println!("{:?}", opt); // Ah, ha, ha, ha, opt stayin' alive, stayin' alive (Some("hello")

    // as_ref() is useful with map - read without consuming
    let mut path = Some(PathBuf::from("/home"));
    let len = path.as_ref().map(|p| p.as_os_str().len());
    println!("Path length: {:?}", len); // Some(5)

    // as_mut is useful with map - modify in place
    path.as_mut().map(|p| p.push("user"));
    // if let Some(p) = path.as_mut() {
    //     p.push("user");
    // }
    println!("{:?}", path); // Some("/home/user")
}
