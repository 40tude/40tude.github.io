fn get_first_char(s: Option<&str>) -> Option<char> {
    let text = s?; // If None, return None immediately
    text.chars().next()
}

// Without ? - verbose equivalent
fn get_first_char_verbose(s: Option<&str>) -> Option<char> {
    match s {
        Some(text) => text.chars().next(),
        None => None,
    }
}

// Chaining multiple ?
fn get_second_char(s: Option<&str>) -> Option<char> {
    let text = s?;
    let mut chars = text.chars();
    chars.next()?; // Skip first
    chars.next() // Return second
}

fn main() {
    println!("{:?}", get_first_char_verbose(Some("hello"))); // Some('h')
    println!("{:?}\n", get_first_char_verbose(None)); // None

    println!("{:?}", get_first_char(Some("hello"))); // Some('h')
    println!("{:?}\n", get_first_char(None)); // None

    println!("{:?}", get_second_char(Some("hi"))); // Some('i')
    println!("{:?}", get_second_char(Some("x"))); // None (only 1 char)
    println!("{:?}", get_second_char(None)); // None
}
