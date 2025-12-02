// cargo run --example 03ex

use std::path::PathBuf;

struct Editor {
    path_to_file: Option<PathBuf>,
}

fn save_file(editor: &Editor) {
    // Extract file name from path, converting OsStr to String
    let my_file_name = match &editor.path_to_file {
        // Some(path) => path.file_name().map(|s| s.to_string_lossy().to_string()),
        Some(path) => path.file_name(),
        None => return,
    };

    if let Some(name) = my_file_name {
        println!("Saving file: {:?}", name);
    }
}

fn main() {
    let mut editor = Editor {
        // Uncomment one to test both cases
        path_to_file: Some(PathBuf::from(r"tmp/my_file.txt")),
        // path_to_file: None,
    };
    save_file(&editor);

    editor.path_to_file = Some(PathBuf::from(r"tmp/my_file2.txt"));
    save_file(&editor);
}
