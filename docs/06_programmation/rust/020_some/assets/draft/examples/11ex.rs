use std::fs::{self, File};

struct Editor {
    file: Option<File>,
}

impl Editor {
    fn is_open(&self) -> bool {
        self.file.is_some()
    }

    fn close(&mut self) {
        if let Some(_f) = self.file.take() {
            // _f is File, self.file is now None automatically
            println!("Closing file");
            // _f is dropped and the file is automatically closed at the end of the block
        }
    }
}

fn main() {
    let mut editor = Editor {
        file: Some(File::create("temp.txt").expect("Failed to create temp.txt")),
    };
    println!("Is open: {}", editor.is_open()); // true
    editor.close();
    println!("Is open: {}", editor.is_open()); // false

    // Clean up: remove temp.txt if it exists
    if fs::metadata("temp.txt").is_ok() {
        fs::remove_file("temp.txt").expect("Failed to delete temp.txt");
        println!("temp.txt deleted");
    }
}
