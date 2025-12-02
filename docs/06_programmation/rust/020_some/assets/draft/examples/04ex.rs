// cargo run --example 04ex

use std::path::PathBuf;

struct Editor {
    path_to_file: Option<PathBuf>,
}

fn save_file(editor: &Editor) {
    let Some(my_path) = &editor.path_to_file else {
        eprintln!("No path to file available");
        return;
    };

    if let Some(name) = my_path.file_name().map(|s| s.to_string_lossy().to_string()) {
        println!("Saving file: {}", name);
    }
}

fn main() {
    let mut editor = Editor {
        // Uncomment one to test both cases
        // path_to_file: Some(PathBuf::from(r"tmp/my_file.txt")),
        path_to_file: None,
    };
    save_file(&editor);

    editor.path_to_file = Some(PathBuf::from(r"tmp/my_file2.txt"));
    save_file(&editor);
}
