// cargo run --example 01ex

struct Editor {}

impl Editor {
    fn get_selection(&self) -> Option<String> {
        // Simulate selection: uncomment one to test both cases
        Some("lorem ipsum".to_string())
        // None
    }
}

fn main() {
    let my_editor = Editor {};

    let selection = my_editor.get_selection();
    if selection.is_some() {
        println!("Selection: {}", selection.unwrap());
    } else {
        println!("No text selected");
    }
}
