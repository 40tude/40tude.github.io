// cargo run --example 02_2ex
use std::path::PathBuf;

struct Editor {
    path_to_file: Option<PathBuf>,
}

impl Editor {
    fn set_path(&mut self, path: PathBuf) {
        self.path_to_file = Some(path);
    }
}

fn main() {
    let mut my_editor = Editor { path_to_file: None };

    // Uncomment one to test both cases
    let new_path = Some(PathBuf::from(r"tmp/my_file.txt"));
    // let new_path = None;

    if let Some(path) = new_path {
        my_editor.set_path(path);
    }

    println!("The file: {:?}", my_editor.path_to_file);
}
